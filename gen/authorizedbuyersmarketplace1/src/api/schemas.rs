use super::*;
/// Request to accept a proposal. Accepting a proposal implies acceptance of the publisher terms_and_conditions, if any.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals accept buyers](BuyerProposalAcceptCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AcceptProposalRequest {
    /// The last known client revision number of the proposal.
    #[serde(rename="proposalRevision")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub proposal_revision: Option<i64>,
}

impl client::RequestValue for AcceptProposalRequest {}


/// Request message for activating a client.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clients activate buyers](BuyerClientActivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivateClientRequest { _never_set: Option<bool> }

impl client::RequestValue for ActivateClientRequest {}


/// Request message for activating a client user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clients users activate buyers](BuyerClientUserActivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivateClientUserRequest { _never_set: Option<bool> }

impl client::RequestValue for ActivateClientUserRequest {}


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
    /// The type of the ad slot size.
    #[serde(rename="type")]
    
    pub type_: Option<AdSizeTypeEnum>,
    /// The width of the ad slot in pixels. This field will be present only when size type is `PIXEL`.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub width: Option<i64>,
}

impl client::Part for AdSize {}


/// Request message for adding creative to be used in the bidding process for the finalized deal.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [finalized deals add creative buyers](BuyerFinalizedDealAddCreativeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddCreativeRequest {
    /// Name of the creative to add to the finalized deal, in the format `buyers/{buyerAccountId}/creatives/{creativeId}`. See creative.name.
    
    pub creative: Option<String>,
}

impl client::RequestValue for AddCreativeRequest {}


/// Request to add a note.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals add note buyers](BuyerProposalAddNoteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddNoteRequest {
    /// The note to add.
    
    pub note: Option<Note>,
}

impl client::RequestValue for AddNoteRequest {}


/// Defines a segment of inventory that buyer wants to buy. It’s created by buyer and could be shared with multiple buyers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [auction packages get buyers](BuyerAuctionPackageGetCall) (response)
/// * [auction packages subscribe buyers](BuyerAuctionPackageSubscribeCall) (response)
/// * [auction packages subscribe clients buyers](BuyerAuctionPackageSubscribeClientCall) (response)
/// * [auction packages unsubscribe buyers](BuyerAuctionPackageUnsubscribeCall) (response)
/// * [auction packages unsubscribe clients buyers](BuyerAuctionPackageUnsubscribeClientCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuctionPackage {
    /// Output only. Time the auction package was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The buyer that created this auction package. Format: `buyers/{buyerAccountId}`
    
    pub creator: Option<String>,
    /// Output only. A description of the auction package.
    
    pub description: Option<String>,
    /// The display_name assigned to the auction package.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Immutable. The unique identifier for the auction package. Format: `buyers/{accountId}/auctionPackages/{auctionPackageId}` The auction_package_id part of name is sent in the BidRequest to all RTB bidders and is returned as deal_id by the bidder in the BidResponse.
    
    pub name: Option<String>,
    /// Output only. The list of clients of the current buyer that are subscribed to the AuctionPackage. Format: `buyers/{buyerAccountId}/clients/{clientAccountId}`
    #[serde(rename="subscribedClients")]
    
    pub subscribed_clients: Option<Vec<String>>,
    /// Output only. Time the auction package was last updated. This value is only increased when this auction package is updated but never when a buyer subscribed.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for AuctionPackage {}


/// Request message for batch updating deals.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals deals batch update buyers](BuyerProposalDealBatchUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateDealsRequest {
    /// Required. List of request messages to update deals.
    
    pub requests: Option<Vec<UpdateDealRequest>>,
}

impl client::RequestValue for BatchUpdateDealsRequest {}


/// Response message for batch updating deals.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals deals batch update buyers](BuyerProposalDealBatchUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateDealsResponse {
    /// Deals updated.
    
    pub deals: Option<Vec<Deal>>,
}

impl client::ResponseResult for BatchUpdateDealsResponse {}


/// Request to cancel an ongoing negotiation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals cancel negotiation buyers](BuyerProposalCancelNegotiationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelNegotiationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelNegotiationRequest {}


/// A client represents an agency, a brand, or an advertiser customer of the buyer. Based on the client’s role, its client users will have varying levels of restricted access to the Marketplace and certain other sections of the Authorized Buyers UI.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clients activate buyers](BuyerClientActivateCall) (response)
/// * [clients create buyers](BuyerClientCreateCall) (request|response)
/// * [clients deactivate buyers](BuyerClientDeactivateCall) (response)
/// * [clients get buyers](BuyerClientGetCall) (response)
/// * [clients patch buyers](BuyerClientPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Client {
    /// Required. Display name shown to publishers. Must be unique for clients without partnerClientId specified. Maximum length of 255 characters is allowed.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The resource name of the client. Format: `buyers/{accountId}/clients/{clientAccountId}`
    
    pub name: Option<String>,
    /// Arbitrary unique identifier provided by the buyer. This field can be used to associate a client with an identifier in the namespace of the buyer, lookup clients by that identifier and verify whether an Authorized Buyers account of the client already exists. If present, must be unique across all the clients.
    #[serde(rename="partnerClientId")]
    
    pub partner_client_id: Option<String>,
    /// Required. The role assigned to the client. Each role implies a set of permissions granted to the client.
    
    pub role: Option<ClientRoleEnum>,
    /// Whether the client will be visible to sellers.
    #[serde(rename="sellerVisible")]
    
    pub seller_visible: Option<bool>,
    /// Output only. The state of the client.
    
    pub state: Option<ClientStateEnum>,
}

impl client::RequestValue for Client {}
impl client::ResponseResult for Client {}


/// A user of a client who has restricted access to the Marketplace and certain other sections of the Authorized Buyers UI based on the role granted to the associated client.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clients users activate buyers](BuyerClientUserActivateCall) (response)
/// * [clients users create buyers](BuyerClientUserCreateCall) (request|response)
/// * [clients users deactivate buyers](BuyerClientUserDeactivateCall) (response)
/// * [clients users get buyers](BuyerClientUserGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClientUser {
    /// Required. The client user's email address that has to be unique across all users for the same client.
    
    pub email: Option<String>,
    /// Output only. The resource name of the client user. Format: `buyers/{accountId}/clients/{clientAccountId}/users/{userId}`
    
    pub name: Option<String>,
    /// Output only. The state of the client user.
    
    pub state: Option<ClientUserStateEnum>,
}

impl client::RequestValue for ClientUser {}
impl client::ResponseResult for ClientUser {}


/// Contains information on how a buyer or seller can be reached.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Contact {
    /// The display_name of the contact.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Email address for the contact.
    
    pub email: Option<String>,
}

impl client::Part for Contact {}


/// Message captures data about the creatives in the deal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeRequirements {
    /// Output only. The format of the creative, only applicable for programmatic guaranteed and preferred deals.
    #[serde(rename="creativeFormat")]
    
    pub creative_format: Option<CreativeRequirementCreativeFormatEnum>,
    /// Output only. Specifies the creative pre-approval policy.
    #[serde(rename="creativePreApprovalPolicy")]
    
    pub creative_pre_approval_policy: Option<CreativeRequirementCreativePreApprovalPolicyEnum>,
    /// Output only. Specifies whether the creative is safeFrame compatible.
    #[serde(rename="creativeSafeFrameCompatibility")]
    
    pub creative_safe_frame_compatibility: Option<CreativeRequirementCreativeSafeFrameCompatibilityEnum>,
    /// Output only. The max duration of the video creative in milliseconds. only applicable for deals with video creatives.
    #[serde(rename="maxAdDurationMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_ad_duration_ms: Option<i64>,
    /// Output only. Specifies the creative source for programmatic deals. PUBLISHER means creative is provided by seller and ADVERTISER means creative is provided by the buyer.
    #[serde(rename="programmaticCreativeSource")]
    
    pub programmatic_creative_source: Option<CreativeRequirementProgrammaticCreativeSourceEnum>,
    /// Output only. Skippable video ads allow viewers to skip ads after 5 seconds. Only applicable for deals with video creatives.
    #[serde(rename="skippableAdType")]
    
    pub skippable_ad_type: Option<CreativeRequirementSkippableAdTypeEnum>,
}

impl client::Part for CreativeRequirements {}


/// Generic targeting used for targeting dimensions that contains a list of included and excluded numeric IDs. This cannot be filtered using list filter syntax.
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


/// Defines targeting for a period of time on a specific week day.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DayPart {
    /// Day of week for the period.
    #[serde(rename="dayOfWeek")]
    
    pub day_of_week: Option<DayPartDayOfWeekEnum>,
    /// Hours in 24 hour time between 0 and 24, inclusive. Note: 24 is logically equivalent to 0, but is supported since in some cases there may need to be differentiation made between midnight on one day and midnight on the next day. Accepted values for minutes are [0, 15, 30, 45]. 0 is the only acceptable minute value for hour 24. Seconds and nanos are ignored.
    #[serde(rename="endTime")]
    
    pub end_time: Option<TimeOfDay>,
    /// Hours in 24 hour time between 0 and 24, inclusive. Note: 24 is logically equivalent to 0, but is supported since in some cases there may need to be differentiation made between midnight on one day and midnight on the next day. Accepted values for minutes are [0, 15, 30, 45]. 0 is the only acceptable minute value for hour 24. Seconds and nanos are ignored.
    #[serde(rename="startTime")]
    
    pub start_time: Option<TimeOfDay>,
}

impl client::Part for DayPart {}


/// Represents Daypart targeting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DayPartTargeting {
    /// The targeted weekdays and times
    #[serde(rename="dayParts")]
    
    pub day_parts: Option<Vec<DayPart>>,
    /// The time zone type of the day parts
    #[serde(rename="timeZoneType")]
    
    pub time_zone_type: Option<DayPartTargetingTimeZoneTypeEnum>,
}

impl client::Part for DayPartTargeting {}


/// Request message for disabling a client.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clients deactivate buyers](BuyerClientDeactivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeactivateClientRequest { _never_set: Option<bool> }

impl client::RequestValue for DeactivateClientRequest {}


/// Request message for deactivating a client user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clients users deactivate buyers](BuyerClientUserDeactivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeactivateClientUserRequest { _never_set: Option<bool> }

impl client::RequestValue for DeactivateClientUserRequest {}


/// A deal represents a segment of inventory for displaying ads that contains the terms and targeting information that is used for serving as well as the deal stats and status. Note: A proposal may contain multiple deals.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals deals get buyers](BuyerProposalDealGetCall) (response)
/// * [proposals deals patch buyers](BuyerProposalDealPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Deal {
    /// Output only. When the client field is populated, this field refers to the buyer who creates and manages the client buyer and gets billed on behalf of the client buyer; when the buyer field is populated, this field is the same value as buyer. Format : `buyers/{buyerAccountId}`
    #[serde(rename="billedBuyer")]
    
    pub billed_buyer: Option<String>,
    /// Output only. Refers to a buyer in The Realtime-bidding API. Format: `buyers/{buyerAccountId}`
    
    pub buyer: Option<String>,
    /// Output only. Refers to a Client. Format: `buyers/{buyerAccountId}/clients/{clientAccountid}`
    
    pub client: Option<String>,
    /// Output only. The time of the deal creation.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Metadata about the creatives of this deal.
    #[serde(rename="creativeRequirements")]
    
    pub creative_requirements: Option<CreativeRequirements>,
    /// Output only. Type of deal.
    #[serde(rename="dealType")]
    
    pub deal_type: Option<DealDealTypeEnum>,
    /// Output only. Specifies the pacing set by the publisher.
    #[serde(rename="deliveryControl")]
    
    pub delivery_control: Option<DeliveryControl>,
    /// Output only. Free text description for the deal terms.
    
    pub description: Option<String>,
    /// Output only. The name of the deal. Maximum length of 255 unicode characters is allowed. Control characters are not allowed. Buyers cannot update this field. Note: Not to be confused with name, which is a unique identifier of the deal.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Specified by buyers in request for proposal (RFP) to notify publisher the total estimated spend for the proposal. Publishers will receive this information and send back proposed deals accordingly.
    #[serde(rename="estimatedGrossSpend")]
    
    pub estimated_gross_spend: Option<Money>,
    /// Proposed flight end time of the deal. This will generally be stored in a granularity of a second. A value is not necessary for Private Auction deals.
    #[serde(rename="flightEndTime")]
    
    pub flight_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Proposed flight start time of the deal. This will generally be stored in the granularity of one second since deal serving starts at seconds boundary. Any time specified with more granularity (for example, in milliseconds) will be truncated towards the start of time in seconds.
    #[serde(rename="flightStartTime")]
    
    pub flight_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. The unique identifier of the deal. Auto-generated by the server when a deal is created. Format: buyers/{accountId}/proposals/{proposalId}/deals/{dealId}
    
    pub name: Option<String>,
    /// The terms for preferred deals.
    #[serde(rename="preferredDealTerms")]
    
    pub preferred_deal_terms: Option<PreferredDealTerms>,
    /// The terms for private auction deals.
    #[serde(rename="privateAuctionTerms")]
    
    pub private_auction_terms: Option<PrivateAuctionTerms>,
    /// The terms for programmatic guaranteed deals.
    #[serde(rename="programmaticGuaranteedTerms")]
    
    pub programmatic_guaranteed_terms: Option<ProgrammaticGuaranteedTerms>,
    /// Output only. The revision number for the proposal and is the same value as proposal.proposal_revision. Each update to deal causes the proposal revision number to auto-increment. The buyer keeps track of the last revision number they know of and pass it in when making an update. If the head revision number on the server has since incremented, then an ABORTED error is returned during the update operation to let the buyer know that a subsequent update was made.
    #[serde(rename="proposalRevision")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub proposal_revision: Option<i64>,
    /// Immutable. Reference to the seller on the deal. Format: `buyers/{buyerAccountId}/publisherProfiles/{publisherProfileId}`
    #[serde(rename="publisherProfile")]
    
    pub publisher_profile: Option<String>,
    /// Output only. Time zone of the seller used to mark the boundaries of a day for daypart targeting and CPD billing.
    #[serde(rename="sellerTimeZone")]
    
    pub seller_time_zone: Option<TimeZone>,
    /// Specifies the subset of inventory targeted by the deal. Can be updated by the buyer before the deal is finalized.
    
    pub targeting: Option<MarketplaceTargeting>,
    /// Output only. The time when the deal was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Deal {}
impl client::ResponseResult for Deal {}


/// Information related to deal pausing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DealPausingInfo {
    /// The reason for the pausing of the deal; empty for active deals.
    #[serde(rename="pauseReason")]
    
    pub pause_reason: Option<String>,
    /// The party that first paused the deal; unspecified for active deals.
    #[serde(rename="pauseRole")]
    
    pub pause_role: Option<DealPausingInfoPauseRoleEnum>,
    /// Whether pausing is consented between buyer and seller for the deal.
    #[serde(rename="pausingConsented")]
    
    pub pausing_consented: Option<bool>,
}

impl client::Part for DealPausingInfo {}


/// Message contains details about how the deal will be paced.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryControl {
    /// Output only. Specifies roadblocking in a main companion lineitem.
    #[serde(rename="companionDeliveryType")]
    
    pub companion_delivery_type: Option<DeliveryControlCompanionDeliveryTypeEnum>,
    /// Output only. Specifies strategy to use for selecting a creative when multiple creatives of the same size are available.
    #[serde(rename="creativeRotationType")]
    
    pub creative_rotation_type: Option<DeliveryControlCreativeRotationTypeEnum>,
    /// Output only. Specifies how the impression delivery will be paced.
    #[serde(rename="deliveryRateType")]
    
    pub delivery_rate_type: Option<DeliveryControlDeliveryRateTypeEnum>,
    /// Output only. Specifies any frequency caps. Cannot be filtered within ListDealsRequest.
    #[serde(rename="frequencyCap")]
    
    pub frequency_cap: Option<Vec<FrequencyCap>>,
    /// Output only. Specifies the roadblocking type in display creatives.
    #[serde(rename="roadblockingType")]
    
    pub roadblocking_type: Option<DeliveryControlRoadblockingTypeEnum>,
}

impl client::Part for DeliveryControl {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clients users delete buyers](BuyerClientUserDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// A finalized deal is a snapshot of the deal when both buyer and seller accept the deal. The buyer or seller can update the deal after it’s been finalized and renegotiate on the deal targeting, terms and other fields, while at the same time the finalized snapshot of the deal can still be retrieved using this API. The finalized deal contains a copy of the deal as it existed when most recently finalized, as well as fields related to deal serving such as pause/resume status, RTB metrics, and more.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [finalized deals add creative buyers](BuyerFinalizedDealAddCreativeCall) (response)
/// * [finalized deals get buyers](BuyerFinalizedDealGetCall) (response)
/// * [finalized deals pause buyers](BuyerFinalizedDealPauseCall) (response)
/// * [finalized deals resume buyers](BuyerFinalizedDealResumeCall) (response)
/// * [finalized deals set ready to serve buyers](BuyerFinalizedDealSetReadyToServeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FinalizedDeal {
    /// A copy of the Deal made upon finalization. During renegotiation, this will reflect the last finalized deal before renegotiation was initiated.
    
    pub deal: Option<Deal>,
    /// Information related to deal pausing for the deal.
    #[serde(rename="dealPausingInfo")]
    
    pub deal_pausing_info: Option<DealPausingInfo>,
    /// Serving status of the deal.
    #[serde(rename="dealServingStatus")]
    
    pub deal_serving_status: Option<FinalizedDealDealServingStatusEnum>,
    /// The resource name of the finalized deal. Format: `buyers/{accountId}/finalizeddeals/{finalizedDealId}`
    
    pub name: Option<String>,
    /// Whether the Programmatic Guaranteed deal is ready for serving.
    #[serde(rename="readyToServe")]
    
    pub ready_to_serve: Option<bool>,
    /// Real-time bidding metrics for this deal.
    #[serde(rename="rtbMetrics")]
    
    pub rtb_metrics: Option<RtbMetrics>,
}

impl client::ResponseResult for FinalizedDeal {}


/// Represents a list of targeted and excluded mobile application IDs that publishers own. Android App ID, for example, com.google.android.apps.maps, can be found in Google Play Store URL. iOS App ID (which is a number) can be found at the end of iTunes store URL. First party mobile applications is either included or excluded.
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


/// Message contains details about publisher-set frequency caps of the delivery.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FrequencyCap {
    /// The maximum number of impressions that can be served to a user within the specified time period.
    #[serde(rename="maxImpressions")]
    
    pub max_impressions: Option<i32>,
    /// The time unit. Along with num_time_units defines the amount of time over which impressions per user are counted and capped.
    #[serde(rename="timeUnitType")]
    
    pub time_unit_type: Option<FrequencyCapTimeUnitTypeEnum>,
    /// The amount of time, in the units specified by time_unit_type. Defines the amount of time over which impressions per user are counted and capped.
    #[serde(rename="timeUnitsCount")]
    
    pub time_units_count: Option<i32>,
}

impl client::Part for FrequencyCap {}


/// Represents the size of an ad unit that can be targeted on a bid request.
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


/// Targeting of the inventory types a bid request can originate from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventoryTypeTargeting {
    /// The list of targeted inventory types for the bid request.
    #[serde(rename="inventoryTypes")]
    
    pub inventory_types: Option<Vec<InventoryTypeTargetingInventoryTypesEnum>>,
}

impl client::Part for InventoryTypeTargeting {}


/// Response message for listing auction packages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [auction packages list buyers](BuyerAuctionPackageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAuctionPackagesResponse {
    /// The list of auction packages.
    #[serde(rename="auctionPackages")]
    
    pub auction_packages: Option<Vec<AuctionPackage>>,
    /// Continuation token for fetching the next page of results. Pass this value in the ListAuctionPackagesRequest.pageToken field in the subsequent call to the `ListAuctionPackages` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAuctionPackagesResponse {}


/// Response message for the list method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clients users list buyers](BuyerClientUserListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListClientUsersResponse {
    /// The returned list of client users.
    #[serde(rename="clientUsers")]
    
    pub client_users: Option<Vec<ClientUser>>,
    /// A token to retrieve the next page of results. Pass this value in the ListClientUsersRequest.pageToken field in the subsequent call to the list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListClientUsersResponse {}


/// Response message for the list method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clients list buyers](BuyerClientListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListClientsResponse {
    /// The returned list of clients.
    
    pub clients: Option<Vec<Client>>,
    /// A token to retrieve the next page of results. Pass this value in the ListClientsRequest.pageToken field in the subsequent call to the list method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListClientsResponse {}


/// Response message for listing deals in a proposal.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals deals list buyers](BuyerProposalDealListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDealsResponse {
    /// The list of deals.
    
    pub deals: Option<Vec<Deal>>,
    /// Token to fetch the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDealsResponse {}


/// Response message for listing finalized deals.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [finalized deals list bidders](BidderFinalizedDealListCall) (response)
/// * [finalized deals list buyers](BuyerFinalizedDealListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFinalizedDealsResponse {
    /// The list of finalized deals.
    #[serde(rename="finalizedDeals")]
    
    pub finalized_deals: Option<Vec<FinalizedDeal>>,
    /// Token to fetch the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListFinalizedDealsResponse {}


/// Response message for listing proposals.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals list buyers](BuyerProposalListCall) (response)
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
/// * [publisher profiles list buyers](BuyerPublisherProfileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPublisherProfilesResponse {
    /// Token to fetch the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of matching publisher profiles.
    #[serde(rename="publisherProfiles")]
    
    pub publisher_profiles: Option<Vec<PublisherProfile>>,
}

impl client::ResponseResult for ListPublisherProfilesResponse {}


/// Targeting represents different criteria that can be used to target inventory. For example, they can choose to target inventory only if the user is in the US. Multiple types of targeting are always applied as a logical AND, unless noted otherwise.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MarketplaceTargeting {
    /// Daypart targeting information.
    #[serde(rename="daypartTargeting")]
    
    pub daypart_targeting: Option<DayPartTargeting>,
    /// Output only. Geo criteria IDs to be included/excluded.
    #[serde(rename="geoTargeting")]
    
    pub geo_targeting: Option<CriteriaTargeting>,
    /// Output only. Inventory sizes to be included/excluded.
    #[serde(rename="inventorySizeTargeting")]
    
    pub inventory_size_targeting: Option<InventorySizeTargeting>,
    /// Output only. Inventory type targeting information.
    #[serde(rename="inventoryTypeTargeting")]
    
    pub inventory_type_targeting: Option<InventoryTypeTargeting>,
    /// Output only. Placement targeting information, for example, URL, mobile applications.
    #[serde(rename="placementTargeting")]
    
    pub placement_targeting: Option<PlacementTargeting>,
    /// Output only. Technology targeting information, for example, operating system, device category.
    #[serde(rename="technologyTargeting")]
    
    pub technology_targeting: Option<TechnologyTargeting>,
    /// Buyer user list targeting information. User lists can be uploaded using https://developers.google.com/authorized-buyers/rtb/bulk-uploader.
    #[serde(rename="userListTargeting")]
    
    pub user_list_targeting: Option<CriteriaTargeting>,
    /// Output only. Video targeting information.
    #[serde(rename="videoTargeting")]
    
    pub video_targeting: Option<VideoTargeting>,
}

impl client::Part for MarketplaceTargeting {}


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


/// A text note attached to the proposal to facilitate the communication between buyers and sellers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Note {
    /// Output only. When this note was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The role who created the note.
    #[serde(rename="creatorRole")]
    
    pub creator_role: Option<NoteCreatorRoleEnum>,
    /// The text of the note. Maximum length is 1024 characters.
    
    pub note: Option<String>,
}

impl client::Part for Note {}


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


/// Request message for pausing a finalized deal.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [finalized deals pause buyers](BuyerFinalizedDealPauseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PauseFinalizedDealRequest {
    /// The reason to pause the finalized deal, will be displayed to the seller. Maximum length is 1000 characters.
    
    pub reason: Option<String>,
}

impl client::RequestValue for PauseFinalizedDealRequest {}


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
    #[serde(rename="uriTargeting")]
    
    pub uri_targeting: Option<UriTargeting>,
}

impl client::Part for PlacementTargeting {}


/// Pricing terms for Preferred Deals.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PreferredDealTerms {
    /// Fixed price for the deal.
    #[serde(rename="fixedPrice")]
    
    pub fixed_price: Option<Price>,
}

impl client::Part for PreferredDealTerms {}


/// Represents a price and a pricing type for a deal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Price {
    /// The actual price with currency specified.
    
    pub amount: Option<Money>,
    /// The pricing type for the deal.
    #[serde(rename="type")]
    
    pub type_: Option<PriceTypeEnum>,
}

impl client::Part for Price {}


/// Pricing terms for Private Auctions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateAuctionTerms {
    /// The minimum price buyer has to bid to compete in the private auction.
    #[serde(rename="floorPrice")]
    
    pub floor_price: Option<Price>,
    /// Output only. True if open auction buyers are allowed to compete with invited buyers in this private auction.
    #[serde(rename="openAuctionAllowed")]
    
    pub open_auction_allowed: Option<bool>,
}

impl client::Part for PrivateAuctionTerms {}


/// Buyers are allowed to store certain types of private data in a proposal or deal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateData {
    /// A buyer specified reference ID. This can be queried in the list operations (max-length: 1024 unicode code units).
    #[serde(rename="referenceId")]
    
    pub reference_id: Option<String>,
}

impl client::Part for PrivateData {}


/// Pricing terms for Programmatic Guaranteed Deals.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProgrammaticGuaranteedTerms {
    /// Fixed price for the deal.
    #[serde(rename="fixedPrice")]
    
    pub fixed_price: Option<Price>,
    /// Count of guaranteed looks.
    #[serde(rename="guaranteedLooks")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub guaranteed_looks: Option<i64>,
    /// The lifetime impression cap for CPM Sponsorship deals. Deal will stop serving when cap is reached.
    #[serde(rename="impressionCap")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub impression_cap: Option<i64>,
    /// Daily minimum looks for CPD deal types.
    #[serde(rename="minimumDailyLooks")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub minimum_daily_looks: Option<i64>,
    /// For sponsorship deals, this is the percentage of the seller's eligible impressions that the deal will serve until the cap is reached. Valid value is within range 0~100.
    #[serde(rename="percentShareOfVoice")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub percent_share_of_voice: Option<i64>,
    /// The reservation type for a Programmatic Guaranteed deal. This indicates whether the number of impressions is fixed, or a percent of available impressions. If not specified, the default reservation type is STANDARD.
    #[serde(rename="reservationType")]
    
    pub reservation_type: Option<ProgrammaticGuaranteedTermReservationTypeEnum>,
}

impl client::Part for ProgrammaticGuaranteedTerms {}


/// Represents a proposal in the Marketplace. A proposal is the unit of negotiation between a seller and a buyer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals accept buyers](BuyerProposalAcceptCall) (response)
/// * [proposals add note buyers](BuyerProposalAddNoteCall) (response)
/// * [proposals cancel negotiation buyers](BuyerProposalCancelNegotiationCall) (response)
/// * [proposals get buyers](BuyerProposalGetCall) (response)
/// * [proposals patch buyers](BuyerProposalPatchCall) (request|response)
/// * [proposals send rfp buyers](BuyerProposalSendRfpCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Proposal {
    /// Output only. When the client field is populated, this field refers to the buyer who creates and manages the client buyer and gets billed on behalf of the client buyer; when the buyer field is populated, this field is the same value as buyer. Format : `buyers/{buyerAccountId}`
    #[serde(rename="billedBuyer")]
    
    pub billed_buyer: Option<String>,
    /// Output only. Refers to a buyer in The Realtime-bidding API. Format: `buyers/{buyerAccountId}`
    
    pub buyer: Option<String>,
    /// Contact information for the buyer.
    #[serde(rename="buyerContacts")]
    
    pub buyer_contacts: Option<Vec<Contact>>,
    /// Buyer private data (hidden from seller).
    #[serde(rename="buyerPrivateData")]
    
    pub buyer_private_data: Option<PrivateData>,
    /// Output only. Refers to a Client. Format: `buyers/{buyerAccountId}/clients/{clientAccountid}`
    
    pub client: Option<String>,
    /// Output only. Type of deal the proposal contains.
    #[serde(rename="dealType")]
    
    pub deal_type: Option<ProposalDealTypeEnum>,
    /// Output only. The descriptive name for the proposal. Maximum length of 255 unicode characters is allowed. Control characters are not allowed. Buyers cannot update this field. Note: Not to be confused with name, which is a unique identifier of the proposal.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. True if the proposal was previously finalized and is now being renegotiated.
    #[serde(rename="isRenegotiating")]
    
    pub is_renegotiating: Option<bool>,
    /// Output only. The role of the last user that either updated the proposal or left a comment.
    #[serde(rename="lastUpdaterOrCommentorRole")]
    
    pub last_updater_or_commentor_role: Option<ProposalLastUpdaterOrCommentorRoleEnum>,
    /// Immutable. The name of the proposal serving as a unique identifier. Format: buyers/{accountId}/proposals/{proposalId}
    
    pub name: Option<String>,
    /// A list of notes from the buyer and the seller attached to this proposal.
    
    pub notes: Option<Vec<Note>>,
    /// Output only. Indicates whether the buyer/seller created the proposal.
    #[serde(rename="originatorRole")]
    
    pub originator_role: Option<ProposalOriginatorRoleEnum>,
    /// Whether pausing is allowed for the proposal. This is a negotiable term between buyers and publishers.
    #[serde(rename="pausingConsented")]
    
    pub pausing_consented: Option<bool>,
    /// Output only. The revision number for the proposal. Each update to the proposal or deal causes the proposal revision number to auto-increment. The buyer keeps track of the last revision number they know of and pass it in when making an update. If the head revision number on the server has since incremented, then an ABORTED error is returned during the update operation to let the buyer know that a subsequent update was made.
    #[serde(rename="proposalRevision")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub proposal_revision: Option<i64>,
    /// Immutable. Reference to the seller on the proposal. Format: `buyers/{buyerAccountId}/publisherProfiles/{publisherProfileId}` Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error.
    #[serde(rename="publisherProfile")]
    
    pub publisher_profile: Option<String>,
    /// Output only. Contact information for the seller.
    #[serde(rename="sellerContacts")]
    
    pub seller_contacts: Option<Vec<Contact>>,
    /// Output only. Indicates the state of the proposal.
    
    pub state: Option<ProposalStateEnum>,
    /// Output only. The terms and conditions associated with this proposal. Accepting a proposal implies acceptance of this field. This is created by the seller, the buyer can only view it.
    #[serde(rename="termsAndConditions")]
    
    pub terms_and_conditions: Option<String>,
    /// Output only. The time when the proposal was last revised.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Proposal {}
impl client::ResponseResult for Proposal {}


/// The values in the publisher profile are supplied by the publisher. All fields are not filterable unless stated otherwise.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [publisher profiles get buyers](BuyerPublisherProfileGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublisherProfile {
    /// Description on the publisher's audience.
    #[serde(rename="audienceDescription")]
    
    pub audience_description: Option<String>,
    /// Contact information for direct reservation deals. This is free text entered by the publisher and may include information like names, phone numbers and email addresses.
    #[serde(rename="directDealsContact")]
    
    pub direct_deals_contact: Option<String>,
    /// Display name of the publisher profile. Can be used to filter the response of the publisherProfiles.list method.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The list of domains represented in this publisher profile. Empty if this is a parent profile. These are top private domains, meaning that these will not contain a string like "photos.google.co.uk/123", but will instead contain "google.co.uk". Can be used to filter the response of the publisherProfiles.list method.
    
    pub domains: Option<Vec<String>>,
    /// Indicates if this profile is the parent profile of the seller. A parent profile represents all the inventory from the seller, as opposed to child profile that is created to brand a portion of inventory. One seller has only one parent publisher profile, and can have multiple child profiles. See https://support.google.com/admanager/answer/6035806 for details. Can be used to filter the response of the publisherProfiles.list method by setting the filter to "is_parent: true".
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
    /// Name of the publisher profile. Format: `buyers/{buyer}/publisherProfiles/{publisher_profile}`
    
    pub name: Option<String>,
    /// Overview of the publisher.
    
    pub overview: Option<String>,
    /// Statement explaining what's unique about publisher's business, and why buyers should partner with the publisher.
    #[serde(rename="pitchStatement")]
    
    pub pitch_statement: Option<String>,
    /// Contact information for programmatic deals. This is free text entered by the publisher and may include information like names, phone numbers and email addresses.
    #[serde(rename="programmaticDealsContact")]
    
    pub programmatic_deals_contact: Option<String>,
    /// A unique identifying code for the seller. This value is the same for all of the seller's parent and child publisher profiles. Can be used to filter the response of the publisherProfiles.list method.
    #[serde(rename="publisherCode")]
    
    pub publisher_code: Option<String>,
    /// URL to a sample content page.
    #[serde(rename="samplePageUrl")]
    
    pub sample_page_url: Option<String>,
    /// Up to three key metrics and rankings. For example, "#1 Mobile News Site for 20 Straight Months".
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
    /// The app store the app belongs to. Can be used to filter the response of the publisherProfiles.list method.
    #[serde(rename="appStore")]
    
    pub app_store: Option<PublisherProfileMobileApplicationAppStoreEnum>,
    /// The external ID for the app from its app store. Can be used to filter the response of the publisherProfiles.list method.
    #[serde(rename="externalAppId")]
    
    pub external_app_id: Option<String>,
    /// The name of the app.
    
    pub name: Option<String>,
}

impl client::Part for PublisherProfileMobileApplication {}


/// Request message for resuming a finalized deal.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [finalized deals resume buyers](BuyerFinalizedDealResumeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResumeFinalizedDealRequest { _never_set: Option<bool> }

impl client::RequestValue for ResumeFinalizedDealRequest {}


/// Real-time bidding metrics. For what each metric means refer to [Report metrics](https://support.google.com/adxbuyer/answer/6115195#report-metrics)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RtbMetrics {
    /// Ad impressions in last 7 days.
    #[serde(rename="adImpressions7Days")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ad_impressions7_days: Option<i64>,
    /// Bid rate in last 7 days, calculated by (bids / bid requests).
    #[serde(rename="bidRate7Days")]
    
    pub bid_rate7_days: Option<f64>,
    /// Bid requests in last 7 days.
    #[serde(rename="bidRequests7Days")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bid_requests7_days: Option<i64>,
    /// Bids in last 7 days.
    #[serde(rename="bids7Days")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bids7_days: Option<i64>,
    /// Filtered bid rate in last 7 days, calculated by (filtered bids / bids).
    #[serde(rename="filteredBidRate7Days")]
    
    pub filtered_bid_rate7_days: Option<f64>,
    /// Must bid rate for current month.
    #[serde(rename="mustBidRateCurrentMonth")]
    
    pub must_bid_rate_current_month: Option<f64>,
}

impl client::Part for RtbMetrics {}


/// Request to send an RFP. All fields in this request are proposed to publisher and subject to changes by publisher during later negotiation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [proposals send rfp buyers](BuyerProposalSendRfpCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SendRfpRequest {
    /// Contact information for the buyer.
    #[serde(rename="buyerContacts")]
    
    pub buyer_contacts: Option<Vec<Contact>>,
    /// If the current buyer is sending the RFP on behalf of its client, use this field to specify the name of the client in the format: `buyers/{accountId}/clients/{clientAccountid}`.
    
    pub client: Option<String>,
    /// Required. The display name of the proposal being created by this RFP.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Specified by buyers in request for proposal (RFP) to notify publisher the total estimated spend for the proposal. Publishers will receive this information and send back proposed deals accordingly.
    #[serde(rename="estimatedGrossSpend")]
    
    pub estimated_gross_spend: Option<Money>,
    /// Required. Proposed flight end time of the RFP. A timestamp in RFC3339 UTC "Zulu" format. Note that the specified value will be truncated to a granularity of one second.
    #[serde(rename="flightEndTime")]
    
    pub flight_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Proposed flight start time of the RFP. A timestamp in RFC3339 UTC "Zulu" format. Note that the specified value will be truncated to a granularity of one second.
    #[serde(rename="flightStartTime")]
    
    pub flight_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Geo criteria IDs to be targeted. Refer to Geo tables.
    #[serde(rename="geoTargeting")]
    
    pub geo_targeting: Option<CriteriaTargeting>,
    /// Inventory sizes to be targeted.
    #[serde(rename="inventorySizeTargeting")]
    
    pub inventory_size_targeting: Option<InventorySizeTargeting>,
    /// A message that is sent to the publisher. Maximum length is 1024 characters.
    
    pub note: Option<String>,
    /// The terms for preferred deals.
    #[serde(rename="preferredDealTerms")]
    
    pub preferred_deal_terms: Option<PreferredDealTerms>,
    /// The terms for programmatic guaranteed deals.
    #[serde(rename="programmaticGuaranteedTerms")]
    
    pub programmatic_guaranteed_terms: Option<ProgrammaticGuaranteedTerms>,
    /// Required. The profile of the publisher who will receive this RFP in the format: `buyers/{accountId}/publisherProfiles/{publisherProfileId}`.
    #[serde(rename="publisherProfile")]
    
    pub publisher_profile: Option<String>,
}

impl client::RequestValue for SendRfpRequest {}


/// Request message for setting ready to serve for a finalized deal.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [finalized deals set ready to serve buyers](BuyerFinalizedDealSetReadyToServeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetReadyToServeRequest { _never_set: Option<bool> }

impl client::RequestValue for SetReadyToServeRequest {}


/// Request message for SubscribeAuctionPackage.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [auction packages subscribe buyers](BuyerAuctionPackageSubscribeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscribeAuctionPackageRequest { _never_set: Option<bool> }

impl client::RequestValue for SubscribeAuctionPackageRequest {}


/// Request message for SubscribeAuctionPackageClients.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [auction packages subscribe clients buyers](BuyerAuctionPackageSubscribeClientCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscribeClientsRequest {
    /// Optional. A list of client buyers to subscribe to the auction package, with client buyer in the format `buyers/{accountId}/clients/{clientAccountId}`. The current buyer will be subscribed to the auction package regardless of the list contents if not already.
    
    pub clients: Option<Vec<String>>,
}

impl client::RequestValue for SubscribeClientsRequest {}


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


/// Represents a time zone from the [IANA Time Zone Database](https://www.iana.org/time-zones).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeZone {
    /// IANA Time Zone Database time zone, e.g. "America/New_York".
    
    pub id: Option<String>,
    /// Optional. IANA Time Zone Database version number, e.g. "2019a".
    
    pub version: Option<String>,
}

impl client::Part for TimeZone {}


/// Request message for UnsubscribeAuctionPackage.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [auction packages unsubscribe buyers](BuyerAuctionPackageUnsubscribeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnsubscribeAuctionPackageRequest { _never_set: Option<bool> }

impl client::RequestValue for UnsubscribeAuctionPackageRequest {}


/// Request message for UnsubscribeAuctionPackage.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [auction packages unsubscribe clients buyers](BuyerAuctionPackageUnsubscribeClientCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnsubscribeClientsRequest {
    /// Optional. A list of client buyers to unsubscribe from the auction package, with client buyer in the format `buyers/{accountId}/clients/{clientAccountId}`.
    
    pub clients: Option<Vec<String>>,
}

impl client::RequestValue for UnsubscribeClientsRequest {}


/// Request message for updating the deal at the given revision number.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDealRequest {
    /// Required. The deal to update. The deal's `name` field is used to identify the deal to be updated. Note: proposal_revision will have to be provided within the resource or else an error will be thrown. Format: buyers/{accountId}/proposals/{proposalId}/deals/{dealId}
    
    pub deal: Option<Deal>,
    /// List of fields to be updated. If empty or unspecified, the service will update all fields populated in the update request excluding the output only fields and primitive fields with default value. Note that explicit field mask is required in order to reset a primitive field back to its default value, for example, false for boolean fields, 0 for integer fields. A special field mask consisting of a single path "*" can be used to indicate full replacement(the equivalent of PUT method), updatable fields unset or unspecified in the input will be cleared or set to default value. Output only fields will be ignored regardless of the value of updateMask.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::Part for UpdateDealRequest {}


/// Represents a list of targeted and excluded URLs (for example, google.com). For Private Auction Deals, URLs are either included or excluded. For Programmatic Guaranteed and Preferred Deals, this doesn't apply.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UriTargeting {
    /// A list of URLs to be excluded.
    #[serde(rename="excludedUris")]
    
    pub excluded_uris: Option<Vec<String>>,
    /// A list of URLs to be included.
    #[serde(rename="targetedUris")]
    
    pub targeted_uris: Option<Vec<String>>,
}

impl client::Part for UriTargeting {}


/// Represents targeting information about video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoTargeting {
    /// A list of video positions to be excluded. When this field is populated, the targeted_position_types field must be empty.
    #[serde(rename="excludedPositionTypes")]
    
    pub excluded_position_types: Option<Vec<VideoTargetingExcludedPositionTypesEnum>>,
    /// A list of video positions to be included. When this field is populated, the excluded_position_types field must be empty.
    #[serde(rename="targetedPositionTypes")]
    
    pub targeted_position_types: Option<Vec<VideoTargetingTargetedPositionTypesEnum>>,
}

impl client::Part for VideoTargeting {}


