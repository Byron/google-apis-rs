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
    /// When this is false, bid requests that include a deal ID for a private auction or preferred deal are always sent to your bidder. When true, all active pretargeting configs will be applied to private auctions and preferred deals. Programmatic Guaranteed deals (when enabled) are always sent to your bidder.
    #[serde(rename="applyPretargetingToNonGuaranteedDeals")]
    
    pub apply_pretargeting_to_non_guaranteed_deals: Option<bool>,
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


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert marketplacedeals](MarketplacedealInsertCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddOrderDealsRequest {
    /// The list of deals to add
    
    pub deals: Option<Vec<MarketplaceDeal>>,
    /// The last known proposal revision number.
    #[serde(rename="proposalRevisionNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub proposal_revision_number: Option<i64>,
    /// Indicates an optional action to take on the proposal
    #[serde(rename="updateAction")]
    
    pub update_action: Option<String>,
}

impl client::RequestValue for AddOrderDealsRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert marketplacedeals](MarketplacedealInsertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddOrderDealsResponse {
    /// List of deals added (in the same proposal as passed in the request)
    
    pub deals: Option<Vec<MarketplaceDeal>>,
    /// The updated revision number for the proposal.
    #[serde(rename="proposalRevisionNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub proposal_revision_number: Option<i64>,
}

impl client::ResponseResult for AddOrderDealsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert marketplacenotes](MarketplacenoteInsertCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddOrderNotesRequest {
    /// The list of notes to add.
    
    pub notes: Option<Vec<MarketplaceNote>>,
}

impl client::RequestValue for AddOrderNotesRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert marketplacenotes](MarketplacenoteInsertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddOrderNotesResponse {
    /// no description provided
    
    pub notes: Option<Vec<MarketplaceNote>>,
}

impl client::ResponseResult for AddOrderNotesResponse {}


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


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Buyer {
    /// Adx account id of the buyer.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
}

impl client::Part for Buyer {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactInformation {
    /// Email address of the contact.
    
    pub email: Option<String>,
    /// The name of the contact.
    
    pub name: Option<String>,
}

impl client::Part for ContactInformation {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert proposals](ProposalInsertCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateOrdersRequest {
    /// The list of proposals to create.
    
    pub proposals: Option<Vec<Proposal>>,
    /// Web property id of the seller creating these orders
    #[serde(rename="webPropertyCode")]
    
    pub web_property_code: Option<String>,
}

impl client::RequestValue for CreateOrdersRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert proposals](ProposalInsertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateOrdersResponse {
    /// The list of proposals successfully created.
    
    pub proposals: Option<Vec<Proposal>>,
}

impl client::ResponseResult for CreateOrdersResponse {}


/// A creative and its classification data.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add deal creatives](CreativeAddDealCall) (none)
/// * [get creatives](CreativeGetCall) (response)
/// * [insert creatives](CreativeInsertCall) (request|response)
/// * [list creatives](CreativeListCall) (none)
/// * [list deals creatives](CreativeListDealCall) (none)
/// * [remove deal creatives](CreativeRemoveDealCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Creative {
    /// The HTML snippet that displays the ad when inserted in the web page. If set, videoURL, videoVastXML, and nativeAd should not be set.
    #[serde(rename="HTMLSnippet")]
    
    pub html_snippet: Option<String>,
    /// Account id.
    #[serde(rename="accountId")]
    
    pub account_id: Option<i32>,
    /// The link to the Ad Preferences page. This is only supported for native ads.
    #[serde(rename="adChoicesDestinationUrl")]
    
    pub ad_choices_destination_url: Option<String>,
    /// no description provided
    #[serde(rename="adTechnologyProviders")]
    
    pub ad_technology_providers: Option<CreativeAdTechnologyProviders>,
    /// Detected advertiser id, if any. Read-only. This field should not be set in requests.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub advertiser_id: Option<Vec<i64>>,
    /// The name of the company being advertised in the creative. A list of advertisers is provided in the advertisers.txt file.
    #[serde(rename="advertiserName")]
    
    pub advertiser_name: Option<String>,
    /// The agency id for this creative.
    #[serde(rename="agencyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub agency_id: Option<i64>,
    /// The last upload timestamp of this creative if it was uploaded via API. Read-only. The value of this field is generated, and will be ignored for uploads. (formatted RFC 3339 timestamp).
    #[serde(rename="apiUploadTimestamp")]
    
    pub api_upload_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// List of buyer selectable attributes for the ads that may be shown from this snippet. Each attribute is represented by an integer as defined in  buyer-declarable-creative-attributes.txt.
    
    pub attribute: Option<Vec<i32>>,
    /// A buyer-specific id identifying the creative in this ad.
    #[serde(rename="buyerCreativeId")]
    
    pub buyer_creative_id: Option<String>,
    /// The set of destination urls for the snippet.
    #[serde(rename="clickThroughUrl")]
    
    pub click_through_url: Option<Vec<String>>,
    /// Shows any corrections that were applied to this creative. Read-only. This field should not be set in requests.
    
    pub corrections: Option<Vec<CreativeCorrections>>,
    /// Creative status identity type that the creative item applies to. Ad Exchange real-time bidding is migrating to the sizeless creative verification. Originally, Ad Exchange assigned creative verification status to a unique combination of a buyer creative ID and creative dimensions. Post-migration, a single verification status will be assigned at the buyer creative ID level. This field allows to distinguish whether a given creative status applies to a unique combination of a buyer creative ID and creative dimensions, or to a buyer creative ID as a whole.
    #[serde(rename="creativeStatusIdentityType")]
    
    pub creative_status_identity_type: Option<String>,
    /// Top-level deals status. Read-only. This field should not be set in requests. If disapproved, an entry for auctionType=DIRECT_DEALS (or ALL) in servingRestrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case it may be preferable to read from servingRestrictions directly.
    #[serde(rename="dealsStatus")]
    
    pub deals_status: Option<String>,
    /// Detected domains for this creative. Read-only. This field should not be set in requests.
    #[serde(rename="detectedDomains")]
    
    pub detected_domains: Option<Vec<String>>,
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
    /// Detected languages for this creative. Read-only. This field should not be set in requests.
    
    pub languages: Option<Vec<String>>,
    /// If nativeAd is set, HTMLSnippet, videoVastXML, and the videoURL outside of nativeAd should not be set. (The videoURL inside nativeAd can be set.)
    #[serde(rename="nativeAd")]
    
    pub native_ad: Option<CreativeNativeAd>,
    /// Top-level open auction status. Read-only. This field should not be set in requests. If disapproved, an entry for auctionType=OPEN_AUCTION (or ALL) in servingRestrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case it may be preferable to read from ServingRestrictions directly.
    #[serde(rename="openAuctionStatus")]
    
    pub open_auction_status: Option<String>,
    /// Detected product categories, if any. Each category is represented by an integer as defined in  ad-product-categories.txt. Read-only. This field should not be set in requests.
    #[serde(rename="productCategories")]
    
    pub product_categories: Option<Vec<i32>>,
    /// All restricted categories for the ads that may be shown from this snippet. Each category is represented by an integer as defined in the  ad-restricted-categories.txt.
    #[serde(rename="restrictedCategories")]
    
    pub restricted_categories: Option<Vec<i32>>,
    /// Detected sensitive categories, if any. Each category is represented by an integer as defined in  ad-sensitive-categories.txt. Read-only. This field should not be set in requests.
    #[serde(rename="sensitiveCategories")]
    
    pub sensitive_categories: Option<Vec<i32>>,
    /// The granular status of this ad in specific contexts. A context here relates to where something ultimately serves (for example, a physical location, a platform, an HTTPS vs HTTP request, or the type of auction). Read-only. This field should not be set in requests. See the examples in the Creatives guide for more details.
    #[serde(rename="servingRestrictions")]
    
    pub serving_restrictions: Option<Vec<CreativeServingRestrictions>>,
    /// List of vendor types for the ads that may be shown from this snippet. Each vendor type is represented by an integer as defined in vendors.txt.
    #[serde(rename="vendorType")]
    
    pub vendor_type: Option<Vec<i32>>,
    /// The version for this creative. Read-only. This field should not be set in requests.
    
    pub version: Option<i32>,
    /// The URL to fetch a video ad. If set, HTMLSnippet, videoVastXML, and nativeAd should not be set. Note, this is different from resource.native_ad.video_url above.
    #[serde(rename="videoURL")]
    
    pub video_url: Option<String>,
    /// The contents of a VAST document for a video ad. This document should conform to the VAST 2.0 or 3.0 standard. If set, HTMLSnippet, videoURL, and nativeAd and should not be set.
    #[serde(rename="videoVastXML")]
    
    pub video_vast_xml: Option<String>,
    /// Ad width.
    
    pub width: Option<i32>,
}

impl client::RequestValue for Creative {}
impl client::Resource for Creative {}
impl client::ResponseResult for Creative {}


/// The external deal ids associated with a creative.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list deals creatives](CreativeListDealCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeDealIds {
    /// A list of external deal ids and ARC approval status.
    #[serde(rename="dealStatuses")]
    
    pub deal_statuses: Option<Vec<CreativeDealIdsDealStatuses>>,
    /// Resource type.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for CreativeDealIds {}


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


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DealServingMetadata {
    /// True if alcohol ads are allowed for this deal (read-only). This field is only populated when querying for finalized orders using the method GetFinalizedOrderDeals
    #[serde(rename="alcoholAdsAllowed")]
    
    pub alcohol_ads_allowed: Option<bool>,
    /// Tracks which parties (if any) have paused a deal. (readonly, except via PauseResumeOrderDeals action)
    #[serde(rename="dealPauseStatus")]
    
    pub deal_pause_status: Option<DealServingMetadataDealPauseStatus>,
}

impl client::Part for DealServingMetadata {}


/// Tracks which parties (if any) have paused a deal. The deal is considered paused if has_buyer_paused || has_seller_paused. Each of the has_buyer_paused or the has_seller_paused bits can be set independently.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DealServingMetadataDealPauseStatus {
    /// no description provided
    #[serde(rename="buyerPauseReason")]
    
    pub buyer_pause_reason: Option<String>,
    /// If the deal is paused, records which party paused the deal first.
    #[serde(rename="firstPausedBy")]
    
    pub first_paused_by: Option<String>,
    /// no description provided
    #[serde(rename="hasBuyerPaused")]
    
    pub has_buyer_paused: Option<bool>,
    /// no description provided
    #[serde(rename="hasSellerPaused")]
    
    pub has_seller_paused: Option<bool>,
    /// no description provided
    #[serde(rename="sellerPauseReason")]
    
    pub seller_pause_reason: Option<String>,
}

impl client::Part for DealServingMetadataDealPauseStatus {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DealTerms {
    /// Visibility of the URL in bid requests.
    #[serde(rename="brandingType")]
    
    pub branding_type: Option<String>,
    /// Indicates that this ExternalDealId exists under at least two different AdxInventoryDeals. Currently, the only case that the same ExternalDealId will exist is programmatic cross sell case.
    #[serde(rename="crossListedExternalDealIdType")]
    
    pub cross_listed_external_deal_id_type: Option<String>,
    /// Description for the proposed terms of the deal.
    
    pub description: Option<String>,
    /// Non-binding estimate of the estimated gross spend for this deal Can be set by buyer or seller.
    #[serde(rename="estimatedGrossSpend")]
    
    pub estimated_gross_spend: Option<Price>,
    /// Non-binding estimate of the impressions served per day Can be set by buyer or seller.
    #[serde(rename="estimatedImpressionsPerDay")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub estimated_impressions_per_day: Option<i64>,
    /// The terms for guaranteed fixed price deals.
    #[serde(rename="guaranteedFixedPriceTerms")]
    
    pub guaranteed_fixed_price_terms: Option<DealTermsGuaranteedFixedPriceTerms>,
    /// The terms for non-guaranteed auction deals.
    #[serde(rename="nonGuaranteedAuctionTerms")]
    
    pub non_guaranteed_auction_terms: Option<DealTermsNonGuaranteedAuctionTerms>,
    /// The terms for non-guaranteed fixed price deals.
    #[serde(rename="nonGuaranteedFixedPriceTerms")]
    
    pub non_guaranteed_fixed_price_terms: Option<DealTermsNonGuaranteedFixedPriceTerms>,
    /// The terms for rubicon non-guaranteed deals.
    #[serde(rename="rubiconNonGuaranteedTerms")]
    
    pub rubicon_non_guaranteed_terms: Option<DealTermsRubiconNonGuaranteedTerms>,
    /// For deals with Cost Per Day billing, defines the timezone used to mark the boundaries of a day (buyer-readonly)
    #[serde(rename="sellerTimeZone")]
    
    pub seller_time_zone: Option<String>,
}

impl client::Part for DealTerms {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DealTermsGuaranteedFixedPriceTerms {
    /// External billing info for this Deal. This field is relevant when external billing info such as price has a different currency code than DFP/AdX.
    #[serde(rename="billingInfo")]
    
    pub billing_info: Option<DealTermsGuaranteedFixedPriceTermsBillingInfo>,
    /// Fixed price for the specified buyer.
    #[serde(rename="fixedPrices")]
    
    pub fixed_prices: Option<Vec<PricePerBuyer>>,
    /// Guaranteed impressions as a percentage. This is the percentage of guaranteed looks that the buyer is guaranteeing to buy.
    #[serde(rename="guaranteedImpressions")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub guaranteed_impressions: Option<i64>,
    /// Count of guaranteed looks. Required for deal, optional for product. For CPD deals, buyer changes to guaranteed_looks will be ignored.
    #[serde(rename="guaranteedLooks")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub guaranteed_looks: Option<i64>,
    /// Count of minimum daily looks for a CPD deal. For CPD deals, buyer should negotiate on this field instead of guaranteed_looks.
    #[serde(rename="minimumDailyLooks")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub minimum_daily_looks: Option<i64>,
}

impl client::Part for DealTermsGuaranteedFixedPriceTerms {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DealTermsGuaranteedFixedPriceTermsBillingInfo {
    /// The timestamp (in ms since epoch) when the original reservation price for the deal was first converted to DFP currency. This is used to convert the contracted price into buyer's currency without discrepancy.
    #[serde(rename="currencyConversionTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub currency_conversion_time_ms: Option<i64>,
    /// The DFP line item id associated with this deal. For features like CPD, buyers can retrieve the DFP line item for billing reconciliation.
    #[serde(rename="dfpLineItemId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub dfp_line_item_id: Option<i64>,
    /// The original contracted quantity (# impressions) for this deal. To ensure delivery, sometimes the publisher will book the deal with a impression buffer, such that guaranteed_looks is greater than the contracted quantity. However clients are billed using the original contracted quantity.
    #[serde(rename="originalContractedQuantity")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub original_contracted_quantity: Option<i64>,
    /// The original reservation price for the deal, if the currency code is different from the one used in negotiation.
    
    pub price: Option<Price>,
}

impl client::Part for DealTermsGuaranteedFixedPriceTermsBillingInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DealTermsNonGuaranteedAuctionTerms {
    /// True if open auction buyers are allowed to compete with invited buyers in this private auction (buyer-readonly).
    #[serde(rename="autoOptimizePrivateAuction")]
    
    pub auto_optimize_private_auction: Option<bool>,
    /// Reserve price for the specified buyer.
    #[serde(rename="reservePricePerBuyers")]
    
    pub reserve_price_per_buyers: Option<Vec<PricePerBuyer>>,
}

impl client::Part for DealTermsNonGuaranteedAuctionTerms {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DealTermsNonGuaranteedFixedPriceTerms {
    /// Fixed price for the specified buyer.
    #[serde(rename="fixedPrices")]
    
    pub fixed_prices: Option<Vec<PricePerBuyer>>,
}

impl client::Part for DealTermsNonGuaranteedFixedPriceTerms {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DealTermsRubiconNonGuaranteedTerms {
    /// Optional price for Rubicon priority access in the auction.
    #[serde(rename="priorityPrice")]
    
    pub priority_price: Option<Price>,
    /// Optional price for Rubicon standard access in the auction.
    #[serde(rename="standardPrice")]
    
    pub standard_price: Option<Price>,
}

impl client::Part for DealTermsRubiconNonGuaranteedTerms {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete marketplacedeals](MarketplacedealDeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteOrderDealsRequest {
    /// List of deals to delete for a given proposal
    #[serde(rename="dealIds")]
    
    pub deal_ids: Option<Vec<String>>,
    /// The last known proposal revision number.
    #[serde(rename="proposalRevisionNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub proposal_revision_number: Option<i64>,
    /// Indicates an optional action to take on the proposal
    #[serde(rename="updateAction")]
    
    pub update_action: Option<String>,
}

impl client::RequestValue for DeleteOrderDealsRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete marketplacedeals](MarketplacedealDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteOrderDealsResponse {
    /// List of deals deleted (in the same proposal as passed in the request)
    
    pub deals: Option<Vec<MarketplaceDeal>>,
    /// The updated revision number for the proposal.
    #[serde(rename="proposalRevisionNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub proposal_revision_number: Option<i64>,
}

impl client::ResponseResult for DeleteOrderDealsResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryControl {
    /// no description provided
    #[serde(rename="creativeBlockingLevel")]
    
    pub creative_blocking_level: Option<String>,
    /// no description provided
    #[serde(rename="deliveryRateType")]
    
    pub delivery_rate_type: Option<String>,
    /// no description provided
    #[serde(rename="frequencyCaps")]
    
    pub frequency_caps: Option<Vec<DeliveryControlFrequencyCap>>,
}

impl client::Part for DeliveryControl {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryControlFrequencyCap {
    /// no description provided
    #[serde(rename="maxImpressions")]
    
    pub max_impressions: Option<i32>,
    /// no description provided
    #[serde(rename="numTimeUnits")]
    
    pub num_time_units: Option<i32>,
    /// no description provided
    #[serde(rename="timeUnitType")]
    
    pub time_unit_type: Option<String>,
}

impl client::Part for DeliveryControlFrequencyCap {}


/// This message carries publisher provided breakdown. E.g. {dimension_type: 'COUNTRY', [{dimension_value: {id: 1, name: 'US'}}, {dimension_value: {id: 2, name: 'UK'}}]}
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dimension {
    /// no description provided
    #[serde(rename="dimensionType")]
    
    pub dimension_type: Option<String>,
    /// no description provided
    #[serde(rename="dimensionValues")]
    
    pub dimension_values: Option<Vec<DimensionDimensionValue>>,
}

impl client::Part for Dimension {}


/// Value of the dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionDimensionValue {
    /// Id of the dimension.
    
    pub id: Option<i32>,
    /// Name of the dimension mainly for debugging purposes, except for the case of CREATIVE_SIZE. For CREATIVE_SIZE, strings are used instead of ids.
    
    pub name: Option<String>,
    /// Percent of total impressions for a dimension type. e.g. {dimension_type: 'GENDER', [{dimension_value: {id: 1, name: 'MALE', percentage: 60}}]} Gender MALE is 60% of all impressions which have gender.
    
    pub percentage: Option<i32>,
}

impl client::Part for DimensionDimensionValue {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update marketplacedeals](MarketplacedealUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EditAllOrderDealsRequest {
    /// List of deals to edit. Service may perform 3 different operations based on comparison of deals in this list vs deals already persisted in database: 1. Add new deal to proposal If a deal in this list does not exist in the proposal, the service will create a new deal and add it to the proposal. Validation will follow AddOrderDealsRequest. 2. Update existing deal in the proposal If a deal in this list already exist in the proposal, the service will update that existing deal to this new deal in the request. Validation will follow UpdateOrderDealsRequest. 3. Delete deals from the proposal (just need the id) If a existing deal in the proposal is not present in this list, the service will delete that deal from the proposal. Validation will follow DeleteOrderDealsRequest.
    
    pub deals: Option<Vec<MarketplaceDeal>>,
    /// If specified, also updates the proposal in the batch transaction. This is useful when the proposal and the deals need to be updated in one transaction.
    
    pub proposal: Option<Proposal>,
    /// The last known revision number for the proposal.
    #[serde(rename="proposalRevisionNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub proposal_revision_number: Option<i64>,
    /// Indicates an optional action to take on the proposal
    #[serde(rename="updateAction")]
    
    pub update_action: Option<String>,
}

impl client::RequestValue for EditAllOrderDealsRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update marketplacedeals](MarketplacedealUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EditAllOrderDealsResponse {
    /// List of all deals in the proposal after edit.
    
    pub deals: Option<Vec<MarketplaceDeal>>,
    /// The latest revision number after the update has been applied.
    #[serde(rename="orderRevisionNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub order_revision_number: Option<i64>,
}

impl client::ResponseResult for EditAllOrderDealsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search products](ProductSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetOffersResponse {
    /// The returned list of products.
    
    pub products: Option<Vec<Product>>,
}

impl client::ResponseResult for GetOffersResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list marketplacedeals](MarketplacedealListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetOrderDealsResponse {
    /// List of deals for the proposal
    
    pub deals: Option<Vec<MarketplaceDeal>>,
}

impl client::ResponseResult for GetOrderDealsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list marketplacenotes](MarketplacenoteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetOrderNotesResponse {
    /// The list of matching notes. The notes for a proposal are ordered from oldest to newest. If the notes span multiple proposals, they will be grouped by proposal, with the notes for the most recently modified proposal appearing first.
    
    pub notes: Option<Vec<MarketplaceNote>>,
}

impl client::ResponseResult for GetOrderNotesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search proposals](ProposalSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetOrdersResponse {
    /// The list of matching proposals.
    
    pub proposals: Option<Vec<Proposal>>,
}

impl client::ResponseResult for GetOrdersResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list pubprofiles](PubprofileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetPublisherProfilesByAccountIdResponse {
    /// Profiles for the requested publisher
    
    pub profiles: Option<Vec<PublisherProfileApiProto>>,
}

impl client::ResponseResult for GetPublisherProfilesByAccountIdResponse {}


/// A proposal can contain multiple deals. A deal contains the terms and targeting information that is used for serving.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MarketplaceDeal {
    /// Buyer private data (hidden from seller).
    #[serde(rename="buyerPrivateData")]
    
    pub buyer_private_data: Option<PrivateData>,
    /// The time (ms since epoch) of the deal creation. (readonly)
    #[serde(rename="creationTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creation_time_ms: Option<i64>,
    /// Specifies the creative pre-approval policy (buyer-readonly)
    #[serde(rename="creativePreApprovalPolicy")]
    
    pub creative_pre_approval_policy: Option<String>,
    /// Specifies whether the creative is safeFrame compatible (buyer-readonly)
    #[serde(rename="creativeSafeFrameCompatibility")]
    
    pub creative_safe_frame_compatibility: Option<String>,
    /// A unique deal-id for the deal (readonly).
    #[serde(rename="dealId")]
    
    pub deal_id: Option<String>,
    /// Metadata about the serving status of this deal (readonly, writes via custom actions)
    #[serde(rename="dealServingMetadata")]
    
    pub deal_serving_metadata: Option<DealServingMetadata>,
    /// The set of fields around delivery control that are interesting for a buyer to see but are non-negotiable. These are set by the publisher. This message is assigned an id of 100 since some day we would want to model this as a protobuf extension.
    #[serde(rename="deliveryControl")]
    
    pub delivery_control: Option<DeliveryControl>,
    /// The external deal id assigned to this deal once the deal is finalized. This is the deal-id that shows up in serving/reporting etc. (readonly)
    #[serde(rename="externalDealId")]
    
    pub external_deal_id: Option<String>,
    /// Proposed flight end time of the deal (ms since epoch) This will generally be stored in a granularity of a second. (updatable)
    #[serde(rename="flightEndTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub flight_end_time_ms: Option<i64>,
    /// Proposed flight start time of the deal (ms since epoch) This will generally be stored in a granularity of a second. (updatable)
    #[serde(rename="flightStartTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub flight_start_time_ms: Option<i64>,
    /// Description for the deal terms. (buyer-readonly)
    #[serde(rename="inventoryDescription")]
    
    pub inventory_description: Option<String>,
    /// Indicates whether the current deal is a RFP template. RFP template is created by buyer and not based on seller created products.
    #[serde(rename="isRfpTemplate")]
    
    pub is_rfp_template: Option<bool>,
    /// True, if the buyside inventory setup is complete for this deal. (readonly, except via OrderSetupCompleted action)
    #[serde(rename="isSetupComplete")]
    
    pub is_setup_complete: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "adexchangebuyer#marketplaceDeal".
    
    pub kind: Option<String>,
    /// The time (ms since epoch) when the deal was last updated. (readonly)
    #[serde(rename="lastUpdateTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_update_time_ms: Option<i64>,
    /// no description provided
    #[serde(rename="makegoodRequestedReason")]
    
    pub makegood_requested_reason: Option<String>,
    /// The name of the deal. (updatable)
    
    pub name: Option<String>,
    /// The product-id from which this deal was created. (readonly, except on create)
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The revision number of the product that the deal was created from (readonly, except on create)
    #[serde(rename="productRevisionNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub product_revision_number: Option<i64>,
    /// Specifies the creative source for programmatic deals, PUBLISHER means creative is provided by seller and ADVERTISR means creative is provided by buyer. (buyer-readonly)
    #[serde(rename="programmaticCreativeSource")]
    
    pub programmatic_creative_source: Option<String>,
    /// no description provided
    #[serde(rename="proposalId")]
    
    pub proposal_id: Option<String>,
    /// Optional Seller contact information for the deal (buyer-readonly)
    #[serde(rename="sellerContacts")]
    
    pub seller_contacts: Option<Vec<ContactInformation>>,
    /// The shared targeting visible to buyers and sellers. Each shared targeting entity is AND'd together. (updatable)
    #[serde(rename="sharedTargetings")]
    
    pub shared_targetings: Option<Vec<SharedTargeting>>,
    /// The syndication product associated with the deal. (readonly, except on create)
    #[serde(rename="syndicationProduct")]
    
    pub syndication_product: Option<String>,
    /// The negotiable terms of the deal. (updatable)
    
    pub terms: Option<DealTerms>,
    /// no description provided
    #[serde(rename="webPropertyCode")]
    
    pub web_property_code: Option<String>,
}

impl client::Part for MarketplaceDeal {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MarketplaceDealParty {
    /// The buyer/seller associated with the deal. One of buyer/seller is specified for a deal-party.
    
    pub buyer: Option<Buyer>,
    /// The buyer/seller associated with the deal. One of buyer/seller is specified for a deal party.
    
    pub seller: Option<Seller>,
}

impl client::Part for MarketplaceDealParty {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MarketplaceLabel {
    /// The accountId of the party that created the label.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The creation time (in ms since epoch) for the label.
    #[serde(rename="createTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub create_time_ms: Option<i64>,
    /// Information about the party that created the label.
    #[serde(rename="deprecatedMarketplaceDealParty")]
    
    pub deprecated_marketplace_deal_party: Option<MarketplaceDealParty>,
    /// The label to use.
    
    pub label: Option<String>,
}

impl client::Part for MarketplaceLabel {}


/// A proposal is associated with a bunch of notes which may optionally be associated with a deal and/or revision number.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MarketplaceNote {
    /// The role of the person (buyer/seller) creating the note. (readonly)
    #[serde(rename="creatorRole")]
    
    pub creator_role: Option<String>,
    /// Notes can optionally be associated with a deal. (readonly, except on create)
    #[serde(rename="dealId")]
    
    pub deal_id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "adexchangebuyer#marketplaceNote".
    
    pub kind: Option<String>,
    /// The actual note to attach. (readonly, except on create)
    
    pub note: Option<String>,
    /// The unique id for the note. (readonly)
    #[serde(rename="noteId")]
    
    pub note_id: Option<String>,
    /// The proposalId that a note is attached to. (readonly)
    #[serde(rename="proposalId")]
    
    pub proposal_id: Option<String>,
    /// If the note is associated with a proposal revision number, then store that here. (readonly, except on create)
    #[serde(rename="proposalRevisionNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub proposal_revision_number: Option<i64>,
    /// The timestamp (ms since epoch) that this note was created. (readonly)
    #[serde(rename="timestampMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub timestamp_ms: Option<i64>,
}

impl client::Part for MarketplaceNote {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MobileApplication {
    /// no description provided
    #[serde(rename="appStore")]
    
    pub app_store: Option<String>,
    /// no description provided
    #[serde(rename="externalAppId")]
    
    pub external_app_id: Option<String>,
}

impl client::Part for MobileApplication {}


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
    /// Requests where the predicted viewability is below the specified decile will not match. E.g. if the buyer sets this value to 5, requests from slots where the predicted viewability is below 50% will not match. If the predicted viewability is unknown this field will be ignored.
    #[serde(rename="minimumViewabilityDecile")]
    
    pub minimum_viewability_decile: Option<i32>,
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
    /// Requests containing the specified type of user data will match. Possible values are HOSTED_MATCH_DATA, which means the request is cookie-targetable and has a match in the buyer's hosted match table, and COOKIE_OR_IDFA, which means the request has either a targetable cookie or an iOS IDFA.
    #[serde(rename="userIdentifierDataRequired")]
    
    pub user_identifier_data_required: Option<Vec<String>>,
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
    /// Video requests satisfying any of these player size constraints will match.
    #[serde(rename="videoPlayerSizes")]
    
    pub video_player_sizes: Option<Vec<PretargetingConfigVideoPlayerSizes>>,
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


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Price {
    /// The price value in micros.
    #[serde(rename="amountMicros")]
    
    pub amount_micros: Option<f64>,
    /// The currency code for the price.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// In case of CPD deals, the expected CPM in micros.
    #[serde(rename="expectedCpmMicros")]
    
    pub expected_cpm_micros: Option<f64>,
    /// The pricing type for the deal/product.
    #[serde(rename="pricingType")]
    
    pub pricing_type: Option<String>,
}

impl client::Part for Price {}


/// Used to specify pricing rules for buyers. Each PricePerBuyer in a product can become [0,1] deals. To check if there is a PricePerBuyer for a particular buyer we look for the most specific matching rule - we first look for a rule matching the buyer and otherwise look for a matching rule where no buyer is set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PricePerBuyer {
    /// Optional access type for this buyer.
    #[serde(rename="auctionTier")]
    
    pub auction_tier: Option<String>,
    /// Reference to the buyer that will get billed.
    #[serde(rename="billedBuyer")]
    
    pub billed_buyer: Option<Buyer>,
    /// The buyer who will pay this price. If unset, all buyers can pay this price (if the advertisers match, and there's no more specific rule matching the buyer).
    
    pub buyer: Option<Buyer>,
    /// The specified price
    
    pub price: Option<Price>,
}

impl client::Part for PricePerBuyer {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateData {
    /// no description provided
    #[serde(rename="referenceId")]
    
    pub reference_id: Option<String>,
    /// no description provided
    #[serde(rename="referencePayload")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub reference_payload: Option<Vec<u8>>,
}

impl client::Part for PrivateData {}


/// A product is segment of inventory that a seller wishes to sell. It is associated with certain terms and targeting information which helps buyer know more about the inventory. Each field in a product can have one of the following setting:
/// 
/// (readonly) - It is an error to try and set this field. (buyer-readonly) - Only the seller can set this field. (seller-readonly) - Only the buyer can set this field. (updatable) - The field is updatable at all times by either buyer or the seller.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get products](ProductGetCall) (response)
/// * [search products](ProductSearchCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    /// The billed buyer corresponding to the buyer that created the offer. (readonly, except on create)
    #[serde(rename="billedBuyer")]
    
    pub billed_buyer: Option<Buyer>,
    /// The buyer that created the offer if this is a buyer initiated offer (readonly, except on create)
    
    pub buyer: Option<Buyer>,
    /// Creation time in ms. since epoch (readonly)
    #[serde(rename="creationTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creation_time_ms: Option<i64>,
    /// Optional contact information for the creator of this product. (buyer-readonly)
    #[serde(rename="creatorContacts")]
    
    pub creator_contacts: Option<Vec<ContactInformation>>,
    /// The role that created the offer. Set to BUYER for buyer initiated offers.
    #[serde(rename="creatorRole")]
    
    pub creator_role: Option<String>,
    /// The set of fields around delivery control that are interesting for a buyer to see but are non-negotiable. These are set by the publisher. This message is assigned an id of 100 since some day we would want to model this as a protobuf extension.
    #[serde(rename="deliveryControl")]
    
    pub delivery_control: Option<DeliveryControl>,
    /// The proposed end time for the deal (ms since epoch) (buyer-readonly)
    #[serde(rename="flightEndTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub flight_end_time_ms: Option<i64>,
    /// Inventory availability dates. (times are in ms since epoch) The granularity is generally in the order of seconds. (buyer-readonly)
    #[serde(rename="flightStartTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub flight_start_time_ms: Option<i64>,
    /// If the creator has already signed off on the product, then the buyer can finalize the deal by accepting the product as is. When copying to a proposal, if any of the terms are changed, then auto_finalize is automatically set to false.
    #[serde(rename="hasCreatorSignedOff")]
    
    pub has_creator_signed_off: Option<bool>,
    /// What exchange will provide this inventory (readonly, except on create).
    #[serde(rename="inventorySource")]
    
    pub inventory_source: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "adexchangebuyer#product".
    
    pub kind: Option<String>,
    /// Optional List of labels for the product (optional, buyer-readonly).
    
    pub labels: Option<Vec<MarketplaceLabel>>,
    /// Time of last update in ms. since epoch (readonly)
    #[serde(rename="lastUpdateTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_update_time_ms: Option<i64>,
    /// Optional legacy offer id if this offer is a preferred deal offer.
    #[serde(rename="legacyOfferId")]
    
    pub legacy_offer_id: Option<String>,
    /// Marketplace publisher profile Id. This Id differs from the regular publisher_profile_id in that 1. This is a new id, the old Id will be deprecated in 2017. 2. This id uniquely identifies a publisher profile by itself.
    #[serde(rename="marketplacePublisherProfileId")]
    
    pub marketplace_publisher_profile_id: Option<String>,
    /// The name for this product as set by the seller. (buyer-readonly)
    
    pub name: Option<String>,
    /// Optional private auction id if this offer is a private auction offer.
    #[serde(rename="privateAuctionId")]
    
    pub private_auction_id: Option<String>,
    /// The unique id for the product (readonly)
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// Id of the publisher profile for a given seller. A (seller.account_id, publisher_profile_id) pair uniquely identifies a publisher profile. Buyers can call the PublisherProfiles::List endpoint to get a list of publisher profiles for a given seller.
    #[serde(rename="publisherProfileId")]
    
    pub publisher_profile_id: Option<String>,
    /// Publisher self-provided forecast information.
    #[serde(rename="publisherProvidedForecast")]
    
    pub publisher_provided_forecast: Option<PublisherProvidedForecast>,
    /// The revision number of the product. (readonly)
    #[serde(rename="revisionNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub revision_number: Option<i64>,
    /// Information about the seller that created this product (readonly, except on create)
    
    pub seller: Option<Seller>,
    /// Targeting that is shared between the buyer and the seller. Each targeting criteria has a specified key and for each key there is a list of inclusion value or exclusion values. (buyer-readonly)
    #[serde(rename="sharedTargetings")]
    
    pub shared_targetings: Option<Vec<SharedTargeting>>,
    /// The state of the product. (buyer-readonly)
    
    pub state: Option<String>,
    /// The syndication product associated with the deal. (readonly, except on create)
    #[serde(rename="syndicationProduct")]
    
    pub syndication_product: Option<String>,
    /// The negotiable terms of the deal (buyer-readonly)
    
    pub terms: Option<DealTerms>,
    /// The web property code for the seller. This field is meant to be copied over as is when creating deals.
    #[serde(rename="webPropertyCode")]
    
    pub web_property_code: Option<String>,
}

impl client::Resource for Product {}
impl client::ResponseResult for Product {}


/// Represents a proposal in the marketplace. A proposal is the unit of negotiation between a seller and a buyer and contains deals which are served. Each field in a proposal can have one of the following setting:
/// 
/// (readonly) - It is an error to try and set this field. (buyer-readonly) - Only the seller can set this field. (seller-readonly) - Only the buyer can set this field. (updatable) - The field is updatable at all times by either buyer or the seller.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get proposals](ProposalGetCall) (response)
/// * [insert proposals](ProposalInsertCall) (none)
/// * [patch proposals](ProposalPatchCall) (request|response)
/// * [search proposals](ProposalSearchCall) (none)
/// * [setupcomplete proposals](ProposalSetupcompleteCall) (none)
/// * [update proposals](ProposalUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Proposal {
    /// Reference to the buyer that will get billed for this proposal. (readonly)
    #[serde(rename="billedBuyer")]
    
    pub billed_buyer: Option<Buyer>,
    /// Reference to the buyer on the proposal. (readonly, except on create)
    
    pub buyer: Option<Buyer>,
    /// Optional contact information of the buyer. (seller-readonly)
    #[serde(rename="buyerContacts")]
    
    pub buyer_contacts: Option<Vec<ContactInformation>>,
    /// Private data for buyer. (hidden from seller).
    #[serde(rename="buyerPrivateData")]
    
    pub buyer_private_data: Option<PrivateData>,
    /// IDs of DBM advertisers permission to this proposal.
    #[serde(rename="dbmAdvertiserIds")]
    
    pub dbm_advertiser_ids: Option<Vec<String>>,
    /// When an proposal is in an accepted state, indicates whether the buyer has signed off. Once both sides have signed off on a deal, the proposal can be finalized by the seller. (seller-readonly)
    #[serde(rename="hasBuyerSignedOff")]
    
    pub has_buyer_signed_off: Option<bool>,
    /// When an proposal is in an accepted state, indicates whether the buyer has signed off Once both sides have signed off on a deal, the proposal can be finalized by the seller. (buyer-readonly)
    #[serde(rename="hasSellerSignedOff")]
    
    pub has_seller_signed_off: Option<bool>,
    /// What exchange will provide this inventory (readonly, except on create).
    #[serde(rename="inventorySource")]
    
    pub inventory_source: Option<String>,
    /// True if the proposal is being renegotiated (readonly).
    #[serde(rename="isRenegotiating")]
    
    pub is_renegotiating: Option<bool>,
    /// True, if the buyside inventory setup is complete for this proposal. (readonly, except via OrderSetupCompleted action) Deprecated in favor of deal level setup complete flag.
    #[serde(rename="isSetupComplete")]
    
    pub is_setup_complete: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "adexchangebuyer#proposal".
    
    pub kind: Option<String>,
    /// List of labels associated with the proposal. (readonly)
    
    pub labels: Option<Vec<MarketplaceLabel>>,
    /// The role of the last user that either updated the proposal or left a comment. (readonly)
    #[serde(rename="lastUpdaterOrCommentorRole")]
    
    pub last_updater_or_commentor_role: Option<String>,
    /// The name for the proposal (updatable)
    
    pub name: Option<String>,
    /// Optional negotiation id if this proposal is a preferred deal proposal.
    #[serde(rename="negotiationId")]
    
    pub negotiation_id: Option<String>,
    /// Indicates whether the buyer/seller created the proposal.(readonly)
    #[serde(rename="originatorRole")]
    
    pub originator_role: Option<String>,
    /// Optional private auction id if this proposal is a private auction proposal.
    #[serde(rename="privateAuctionId")]
    
    pub private_auction_id: Option<String>,
    /// The unique id of the proposal. (readonly).
    #[serde(rename="proposalId")]
    
    pub proposal_id: Option<String>,
    /// The current state of the proposal. (readonly)
    #[serde(rename="proposalState")]
    
    pub proposal_state: Option<String>,
    /// The revision number for the proposal (readonly).
    #[serde(rename="revisionNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub revision_number: Option<i64>,
    /// The time (ms since epoch) when the proposal was last revised (readonly).
    #[serde(rename="revisionTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub revision_time_ms: Option<i64>,
    /// Reference to the seller on the proposal. (readonly, except on create)
    
    pub seller: Option<Seller>,
    /// Optional contact information of the seller (buyer-readonly).
    #[serde(rename="sellerContacts")]
    
    pub seller_contacts: Option<Vec<ContactInformation>>,
}

impl client::RequestValue for Proposal {}
impl client::Resource for Proposal {}
impl client::ResponseResult for Proposal {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublisherProfileApiProto {
    /// Publisher provided info on its audience.
    
    pub audience: Option<String>,
    /// A pitch statement for the buyer
    #[serde(rename="buyerPitchStatement")]
    
    pub buyer_pitch_statement: Option<String>,
    /// Direct contact for the publisher profile.
    #[serde(rename="directContact")]
    
    pub direct_contact: Option<String>,
    /// Exchange where this publisher profile is from. E.g. AdX, Rubicon etc...
    
    pub exchange: Option<String>,
    /// Link to publisher's Google+ page.
    #[serde(rename="googlePlusLink")]
    
    pub google_plus_link: Option<String>,
    /// True, if this is the parent profile, which represents all domains owned by the publisher.
    #[serde(rename="isParent")]
    
    pub is_parent: Option<bool>,
    /// True, if this profile is published. Deprecated for state.
    #[serde(rename="isPublished")]
    
    pub is_published: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "adexchangebuyer#publisherProfileApiProto".
    
    pub kind: Option<String>,
    /// The url to the logo for the publisher.
    #[serde(rename="logoUrl")]
    
    pub logo_url: Option<String>,
    /// The url for additional marketing and sales materials.
    #[serde(rename="mediaKitLink")]
    
    pub media_kit_link: Option<String>,
    /// no description provided
    
    pub name: Option<String>,
    /// Publisher provided overview.
    
    pub overview: Option<String>,
    /// The pair of (seller.account_id, profile_id) uniquely identifies a publisher profile for a given publisher.
    #[serde(rename="profileId")]
    
    pub profile_id: Option<i32>,
    /// Programmatic contact for the publisher profile.
    #[serde(rename="programmaticContact")]
    
    pub programmatic_contact: Option<String>,
    /// The list of app IDs represented in this pubisher profile. Empty if this is a parent profile. Deprecated in favor of publisher_app.
    #[serde(rename="publisherAppIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub publisher_app_ids: Option<Vec<i64>>,
    /// The list of apps represented in this pubisher profile. Empty if this is a parent profile.
    #[serde(rename="publisherApps")]
    
    pub publisher_apps: Option<Vec<MobileApplication>>,
    /// The list of domains represented in this publisher profile. Empty if this is a parent profile.
    #[serde(rename="publisherDomains")]
    
    pub publisher_domains: Option<Vec<String>>,
    /// Unique Id for publisher profile.
    #[serde(rename="publisherProfileId")]
    
    pub publisher_profile_id: Option<String>,
    /// Publisher provided forecasting information.
    #[serde(rename="publisherProvidedForecast")]
    
    pub publisher_provided_forecast: Option<PublisherProvidedForecast>,
    /// Link to publisher rate card
    #[serde(rename="rateCardInfoLink")]
    
    pub rate_card_info_link: Option<String>,
    /// Link for a sample content page.
    #[serde(rename="samplePageLink")]
    
    pub sample_page_link: Option<String>,
    /// Seller of the publisher profile.
    
    pub seller: Option<Seller>,
    /// State of the publisher profile.
    
    pub state: Option<String>,
    /// Publisher provided key metrics and rankings.
    #[serde(rename="topHeadlines")]
    
    pub top_headlines: Option<Vec<String>>,
}

impl client::Part for PublisherProfileApiProto {}


/// This message carries publisher provided forecasting information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublisherProvidedForecast {
    /// Publisher provided dimensions. E.g. geo, sizes etc...
    
    pub dimensions: Option<Vec<Dimension>>,
    /// Publisher provided weekly impressions.
    #[serde(rename="weeklyImpressions")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub weekly_impressions: Option<i64>,
    /// Publisher provided weekly uniques.
    #[serde(rename="weeklyUniques")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub weekly_uniques: Option<i64>,
}

impl client::Part for PublisherProvidedForecast {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Seller {
    /// The unique id for the seller. The seller fills in this field. The seller account id is then available to buyer in the product.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Optional sub-account id for the seller.
    #[serde(rename="subAccountId")]
    
    pub sub_account_id: Option<String>,
}

impl client::Part for Seller {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SharedTargeting {
    /// The list of values to exclude from targeting. Each value is AND'd together.
    
    pub exclusions: Option<Vec<TargetingValue>>,
    /// The list of value to include as part of the targeting. Each value is OR'd together.
    
    pub inclusions: Option<Vec<TargetingValue>>,
    /// The key representing the shared targeting criterion.
    
    pub key: Option<String>,
}

impl client::Part for SharedTargeting {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetingValue {
    /// The creative size value to exclude/include.
    #[serde(rename="creativeSizeValue")]
    
    pub creative_size_value: Option<TargetingValueCreativeSize>,
    /// The daypart targeting to include / exclude. Filled in when the key is GOOG_DAYPART_TARGETING.
    #[serde(rename="dayPartTargetingValue")]
    
    pub day_part_targeting_value: Option<TargetingValueDayPartTargeting>,
    /// no description provided
    #[serde(rename="demogAgeCriteriaValue")]
    
    pub demog_age_criteria_value: Option<TargetingValueDemogAgeCriteria>,
    /// no description provided
    #[serde(rename="demogGenderCriteriaValue")]
    
    pub demog_gender_criteria_value: Option<TargetingValueDemogGenderCriteria>,
    /// The long value to exclude/include.
    #[serde(rename="longValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub long_value: Option<i64>,
    /// no description provided
    #[serde(rename="requestPlatformTargetingValue")]
    
    pub request_platform_targeting_value: Option<TargetingValueRequestPlatformTargeting>,
    /// The string value to exclude/include.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
}

impl client::Part for TargetingValue {}


/// Next Id: 7
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetingValueCreativeSize {
    /// The formats allowed by the publisher.
    #[serde(rename="allowedFormats")]
    
    pub allowed_formats: Option<Vec<String>>,
    /// For video size type, the list of companion sizes.
    #[serde(rename="companionSizes")]
    
    pub companion_sizes: Option<Vec<TargetingValueSize>>,
    /// The Creative size type.
    #[serde(rename="creativeSizeType")]
    
    pub creative_size_type: Option<String>,
    /// The native template for native ad.
    #[serde(rename="nativeTemplate")]
    
    pub native_template: Option<String>,
    /// For regular or video creative size type, specifies the size of the creative.
    
    pub size: Option<TargetingValueSize>,
    /// The skippable ad type for video size.
    #[serde(rename="skippableAdType")]
    
    pub skippable_ad_type: Option<String>,
}

impl client::Part for TargetingValueCreativeSize {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetingValueDayPartTargeting {
    /// no description provided
    #[serde(rename="dayParts")]
    
    pub day_parts: Option<Vec<TargetingValueDayPartTargetingDayPart>>,
    /// no description provided
    #[serde(rename="timeZoneType")]
    
    pub time_zone_type: Option<String>,
}

impl client::Part for TargetingValueDayPartTargeting {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetingValueDayPartTargetingDayPart {
    /// no description provided
    #[serde(rename="dayOfWeek")]
    
    pub day_of_week: Option<String>,
    /// no description provided
    #[serde(rename="endHour")]
    
    pub end_hour: Option<i32>,
    /// no description provided
    #[serde(rename="endMinute")]
    
    pub end_minute: Option<i32>,
    /// no description provided
    #[serde(rename="startHour")]
    
    pub start_hour: Option<i32>,
    /// no description provided
    #[serde(rename="startMinute")]
    
    pub start_minute: Option<i32>,
}

impl client::Part for TargetingValueDayPartTargetingDayPart {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetingValueDemogAgeCriteria {
    /// no description provided
    #[serde(rename="demogAgeCriteriaIds")]
    
    pub demog_age_criteria_ids: Option<Vec<String>>,
}

impl client::Part for TargetingValueDemogAgeCriteria {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetingValueDemogGenderCriteria {
    /// no description provided
    #[serde(rename="demogGenderCriteriaIds")]
    
    pub demog_gender_criteria_ids: Option<Vec<String>>,
}

impl client::Part for TargetingValueDemogGenderCriteria {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetingValueRequestPlatformTargeting {
    /// no description provided
    #[serde(rename="requestPlatforms")]
    
    pub request_platforms: Option<Vec<String>>,
}

impl client::Part for TargetingValueRequestPlatformTargeting {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetingValueSize {
    /// The height of the creative.
    
    pub height: Option<i32>,
    /// The width of the creative.
    
    pub width: Option<i32>,
}

impl client::Part for TargetingValueSize {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updateproposal marketplaceprivateauction](MarketplaceprivateauctionUpdateproposalCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePrivateAuctionProposalRequest {
    /// The externalDealId of the deal to be updated.
    #[serde(rename="externalDealId")]
    
    pub external_deal_id: Option<String>,
    /// Optional note to be added.
    
    pub note: Option<MarketplaceNote>,
    /// The current revision number of the proposal to be updated.
    #[serde(rename="proposalRevisionNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub proposal_revision_number: Option<i64>,
    /// The proposed action on the private auction proposal.
    #[serde(rename="updateAction")]
    
    pub update_action: Option<String>,
}

impl client::RequestValue for UpdatePrivateAuctionProposalRequest {}


/// Your bidder locations that have distinct URLs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountBidderLocation {
    /// The protocol that the bidder endpoint is using. OpenRTB protocols with prefix PROTOCOL_OPENRTB_PROTOBUF use proto buffer, otherwise use JSON.  Allowed values:  
    /// - PROTOCOL_ADX 
    /// - PROTOCOL_OPENRTB_2_2 
    /// - PROTOCOL_OPENRTB_2_3 
    /// - PROTOCOL_OPENRTB_2_4 
    /// - PROTOCOL_OPENRTB_2_5 
    /// - PROTOCOL_OPENRTB_PROTOBUF_2_3 
    /// - PROTOCOL_OPENRTB_PROTOBUF_2_4 
    /// - PROTOCOL_OPENRTB_PROTOBUF_2_5
    #[serde(rename="bidProtocol")]
    
    pub bid_protocol: Option<String>,
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
    /// All known serving contexts containing serving status information.
    
    pub contexts: Option<Vec<CreativeCorrectionsContexts>>,
    /// Additional details about the correction.
    
    pub details: Option<Vec<String>>,
    /// The type of correction that was applied to the creative.
    
    pub reason: Option<String>,
}

impl client::NestedType for CreativeCorrections {}
impl client::Part for CreativeCorrections {}


/// All known serving contexts containing serving status information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeCorrectionsContexts {
    /// Only set when contextType=AUCTION_TYPE. Represents the auction types this correction applies to.
    #[serde(rename="auctionType")]
    
    pub auction_type: Option<Vec<String>>,
    /// The type of context (e.g., location, platform, auction type, SSL-ness).
    #[serde(rename="contextType")]
    
    pub context_type: Option<String>,
    /// Only set when contextType=LOCATION. Represents the geo criterias this correction applies to.
    #[serde(rename="geoCriteriaId")]
    
    pub geo_criteria_id: Option<Vec<i32>>,
    /// Only set when contextType=PLATFORM. Represents the platforms this correction applies to.
    
    pub platform: Option<Vec<String>>,
}

impl client::NestedType for CreativeCorrectionsContexts {}
impl client::Part for CreativeCorrectionsContexts {}


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
    /// The filtering status code as defined in  creative-status-codes.txt.
    #[serde(rename="filteringStatus")]
    
    pub filtering_status: Option<i32>,
}

impl client::NestedType for CreativeFilteringReasonsReasons {}
impl client::Part for CreativeFilteringReasonsReasons {}


/// If nativeAd is set, HTMLSnippet, videoVastXML, and the videoURL outside of nativeAd should not be set. (The videoURL inside nativeAd can be set.)
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
    /// The URL that the browser/SDK will load when the user clicks the ad.
    #[serde(rename="clickLinkUrl")]
    
    pub click_link_url: Option<String>,
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
    /// The URL of the XML VAST for a native ad. Note this is a separate field from resource.video_url.
    #[serde(rename="videoURL")]
    
    pub video_url: Option<String>,
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


/// The granular status of this ad in specific contexts. A context here relates to where something ultimately serves (for example, a physical location, a platform, an HTTPS vs HTTP request, or the type of auction). Read-only. This field should not be set in requests. See the examples in the Creatives guide for more details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeServingRestrictions {
    /// All known contexts/restrictions.
    
    pub contexts: Option<Vec<CreativeServingRestrictionsContexts>>,
    /// The reasons for disapproval within this restriction, if any. Note that not all disapproval reasons may be categorized, so it is possible for the creative to have a status of DISAPPROVED or CONDITIONALLY_APPROVED with an empty list for disapproval_reasons. In this case, please reach out to your TAM to help debug the issue.
    #[serde(rename="disapprovalReasons")]
    
    pub disapproval_reasons: Option<Vec<CreativeServingRestrictionsDisapprovalReasons>>,
    /// Why the creative is ineligible to serve in this context (e.g., it has been explicitly disapproved or is pending review).
    
    pub reason: Option<String>,
}

impl client::NestedType for CreativeServingRestrictions {}
impl client::Part for CreativeServingRestrictions {}


/// All known contexts/restrictions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeServingRestrictionsContexts {
    /// Only set when contextType=AUCTION_TYPE. Represents the auction types this restriction applies to.
    #[serde(rename="auctionType")]
    
    pub auction_type: Option<Vec<String>>,
    /// The type of context (e.g., location, platform, auction type, SSL-ness).
    #[serde(rename="contextType")]
    
    pub context_type: Option<String>,
    /// Only set when contextType=LOCATION. Represents the geo criterias this restriction applies to. Impressions are considered to match a context if either the user location or publisher location matches a given geoCriteriaId.
    #[serde(rename="geoCriteriaId")]
    
    pub geo_criteria_id: Option<Vec<i32>>,
    /// Only set when contextType=PLATFORM. Represents the platforms this restriction applies to.
    
    pub platform: Option<Vec<String>>,
}

impl client::NestedType for CreativeServingRestrictionsContexts {}
impl client::Part for CreativeServingRestrictionsContexts {}


/// The reasons for disapproval within this restriction, if any. Note that not all disapproval reasons may be categorized, so it is possible for the creative to have a status of DISAPPROVED or CONDITIONALLY_APPROVED with an empty list for disapproval_reasons. In this case, please reach out to your TAM to help debug the issue.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeServingRestrictionsDisapprovalReasons {
    /// Additional details about the reason for disapproval.
    
    pub details: Option<Vec<String>>,
    /// The categorized reason for disapproval.
    
    pub reason: Option<String>,
}

impl client::NestedType for CreativeServingRestrictionsDisapprovalReasons {}
impl client::Part for CreativeServingRestrictionsDisapprovalReasons {}


/// A list of external deal ids and ARC approval status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeDealIdsDealStatuses {
    /// ARC approval status.
    #[serde(rename="arcStatus")]
    
    pub arc_status: Option<String>,
    /// External deal ID.
    #[serde(rename="dealId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub deal_id: Option<i64>,
    /// Publisher ID.
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<i32>,
}

impl client::NestedType for CreativeDealIdsDealStatuses {}
impl client::Part for CreativeDealIdsDealStatuses {}


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


/// Video requests satisfying any of these player size constraints will match.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PretargetingConfigVideoPlayerSizes {
    /// The type of aspect ratio. Leave this field blank to match all aspect ratios.
    #[serde(rename="aspectRatio")]
    
    pub aspect_ratio: Option<String>,
    /// The minimum player height in pixels. Leave this field blank to match any player height.
    #[serde(rename="minHeight")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_height: Option<i64>,
    /// The minimum player width in pixels. Leave this field blank to match any player width.
    #[serde(rename="minWidth")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_width: Option<i64>,
}

impl client::NestedType for PretargetingConfigVideoPlayerSizes {}
impl client::Part for PretargetingConfigVideoPlayerSizes {}


