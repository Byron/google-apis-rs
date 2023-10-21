use super::*;
/// An absolute date range, specified by its start date and end date. The supported range of dates begins 30 days before today and ends today. Validity checked upon filter set creation. If a filter set with an absolute date range is run at a later date more than 30 days after start_date, it will fail.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AbsoluteDateRange {
    /// The end date of the range (inclusive). Must be within the 30 days leading up to current date, and must be equal to or after start_date.
    #[serde(rename="endDate")]
    
    pub end_date: Option<Date>,
    /// The start date of the range (inclusive). Must be within the 30 days leading up to current date, and must be equal to or before end_date.
    #[serde(rename="startDate")]
    
    pub start_date: Option<Date>,
}

impl client::Part for AbsoluteDateRange {}


/// Request to accept a proposal.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals accept accounts](AccountProposalAcceptCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AcceptProposalRequest {
    /// The last known client revision number of the proposal.
    #[serde(rename="proposalRevision")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub proposal_revision: Option<i64>,
}

impl client::RequestValue for AcceptProposalRequest {}


/// Represents size of a single ad slot, or a creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdSize {
    /// The height of the ad slot in pixels. This field will be present only when size type is `PIXEL`.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub height: Option<i64>,
    /// The size type of the ad slot.
    #[serde(rename="sizeType")]
    
    pub size_type: Option<String>,
    /// The width of the ad slot in pixels. This field will be present only when size type is `PIXEL`.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub width: Option<i64>,
}

impl client::Part for AdSize {}


/// Detected ad technology provider information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdTechnologyProviders {
    /// The detected ad technology provider IDs for this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/providers.csv for mapping of provider ID to provided name, a privacy policy URL, and a list of domains which can be attributed to the provider. If the creative contains provider IDs that are outside of those listed in the `BidRequest.adslot.consented_providers_settings.consented_providers` field on the (Google bid protocol)[https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto] and the `BidRequest.user.ext.consented_providers_settings.consented_providers` field on the (OpenRTB protocol)[https://developers.google.com/authorized-buyers/rtb/downloads/openrtb-adx-proto], and a bid is submitted with that creative for an impression that will serve to an EEA user, the bid will be filtered before the auction.
    #[serde(rename="detectedProviderIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub detected_provider_ids: Option<Vec<i64>>,
    /// Whether the creative contains an unidentified ad technology provider. If true for a given creative, any bid submitted with that creative for an impression that will serve to an EEA user will be filtered before the auction.
    #[serde(rename="hasUnidentifiedProvider")]
    
    pub has_unidentified_provider: Option<bool>,
}

impl client::Part for AdTechnologyProviders {}


/// A request for associating a deal and a creative.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives deal associations add accounts](AccountCreativeDealAssociationAddCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddDealAssociationRequest {
    /// The association between a creative and a deal that should be added.
    
    pub association: Option<CreativeDealAssociation>,
}

impl client::RequestValue for AddDealAssociationRequest {}


/// Request message for adding a note to a given proposal.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals add note accounts](AccountProposalAddNoteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddNoteRequest {
    /// Details of the note to add.
    
    pub note: Option<Note>,
}

impl client::RequestValue for AddNoteRequest {}


/// Output only. The app type the restriction applies to for mobile device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppContext {
    /// The app types this restriction applies to.
    #[serde(rename="appTypes")]
    
    pub app_types: Option<Vec<String>>,
}

impl client::Part for AppContext {}


/// Output only. The auction type the restriction applies to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuctionContext {
    /// The auction types this restriction applies to.
    #[serde(rename="auctionTypes")]
    
    pub auction_types: Option<Vec<String>>,
}

impl client::Part for AuctionContext {}


/// The set of metrics that are measured in numbers of bids, representing how many bids with the specified dimension values were considered eligible at each stage of the bidding funnel;
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BidMetricsRow {
    /// The number of bids that Ad Exchange received from the buyer.
    
    pub bids: Option<MetricValue>,
    /// The number of bids that were permitted to compete in the auction.
    #[serde(rename="bidsInAuction")]
    
    pub bids_in_auction: Option<MetricValue>,
    /// The number of bids for which the buyer was billed.
    #[serde(rename="billedImpressions")]
    
    pub billed_impressions: Option<MetricValue>,
    /// The number of bids that won the auction.
    #[serde(rename="impressionsWon")]
    
    pub impressions_won: Option<MetricValue>,
    /// The number of bids for which the corresponding impression was measurable for viewability (as defined by Active View).
    #[serde(rename="measurableImpressions")]
    
    pub measurable_impressions: Option<MetricValue>,
    /// The number of bids that won the auction and also won the mediation waterfall (if any).
    #[serde(rename="reachedQueries")]
    
    pub reached_queries: Option<MetricValue>,
    /// The values of all dimensions associated with metric values in this row.
    #[serde(rename="rowDimensions")]
    
    pub row_dimensions: Option<RowDimensions>,
    /// The number of bids for which the corresponding impression was viewable (as defined by Active View).
    #[serde(rename="viewableImpressions")]
    
    pub viewable_impressions: Option<MetricValue>,
}

impl client::Part for BidMetricsRow {}


/// The number of impressions with the specified dimension values that were considered to have no applicable bids, as described by the specified status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BidResponseWithoutBidsStatusRow {
    /// The number of impressions for which there was a bid response with the specified status.
    #[serde(rename="impressionCount")]
    
    pub impression_count: Option<MetricValue>,
    /// The values of all dimensions associated with metric values in this row.
    #[serde(rename="rowDimensions")]
    
    pub row_dimensions: Option<RowDimensions>,
    /// The status specifying why the bid responses were considered to have no applicable bids.
    
    pub status: Option<String>,
}

impl client::Part for BidResponseWithoutBidsStatusRow {}


/// Represents a buyer of inventory. Each buyer is identified by a unique Authorized Buyers account ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Buyer {
    /// Authorized Buyers account ID of the buyer.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
}

impl client::Part for Buyer {}


/// The number of impressions with the specified dimension values where the corresponding bid request or bid response was not successful, as described by the specified callout status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CalloutStatusRow {
    /// The ID of the callout status. See [callout-status-codes](https://developers.google.com/authorized-buyers/rtb/downloads/callout-status-codes).
    #[serde(rename="calloutStatusId")]
    
    pub callout_status_id: Option<i32>,
    /// The number of impressions for which there was a bid request or bid response with the specified callout status.
    #[serde(rename="impressionCount")]
    
    pub impression_count: Option<MetricValue>,
    /// The values of all dimensions associated with metric values in this row.
    #[serde(rename="rowDimensions")]
    
    pub row_dimensions: Option<RowDimensions>,
}

impl client::Part for CalloutStatusRow {}


/// Request to cancel an ongoing negotiation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals cancel negotiation accounts](AccountProposalCancelNegotiationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelNegotiationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelNegotiationRequest {}


/// A client resource represents a client buyer—an agency, a brand, or an advertiser customer of the sponsor buyer. Users associated with the client buyer have restricted access to the Marketplace and certain other sections of the Authorized Buyers UI based on the role granted to the client buyer. All fields are required unless otherwise specified.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clients create accounts](AccountClientCreateCall) (request|response)
/// * [clients get accounts](AccountClientGetCall) (response)
/// * [clients update accounts](AccountClientUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Client {
    /// The globally-unique numerical ID of the client. The value of this field is ignored in create and update operations.
    #[serde(rename="clientAccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub client_account_id: Option<i64>,
    /// Name used to represent this client to publishers. You may have multiple clients that map to the same entity, but for each client the combination of `clientName` and entity must be unique. You can specify this field as empty. Maximum length of 255 characters is allowed.
    #[serde(rename="clientName")]
    
    pub client_name: Option<String>,
    /// Numerical identifier of the client entity. The entity can be an advertiser, a brand, or an agency. This identifier is unique among all the entities with the same type. The value of this field is ignored if the entity type is not provided. A list of all known advertisers with their identifiers is available in the [advertisers.txt](https://storage.googleapis.com/adx-rtb-dictionaries/advertisers.txt) file. A list of all known brands with their identifiers is available in the [brands.txt](https://storage.googleapis.com/adx-rtb-dictionaries/brands.txt) file. A list of all known agencies with their identifiers is available in the [agencies.txt](https://storage.googleapis.com/adx-rtb-dictionaries/agencies.txt) file.
    #[serde(rename="entityId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub entity_id: Option<i64>,
    /// The name of the entity. This field is automatically fetched based on the type and ID. The value of this field is ignored in create and update operations.
    #[serde(rename="entityName")]
    
    pub entity_name: Option<String>,
    /// An optional field for specifying the type of the client entity: `ADVERTISER`, `BRAND`, or `AGENCY`.
    #[serde(rename="entityType")]
    
    pub entity_type: Option<String>,
    /// Optional arbitrary unique identifier of this client buyer from the standpoint of its Ad Exchange sponsor buyer. This field can be used to associate a client buyer with the identifier in the namespace of its sponsor buyer, lookup client buyers by that identifier and verify whether an Ad Exchange counterpart of a given client buyer already exists. If present, must be unique among all the client buyers for its Ad Exchange sponsor buyer.
    #[serde(rename="partnerClientId")]
    
    pub partner_client_id: Option<String>,
    /// The role which is assigned to the client buyer. Each role implies a set of permissions granted to the client. Must be one of `CLIENT_DEAL_VIEWER`, `CLIENT_DEAL_NEGOTIATOR` or `CLIENT_DEAL_APPROVER`.
    
    pub role: Option<String>,
    /// The status of the client buyer.
    
    pub status: Option<String>,
    /// Whether the client buyer will be visible to sellers.
    #[serde(rename="visibleToSeller")]
    
    pub visible_to_seller: Option<bool>,
}

impl client::RequestValue for Client {}
impl client::ResponseResult for Client {}


/// A client user is created under a client buyer and has restricted access to the Marketplace and certain other sections of the Authorized Buyers UI based on the role granted to the associated client buyer. The only way a new client user can be created is through accepting an email invitation (see the accounts.clients.invitations.create method). All fields are required unless otherwise specified.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clients users get accounts](AccountClientUserGetCall) (response)
/// * [clients users update accounts](AccountClientUserUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClientUser {
    /// Numerical account ID of the client buyer with which the user is associated; the buyer must be a client of the current sponsor buyer. The value of this field is ignored in an update operation.
    #[serde(rename="clientAccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub client_account_id: Option<i64>,
    /// User's email address. The value of this field is ignored in an update operation.
    
    pub email: Option<String>,
    /// The status of the client user.
    
    pub status: Option<String>,
    /// The unique numerical ID of the client user that has accepted an invitation. The value of this field is ignored in an update operation.
    #[serde(rename="userId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub user_id: Option<i64>,
}

impl client::RequestValue for ClientUser {}
impl client::ResponseResult for ClientUser {}


/// An invitation for a new client user to get access to the Authorized Buyers UI. All fields are required unless otherwise specified.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clients invitations create accounts](AccountClientInvitationCreateCall) (request|response)
/// * [clients invitations get accounts](AccountClientInvitationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClientUserInvitation {
    /// Numerical account ID of the client buyer that the invited user is associated with. The value of this field is ignored in create operations.
    #[serde(rename="clientAccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub client_account_id: Option<i64>,
    /// The email address to which the invitation is sent. Email addresses should be unique among all client users under each sponsor buyer.
    
    pub email: Option<String>,
    /// The unique numerical ID of the invitation that is sent to the user. The value of this field is ignored in create operations.
    #[serde(rename="invitationId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub invitation_id: Option<i64>,
}

impl client::RequestValue for ClientUserInvitation {}
impl client::ResponseResult for ClientUserInvitation {}


/// Request message for indicating that the proposal’s setup step is complete.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals complete setup accounts](AccountProposalCompleteSetupCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompleteSetupRequest { _never_set: Option<bool> }

impl client::RequestValue for CompleteSetupRequest {}


/// Contains information on how a buyer or seller can be reached.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactInformation {
    /// Email address for the contact.
    
    pub email: Option<String>,
    /// The name of the contact.
    
    pub name: Option<String>,
}

impl client::Part for ContactInformation {}


/// Output only. Shows any corrections that were applied to this creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Correction {
    /// The contexts for the correction.
    
    pub contexts: Option<Vec<ServingContext>>,
    /// Additional details about what was corrected.
    
    pub details: Option<Vec<String>>,
    /// The type of correction that was applied to the creative.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Correction {}


/// A creative and its classification data.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives create accounts](AccountCreativeCreateCall) (request|response)
/// * [creatives get accounts](AccountCreativeGetCall) (response)
/// * [creatives update accounts](AccountCreativeUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Creative {
    /// The account that this creative belongs to. Can be used to filter the response of the creatives.list method.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The link to AdChoices destination page.
    #[serde(rename="adChoicesDestinationUrl")]
    
    pub ad_choices_destination_url: Option<String>,
    /// Output only. The detected ad technology providers.
    #[serde(rename="adTechnologyProviders")]
    
    pub ad_technology_providers: Option<AdTechnologyProviders>,
    /// The name of the company being advertised in the creative.
    #[serde(rename="advertiserName")]
    
    pub advertiser_name: Option<String>,
    /// The agency ID for this creative.
    #[serde(rename="agencyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub agency_id: Option<i64>,
    /// Output only. The last update timestamp of the creative through the API.
    #[serde(rename="apiUpdateTime")]
    
    pub api_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// All attributes for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method.
    
    pub attributes: Option<Vec<String>>,
    /// The set of destination URLs for the creative.
    #[serde(rename="clickThroughUrls")]
    
    pub click_through_urls: Option<Vec<String>>,
    /// Output only. Shows any corrections that were applied to this creative.
    
    pub corrections: Option<Vec<Correction>>,
    /// The buyer-defined creative ID of this creative. Can be used to filter the response of the creatives.list method.
    #[serde(rename="creativeId")]
    
    pub creative_id: Option<String>,
    /// Output only. The top-level deals status of this creative. If disapproved, an entry for 'auctionType=DIRECT_DEALS' (or 'ALL') in serving_restrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case, it may be preferable to read from serving_restrictions directly. Can be used to filter the response of the creatives.list method.
    #[serde(rename="dealsStatus")]
    
    pub deals_status: Option<String>,
    /// The set of declared destination URLs for the creative.
    #[serde(rename="declaredClickThroughUrls")]
    
    pub declared_click_through_urls: Option<Vec<String>>,
    /// Output only. Detected advertiser IDs, if any.
    #[serde(rename="detectedAdvertiserIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub detected_advertiser_ids: Option<Vec<i64>>,
    /// Output only. The detected domains for this creative.
    #[serde(rename="detectedDomains")]
    
    pub detected_domains: Option<Vec<String>>,
    /// Output only. The detected languages for this creative. The order is arbitrary. The codes are 2 or 5 characters and are documented at https://developers.google.com/adwords/api/docs/appendix/languagecodes.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<String>>,
    /// Output only. Detected product categories, if any. See the ad-product-categories.txt file in the technical documentation for a list of IDs.
    #[serde(rename="detectedProductCategories")]
    
    pub detected_product_categories: Option<Vec<i32>>,
    /// Output only. Detected sensitive categories, if any. See the ad-sensitive-categories.txt file in the technical documentation for a list of IDs. You should use these IDs along with the excluded-sensitive-category field in the bid request to filter your bids.
    #[serde(rename="detectedSensitiveCategories")]
    
    pub detected_sensitive_categories: Option<Vec<i32>>,
    /// An HTML creative.
    
    pub html: Option<HtmlContent>,
    /// The set of URLs to be called to record an impression.
    #[serde(rename="impressionTrackingUrls")]
    
    pub impression_tracking_urls: Option<Vec<String>>,
    /// A native creative.
    
    pub native: Option<NativeContent>,
    /// Output only. The top-level open auction status of this creative. If disapproved, an entry for 'auctionType = OPEN_AUCTION' (or 'ALL') in serving_restrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case, it may be preferable to read from serving_restrictions directly. Can be used to filter the response of the creatives.list method.
    #[serde(rename="openAuctionStatus")]
    
    pub open_auction_status: Option<String>,
    /// All restricted categories for the ads that may be shown from this creative.
    #[serde(rename="restrictedCategories")]
    
    pub restricted_categories: Option<Vec<String>>,
    /// Output only. The granular status of this ad in specific contexts. A context here relates to where something ultimately serves (for example, a physical location, a platform, an HTTPS versus HTTP request, or the type of auction).
    #[serde(rename="servingRestrictions")]
    
    pub serving_restrictions: Option<Vec<ServingRestriction>>,
    /// All vendor IDs for the ads that may be shown from this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/vendors.txt for possible values.
    #[serde(rename="vendorIds")]
    
    pub vendor_ids: Option<Vec<i32>>,
    /// Output only. The version of this creative.
    
    pub version: Option<i32>,
    /// A video creative.
    
    pub video: Option<VideoContent>,
}

impl client::RequestValue for Creative {}
impl client::ResponseResult for Creative {}


/// The association between a creative and a deal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeDealAssociation {
    /// The account the creative belongs to.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The ID of the creative associated with the deal.
    #[serde(rename="creativeId")]
    
    pub creative_id: Option<String>,
    /// The externalDealId for the deal associated with the creative.
    #[serde(rename="dealsId")]
    
    pub deals_id: Option<String>,
}

impl client::Part for CreativeDealAssociation {}


/// Represents creative restrictions associated to Programmatic Guaranteed/ Preferred Deal in Ad Manager. This doesn't apply to Private Auction and AdX Preferred Deals.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeRestrictions {
    /// The format of the environment that the creatives will be displayed in.
    #[serde(rename="creativeFormat")]
    
    pub creative_format: Option<String>,
    /// no description provided
    #[serde(rename="creativeSpecifications")]
    
    pub creative_specifications: Option<Vec<CreativeSpecification>>,
    /// Skippable video ads allow viewers to skip ads after 5 seconds.
    #[serde(rename="skippableAdType")]
    
    pub skippable_ad_type: Option<String>,
}

impl client::Part for CreativeRestrictions {}


/// Specifies the size of the creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeSize {
    /// What formats are allowed by the publisher. If this repeated field is empty then all formats are allowed. For example, if this field contains AllowedFormatType.AUDIO then the publisher only allows an audio ad (without any video).
    #[serde(rename="allowedFormats")]
    
    pub allowed_formats: Option<Vec<String>>,
    /// For video creatives specifies the sizes of companion ads (if present). Companion sizes may be filled in only when creative_size_type = VIDEO
    #[serde(rename="companionSizes")]
    
    pub companion_sizes: Option<Vec<Size>>,
    /// The creative size type.
    #[serde(rename="creativeSizeType")]
    
    pub creative_size_type: Option<String>,
    /// Output only. The native template for this creative. It will have a value only if creative_size_type = CreativeSizeType.NATIVE.
    #[serde(rename="nativeTemplate")]
    
    pub native_template: Option<String>,
    /// For regular or video creative size type, specifies the size of the creative
    
    pub size: Option<Size>,
    /// The type of skippable ad for this creative. It will have a value only if creative_size_type = CreativeSizeType.VIDEO.
    #[serde(rename="skippableAdType")]
    
    pub skippable_ad_type: Option<String>,
}

impl client::Part for CreativeSize {}


/// Represents information for a creative that is associated with a Programmatic Guaranteed/Preferred Deal in Ad Manager.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeSpecification {
    /// Companion sizes may be filled in only when this is a video creative.
    #[serde(rename="creativeCompanionSizes")]
    
    pub creative_companion_sizes: Option<Vec<AdSize>>,
    /// The size of the creative.
    #[serde(rename="creativeSize")]
    
    pub creative_size: Option<AdSize>,
}

impl client::Part for CreativeSpecification {}


/// The number of bids with the specified dimension values that did not win the auction (either were filtered pre-auction or lost the auction), as described by the specified creative status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeStatusRow {
    /// The number of bids with the specified status.
    #[serde(rename="bidCount")]
    
    pub bid_count: Option<MetricValue>,
    /// The ID of the creative status. See [creative-status-codes](https://developers.google.com/authorized-buyers/rtb/downloads/creative-status-codes).
    #[serde(rename="creativeStatusId")]
    
    pub creative_status_id: Option<i32>,
    /// The values of all dimensions associated with metric values in this row.
    #[serde(rename="rowDimensions")]
    
    pub row_dimensions: Option<RowDimensions>,
}

impl client::Part for CreativeStatusRow {}


/// Generic targeting used for targeting dimensions that contains a list of included and excluded numeric IDs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CriteriaTargeting {
    /// A list of numeric IDs to be excluded.
    #[serde(rename="excludedCriteriaIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub excluded_criteria_ids: Option<Vec<i64>>,
    /// A list of numeric IDs to be included.
    #[serde(rename="targetedCriteriaIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub targeted_criteria_ids: Option<Vec<i64>>,
}

impl client::Part for CriteriaTargeting {}


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


/// Daypart targeting message that specifies if the ad can be shown only during certain parts of a day/week.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DayPart {
    /// The day of the week to target. If unspecified, applicable to all days.
    #[serde(rename="dayOfWeek")]
    
    pub day_of_week: Option<String>,
    /// The ending time of the day for the ad to show (minute level granularity). The end time is exclusive. This field is not available for filtering in PQL queries.
    #[serde(rename="endTime")]
    
    pub end_time: Option<TimeOfDay>,
    /// The starting time of day for the ad to show (minute level granularity). The start time is inclusive. This field is not available for filtering in PQL queries.
    #[serde(rename="startTime")]
    
    pub start_time: Option<TimeOfDay>,
}

impl client::Part for DayPart {}


/// Specifies the day part targeting criteria.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DayPartTargeting {
    /// A list of day part targeting criterion.
    #[serde(rename="dayParts")]
    
    pub day_parts: Option<Vec<DayPart>>,
    /// The timezone to use for interpreting the day part targeting.
    #[serde(rename="timeZoneType")]
    
    pub time_zone_type: Option<String>,
}

impl client::Part for DayPartTargeting {}


/// A deal represents a segment of inventory for displaying ads on. A proposal can contain multiple deals. A deal contains the terms and targeting information that is used for serving.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Deal {
    /// Proposed flight end time of the deal. This will generally be stored in a granularity of a second. A value is not required for Private Auction deals or Preferred Deals.
    #[serde(rename="availableEndTime")]
    
    pub available_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Proposed flight start time of the deal. This will generally be stored in the granularity of one second since deal serving starts at seconds boundary. Any time specified with more granularity (for example, in milliseconds) will be truncated towards the start of time in seconds.
    #[serde(rename="availableStartTime")]
    
    pub available_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Buyer private data (hidden from seller).
    #[serde(rename="buyerPrivateData")]
    
    pub buyer_private_data: Option<PrivateData>,
    /// The product ID from which this deal was created. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error.
    #[serde(rename="createProductId")]
    
    pub create_product_id: Option<String>,
    /// Optional. Revision number of the product that the deal was created from. If present on create, and the server `product_revision` has advanced since the passed-in `create_product_revision`, an `ABORTED` error will be returned. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error.
    #[serde(rename="createProductRevision")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub create_product_revision: Option<i64>,
    /// Output only. The time of the deal creation.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Specifies the creative pre-approval policy.
    #[serde(rename="creativePreApprovalPolicy")]
    
    pub creative_pre_approval_policy: Option<String>,
    /// Output only. Restricitions about the creatives associated with the deal (for example, size) This is available for Programmatic Guaranteed/Preferred Deals in Ad Manager.
    #[serde(rename="creativeRestrictions")]
    
    pub creative_restrictions: Option<CreativeRestrictions>,
    /// Output only. Specifies whether the creative is safeFrame compatible.
    #[serde(rename="creativeSafeFrameCompatibility")]
    
    pub creative_safe_frame_compatibility: Option<String>,
    /// Output only. A unique deal ID for the deal (server-assigned).
    #[serde(rename="dealId")]
    
    pub deal_id: Option<String>,
    /// Output only. Metadata about the serving status of this deal.
    #[serde(rename="dealServingMetadata")]
    
    pub deal_serving_metadata: Option<DealServingMetadata>,
    /// The negotiable terms of the deal.
    #[serde(rename="dealTerms")]
    
    pub deal_terms: Option<DealTerms>,
    /// The set of fields around delivery control that are interesting for a buyer to see but are non-negotiable. These are set by the publisher.
    #[serde(rename="deliveryControl")]
    
    pub delivery_control: Option<DeliveryControl>,
    /// Description for the deal terms.
    
    pub description: Option<String>,
    /// The name of the deal.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The external deal ID assigned to this deal once the deal is finalized. This is the deal ID that shows up in serving/reporting etc.
    #[serde(rename="externalDealId")]
    
    pub external_deal_id: Option<String>,
    /// Output only. True, if the buyside inventory setup is complete for this deal.
    #[serde(rename="isSetupComplete")]
    
    pub is_setup_complete: Option<bool>,
    /// Output only. Specifies the creative source for programmatic deals. PUBLISHER means creative is provided by seller and ADVERTISER means creative is provided by buyer.
    #[serde(rename="programmaticCreativeSource")]
    
    pub programmatic_creative_source: Option<String>,
    /// Output only. ID of the proposal that this deal is part of.
    #[serde(rename="proposalId")]
    
    pub proposal_id: Option<String>,
    /// Output only. Seller contact information for the deal.
    #[serde(rename="sellerContacts")]
    
    pub seller_contacts: Option<Vec<ContactInformation>>,
    /// The syndication product associated with the deal. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error.
    #[serde(rename="syndicationProduct")]
    
    pub syndication_product: Option<String>,
    /// Output only. Specifies the subset of inventory targeted by the deal.
    
    pub targeting: Option<MarketplaceTargeting>,
    /// The shared targeting visible to buyers and sellers. Each shared targeting entity is AND'd together.
    #[serde(rename="targetingCriterion")]
    
    pub targeting_criterion: Option<Vec<TargetingCriteria>>,
    /// Output only. The time when the deal was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The web property code for the seller copied over from the product.
    #[serde(rename="webPropertyCode")]
    
    pub web_property_code: Option<String>,
}

impl client::Part for Deal {}


/// Tracks which parties (if any) have paused a deal. The deal is considered paused if either hasBuyerPaused or hasSellPaused is true.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DealPauseStatus {
    /// The buyer's reason for pausing, if the buyer paused the deal.
    #[serde(rename="buyerPauseReason")]
    
    pub buyer_pause_reason: Option<String>,
    /// The role of the person who first paused this deal.
    #[serde(rename="firstPausedBy")]
    
    pub first_paused_by: Option<String>,
    /// True, if the buyer has paused the deal unilaterally.
    #[serde(rename="hasBuyerPaused")]
    
    pub has_buyer_paused: Option<bool>,
    /// True, if the seller has paused the deal unilaterally.
    #[serde(rename="hasSellerPaused")]
    
    pub has_seller_paused: Option<bool>,
    /// The seller's reason for pausing, if the seller paused the deal.
    #[serde(rename="sellerPauseReason")]
    
    pub seller_pause_reason: Option<String>,
}

impl client::Part for DealPauseStatus {}


/// Message captures metadata about the serving status of a deal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DealServingMetadata {
    /// Output only. Tracks which parties (if any) have paused a deal.
    #[serde(rename="dealPauseStatus")]
    
    pub deal_pause_status: Option<DealPauseStatus>,
}

impl client::Part for DealServingMetadata {}


/// The deal terms specify the details of a Product/deal. They specify things like price per buyer, the type of pricing model (for example, fixed price, auction) and expected impressions from the publisher.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DealTerms {
    /// Visibility of the URL in bid requests. (default: BRANDED)
    #[serde(rename="brandingType")]
    
    pub branding_type: Option<String>,
    /// Publisher provided description for the terms.
    
    pub description: Option<String>,
    /// Non-binding estimate of the estimated gross spend for this deal. Can be set by buyer or seller.
    #[serde(rename="estimatedGrossSpend")]
    
    pub estimated_gross_spend: Option<Price>,
    /// Non-binding estimate of the impressions served per day. Can be set by buyer or seller.
    #[serde(rename="estimatedImpressionsPerDay")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub estimated_impressions_per_day: Option<i64>,
    /// The terms for guaranteed fixed price deals.
    #[serde(rename="guaranteedFixedPriceTerms")]
    
    pub guaranteed_fixed_price_terms: Option<GuaranteedFixedPriceTerms>,
    /// The terms for non-guaranteed auction deals.
    #[serde(rename="nonGuaranteedAuctionTerms")]
    
    pub non_guaranteed_auction_terms: Option<NonGuaranteedAuctionTerms>,
    /// The terms for non-guaranteed fixed price deals.
    #[serde(rename="nonGuaranteedFixedPriceTerms")]
    
    pub non_guaranteed_fixed_price_terms: Option<NonGuaranteedFixedPriceTerms>,
    /// The time zone name. For deals with Cost Per Day billing, defines the time zone used to mark the boundaries of a day. It should be an IANA TZ name, such as "America/Los_Angeles". For more information, see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones.
    #[serde(rename="sellerTimeZone")]
    
    pub seller_time_zone: Option<String>,
}

impl client::Part for DealTerms {}


/// Message contains details about how the deals will be paced.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryControl {
    /// Output only. Specified the creative blocking levels to be applied.
    #[serde(rename="creativeBlockingLevel")]
    
    pub creative_blocking_level: Option<String>,
    /// Output only. Specifies how the impression delivery will be paced.
    #[serde(rename="deliveryRateType")]
    
    pub delivery_rate_type: Option<String>,
    /// Output only. Specifies any frequency caps.
    #[serde(rename="frequencyCaps")]
    
    pub frequency_caps: Option<Vec<FrequencyCap>>,
}

impl client::Part for DeliveryControl {}


/// Output only. The reason and details for a disapproval.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Disapproval {
    /// Additional details about the reason for disapproval.
    
    pub details: Option<Vec<String>>,
    /// The categorized reason for disapproval.
    
    pub reason: Option<String>,
}

impl client::Part for Disapproval {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives deal associations add accounts](AccountCreativeDealAssociationAddCall) (response)
/// * [creatives deal associations remove accounts](AccountCreativeDealAssociationRemoveCall) (response)
/// * [creatives stop watching accounts](AccountCreativeStopWatchingCall) (response)
/// * [creatives watch accounts](AccountCreativeWatchCall) (response)
/// * [accounts filter sets delete bidders](BidderAccountFilterSetDeleteCall) (response)
/// * [filter sets delete bidders](BidderFilterSetDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// A set of filters that is applied to a request for data. Within a filter set, an AND operation is performed across the filters represented by each field. An OR operation is performed across the filters represented by the multiple values of a repeated field, for example, “format=VIDEO AND deal_id=12 AND (seller_network_id=34 OR seller_network_id=56)”.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [accounts filter sets create bidders](BidderAccountFilterSetCreateCall) (request|response)
/// * [accounts filter sets get bidders](BidderAccountFilterSetGetCall) (response)
/// * [filter sets create bidders](BidderFilterSetCreateCall) (request|response)
/// * [filter sets get bidders](BidderFilterSetGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterSet {
    /// An absolute date range, defined by a start date and an end date. Interpreted relative to Pacific time zone.
    #[serde(rename="absoluteDateRange")]
    
    pub absolute_date_range: Option<AbsoluteDateRange>,
    /// The set of dimensions along which to break down the response; may be empty. If multiple dimensions are requested, the breakdown is along the Cartesian product of the requested dimensions.
    #[serde(rename="breakdownDimensions")]
    
    pub breakdown_dimensions: Option<Vec<String>>,
    /// The ID of the creative on which to filter; optional. This field may be set only for a filter set that accesses account-level troubleshooting data, for example, one whose name matches the `bidders/*/accounts/*/filterSets/*` pattern.
    #[serde(rename="creativeId")]
    
    pub creative_id: Option<String>,
    /// The ID of the deal on which to filter; optional. This field may be set only for a filter set that accesses account-level troubleshooting data, for example, one whose name matches the `bidders/*/accounts/*/filterSets/*` pattern.
    #[serde(rename="dealId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub deal_id: Option<i64>,
    /// The environment on which to filter; optional.
    
    pub environment: Option<String>,
    /// Creative format bidded on or allowed to bid on, can be empty.
    
    pub format: Option<String>,
    /// Creative formats bidded on or allowed to bid on, can be empty. Although this field is a list, it can only be populated with a single item. A HTTP 400 bad request error will be returned in the response if you specify multiple items.
    
    pub formats: Option<Vec<String>>,
    /// A user-defined name of the filter set. Filter set names must be unique globally and match one of the patterns: - `bidders/*/filterSets/*` (for accessing bidder-level troubleshooting data) - `bidders/*/accounts/*/filterSets/*` (for accessing account-level troubleshooting data) This field is required in create operations.
    
    pub name: Option<String>,
    /// The list of platforms on which to filter; may be empty. The filters represented by multiple platforms are ORed together (for example, if non-empty, results must match any one of the platforms).
    
    pub platforms: Option<Vec<String>>,
    /// For Open Bidding partners only. The list of publisher identifiers on which to filter; may be empty. The filters represented by multiple publisher identifiers are ORed together.
    #[serde(rename="publisherIdentifiers")]
    
    pub publisher_identifiers: Option<Vec<String>>,
    /// An open-ended realtime time range, defined by the aggregation start timestamp.
    #[serde(rename="realtimeTimeRange")]
    
    pub realtime_time_range: Option<RealtimeTimeRange>,
    /// A relative date range, defined by an offset from today and a duration. Interpreted relative to Pacific time zone.
    #[serde(rename="relativeDateRange")]
    
    pub relative_date_range: Option<RelativeDateRange>,
    /// For Authorized Buyers only. The list of IDs of the seller (publisher) networks on which to filter; may be empty. The filters represented by multiple seller network IDs are ORed together (for example, if non-empty, results must match any one of the publisher networks). See [seller-network-ids](https://developers.google.com/authorized-buyers/rtb/downloads/seller-network-ids) file for the set of existing seller network IDs.
    #[serde(rename="sellerNetworkIds")]
    
    pub seller_network_ids: Option<Vec<i32>>,
    /// The granularity of time intervals if a time series breakdown is preferred; optional.
    #[serde(rename="timeSeriesGranularity")]
    
    pub time_series_granularity: Option<String>,
}

impl client::RequestValue for FilterSet {}
impl client::ResponseResult for FilterSet {}


/// The number of filtered bids with the specified dimension values that have the specified creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilteredBidCreativeRow {
    /// The number of bids with the specified creative.
    #[serde(rename="bidCount")]
    
    pub bid_count: Option<MetricValue>,
    /// The ID of the creative.
    #[serde(rename="creativeId")]
    
    pub creative_id: Option<String>,
    /// The values of all dimensions associated with metric values in this row.
    #[serde(rename="rowDimensions")]
    
    pub row_dimensions: Option<RowDimensions>,
}

impl client::Part for FilteredBidCreativeRow {}


/// The number of filtered bids with the specified dimension values, among those filtered due to the requested filtering reason (for example, creative status), that have the specified detail.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilteredBidDetailRow {
    /// The number of bids with the specified detail.
    #[serde(rename="bidCount")]
    
    pub bid_count: Option<MetricValue>,
    /// The ID of the detail, can be numeric or text. The associated value can be looked up in the dictionary file corresponding to the DetailType in the response message.
    
    pub detail: Option<String>,
    /// Note: this field will be deprecated, use "detail" field instead. When "detail" field represents an integer value, this field is populated as the same integer value "detail" field represents, otherwise this field will be 0. The ID of the detail. The associated value can be looked up in the dictionary file corresponding to the DetailType in the response message.
    #[serde(rename="detailId")]
    
    pub detail_id: Option<i32>,
    /// The values of all dimensions associated with metric values in this row.
    #[serde(rename="rowDimensions")]
    
    pub row_dimensions: Option<RowDimensions>,
}

impl client::Part for FilteredBidDetailRow {}


/// Represents a list of targeted and excluded mobile application IDs that publishers own. Mobile application IDs are from App Store and Google Play Store. Android App ID, for example, com.google.android.apps.maps, can be found in Google Play Store URL. iOS App ID (which is a number) can be found at the end of iTunes store URL. First party mobile applications is either included or excluded.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirstPartyMobileApplicationTargeting {
    /// A list of application IDs to be excluded.
    #[serde(rename="excludedAppIds")]
    
    pub excluded_app_ids: Option<Vec<String>>,
    /// A list of application IDs to be included.
    #[serde(rename="targetedAppIds")]
    
    pub targeted_app_ids: Option<Vec<String>>,
}

impl client::Part for FirstPartyMobileApplicationTargeting {}


/// Frequency cap.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FrequencyCap {
    /// The maximum number of impressions that can be served to a user within the specified time period.
    #[serde(rename="maxImpressions")]
    
    pub max_impressions: Option<i32>,
    /// The amount of time, in the units specified by time_unit_type. Defines the amount of time over which impressions per user are counted and capped.
    #[serde(rename="numTimeUnits")]
    
    pub num_time_units: Option<i32>,
    /// The time unit. Along with num_time_units defines the amount of time over which impressions per user are counted and capped.
    #[serde(rename="timeUnitType")]
    
    pub time_unit_type: Option<String>,
}

impl client::Part for FrequencyCap {}


/// Terms for Programmatic Guaranteed Deals.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GuaranteedFixedPriceTerms {
    /// Fixed price for the specified buyer.
    #[serde(rename="fixedPrices")]
    
    pub fixed_prices: Option<Vec<PricePerBuyer>>,
    /// Guaranteed impressions as a percentage. This is the percentage of guaranteed looks that the buyer is guaranteeing to buy.
    #[serde(rename="guaranteedImpressions")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub guaranteed_impressions: Option<i64>,
    /// Count of guaranteed looks. Required for deal, optional for product.
    #[serde(rename="guaranteedLooks")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub guaranteed_looks: Option<i64>,
    /// The lifetime impression cap for CPM sponsorship deals. The deal will stop serving when the cap is reached.
    #[serde(rename="impressionCap")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub impression_cap: Option<i64>,
    /// Daily minimum looks for CPD deal types.
    #[serde(rename="minimumDailyLooks")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub minimum_daily_looks: Option<i64>,
    /// For sponsorship deals, this is the percentage of the seller's eligible impressions that the deal will serve until the cap is reached.
    #[serde(rename="percentShareOfVoice")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub percent_share_of_voice: Option<i64>,
    /// The reservation type for a Programmatic Guaranteed deal. This indicates whether the number of impressions is fixed, or a percent of available impressions. If not specified, the default reservation type is STANDARD.
    #[serde(rename="reservationType")]
    
    pub reservation_type: Option<String>,
}

impl client::Part for GuaranteedFixedPriceTerms {}


/// HTML content for a creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HtmlContent {
    /// The height of the HTML snippet in pixels.
    
    pub height: Option<i32>,
    /// The HTML snippet that displays the ad when inserted in the web page.
    
    pub snippet: Option<String>,
    /// The width of the HTML snippet in pixels.
    
    pub width: Option<i32>,
}

impl client::Part for HtmlContent {}


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


/// The set of metrics that are measured in numbers of impressions, representing how many impressions with the specified dimension values were considered eligible at each stage of the bidding funnel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImpressionMetricsRow {
    /// The number of impressions available to the buyer on Ad Exchange. In some cases this value may be unavailable.
    #[serde(rename="availableImpressions")]
    
    pub available_impressions: Option<MetricValue>,
    /// The number of impressions for which Ad Exchange sent the buyer a bid request.
    #[serde(rename="bidRequests")]
    
    pub bid_requests: Option<MetricValue>,
    /// The number of impressions that match the buyer's inventory pretargeting.
    #[serde(rename="inventoryMatches")]
    
    pub inventory_matches: Option<MetricValue>,
    /// The number of impressions for which Ad Exchange received a response from the buyer that contained at least one applicable bid.
    #[serde(rename="responsesWithBids")]
    
    pub responses_with_bids: Option<MetricValue>,
    /// The values of all dimensions associated with metric values in this row.
    #[serde(rename="rowDimensions")]
    
    pub row_dimensions: Option<RowDimensions>,
    /// The number of impressions for which the buyer successfully sent a response to Ad Exchange.
    #[serde(rename="successfulResponses")]
    
    pub successful_responses: Option<MetricValue>,
}

impl client::Part for ImpressionMetricsRow {}


/// Represents the size of an ad unit that can be targeted on an ad request. It only applies to Private Auction, AdX Preferred Deals and Auction Packages. This targeting does not apply to Programmatic Guaranteed and Preferred Deals in Ad Manager.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventorySizeTargeting {
    /// A list of inventory sizes to be excluded.
    #[serde(rename="excludedInventorySizes")]
    
    pub excluded_inventory_sizes: Option<Vec<AdSize>>,
    /// A list of inventory sizes to be included.
    #[serde(rename="targetedInventorySizes")]
    
    pub targeted_inventory_sizes: Option<Vec<AdSize>>,
}

impl client::Part for InventorySizeTargeting {}


/// Response message for listing the metrics that are measured in number of bids.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [accounts filter sets bid metrics list bidders](BidderAccountFilterSetBidMetricListCall) (response)
/// * [filter sets bid metrics list bidders](BidderFilterSetBidMetricListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBidMetricsResponse {
    /// List of rows, each containing a set of bid metrics.
    #[serde(rename="bidMetricsRows")]
    
    pub bid_metrics_rows: Option<Vec<BidMetricsRow>>,
    /// A token to retrieve the next page of results. Pass this value in the ListBidMetricsRequest.pageToken field in the subsequent call to the bidMetrics.list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListBidMetricsResponse {}


/// Response message for listing all reasons that bid responses resulted in an error.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [accounts filter sets bid response errors list bidders](BidderAccountFilterSetBidResponseErrorListCall) (response)
/// * [filter sets bid response errors list bidders](BidderFilterSetBidResponseErrorListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBidResponseErrorsResponse {
    /// List of rows, with counts of bid responses aggregated by callout status.
    #[serde(rename="calloutStatusRows")]
    
    pub callout_status_rows: Option<Vec<CalloutStatusRow>>,
    /// A token to retrieve the next page of results. Pass this value in the ListBidResponseErrorsRequest.pageToken field in the subsequent call to the bidResponseErrors.list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListBidResponseErrorsResponse {}


/// Response message for listing all reasons that bid responses were considered to have no applicable bids.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [accounts filter sets bid responses without bids list bidders](BidderAccountFilterSetBidResponsesWithoutBidListCall) (response)
/// * [filter sets bid responses without bids list bidders](BidderFilterSetBidResponsesWithoutBidListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBidResponsesWithoutBidsResponse {
    /// List of rows, with counts of bid responses without bids aggregated by status.
    #[serde(rename="bidResponseWithoutBidsStatusRows")]
    
    pub bid_response_without_bids_status_rows: Option<Vec<BidResponseWithoutBidsStatusRow>>,
    /// A token to retrieve the next page of results. Pass this value in the ListBidResponsesWithoutBidsRequest.pageToken field in the subsequent call to the bidResponsesWithoutBids.list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListBidResponsesWithoutBidsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clients invitations list accounts](AccountClientInvitationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListClientUserInvitationsResponse {
    /// The returned list of client users.
    
    pub invitations: Option<Vec<ClientUserInvitation>>,
    /// A token to retrieve the next page of results. Pass this value in the ListClientUserInvitationsRequest.pageToken field in the subsequent call to the clients.invitations.list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListClientUserInvitationsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clients users list accounts](AccountClientUserListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListClientUsersResponse {
    /// A token to retrieve the next page of results. Pass this value in the ListClientUsersRequest.pageToken field in the subsequent call to the clients.invitations.list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The returned list of client users.
    
    pub users: Option<Vec<ClientUser>>,
}

impl client::ResponseResult for ListClientUsersResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clients list accounts](AccountClientListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListClientsResponse {
    /// The returned list of clients.
    
    pub clients: Option<Vec<Client>>,
    /// A token to retrieve the next page of results. Pass this value in the ListClientsRequest.pageToken field in the subsequent call to the accounts.clients.list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListClientsResponse {}


/// Response message for listing all creatives associated with a given filtered bid reason.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [accounts filter sets filtered bids creatives list bidders](BidderAccountFilterSetFilteredBidCreativeListCall) (response)
/// * [filter sets filtered bids creatives list bidders](BidderFilterSetFilteredBidCreativeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCreativeStatusBreakdownByCreativeResponse {
    /// List of rows, with counts of bids with a given creative status aggregated by creative.
    #[serde(rename="filteredBidCreativeRows")]
    
    pub filtered_bid_creative_rows: Option<Vec<FilteredBidCreativeRow>>,
    /// A token to retrieve the next page of results. Pass this value in the ListCreativeStatusBreakdownByCreativeRequest.pageToken field in the subsequent call to the filteredBids.creatives.list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCreativeStatusBreakdownByCreativeResponse {}


/// Response message for listing all details associated with a given filtered bid reason.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [accounts filter sets filtered bids details list bidders](BidderAccountFilterSetFilteredBidDetailListCall) (response)
/// * [filter sets filtered bids details list bidders](BidderFilterSetFilteredBidDetailListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCreativeStatusBreakdownByDetailResponse {
    /// The type of detail that the detail IDs represent.
    #[serde(rename="detailType")]
    
    pub detail_type: Option<String>,
    /// List of rows, with counts of bids with a given creative status aggregated by detail.
    #[serde(rename="filteredBidDetailRows")]
    
    pub filtered_bid_detail_rows: Option<Vec<FilteredBidDetailRow>>,
    /// A token to retrieve the next page of results. Pass this value in the ListCreativeStatusBreakdownByDetailRequest.pageToken field in the subsequent call to the filteredBids.details.list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCreativeStatusBreakdownByDetailResponse {}


/// A response for listing creatives.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives list accounts](AccountCreativeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCreativesResponse {
    /// The list of creatives.
    
    pub creatives: Option<Vec<Creative>>,
    /// A token to retrieve the next page of results. Pass this value in the ListCreativesRequest.page_token field in the subsequent call to `ListCreatives` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCreativesResponse {}


/// A response for listing creative and deal associations
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives deal associations list accounts](AccountCreativeDealAssociationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDealAssociationsResponse {
    /// The list of associations.
    
    pub associations: Option<Vec<CreativeDealAssociation>>,
    /// A token to retrieve the next page of results. Pass this value in the ListDealAssociationsRequest.page_token field in the subsequent call to 'ListDealAssociation' method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDealAssociationsResponse {}


/// Response message for listing filter sets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [accounts filter sets list bidders](BidderAccountFilterSetListCall) (response)
/// * [filter sets list bidders](BidderFilterSetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFilterSetsResponse {
    /// The filter sets belonging to the buyer.
    #[serde(rename="filterSets")]
    
    pub filter_sets: Option<Vec<FilterSet>>,
    /// A token to retrieve the next page of results. Pass this value in the ListFilterSetsRequest.pageToken field in the subsequent call to the accounts.filterSets.list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListFilterSetsResponse {}


/// Response message for listing all reasons that bid requests were filtered and not sent to the buyer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [accounts filter sets filtered bid requests list bidders](BidderAccountFilterSetFilteredBidRequestListCall) (response)
/// * [filter sets filtered bid requests list bidders](BidderFilterSetFilteredBidRequestListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFilteredBidRequestsResponse {
    /// List of rows, with counts of filtered bid requests aggregated by callout status.
    #[serde(rename="calloutStatusRows")]
    
    pub callout_status_rows: Option<Vec<CalloutStatusRow>>,
    /// A token to retrieve the next page of results. Pass this value in the ListFilteredBidRequestsRequest.pageToken field in the subsequent call to the filteredBidRequests.list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListFilteredBidRequestsResponse {}


/// Response message for listing all reasons that bids were filtered from the auction.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [accounts filter sets filtered bids list bidders](BidderAccountFilterSetFilteredBidListCall) (response)
/// * [filter sets filtered bids list bidders](BidderFilterSetFilteredBidListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFilteredBidsResponse {
    /// List of rows, with counts of filtered bids aggregated by filtering reason (for example, creative status).
    #[serde(rename="creativeStatusRows")]
    
    pub creative_status_rows: Option<Vec<CreativeStatusRow>>,
    /// A token to retrieve the next page of results. Pass this value in the ListFilteredBidsRequest.pageToken field in the subsequent call to the filteredBids.list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListFilteredBidsResponse {}


/// Response message for listing the metrics that are measured in number of impressions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [accounts filter sets impression metrics list bidders](BidderAccountFilterSetImpressionMetricListCall) (response)
/// * [filter sets impression metrics list bidders](BidderFilterSetImpressionMetricListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListImpressionMetricsResponse {
    /// List of rows, each containing a set of impression metrics.
    #[serde(rename="impressionMetricsRows")]
    
    pub impression_metrics_rows: Option<Vec<ImpressionMetricsRow>>,
    /// A token to retrieve the next page of results. Pass this value in the ListImpressionMetricsRequest.pageToken field in the subsequent call to the impressionMetrics.list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListImpressionMetricsResponse {}


/// Response message for listing all reasons that bids lost in the auction.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [accounts filter sets losing bids list bidders](BidderAccountFilterSetLosingBidListCall) (response)
/// * [filter sets losing bids list bidders](BidderFilterSetLosingBidListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLosingBidsResponse {
    /// List of rows, with counts of losing bids aggregated by loss reason (for example, creative status).
    #[serde(rename="creativeStatusRows")]
    
    pub creative_status_rows: Option<Vec<CreativeStatusRow>>,
    /// A token to retrieve the next page of results. Pass this value in the ListLosingBidsRequest.pageToken field in the subsequent call to the losingBids.list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLosingBidsResponse {}


/// Response message for listing all reasons for which a buyer was not billed for a winning bid.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [accounts filter sets non billable winning bids list bidders](BidderAccountFilterSetNonBillableWinningBidListCall) (response)
/// * [filter sets non billable winning bids list bidders](BidderFilterSetNonBillableWinningBidListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNonBillableWinningBidsResponse {
    /// A token to retrieve the next page of results. Pass this value in the ListNonBillableWinningBidsRequest.pageToken field in the subsequent call to the nonBillableWinningBids.list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of rows, with counts of bids not billed aggregated by reason.
    #[serde(rename="nonBillableWinningBidStatusRows")]
    
    pub non_billable_winning_bid_status_rows: Option<Vec<NonBillableWinningBidStatusRow>>,
}

impl client::ResponseResult for ListNonBillableWinningBidsResponse {}


/// Response message for listing products visible to the buyer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [products list accounts](AccountProductListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListProductsResponse {
    /// List pagination support.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of matching products at their head revision number.
    
    pub products: Option<Vec<Product>>,
}

impl client::ResponseResult for ListProductsResponse {}


/// Response message for listing proposals.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [finalized proposals list accounts](AccountFinalizedProposalListCall) (response)
/// * [proposals list accounts](AccountProposalListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListProposalsResponse {
    /// Continuation token for fetching the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of proposals.
    
    pub proposals: Option<Vec<Proposal>>,
}

impl client::ResponseResult for ListProposalsResponse {}


/// Response message for profiles visible to the buyer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [publisher profiles list accounts](AccountPublisherProfileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPublisherProfilesResponse {
    /// List pagination support
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of matching publisher profiles.
    #[serde(rename="publisherProfiles")]
    
    pub publisher_profiles: Option<Vec<PublisherProfile>>,
}

impl client::ResponseResult for ListPublisherProfilesResponse {}


/// Output only. The Geo criteria the restriction applies to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationContext {
    /// IDs representing the geo location for this context. Refer to the [geo-table.csv](https://storage.googleapis.com/adx-rtb-dictionaries/geo-table.csv) file for different geo criteria IDs.
    #[serde(rename="geoCriteriaIds")]
    
    pub geo_criteria_ids: Option<Vec<i32>>,
}

impl client::Part for LocationContext {}


/// Targeting represents different criteria that can be used by advertisers to target ad inventory. For example, they can choose to target ad requests only if the user is in the US. Multiple types of targeting are always applied as a logical AND, unless noted otherwise.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MarketplaceTargeting {
    /// Geo criteria IDs to be included/excluded.
    #[serde(rename="geoTargeting")]
    
    pub geo_targeting: Option<CriteriaTargeting>,
    /// Inventory sizes to be included/excluded.
    #[serde(rename="inventorySizeTargeting")]
    
    pub inventory_size_targeting: Option<InventorySizeTargeting>,
    /// Placement targeting information, for example, URL, mobile applications.
    #[serde(rename="placementTargeting")]
    
    pub placement_targeting: Option<PlacementTargeting>,
    /// Technology targeting information, for example, operating system, device category.
    #[serde(rename="technologyTargeting")]
    
    pub technology_targeting: Option<TechnologyTargeting>,
    /// Video targeting information.
    #[serde(rename="videoTargeting")]
    
    pub video_targeting: Option<VideoTargeting>,
}

impl client::Part for MarketplaceTargeting {}


/// A metric value, with an expected value and a variance; represents a count that may be either exact or estimated (for example, when sampled).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricValue {
    /// The expected value of the metric.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub value: Option<i64>,
    /// The variance (for example, square of the standard deviation) of the metric value. If value is exact, variance is 0. Can be used to calculate margin of error as a percentage of value, using the following formula, where Z is the standard constant that depends on the preferred size of the confidence interval (for example, for 90% confidence interval, use Z = 1.645): marginOfError = 100 * Z * sqrt(variance) / value
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub variance: Option<i64>,
}

impl client::Part for MetricValue {}


/// Mobile application targeting settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MobileApplicationTargeting {
    /// Publisher owned apps to be targeted or excluded by the publisher to display the ads in.
    #[serde(rename="firstPartyTargeting")]
    
    pub first_party_targeting: Option<FirstPartyMobileApplicationTargeting>,
}

impl client::Part for MobileApplicationTargeting {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Money {
    /// The three-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    
    pub nanos: Option<i32>,
    /// The whole units of the amount. For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub units: Option<i64>,
}

impl client::Part for Money {}


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
    /// The URL to the app store to purchase/download the promoted app.
    #[serde(rename="storeUrl")]
    
    pub store_url: Option<String>,
    /// The URL to fetch a native video ad.
    #[serde(rename="videoUrl")]
    
    pub video_url: Option<String>,
}

impl client::Part for NativeContent {}


/// The number of winning bids with the specified dimension values for which the buyer was not billed, as described by the specified status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NonBillableWinningBidStatusRow {
    /// The number of bids with the specified status.
    #[serde(rename="bidCount")]
    
    pub bid_count: Option<MetricValue>,
    /// The values of all dimensions associated with metric values in this row.
    #[serde(rename="rowDimensions")]
    
    pub row_dimensions: Option<RowDimensions>,
    /// The status specifying why the winning bids were not billed.
    
    pub status: Option<String>,
}

impl client::Part for NonBillableWinningBidStatusRow {}


/// Terms for Private Auctions. Note that Private Auctions can be created only by the seller, but they can be returned in a get or list request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NonGuaranteedAuctionTerms {
    /// True if open auction buyers are allowed to compete with invited buyers in this private auction.
    #[serde(rename="autoOptimizePrivateAuction")]
    
    pub auto_optimize_private_auction: Option<bool>,
    /// Reserve price for the specified buyer.
    #[serde(rename="reservePricesPerBuyer")]
    
    pub reserve_prices_per_buyer: Option<Vec<PricePerBuyer>>,
}

impl client::Part for NonGuaranteedAuctionTerms {}


/// Terms for Preferred Deals.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NonGuaranteedFixedPriceTerms {
    /// Fixed price for the specified buyer.
    #[serde(rename="fixedPrices")]
    
    pub fixed_prices: Option<Vec<PricePerBuyer>>,
}

impl client::Part for NonGuaranteedFixedPriceTerms {}


/// A proposal may be associated to several notes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals add note accounts](AccountProposalAddNoteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Note {
    /// Output only. The timestamp for when this note was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The role of the person (buyer/seller) creating the note.
    #[serde(rename="creatorRole")]
    
    pub creator_role: Option<String>,
    /// The actual note to attach. (max-length: 1024 unicode code units) Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error.
    
    pub note: Option<String>,
    /// Output only. The unique ID for the note.
    #[serde(rename="noteId")]
    
    pub note_id: Option<String>,
    /// Output only. The revision number of the proposal when the note is created.
    #[serde(rename="proposalRevision")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub proposal_revision: Option<i64>,
}

impl client::ResponseResult for Note {}


/// Represents targeting information for operating systems.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperatingSystemTargeting {
    /// IDs of operating systems to be included/excluded.
    #[serde(rename="operatingSystemCriteria")]
    
    pub operating_system_criteria: Option<CriteriaTargeting>,
    /// IDs of operating system versions to be included/excluded.
    #[serde(rename="operatingSystemVersionCriteria")]
    
    pub operating_system_version_criteria: Option<CriteriaTargeting>,
}

impl client::Part for OperatingSystemTargeting {}


/// Request message to pause serving for finalized deals.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [finalized proposals pause accounts](AccountFinalizedProposalPauseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PauseProposalDealsRequest {
    /// The external_deal_id's of the deals to be paused. If empty, all the deals in the proposal will be paused.
    #[serde(rename="externalDealIds")]
    
    pub external_deal_ids: Option<Vec<String>>,
    /// The reason why the deals are being paused. This human readable message will be displayed in the seller's UI. (Max length: 1000 unicode code units.)
    
    pub reason: Option<String>,
}

impl client::RequestValue for PauseProposalDealsRequest {}


/// Request message to pause serving for an already-finalized proposal.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals pause accounts](AccountProposalPauseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PauseProposalRequest {
    /// The reason why the proposal is being paused. This human readable message will be displayed in the seller's UI. (Max length: 1000 unicode code units.)
    
    pub reason: Option<String>,
}

impl client::RequestValue for PauseProposalRequest {}


/// Represents targeting about where the ads can appear, for example, certain sites or mobile applications. Different placement targeting types will be logically OR'ed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlacementTargeting {
    /// Mobile application targeting information in a deal. This doesn't apply to Auction Packages.
    #[serde(rename="mobileApplicationTargeting")]
    
    pub mobile_application_targeting: Option<MobileApplicationTargeting>,
    /// URLs to be included/excluded.
    #[serde(rename="urlTargeting")]
    
    pub url_targeting: Option<UrlTargeting>,
}

impl client::Part for PlacementTargeting {}


/// Output only. The type of platform the restriction applies to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlatformContext {
    /// The platforms this restriction applies to.
    
    pub platforms: Option<Vec<String>>,
}

impl client::Part for PlatformContext {}


/// Represents a price and a pricing type for a product / deal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Price {
    /// The actual price with currency specified.
    
    pub amount: Option<Money>,
    /// The pricing type for the deal/product. (default: CPM)
    #[serde(rename="pricingType")]
    
    pub pricing_type: Option<String>,
}

impl client::Part for Price {}


/// Used to specify pricing rules for buyers/advertisers. Each PricePerBuyer in a product can become 0 or 1 deals. To check if there is a PricePerBuyer for a particular buyer or buyer/advertiser pair, we look for the most specific matching rule - we first look for a rule matching the buyer and advertiser, next a rule with the buyer but an empty advertiser list, and otherwise look for a matching rule where no buyer is set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PricePerBuyer {
    /// The list of advertisers for this price when associated with this buyer. If empty, all advertisers with this buyer pay this price.
    #[serde(rename="advertiserIds")]
    
    pub advertiser_ids: Option<Vec<String>>,
    /// The buyer who will pay this price. If unset, all buyers can pay this price (if the advertisers match, and there's no more specific rule matching the buyer).
    
    pub buyer: Option<Buyer>,
    /// The specified price.
    
    pub price: Option<Price>,
}

impl client::Part for PricePerBuyer {}


/// Buyers are allowed to store certain types of private data in a proposal/deal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateData {
    /// A buyer or seller specified reference ID. This can be queried in the list operations (max-length: 1024 unicode code units).
    #[serde(rename="referenceId")]
    
    pub reference_id: Option<String>,
}

impl client::Part for PrivateData {}


/// A product is a segment of inventory that a seller wants to sell. It is associated with certain terms and targeting information which helps the buyer know more about the inventory.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [products get accounts](AccountProductGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    /// The proposed end time for the deal. The field will be truncated to the order of seconds during serving.
    #[serde(rename="availableEndTime")]
    
    pub available_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Inventory availability dates. The start time will be truncated to seconds during serving. Thus, a field specified as 3:23:34.456 (HH:mm:ss.SSS) will be truncated to 3:23:34 when serving.
    #[serde(rename="availableStartTime")]
    
    pub available_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Creation time.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional contact information for the creator of this product.
    #[serde(rename="creatorContacts")]
    
    pub creator_contacts: Option<Vec<ContactInformation>>,
    /// The display name for this product as set by the seller.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// If the creator has already signed off on the product, then the buyer can finalize the deal by accepting the product as is. When copying to a proposal, if any of the terms are changed, then auto_finalize is automatically set to false.
    #[serde(rename="hasCreatorSignedOff")]
    
    pub has_creator_signed_off: Option<bool>,
    /// The unique ID for the product.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The revision number of the product (auto-assigned by Marketplace).
    #[serde(rename="productRevision")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub product_revision: Option<i64>,
    /// An ID which can be used by the Publisher Profile API to get more information about the seller that created this product.
    #[serde(rename="publisherProfileId")]
    
    pub publisher_profile_id: Option<String>,
    /// Information about the seller that created this product.
    
    pub seller: Option<Seller>,
    /// The syndication product associated with the deal.
    #[serde(rename="syndicationProduct")]
    
    pub syndication_product: Option<String>,
    /// Targeting that is shared between the buyer and the seller. Each targeting criterion has a specified key and for each key there is a list of inclusion value or exclusion values.
    #[serde(rename="targetingCriterion")]
    
    pub targeting_criterion: Option<Vec<TargetingCriteria>>,
    /// The negotiable terms of the deal.
    
    pub terms: Option<DealTerms>,
    /// Time of last update.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The web-property code for the seller. This needs to be copied as is when adding a new deal to a proposal.
    #[serde(rename="webPropertyCode")]
    
    pub web_property_code: Option<String>,
}

impl client::ResponseResult for Product {}


/// Represents a proposal in the Marketplace. A proposal is the unit of negotiation between a seller and a buyer and contains deals which are served. Note: You can’t update, create, or otherwise modify Private Auction deals through the API. Fields are updatable unless noted otherwise.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [finalized proposals pause accounts](AccountFinalizedProposalPauseCall) (response)
/// * [finalized proposals resume accounts](AccountFinalizedProposalResumeCall) (response)
/// * [proposals accept accounts](AccountProposalAcceptCall) (response)
/// * [proposals cancel negotiation accounts](AccountProposalCancelNegotiationCall) (response)
/// * [proposals complete setup accounts](AccountProposalCompleteSetupCall) (response)
/// * [proposals create accounts](AccountProposalCreateCall) (request|response)
/// * [proposals get accounts](AccountProposalGetCall) (response)
/// * [proposals pause accounts](AccountProposalPauseCall) (response)
/// * [proposals resume accounts](AccountProposalResumeCall) (response)
/// * [proposals update accounts](AccountProposalUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Proposal {
    /// Output only. Reference to the buyer that will get billed for this proposal.
    #[serde(rename="billedBuyer")]
    
    pub billed_buyer: Option<Buyer>,
    /// Reference to the buyer on the proposal. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error.
    
    pub buyer: Option<Buyer>,
    /// Contact information for the buyer.
    #[serde(rename="buyerContacts")]
    
    pub buyer_contacts: Option<Vec<ContactInformation>>,
    /// Private data for buyer. (hidden from seller).
    #[serde(rename="buyerPrivateData")]
    
    pub buyer_private_data: Option<PrivateData>,
    /// The deals associated with this proposal. For Private Auction proposals (whose deals have NonGuaranteedAuctionTerms), there will only be one deal.
    
    pub deals: Option<Vec<Deal>>,
    /// The name for the proposal.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. True if the proposal is being renegotiated.
    #[serde(rename="isRenegotiating")]
    
    pub is_renegotiating: Option<bool>,
    /// Output only. True, if the buyside inventory setup is complete for this proposal.
    #[serde(rename="isSetupComplete")]
    
    pub is_setup_complete: Option<bool>,
    /// Output only. The role of the last user that either updated the proposal or left a comment.
    #[serde(rename="lastUpdaterOrCommentorRole")]
    
    pub last_updater_or_commentor_role: Option<String>,
    /// Output only. The notes associated with this proposal.
    
    pub notes: Option<Vec<Note>>,
    /// Output only. Indicates whether the buyer/seller created the proposal.
    #[serde(rename="originatorRole")]
    
    pub originator_role: Option<String>,
    /// Output only. Private auction ID if this proposal is a private auction proposal.
    #[serde(rename="privateAuctionId")]
    
    pub private_auction_id: Option<String>,
    /// Output only. The unique ID of the proposal.
    #[serde(rename="proposalId")]
    
    pub proposal_id: Option<String>,
    /// Output only. The revision number for the proposal. Each update to the proposal or the deal causes the proposal revision number to auto-increment. The buyer keeps track of the last revision number they know of and pass it in when making an update. If the head revision number on the server has since incremented, then an ABORTED error is returned during the update operation to let the buyer know that a subsequent update was made.
    #[serde(rename="proposalRevision")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub proposal_revision: Option<i64>,
    /// Output only. The current state of the proposal.
    #[serde(rename="proposalState")]
    
    pub proposal_state: Option<String>,
    /// Reference to the seller on the proposal. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error.
    
    pub seller: Option<Seller>,
    /// Output only. Contact information for the seller.
    #[serde(rename="sellerContacts")]
    
    pub seller_contacts: Option<Vec<ContactInformation>>,
    /// Output only. The terms and conditions set by the publisher for this proposal.
    #[serde(rename="termsAndConditions")]
    
    pub terms_and_conditions: Option<String>,
    /// Output only. The time when the proposal was last revised.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Proposal {}
impl client::ResponseResult for Proposal {}


/// Represents a publisher profile (https://support.google.com/admanager/answer/6035806) in Marketplace. All fields are read only. All string fields are free-form text entered by the publisher unless noted otherwise.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [publisher profiles get accounts](AccountPublisherProfileGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublisherProfile {
    /// Description on the publisher's audience.
    #[serde(rename="audienceDescription")]
    
    pub audience_description: Option<String>,
    /// Statement explaining what's unique about publisher's business, and why buyers should partner with the publisher.
    #[serde(rename="buyerPitchStatement")]
    
    pub buyer_pitch_statement: Option<String>,
    /// Contact information for direct reservation deals. This is free text entered by the publisher and may include information like names, phone numbers and email addresses.
    #[serde(rename="directDealsContact")]
    
    pub direct_deals_contact: Option<String>,
    /// Name of the publisher profile.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The list of domains represented in this publisher profile. Empty if this is a parent profile. These are top private domains, meaning that these will not contain a string like "photos.google.co.uk/123", but will instead contain "google.co.uk".
    
    pub domains: Option<Vec<String>>,
    /// URL to publisher's Google+ page.
    #[serde(rename="googlePlusUrl")]
    
    pub google_plus_url: Option<String>,
    /// Indicates if this profile is the parent profile of the seller. A parent profile represents all the inventory from the seller, as opposed to child profile that is created to brand a portion of inventory. One seller should have only one parent publisher profile, and can have multiple child profiles. Publisher profiles for the same seller will have same value of field google.ads.adexchange.buyer.v2beta1.PublisherProfile.seller. See https://support.google.com/admanager/answer/6035806 for details.
    #[serde(rename="isParent")]
    
    pub is_parent: Option<bool>,
    /// A Google public URL to the logo for this publisher profile. The logo is stored as a PNG, JPG, or GIF image.
    #[serde(rename="logoUrl")]
    
    pub logo_url: Option<String>,
    /// URL to additional marketing and sales materials.
    #[serde(rename="mediaKitUrl")]
    
    pub media_kit_url: Option<String>,
    /// The list of apps represented in this publisher profile. Empty if this is a parent profile.
    #[serde(rename="mobileApps")]
    
    pub mobile_apps: Option<Vec<PublisherProfileMobileApplication>>,
    /// Overview of the publisher.
    
    pub overview: Option<String>,
    /// Contact information for programmatic deals. This is free text entered by the publisher and may include information like names, phone numbers and email addresses.
    #[serde(rename="programmaticDealsContact")]
    
    pub programmatic_deals_contact: Option<String>,
    /// Unique ID for publisher profile.
    #[serde(rename="publisherProfileId")]
    
    pub publisher_profile_id: Option<String>,
    /// URL to a publisher rate card.
    #[serde(rename="rateCardInfoUrl")]
    
    pub rate_card_info_url: Option<String>,
    /// URL to a sample content page.
    #[serde(rename="samplePageUrl")]
    
    pub sample_page_url: Option<String>,
    /// Seller of the publisher profile.
    
    pub seller: Option<Seller>,
    /// Up to three key metrics and rankings. Max 100 characters each. For example "#1 Mobile News Site for 20 Straight Months".
    #[serde(rename="topHeadlines")]
    
    pub top_headlines: Option<Vec<String>>,
}

impl client::ResponseResult for PublisherProfile {}


/// A mobile application that contains a external app ID, name, and app store.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublisherProfileMobileApplication {
    /// The app store the app belongs to.
    #[serde(rename="appStore")]
    
    pub app_store: Option<String>,
    /// The external ID for the app from its app store.
    #[serde(rename="externalAppId")]
    
    pub external_app_id: Option<String>,
    /// The name of the app.
    
    pub name: Option<String>,
}

impl client::Part for PublisherProfileMobileApplication {}


/// An open-ended realtime time range specified by the start timestamp. For filter sets that specify a realtime time range RTB metrics continue to be aggregated throughout the lifetime of the filter set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RealtimeTimeRange {
    /// The start timestamp of the real-time RTB metrics aggregation.
    #[serde(rename="startTimestamp")]
    
    pub start_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for RealtimeTimeRange {}


/// A relative date range, specified by an offset and a duration. The supported range of dates begins 30 days before today and ends today, for example, the limits for these values are: offset_days >= 0 duration_days >= 1 offset_days + duration_days <= 30
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelativeDateRange {
    /// The number of days in the requested date range, for example, for a range spanning today: 1. For a range spanning the last 7 days: 7.
    #[serde(rename="durationDays")]
    
    pub duration_days: Option<i32>,
    /// The end date of the filter set, specified as the number of days before today, for example, for a range where the last date is today: 0.
    #[serde(rename="offsetDays")]
    
    pub offset_days: Option<i32>,
}

impl client::Part for RelativeDateRange {}


/// A request for removing the association between a deal and a creative.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives deal associations remove accounts](AccountCreativeDealAssociationRemoveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveDealAssociationRequest {
    /// The association between a creative and a deal that should be removed.
    
    pub association: Option<CreativeDealAssociation>,
}

impl client::RequestValue for RemoveDealAssociationRequest {}


/// Request message to resume (unpause) serving for already-finalized deals.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [finalized proposals resume accounts](AccountFinalizedProposalResumeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResumeProposalDealsRequest {
    /// The external_deal_id's of the deals to resume. If empty, all the deals in the proposal will be resumed.
    #[serde(rename="externalDealIds")]
    
    pub external_deal_ids: Option<Vec<String>>,
}

impl client::RequestValue for ResumeProposalDealsRequest {}


/// Request message to resume (unpause) serving for an already-finalized proposal.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals resume accounts](AccountProposalResumeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResumeProposalRequest { _never_set: Option<bool> }

impl client::RequestValue for ResumeProposalRequest {}


/// A response may include multiple rows, breaking down along various dimensions. Encapsulates the values of all dimensions for a given row.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RowDimensions {
    /// The publisher identifier for this row, if a breakdown by [BreakdownDimension.PUBLISHER_IDENTIFIER](https://developers.google.com/authorized-buyers/apis/reference/rest/v2beta1/bidders.accounts.filterSets#FilterSet.BreakdownDimension) was requested.
    #[serde(rename="publisherIdentifier")]
    
    pub publisher_identifier: Option<String>,
    /// The time interval that this row represents.
    #[serde(rename="timeInterval")]
    
    pub time_interval: Option<TimeInterval>,
}

impl client::Part for RowDimensions {}


/// Output only. A security context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecurityContext {
    /// The security types in this context.
    
    pub securities: Option<Vec<String>>,
}

impl client::Part for SecurityContext {}


/// Represents a seller of inventory. Each seller is identified by a unique Ad Manager account ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Seller {
    /// The unique ID for the seller. The seller fills in this field. The seller account ID is then available to buyer in the product.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Output only. Ad manager network code for the seller.
    #[serde(rename="subAccountId")]
    
    pub sub_account_id: Option<String>,
}

impl client::Part for Seller {}


/// The serving context for this restriction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServingContext {
    /// Matches all contexts.
    
    pub all: Option<String>,
    /// Matches impressions for a particular app type.
    #[serde(rename="appType")]
    
    pub app_type: Option<AppContext>,
    /// Matches impressions for a particular auction type.
    #[serde(rename="auctionType")]
    
    pub auction_type: Option<AuctionContext>,
    /// Matches impressions coming from users *or* publishers in a specific location.
    
    pub location: Option<LocationContext>,
    /// Matches impressions coming from a particular platform.
    
    pub platform: Option<PlatformContext>,
    /// Matches impressions for a particular security type.
    #[serde(rename="securityType")]
    
    pub security_type: Option<SecurityContext>,
}

impl client::Part for ServingContext {}


/// Output only. A representation of the status of an ad in a specific context. A context here relates to where something ultimately serves (for example, a user or publisher geo, a platform, an HTTPS versus HTTP request, or the type of auction).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServingRestriction {
    /// The contexts for the restriction.
    
    pub contexts: Option<Vec<ServingContext>>,
    /// Disapproval bound to this restriction. Only present if status=DISAPPROVED. Can be used to filter the response of the creatives.list method.
    
    pub disapproval: Option<Disapproval>,
    /// Any disapprovals bound to this restriction. Only present if status=DISAPPROVED. Can be used to filter the response of the creatives.list method. Deprecated; use disapproval field instead.
    #[serde(rename="disapprovalReasons")]
    
    pub disapproval_reasons: Option<Vec<Disapproval>>,
    /// The status of the creative in this context (for example, it has been explicitly disapproved or is pending review).
    
    pub status: Option<String>,
}

impl client::Part for ServingRestriction {}


/// Message depicting the size of the creative. The units of width and height depend on the type of the targeting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Size {
    /// The height of the creative.
    
    pub height: Option<i32>,
    /// The width of the creative
    
    pub width: Option<i32>,
}

impl client::Part for Size {}


/// A request for stopping notifications for changes to creative Status.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives stop watching accounts](AccountCreativeStopWatchingCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StopWatchingCreativeRequest { _never_set: Option<bool> }

impl client::RequestValue for StopWatchingCreativeRequest {}


/// Advertisers can target different attributes of an ad slot. For example, they can choose to show ads only if the user is in the U.S. Such targeting criteria can be specified as part of Shared Targeting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetingCriteria {
    /// The list of values to exclude from targeting. Each value is AND'd together.
    
    pub exclusions: Option<Vec<TargetingValue>>,
    /// The list of value to include as part of the targeting. Each value is OR'd together.
    
    pub inclusions: Option<Vec<TargetingValue>>,
    /// The key representing the shared targeting criterion. Targeting criteria defined by Google ad servers will begin with GOOG_. Third parties may define their own keys. A list of permissible keys along with the acceptable values will be provided as part of the external documentation.
    
    pub key: Option<String>,
}

impl client::Part for TargetingCriteria {}


/// A polymorphic targeting value used as part of Shared Targeting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetingValue {
    /// The creative size value to include/exclude. Filled in when key = GOOG_CREATIVE_SIZE
    #[serde(rename="creativeSizeValue")]
    
    pub creative_size_value: Option<CreativeSize>,
    /// The daypart targeting to include / exclude. Filled in when the key is GOOG_DAYPART_TARGETING. The definition of this targeting is derived from the structure used by Ad Manager.
    #[serde(rename="dayPartTargetingValue")]
    
    pub day_part_targeting_value: Option<DayPartTargeting>,
    /// The long value to include/exclude.
    #[serde(rename="longValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub long_value: Option<i64>,
    /// The string value to include/exclude.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
}

impl client::Part for TargetingValue {}


/// Represents targeting about various types of technology.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TechnologyTargeting {
    /// IDs of device capabilities to be included/excluded.
    #[serde(rename="deviceCapabilityTargeting")]
    
    pub device_capability_targeting: Option<CriteriaTargeting>,
    /// IDs of device categories to be included/excluded.
    #[serde(rename="deviceCategoryTargeting")]
    
    pub device_category_targeting: Option<CriteriaTargeting>,
    /// Operating system related targeting information.
    #[serde(rename="operatingSystemTargeting")]
    
    pub operating_system_targeting: Option<OperatingSystemTargeting>,
}

impl client::Part for TechnologyTargeting {}


/// An interval of time, with an absolute start and end.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeInterval {
    /// The timestamp marking the end of the range (exclusive) for which data is included.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The timestamp marking the start of the range (inclusive) for which data is included.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TimeInterval {}


/// Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value "24:00:00" for scenarios like business closing time.
    
    pub hours: Option<i32>,
    /// Minutes of hour of day. Must be from 0 to 59.
    
    pub minutes: Option<i32>,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    
    pub nanos: Option<i32>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds.
    
    pub seconds: Option<i32>,
}

impl client::Part for TimeOfDay {}


/// Represents a list of targeted and excluded URLs (for example, google.com). For Private Auction and AdX Preferred Deals, URLs are either included or excluded. For Programmatic Guaranteed and Preferred Deals, this doesn't apply.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlTargeting {
    /// A list of URLs to be excluded.
    #[serde(rename="excludedUrls")]
    
    pub excluded_urls: Option<Vec<String>>,
    /// A list of URLs to be included.
    #[serde(rename="targetedUrls")]
    
    pub targeted_urls: Option<Vec<String>>,
}

impl client::Part for UrlTargeting {}


/// Video content for a creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoContent {
    /// The URL to fetch a video ad.
    #[serde(rename="videoUrl")]
    
    pub video_url: Option<String>,
    /// The contents of a VAST document for a video ad. This document should conform to the VAST 2.0 or 3.0 standard.
    #[serde(rename="videoVastXml")]
    
    pub video_vast_xml: Option<String>,
}

impl client::Part for VideoContent {}


/// Represents targeting information about video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoTargeting {
    /// A list of video positions to be excluded. Position types can either be included or excluded (XOR).
    #[serde(rename="excludedPositionTypes")]
    
    pub excluded_position_types: Option<Vec<String>>,
    /// A list of video positions to be included. When the included list is present, the excluded list must be empty. When the excluded list is present, the included list must be empty.
    #[serde(rename="targetedPositionTypes")]
    
    pub targeted_position_types: Option<Vec<String>>,
}

impl client::Part for VideoTargeting {}


/// A request for watching changes to creative Status.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives watch accounts](AccountCreativeWatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WatchCreativeRequest {
    /// The Pub/Sub topic to publish notifications to. This topic must already exist and must give permission to ad-exchange-buyside-reports@google.com to write to the topic. This should be the full resource name in "projects/{project_id}/topics/{topic_id}" format.
    
    pub topic: Option<String>,
}

impl client::RequestValue for WatchCreativeRequest {}


