use super::*;
/// Request message for ManualTriggerService.ActivateManualTrigger.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [manual triggers activate advertisers](AdvertiserManualTriggerActivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivateManualTriggerRequest { _never_set: Option<bool> }

impl client::RequestValue for ActivateManualTriggerRequest {}


/// Configuration for custom Active View video viewability metrics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActiveViewVideoViewabilityMetricConfig {
    /// Required. The display name of the custom metric.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The minimum visible video duration required (in seconds) in order for an impression to be recorded. You must specify minimum_duration, minimum_quartile or both. If both are specified, an impression meets the metric criteria if either requirement is met (whichever happens first).
    #[serde(rename="minimumDuration")]
    
    pub minimum_duration: Option<String>,
    /// The minimum visible video duration required, based on the video quartiles, in order for an impression to be recorded. You must specify minimum_duration, minimum_quartile or both. If both are specified, an impression meets the metric criteria if either requirement is met (whichever happens first).
    #[serde(rename="minimumQuartile")]
    
    pub minimum_quartile: Option<String>,
    /// Required. The minimum percentage of the video ad's pixels visible on the screen in order for an impression to be recorded.
    #[serde(rename="minimumViewability")]
    
    pub minimum_viewability: Option<String>,
    /// Required. The minimum percentage of the video ad's volume required in order for an impression to be recorded.
    #[serde(rename="minimumVolume")]
    
    pub minimum_volume: Option<String>,
}

impl client::Part for ActiveViewVideoViewabilityMetricConfig {}


/// Details of Adloox settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Adloox {
    /// Adloox's brand safety settings.
    #[serde(rename="excludedAdlooxCategories")]
    
    pub excluded_adloox_categories: Option<Vec<String>>,
}

impl client::Part for Adloox {}


/// A single advertiser in Display & Video 360 (DV360).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assets upload advertisers](AdvertiserAssetUploadCall) (none)
/// * [campaigns targeting types assigned targeting options get advertisers](AdvertiserCampaignTargetingTypeAssignedTargetingOptionGetCall) (none)
/// * [campaigns targeting types assigned targeting options list advertisers](AdvertiserCampaignTargetingTypeAssignedTargetingOptionListCall) (none)
/// * [campaigns bulk list campaign assigned targeting options advertisers](AdvertiserCampaignBulkListCampaignAssignedTargetingOptionCall) (none)
/// * [campaigns create advertisers](AdvertiserCampaignCreateCall) (none)
/// * [campaigns delete advertisers](AdvertiserCampaignDeleteCall) (none)
/// * [campaigns get advertisers](AdvertiserCampaignGetCall) (none)
/// * [campaigns list advertisers](AdvertiserCampaignListCall) (none)
/// * [campaigns patch advertisers](AdvertiserCampaignPatchCall) (none)
/// * [channels sites bulk edit advertisers](AdvertiserChannelSiteBulkEditCall) (none)
/// * [channels sites create advertisers](AdvertiserChannelSiteCreateCall) (none)
/// * [channels sites delete advertisers](AdvertiserChannelSiteDeleteCall) (none)
/// * [channels sites list advertisers](AdvertiserChannelSiteListCall) (none)
/// * [channels sites replace advertisers](AdvertiserChannelSiteReplaceCall) (none)
/// * [channels create advertisers](AdvertiserChannelCreateCall) (none)
/// * [channels get advertisers](AdvertiserChannelGetCall) (none)
/// * [channels list advertisers](AdvertiserChannelListCall) (none)
/// * [channels patch advertisers](AdvertiserChannelPatchCall) (none)
/// * [creatives create advertisers](AdvertiserCreativeCreateCall) (none)
/// * [creatives delete advertisers](AdvertiserCreativeDeleteCall) (none)
/// * [creatives get advertisers](AdvertiserCreativeGetCall) (none)
/// * [creatives list advertisers](AdvertiserCreativeListCall) (none)
/// * [creatives patch advertisers](AdvertiserCreativePatchCall) (none)
/// * [insertion orders targeting types assigned targeting options get advertisers](AdvertiserInsertionOrderTargetingTypeAssignedTargetingOptionGetCall) (none)
/// * [insertion orders targeting types assigned targeting options list advertisers](AdvertiserInsertionOrderTargetingTypeAssignedTargetingOptionListCall) (none)
/// * [insertion orders bulk list insertion order assigned targeting options advertisers](AdvertiserInsertionOrderBulkListInsertionOrderAssignedTargetingOptionCall) (none)
/// * [insertion orders create advertisers](AdvertiserInsertionOrderCreateCall) (none)
/// * [insertion orders delete advertisers](AdvertiserInsertionOrderDeleteCall) (none)
/// * [insertion orders get advertisers](AdvertiserInsertionOrderGetCall) (none)
/// * [insertion orders list advertisers](AdvertiserInsertionOrderListCall) (none)
/// * [insertion orders patch advertisers](AdvertiserInsertionOrderPatchCall) (none)
/// * [invoices list advertisers](AdvertiserInvoiceListCall) (none)
/// * [invoices lookup invoice currency advertisers](AdvertiserInvoiceLookupInvoiceCurrencyCall) (none)
/// * [line items targeting types assigned targeting options create advertisers](AdvertiserLineItemTargetingTypeAssignedTargetingOptionCreateCall) (none)
/// * [line items targeting types assigned targeting options delete advertisers](AdvertiserLineItemTargetingTypeAssignedTargetingOptionDeleteCall) (none)
/// * [line items targeting types assigned targeting options get advertisers](AdvertiserLineItemTargetingTypeAssignedTargetingOptionGetCall) (none)
/// * [line items targeting types assigned targeting options list advertisers](AdvertiserLineItemTargetingTypeAssignedTargetingOptionListCall) (none)
/// * [line items bulk edit line item assigned targeting options advertisers](AdvertiserLineItemBulkEditLineItemAssignedTargetingOptionCall) (none)
/// * [line items bulk list line item assigned targeting options advertisers](AdvertiserLineItemBulkListLineItemAssignedTargetingOptionCall) (none)
/// * [line items create advertisers](AdvertiserLineItemCreateCall) (none)
/// * [line items delete advertisers](AdvertiserLineItemDeleteCall) (none)
/// * [line items generate default advertisers](AdvertiserLineItemGenerateDefaultCall) (none)
/// * [line items get advertisers](AdvertiserLineItemGetCall) (none)
/// * [line items list advertisers](AdvertiserLineItemListCall) (none)
/// * [line items patch advertisers](AdvertiserLineItemPatchCall) (none)
/// * [location lists assigned locations bulk edit advertisers](AdvertiserLocationListAssignedLocationBulkEditCall) (none)
/// * [location lists assigned locations create advertisers](AdvertiserLocationListAssignedLocationCreateCall) (none)
/// * [location lists assigned locations delete advertisers](AdvertiserLocationListAssignedLocationDeleteCall) (none)
/// * [location lists assigned locations list advertisers](AdvertiserLocationListAssignedLocationListCall) (none)
/// * [location lists create advertisers](AdvertiserLocationListCreateCall) (none)
/// * [location lists get advertisers](AdvertiserLocationListGetCall) (none)
/// * [location lists list advertisers](AdvertiserLocationListListCall) (none)
/// * [location lists patch advertisers](AdvertiserLocationListPatchCall) (none)
/// * [manual triggers activate advertisers](AdvertiserManualTriggerActivateCall) (none)
/// * [manual triggers create advertisers](AdvertiserManualTriggerCreateCall) (none)
/// * [manual triggers deactivate advertisers](AdvertiserManualTriggerDeactivateCall) (none)
/// * [manual triggers get advertisers](AdvertiserManualTriggerGetCall) (none)
/// * [manual triggers list advertisers](AdvertiserManualTriggerListCall) (none)
/// * [manual triggers patch advertisers](AdvertiserManualTriggerPatchCall) (none)
/// * [negative keyword lists negative keywords bulk edit advertisers](AdvertiserNegativeKeywordListNegativeKeywordBulkEditCall) (none)
/// * [negative keyword lists negative keywords create advertisers](AdvertiserNegativeKeywordListNegativeKeywordCreateCall) (none)
/// * [negative keyword lists negative keywords delete advertisers](AdvertiserNegativeKeywordListNegativeKeywordDeleteCall) (none)
/// * [negative keyword lists negative keywords list advertisers](AdvertiserNegativeKeywordListNegativeKeywordListCall) (none)
/// * [negative keyword lists negative keywords replace advertisers](AdvertiserNegativeKeywordListNegativeKeywordReplaceCall) (none)
/// * [negative keyword lists create advertisers](AdvertiserNegativeKeywordListCreateCall) (none)
/// * [negative keyword lists delete advertisers](AdvertiserNegativeKeywordListDeleteCall) (none)
/// * [negative keyword lists get advertisers](AdvertiserNegativeKeywordListGetCall) (none)
/// * [negative keyword lists list advertisers](AdvertiserNegativeKeywordListListCall) (none)
/// * [negative keyword lists patch advertisers](AdvertiserNegativeKeywordListPatchCall) (none)
/// * [targeting types assigned targeting options create advertisers](AdvertiserTargetingTypeAssignedTargetingOptionCreateCall) (none)
/// * [targeting types assigned targeting options delete advertisers](AdvertiserTargetingTypeAssignedTargetingOptionDeleteCall) (none)
/// * [targeting types assigned targeting options get advertisers](AdvertiserTargetingTypeAssignedTargetingOptionGetCall) (none)
/// * [targeting types assigned targeting options list advertisers](AdvertiserTargetingTypeAssignedTargetingOptionListCall) (none)
/// * [audit advertisers](AdvertiserAuditCall) (none)
/// * [bulk edit advertiser assigned targeting options advertisers](AdvertiserBulkEditAdvertiserAssignedTargetingOptionCall) (none)
/// * [bulk list advertiser assigned targeting options advertisers](AdvertiserBulkListAdvertiserAssignedTargetingOptionCall) (none)
/// * [create advertisers](AdvertiserCreateCall) (request|response)
/// * [delete advertisers](AdvertiserDeleteCall) (none)
/// * [get advertisers](AdvertiserGetCall) (response)
/// * [list advertisers](AdvertiserListCall) (none)
/// * [patch advertisers](AdvertiserPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Advertiser {
    /// Required. Immutable. Ad server related settings of the advertiser.
    #[serde(rename="adServerConfig")]
    
    pub ad_server_config: Option<AdvertiserAdServerConfig>,
    /// Output only. The unique ID of the advertiser. Assigned by the system.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Required. Creative related settings of the advertiser.
    #[serde(rename="creativeConfig")]
    
    pub creative_config: Option<AdvertiserCreativeConfig>,
    /// Settings that control how advertiser data may be accessed.
    #[serde(rename="dataAccessConfig")]
    
    pub data_access_config: Option<AdvertiserDataAccessConfig>,
    /// Required. The display name of the advertiser. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. Controls whether or not insertion orders and line items of the advertiser can spend their budgets and bid on inventory. * Accepted values are `ENTITY_STATUS_ACTIVE`, `ENTITY_STATUS_PAUSED` and `ENTITY_STATUS_SCHEDULED_FOR_DELETION`. * If set to `ENTITY_STATUS_SCHEDULED_FOR_DELETION`, the advertiser will be deleted 30 days from when it was first scheduled for deletion.
    #[serde(rename="entityStatus")]
    
    pub entity_status: Option<String>,
    /// Required. General settings of the advertiser.
    #[serde(rename="generalConfig")]
    
    pub general_config: Option<AdvertiserGeneralConfig>,
    /// Integration details of the advertiser. Only integrationCode is currently applicable to advertiser. Other fields of IntegrationDetails are not supported and will be ignored if provided.
    #[serde(rename="integrationDetails")]
    
    pub integration_details: Option<IntegrationDetails>,
    /// Output only. The resource name of the advertiser.
    
    pub name: Option<String>,
    /// Required. Immutable. The unique ID of the partner that the advertiser belongs to.
    #[serde(rename="partnerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partner_id: Option<i64>,
    /// Whether integration with Mediaocean (Prisma) is enabled. By enabling this, you agree to the following: On behalf of my company, I authorize Mediaocean (Prisma) to send budget segment plans to Google, and I authorize Google to send corresponding reporting and invoices from DV360 to Mediaocean for the purposes of budget planning, billing, and reconciliation for this advertiser.
    #[serde(rename="prismaEnabled")]
    
    pub prisma_enabled: Option<bool>,
    /// Targeting settings related to ad serving of the advertiser.
    #[serde(rename="servingConfig")]
    
    pub serving_config: Option<AdvertiserTargetingConfig>,
    /// Output only. The timestamp when the advertiser was last updated. Assigned by the system.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Advertiser {}
impl client::Resource for Advertiser {}
impl client::ResponseResult for Advertiser {}


/// Ad server related settings of an advertiser.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvertiserAdServerConfig {
    /// The configuration for advertisers that use both Campaign Manager 360 (CM360) and third-party ad servers.
    #[serde(rename="cmHybridConfig")]
    
    pub cm_hybrid_config: Option<CmHybridConfig>,
    /// The configuration for advertisers that use third-party ad servers only.
    #[serde(rename="thirdPartyOnlyConfig")]
    
    pub third_party_only_config: Option<ThirdPartyOnlyConfig>,
}

impl client::Part for AdvertiserAdServerConfig {}


/// Creatives related settings of an advertiser.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvertiserCreativeConfig {
    /// Whether or not the advertiser is enabled for dynamic creatives.
    #[serde(rename="dynamicCreativeEnabled")]
    
    pub dynamic_creative_enabled: Option<bool>,
    /// An ID for configuring campaign monitoring provided by Integral Ad Service (IAS). The DV360 system will append an IAS "Campaign Monitor" tag containing this ID to the creative tag.
    #[serde(rename="iasClientId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ias_client_id: Option<i64>,
    /// Whether or not to use DV360's Online Behavioral Advertising (OBA) compliance. Warning: Changing OBA settings may cause the audit status of your creatives to be reset by some ad exchanges, making them ineligible to serve until they are re-approved.
    #[serde(rename="obaComplianceDisabled")]
    
    pub oba_compliance_disabled: Option<bool>,
    /// By setting this field to `true`, you, on behalf of your company, authorize Google to use video creatives associated with this Display & Video 360 advertiser to provide reporting and features related to the advertiser's television campaigns. Applicable only when the advertiser has a CM360 hybrid ad server configuration.
    #[serde(rename="videoCreativeDataSharingAuthorized")]
    
    pub video_creative_data_sharing_authorized: Option<bool>,
}

impl client::Part for AdvertiserCreativeConfig {}


/// Settings that control how advertiser related data may be accessed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvertiserDataAccessConfig {
    /// Structured Data Files (SDF) settings for the advertiser. If not specified, the SDF settings of the parent partner are used.
    #[serde(rename="sdfConfig")]
    
    pub sdf_config: Option<AdvertiserSdfConfig>,
}

impl client::Part for AdvertiserDataAccessConfig {}


/// General settings of an advertiser.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvertiserGeneralConfig {
    /// Required. Immutable. Advertiser's currency in ISO 4217 format. Accepted codes and the currencies they represent are: Currency Code : Currency Name * `ARS` : Argentine Peso * `AUD` : Australian Dollar * `BRL` : Brazilian Real * `CAD` : Canadian Dollar * `CHF` : Swiss Franc * `CLP` : Chilean Peso * `CNY` : Chinese Yuan * `COP` : Colombian Peso * `CZK` : Czech Koruna * `DKK` : Danish Krone * `EGP` : Egyption Pound * `EUR` : Euro * `GBP` : British Pound * `HKD` : Hong Kong Dollar * `HUF` : Hungarian Forint * `IDR` : Indonesian Rupiah * `ILS` : Israeli Shekel * `INR` : Indian Rupee * `JPY` : Japanese Yen * `KRW` : South Korean Won * `MXN` : Mexican Pesos * `MYR` : Malaysian Ringgit * `NGN` : Nigerian Naira * `NOK` : Norwegian Krone * `NZD` : New Zealand Dollar * `PEN` : Peruvian Nuevo Sol * `PLN` : Polish Zloty * `RON` : New Romanian Leu * `RUB` : Russian Ruble * `SEK` : Swedish Krona * `TRY` : Turkish Lira * `TWD` : New Taiwan Dollar * `USD` : US Dollar * `ZAR` : South African Rand
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Required. The domain URL of the advertiser's primary website. The system will send this information to publishers that require website URL to associate a campaign with an advertiser. Provide a URL with no path or query string, beginning with `http:` or `https:`. For example, http://www.example.com
    #[serde(rename="domainUrl")]
    
    pub domain_url: Option<String>,
    /// Output only. The standard TZ database name of the advertiser's time zone. For example, `America/New_York`. See more at: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones For CM360 hybrid advertisers, the time zone is the same as that of the associated CM360 account; for third-party only advertisers, the time zone is the same as that of the parent partner.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::Part for AdvertiserGeneralConfig {}


/// Structured Data Files (SDF) settings of an advertiser.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvertiserSdfConfig {
    /// Whether or not this advertiser overrides the SDF configuration of its parent partner. By default, an advertiser inherits the SDF configuration from the parent partner. To override the partner configuration, set this field to `true` and provide the new configuration in sdfConfig.
    #[serde(rename="overridePartnerSdfConfig")]
    
    pub override_partner_sdf_config: Option<bool>,
    /// The SDF configuration for the advertiser. * Required when overridePartnerSdfConfig is `true`. * Output only when overridePartnerSdfConfig is `false`.
    #[serde(rename="sdfConfig")]
    
    pub sdf_config: Option<SdfConfig>,
}

impl client::Part for AdvertiserSdfConfig {}


/// Targeting settings related to ad serving of an advertiser.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvertiserTargetingConfig {
    /// Whether or not connected TV devices are exempt from viewability targeting for all video line items under the advertiser.
    #[serde(rename="exemptTvFromViewabilityTargeting")]
    
    pub exempt_tv_from_viewability_targeting: Option<bool>,
}

impl client::Part for AdvertiserTargetingConfig {}


/// Represents a targetable age range. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_AGE_RANGE`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AgeRangeAssignedTargetingOptionDetails {
    /// The age range of an audience. We only support targeting a continuous age range of an audience. Thus, the age range represented in this field can be 1) targeted solely, or, 2) part of a larger continuous age range. The reach of a continuous age range targeting can be expanded by also targeting an audience of an unknown age. Output only in v1. Required in v2.
    #[serde(rename="ageRange")]
    
    pub age_range: Option<String>,
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_AGE_RANGE`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for AgeRangeAssignedTargetingOptionDetails {}


/// Represents a targetable age range. This will be populated in the age_range_details field when targeting_type is `TARGETING_TYPE_AGE_RANGE`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AgeRangeTargetingOptionDetails {
    /// Output only. The age range of an audience.
    #[serde(rename="ageRange")]
    
    pub age_range: Option<String>,
}

impl client::Part for AgeRangeTargetingOptionDetails {}


/// Details for assigned app targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_APP`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppAssignedTargetingOptionDetails {
    /// Required. The ID of the app. Android's Play store app uses bundle ID, for example `com.google.android.gm`. Apple's App store app ID uses 9 digit string, for example `422689480`.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// Indicates the platform of the targeted app. If this field is not specified, the app platform will be assumed to be mobile (i.e., Android or iOS), and we will derive the appropriate mobile platform from the app ID.
    #[serde(rename="appPlatform")]
    
    pub app_platform: Option<String>,
    /// Output only. The display name of the app.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Indicates if this option is being negatively targeted.
    
    pub negative: Option<bool>,
}

impl client::Part for AppAssignedTargetingOptionDetails {}


/// Details for assigned app category targeting option. This will be populated in the app_category_details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_APP_CATEGORY`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppCategoryAssignedTargetingOptionDetails {
    /// Output only. The display name of the app category.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Indicates if this option is being negatively targeted.
    
    pub negative: Option<bool>,
    /// Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_APP_CATEGORY`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for AppCategoryAssignedTargetingOptionDetails {}


/// Represents a targetable collection of apps. A collection lets you target dynamic groups of related apps that are maintained by the platform, for example `All Apps/Google Play/Games`. This will be populated in the app_category_details field when targeting_type is `TARGETING_TYPE_APP_CATEGORY`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppCategoryTargetingOptionDetails {
    /// Output only. The name of the app collection.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for AppCategoryTargetingOptionDetails {}


/// A single asset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    /// The asset content. For uploaded assets, the content is the serving path.
    
    pub content: Option<String>,
    /// Media ID of the uploaded asset. This is a unique identifier for the asset. This ID can be passed to other API calls, e.g. CreateCreative to associate the asset with a creative.
    #[serde(rename="mediaId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub media_id: Option<i64>,
}

impl client::Part for Asset {}


/// Asset association for the creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AssetAssociation {
    /// The associated asset.
    
    pub asset: Option<Asset>,
    /// The role of this asset for the creative.
    
    pub role: Option<String>,
}

impl client::Part for AssetAssociation {}


/// An assignment between a targetable inventory source and an inventory source group.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assigned inventory sources create inventory source groups](InventorySourceGroupAssignedInventorySourceCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AssignedInventorySource {
    /// Output only. The unique ID of the assigned inventory source. The ID is only unique within a given inventory source group. It may be reused in other contexts.
    #[serde(rename="assignedInventorySourceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub assigned_inventory_source_id: Option<i64>,
    /// Required. The ID of the inventory source entity being targeted.
    #[serde(rename="inventorySourceId")]
    
    pub inventory_source_id: Option<String>,
    /// Output only. The resource name of the assigned inventory source.
    
    pub name: Option<String>,
}

impl client::RequestValue for AssignedInventorySource {}
impl client::ResponseResult for AssignedInventorySource {}


/// An assignment between a location list and a relevant targeting option. Currently, geo region targeting options are the only supported option for assignment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [location lists assigned locations create advertisers](AdvertiserLocationListAssignedLocationCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AssignedLocation {
    /// Output only. The unique ID of the assigned location. The ID is only unique within a location list. It may be reused in other contexts.
    #[serde(rename="assignedLocationId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub assigned_location_id: Option<i64>,
    /// Output only. The resource name of the assigned location.
    
    pub name: Option<String>,
    /// Required. The ID of the targeting option assigned to the location list. Must be of type TARGETING_TYPE_GEO_REGION.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::RequestValue for AssignedLocation {}
impl client::ResponseResult for AssignedLocation {}


/// A single assigned targeting option, which defines the state of a targeting option for an entity with targeting settings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [campaigns targeting types assigned targeting options get advertisers](AdvertiserCampaignTargetingTypeAssignedTargetingOptionGetCall) (response)
/// * [insertion orders targeting types assigned targeting options get advertisers](AdvertiserInsertionOrderTargetingTypeAssignedTargetingOptionGetCall) (response)
/// * [line items targeting types assigned targeting options create advertisers](AdvertiserLineItemTargetingTypeAssignedTargetingOptionCreateCall) (request|response)
/// * [line items targeting types assigned targeting options get advertisers](AdvertiserLineItemTargetingTypeAssignedTargetingOptionGetCall) (response)
/// * [targeting types assigned targeting options create advertisers](AdvertiserTargetingTypeAssignedTargetingOptionCreateCall) (request|response)
/// * [targeting types assigned targeting options get advertisers](AdvertiserTargetingTypeAssignedTargetingOptionGetCall) (response)
/// * [targeting types assigned targeting options create partners](PartnerTargetingTypeAssignedTargetingOptionCreateCall) (request|response)
/// * [targeting types assigned targeting options get partners](PartnerTargetingTypeAssignedTargetingOptionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AssignedTargetingOption {
    /// Age range details. This field will be populated when the targeting_type is `TARGETING_TYPE_AGE_RANGE`.
    #[serde(rename="ageRangeDetails")]
    
    pub age_range_details: Option<AgeRangeAssignedTargetingOptionDetails>,
    /// App category details. This field will be populated when the targeting_type is `TARGETING_TYPE_APP_CATEGORY`.
    #[serde(rename="appCategoryDetails")]
    
    pub app_category_details: Option<AppCategoryAssignedTargetingOptionDetails>,
    /// App details. This field will be populated when the targeting_type is `TARGETING_TYPE_APP`.
    #[serde(rename="appDetails")]
    
    pub app_details: Option<AppAssignedTargetingOptionDetails>,
    /// Output only. The unique ID of the assigned targeting option. The ID is only unique within a given resource and targeting type. It may be reused in other contexts.
    #[serde(rename="assignedTargetingOptionId")]
    
    pub assigned_targeting_option_id: Option<String>,
    /// Audience targeting details. This field will be populated when the targeting_type is `TARGETING_TYPE_AUDIENCE_GROUP`. You can only target one audience group option per resource.
    #[serde(rename="audienceGroupDetails")]
    
    pub audience_group_details: Option<AudienceGroupAssignedTargetingOptionDetails>,
    /// Audio content type details. This field will be populated when the targeting_type is `TARGETING_TYPE_AUDIO_CONTENT_TYPE`.
    #[serde(rename="audioContentTypeDetails")]
    
    pub audio_content_type_details: Option<AudioContentTypeAssignedTargetingOptionDetails>,
    /// Authorized seller status details. This field will be populated when the targeting_type is `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS`. You can only target one authorized seller status option per resource. If a resource doesn't have an authorized seller status option, all authorized sellers indicated as DIRECT or RESELLER in the ads.txt file are targeted by default.
    #[serde(rename="authorizedSellerStatusDetails")]
    
    pub authorized_seller_status_details: Option<AuthorizedSellerStatusAssignedTargetingOptionDetails>,
    /// Browser details. This field will be populated when the targeting_type is `TARGETING_TYPE_BROWSER`.
    #[serde(rename="browserDetails")]
    
    pub browser_details: Option<BrowserAssignedTargetingOptionDetails>,
    /// Business chain details. This field will be populated when the targeting_type is `TARGETING_TYPE_BUSINESS_CHAIN`.
    #[serde(rename="businessChainDetails")]
    
    pub business_chain_details: Option<BusinessChainAssignedTargetingOptionDetails>,
    /// Carrier and ISP details. This field will be populated when the targeting_type is `TARGETING_TYPE_CARRIER_AND_ISP`.
    #[serde(rename="carrierAndIspDetails")]
    
    pub carrier_and_isp_details: Option<CarrierAndIspAssignedTargetingOptionDetails>,
    /// Category details. This field will be populated when the targeting_type is `TARGETING_TYPE_CATEGORY`. Targeting a category will also target its subcategories. If a category is excluded from targeting and a subcategory is included, the exclusion will take precedence.
    #[serde(rename="categoryDetails")]
    
    pub category_details: Option<CategoryAssignedTargetingOptionDetails>,
    /// Channel details. This field will be populated when the targeting_type is `TARGETING_TYPE_CHANNEL`.
    #[serde(rename="channelDetails")]
    
    pub channel_details: Option<ChannelAssignedTargetingOptionDetails>,
    /// Content duration details. This field will be populated when the targeting_type is `TARGETING_TYPE_CONTENT_DURATION`.
    #[serde(rename="contentDurationDetails")]
    
    pub content_duration_details: Option<ContentDurationAssignedTargetingOptionDetails>,
    /// Content genre details. This field will be populated when the targeting_type is `TARGETING_TYPE_CONTENT_GENRE`.
    #[serde(rename="contentGenreDetails")]
    
    pub content_genre_details: Option<ContentGenreAssignedTargetingOptionDetails>,
    /// Content instream position details. This field will be populated when the targeting_type is `TARGETING_TYPE_CONTENT_INSTREAM_POSITION`.
    #[serde(rename="contentInstreamPositionDetails")]
    
    pub content_instream_position_details: Option<ContentInstreamPositionAssignedTargetingOptionDetails>,
    /// Content outstream position details. This field will be populated when the targeting_type is `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION`.
    #[serde(rename="contentOutstreamPositionDetails")]
    
    pub content_outstream_position_details: Option<ContentOutstreamPositionAssignedTargetingOptionDetails>,
    /// Content duration details. This field will be populated when the TargetingType is `TARGETING_TYPE_CONTENT_STREAM_TYPE`.
    #[serde(rename="contentStreamTypeDetails")]
    
    pub content_stream_type_details: Option<ContentStreamTypeAssignedTargetingOptionDetails>,
    /// Day and time details. This field will be populated when the targeting_type is `TARGETING_TYPE_DAY_AND_TIME`.
    #[serde(rename="dayAndTimeDetails")]
    
    pub day_and_time_details: Option<DayAndTimeAssignedTargetingOptionDetails>,
    /// Device make and model details. This field will be populated when the targeting_type is `TARGETING_TYPE_DEVICE_MAKE_MODEL`.
    #[serde(rename="deviceMakeModelDetails")]
    
    pub device_make_model_details: Option<DeviceMakeModelAssignedTargetingOptionDetails>,
    /// Device Type details. This field will be populated when the targeting_type is `TARGETING_TYPE_DEVICE_TYPE`.
    #[serde(rename="deviceTypeDetails")]
    
    pub device_type_details: Option<DeviceTypeAssignedTargetingOptionDetails>,
    /// Digital content label details. This field will be populated when the targeting_type is `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION`. Digital content labels are targeting exclusions. Advertiser level digital content label exclusions, if set, are always applied in serving (even though they aren't visible in resource settings). Resource settings can exclude content labels in addition to advertiser exclusions, but can't override them. A line item won't serve if all the digital content labels are excluded.
    #[serde(rename="digitalContentLabelExclusionDetails")]
    
    pub digital_content_label_exclusion_details: Option<DigitalContentLabelAssignedTargetingOptionDetails>,
    /// Environment details. This field will be populated when the targeting_type is `TARGETING_TYPE_ENVIRONMENT`.
    #[serde(rename="environmentDetails")]
    
    pub environment_details: Option<EnvironmentAssignedTargetingOptionDetails>,
    /// Exchange details. This field will be populated when the targeting_type is `TARGETING_TYPE_EXCHANGE`.
    #[serde(rename="exchangeDetails")]
    
    pub exchange_details: Option<ExchangeAssignedTargetingOptionDetails>,
    /// Gender details. This field will be populated when the targeting_type is `TARGETING_TYPE_GENDER`.
    #[serde(rename="genderDetails")]
    
    pub gender_details: Option<GenderAssignedTargetingOptionDetails>,
    /// Geographic region details. This field will be populated when the targeting_type is `TARGETING_TYPE_GEO_REGION`.
    #[serde(rename="geoRegionDetails")]
    
    pub geo_region_details: Option<GeoRegionAssignedTargetingOptionDetails>,
    /// Household income details. This field will be populated when the targeting_type is `TARGETING_TYPE_HOUSEHOLD_INCOME`.
    #[serde(rename="householdIncomeDetails")]
    
    pub household_income_details: Option<HouseholdIncomeAssignedTargetingOptionDetails>,
    /// Output only. The inheritance status of the assigned targeting option.
    
    pub inheritance: Option<String>,
    /// Inventory source details. This field will be populated when the targeting_type is `TARGETING_TYPE_INVENTORY_SOURCE`.
    #[serde(rename="inventorySourceDetails")]
    
    pub inventory_source_details: Option<InventorySourceAssignedTargetingOptionDetails>,
    /// Inventory source group details. This field will be populated when the targeting_type is `TARGETING_TYPE_INVENTORY_SOURCE_GROUP`.
    #[serde(rename="inventorySourceGroupDetails")]
    
    pub inventory_source_group_details: Option<InventorySourceGroupAssignedTargetingOptionDetails>,
    /// Keyword details. This field will be populated when the targeting_type is `TARGETING_TYPE_KEYWORD`. A maximum of 5000 direct negative keywords can be assigned to a resource. No limit on number of positive keywords that can be assigned.
    #[serde(rename="keywordDetails")]
    
    pub keyword_details: Option<KeywordAssignedTargetingOptionDetails>,
    /// Language details. This field will be populated when the targeting_type is `TARGETING_TYPE_LANGUAGE`.
    #[serde(rename="languageDetails")]
    
    pub language_details: Option<LanguageAssignedTargetingOptionDetails>,
    /// Output only. The resource name for this assigned targeting option.
    
    pub name: Option<String>,
    /// Native content position details. This field will be populated when the targeting_type is `TARGETING_TYPE_NATIVE_CONTENT_POSITION`.
    #[serde(rename="nativeContentPositionDetails")]
    
    pub native_content_position_details: Option<NativeContentPositionAssignedTargetingOptionDetails>,
    /// Keyword details. This field will be populated when the targeting_type is `TARGETING_TYPE_NEGATIVE_KEYWORD_LIST`. A maximum of 4 negative keyword lists can be assigned to a resource.
    #[serde(rename="negativeKeywordListDetails")]
    
    pub negative_keyword_list_details: Option<NegativeKeywordListAssignedTargetingOptionDetails>,
    /// Open Measurement enabled inventory details. This field will be populated when the targeting_type is `TARGETING_TYPE_OMID`.
    #[serde(rename="omidDetails")]
    
    pub omid_details: Option<OmidAssignedTargetingOptionDetails>,
    /// On screen position details. This field will be populated when the targeting_type is `TARGETING_TYPE_ON_SCREEN_POSITION`.
    #[serde(rename="onScreenPositionDetails")]
    
    pub on_screen_position_details: Option<OnScreenPositionAssignedTargetingOptionDetails>,
    /// Operating system details. This field will be populated when the targeting_type is `TARGETING_TYPE_OPERATING_SYSTEM`.
    #[serde(rename="operatingSystemDetails")]
    
    pub operating_system_details: Option<OperatingSystemAssignedTargetingOptionDetails>,
    /// Parental status details. This field will be populated when the targeting_type is `TARGETING_TYPE_PARENTAL_STATUS`.
    #[serde(rename="parentalStatusDetails")]
    
    pub parental_status_details: Option<ParentalStatusAssignedTargetingOptionDetails>,
    /// POI details. This field will be populated when the targeting_type is `TARGETING_TYPE_POI`.
    #[serde(rename="poiDetails")]
    
    pub poi_details: Option<PoiAssignedTargetingOptionDetails>,
    /// Proximity location list details. This field will be populated when the targeting_type is `TARGETING_TYPE_PROXIMITY_LOCATION_LIST`.
    #[serde(rename="proximityLocationListDetails")]
    
    pub proximity_location_list_details: Option<ProximityLocationListAssignedTargetingOptionDetails>,
    /// Regional location list details. This field will be populated when the targeting_type is `TARGETING_TYPE_REGIONAL_LOCATION_LIST`.
    #[serde(rename="regionalLocationListDetails")]
    
    pub regional_location_list_details: Option<RegionalLocationListAssignedTargetingOptionDetails>,
    /// Sensitive category details. This field will be populated when the targeting_type is `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`. Sensitive categories are targeting exclusions. Advertiser level sensitive category exclusions, if set, are always applied in serving (even though they aren't visible in resource settings). Resource settings can exclude sensitive categories in addition to advertiser exclusions, but can't override them.
    #[serde(rename="sensitiveCategoryExclusionDetails")]
    
    pub sensitive_category_exclusion_details: Option<SensitiveCategoryAssignedTargetingOptionDetails>,
    /// Sub-exchange details. This field will be populated when the targeting_type is `TARGETING_TYPE_SUB_EXCHANGE`.
    #[serde(rename="subExchangeDetails")]
    
    pub sub_exchange_details: Option<SubExchangeAssignedTargetingOptionDetails>,
    /// Output only. Identifies the type of this assigned targeting option.
    #[serde(rename="targetingType")]
    
    pub targeting_type: Option<String>,
    /// Third party verification details. This field will be populated when the targeting_type is `TARGETING_TYPE_THIRD_PARTY_VERIFIER`.
    #[serde(rename="thirdPartyVerifierDetails")]
    
    pub third_party_verifier_details: Option<ThirdPartyVerifierAssignedTargetingOptionDetails>,
    /// URL details. This field will be populated when the targeting_type is `TARGETING_TYPE_URL`.
    #[serde(rename="urlDetails")]
    
    pub url_details: Option<UrlAssignedTargetingOptionDetails>,
    /// User rewarded content details. This field will be populated when the targeting_type is `TARGETING_TYPE_USER_REWARDED_CONTENT`.
    #[serde(rename="userRewardedContentDetails")]
    
    pub user_rewarded_content_details: Option<UserRewardedContentAssignedTargetingOptionDetails>,
    /// Video player size details. This field will be populated when the targeting_type is `TARGETING_TYPE_VIDEO_PLAYER_SIZE`.
    #[serde(rename="videoPlayerSizeDetails")]
    
    pub video_player_size_details: Option<VideoPlayerSizeAssignedTargetingOptionDetails>,
    /// Viewability details. This field will be populated when the targeting_type is `TARGETING_TYPE_VIEWABILITY`. You can only target one viewability option per resource.
    #[serde(rename="viewabilityDetails")]
    
    pub viewability_details: Option<ViewabilityAssignedTargetingOptionDetails>,
}

impl client::RequestValue for AssignedTargetingOption {}
impl client::ResponseResult for AssignedTargetingOption {}


/// A single assigned user role, which defines a user's authorized interaction with a specified partner or advertiser.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AssignedUserRole {
    /// The ID of the advertiser that the assigend user role applies to.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Output only. The ID of the assigned user role.
    #[serde(rename="assignedUserRoleId")]
    
    pub assigned_user_role_id: Option<String>,
    /// The ID of the partner that the assigned user role applies to.
    #[serde(rename="partnerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partner_id: Option<i64>,
    /// Required. The user role to assign to a user for the entity.
    #[serde(rename="userRole")]
    
    pub user_role: Option<String>,
}

impl client::Part for AssignedUserRole {}


/// Assigned audience group targeting option details. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_AUDIENCE_GROUP`. The relation between each group is UNION, except for excluded_first_and_third_party_audience_group and excluded_google_audience_group, of which COMPLEMENT is used as an INTERSECTION with other groups.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AudienceGroupAssignedTargetingOptionDetails {
    /// The first and third party audience ids and recencies of the excluded first and third party audience group. Used for negative targeting. The COMPLEMENT of the UNION of this group and other excluded audience groups is used as an INTERSECTION to any positive audience targeting. All items are logically ‘OR’ of each other.
    #[serde(rename="excludedFirstAndThirdPartyAudienceGroup")]
    
    pub excluded_first_and_third_party_audience_group: Option<FirstAndThirdPartyAudienceGroup>,
    /// The Google audience ids of the excluded Google audience group. Used for negative targeting. The COMPLEMENT of the UNION of this group and other excluded audience groups is used as an INTERSECTION to any positive audience targeting. Only contains Affinity, In-market and Installed-apps type Google audiences. All items are logically ‘OR’ of each other.
    #[serde(rename="excludedGoogleAudienceGroup")]
    
    pub excluded_google_audience_group: Option<GoogleAudienceGroup>,
    /// The combined audience ids of the included combined audience group. Contains combined audience ids only.
    #[serde(rename="includedCombinedAudienceGroup")]
    
    pub included_combined_audience_group: Option<CombinedAudienceGroup>,
    /// The custom list ids of the included custom list group. Contains custom list ids only.
    #[serde(rename="includedCustomListGroup")]
    
    pub included_custom_list_group: Option<CustomListGroup>,
    /// The first and third party audience ids and recencies of included first and third party audience groups. Each first and third party audience group contains first and third party audience ids only. The relation between each first and third party audience group is INTERSECTION, and the result is UNION'ed with other audience groups. Repeated groups with same settings will be ignored.
    #[serde(rename="includedFirstAndThirdPartyAudienceGroups")]
    
    pub included_first_and_third_party_audience_groups: Option<Vec<FirstAndThirdPartyAudienceGroup>>,
    /// The Google audience ids of the included Google audience group. Contains Google audience ids only.
    #[serde(rename="includedGoogleAudienceGroup")]
    
    pub included_google_audience_group: Option<GoogleAudienceGroup>,
}

impl client::Part for AudienceGroupAssignedTargetingOptionDetails {}


/// Details for audio content type assigned targeting option. This will be populated in the audio_content_type_details field when targeting_type is `TARGETING_TYPE_AUDIO_CONTENT_TYPE`. Explicitly targeting all options is not supported. Remove all audio content type targeting options to achieve this effect.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AudioContentTypeAssignedTargetingOptionDetails {
    /// The audio content type. Output only in v1. Required in v2.
    #[serde(rename="audioContentType")]
    
    pub audio_content_type: Option<String>,
    /// Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_AUDIO_CONTENT_TYPE`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for AudioContentTypeAssignedTargetingOptionDetails {}


/// Represents a targetable audio content type. This will be populated in the audio_content_type_details field when targeting_type is `TARGETING_TYPE_AUDIO_CONTENT_TYPE`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AudioContentTypeTargetingOptionDetails {
    /// Output only. The audio content type.
    #[serde(rename="audioContentType")]
    
    pub audio_content_type: Option<String>,
}

impl client::Part for AudioContentTypeTargetingOptionDetails {}


/// The length an audio or a video has been played.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AudioVideoOffset {
    /// The offset in percentage of the audio or video duration.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub percentage: Option<i64>,
    /// The offset in seconds from the start of the audio or video.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub seconds: Option<i64>,
}

impl client::Part for AudioVideoOffset {}


/// Response message for AdvertiserService.AuditAdvertiser.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [audit advertisers](AdvertiserAuditCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditAdvertiserResponse {
    /// The number of individual targeting options from the following targeting types that are assigned to a line item under this advertiser. These individual targeting options count towards the limit of 4500000 ad group targeting options per advertiser. Qualifying Targeting types: * Channels, URLs, apps, and collections * Demographic * Google Audiences, including Affinity, Custom Affinity, and In-market audiences * Inventory source * Keyword * Mobile app category * User lists * Video targeting * Viewability
    #[serde(rename="adGroupCriteriaCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ad_group_criteria_count: Option<i64>,
    /// The number of individual targeting options from the following targeting types that are assigned to a line item under this advertiser. These individual targeting options count towards the limit of 900000 campaign targeting options per advertiser. Qualifying Targeting types: * Position * Browser * Connection speed * Day and time * Device and operating system * Digital content label * Sensitive categories * Environment * Geography, including business chains and proximity * ISP * Language * Third-party verification
    #[serde(rename="campaignCriteriaCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub campaign_criteria_count: Option<i64>,
    /// The number of channels created under this advertiser. These channels count towards the limit of 1000 channels per advertiser.
    #[serde(rename="channelsCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub channels_count: Option<i64>,
    /// The number of negative keyword lists created under this advertiser. These negative keyword lists count towards the limit of 20 negative keyword lists per advertiser.
    #[serde(rename="negativeKeywordListsCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub negative_keyword_lists_count: Option<i64>,
    /// The number of negatively targeted channels created under this advertiser. These negatively targeted channels count towards the limit of 5 negatively targeted channels per advertiser.
    #[serde(rename="negativelyTargetedChannelsCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub negatively_targeted_channels_count: Option<i64>,
    /// The number of ACTIVE and PAUSED campaigns under this advertiser. These campaigns count towards the limit of 9999 campaigns per advertiser.
    #[serde(rename="usedCampaignsCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub used_campaigns_count: Option<i64>,
    /// The number of ACTIVE, PAUSED and DRAFT insertion orders under this advertiser. These insertion orders count towards the limit of 9999 insertion orders per advertiser.
    #[serde(rename="usedInsertionOrdersCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub used_insertion_orders_count: Option<i64>,
    /// The number of ACTIVE, PAUSED, and DRAFT line items under this advertiser. These line items count towards the limit of 9999 line items per advertiser.
    #[serde(rename="usedLineItemsCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub used_line_items_count: Option<i64>,
}

impl client::ResponseResult for AuditAdvertiserResponse {}


/// Represents an assigned authorized seller status. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizedSellerStatusAssignedTargetingOptionDetails {
    /// Output only. The authorized seller status to target.
    #[serde(rename="authorizedSellerStatus")]
    
    pub authorized_seller_status: Option<String>,
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for AuthorizedSellerStatusAssignedTargetingOptionDetails {}


/// Represents a targetable authorized seller status. This will be populated in the authorized_seller_status_details field when targeting_type is `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizedSellerStatusTargetingOptionDetails {
    /// Output only. The authorized seller status.
    #[serde(rename="authorizedSellerStatus")]
    
    pub authorized_seller_status: Option<String>,
}

impl client::Part for AuthorizedSellerStatusTargetingOptionDetails {}


/// Settings that control the bid strategy. Bid strategy determines the bid price.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BiddingStrategy {
    /// A strategy that uses a fixed bid price.
    #[serde(rename="fixedBid")]
    
    pub fixed_bid: Option<FixedBidStrategy>,
    /// A strategy that automatically adjusts the bid to optimize to your performance goal while spending the full budget. At insertion order level, the markup_type of line items cannot be set to `PARTNER_REVENUE_MODEL_MARKUP_TYPE_CPM`. In addition, when performance_goal_type is one of: * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_AV_VIEWED` , the line_item_type of the insertion order line items must be either: * `LINE_ITEM_TYPE_DISPLAY_DEFAULT` * `LINE_ITEM_TYPE_VIDEO_DEFAULT` , and when performance_goal_type is either: * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CIVA` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_IVO_TEN` the line_item_type of the insertion order line items must be `LINE_ITEM_TYPE_VIDEO_DEFAULT`.
    #[serde(rename="maximizeSpendAutoBid")]
    
    pub maximize_spend_auto_bid: Option<MaximizeSpendBidStrategy>,
    /// A strategy that automatically adjusts the bid to meet or beat a specified performance goal. It is to be used only for a line item entity.
    #[serde(rename="performanceGoalAutoBid")]
    
    pub performance_goal_auto_bid: Option<PerformanceGoalBidStrategy>,
}

impl client::Part for BiddingStrategy {}


/// Details for assigned browser targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_BROWSER`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BrowserAssignedTargetingOptionDetails {
    /// Output only. The display name of the browser.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Indicates if this option is being negatively targeted. All assigned browser targeting options on the same resource must have the same value for this field.
    
    pub negative: Option<bool>,
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_BROWSER`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for BrowserAssignedTargetingOptionDetails {}


/// Represents a targetable browser. This will be populated in the browser_details field when targeting_type is `TARGETING_TYPE_BROWSER`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BrowserTargetingOptionDetails {
    /// Output only. The display name of the browser.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for BrowserTargetingOptionDetails {}


/// Summarized information of an individual campaign budget.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BudgetSummary {
    /// Corresponds to the external_budget_id of a campaign budget. If the value is not set in the campaign budget, this field will be empty.
    #[serde(rename="externalBudgetId")]
    
    pub external_budget_id: Option<String>,
    /// The sum of charges made under this budget before taxes, in micros of the invoice's currency. For example, if currency_code is `USD`, then 1000000 represents one US dollar.
    #[serde(rename="preTaxAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pre_tax_amount_micros: Option<i64>,
    /// Relevant client, product, and estimate codes from the Mediaocean Prisma tool. Only applicable for campaign budgets with an external_budget_source of EXTERNAL_BUDGET_SOURCE_MEDIA_OCEAN.
    #[serde(rename="prismaCpeCode")]
    
    pub prisma_cpe_code: Option<PrismaCpeCode>,
    /// The amount of tax applied to charges under this budget, in micros of the invoice's currency. For example, if currency_code is `USD`, then 1000000 represents one US dollar.
    #[serde(rename="taxAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub tax_amount_micros: Option<i64>,
    /// The total sum of charges made under this budget, including tax, in micros of the invoice's currency. For example, if currency_code is `USD`, then 1000000 represents one US dollar.
    #[serde(rename="totalAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_amount_micros: Option<i64>,
}

impl client::Part for BudgetSummary {}


/// Request message for BulkEditAdvertiserAssignedTargetingOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [bulk edit advertiser assigned targeting options advertisers](AdvertiserBulkEditAdvertiserAssignedTargetingOptionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditAdvertiserAssignedTargetingOptionsRequest {
    /// The assigned targeting options to create in batch, specified as a list of `CreateAssignedTargetingOptionsRequest`. Supported targeting types: * `TARGETING_TYPE_CHANNEL` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_OMID` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`
    #[serde(rename="createRequests")]
    
    pub create_requests: Option<Vec<CreateAssignedTargetingOptionsRequest>>,
    /// The assigned targeting options to delete in batch, specified as a list of `DeleteAssignedTargetingOptionsRequest`. Supported targeting types: * `TARGETING_TYPE_CHANNEL` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_OMID` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`
    #[serde(rename="deleteRequests")]
    
    pub delete_requests: Option<Vec<DeleteAssignedTargetingOptionsRequest>>,
}

impl client::RequestValue for BulkEditAdvertiserAssignedTargetingOptionsRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [bulk edit advertiser assigned targeting options advertisers](AdvertiserBulkEditAdvertiserAssignedTargetingOptionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditAdvertiserAssignedTargetingOptionsResponse {
    /// The list of assigned targeting options that have been successfully created. This list will be absent if empty.
    #[serde(rename="createdAssignedTargetingOptions")]
    
    pub created_assigned_targeting_options: Option<Vec<AssignedTargetingOption>>,
}

impl client::ResponseResult for BulkEditAdvertiserAssignedTargetingOptionsResponse {}


/// Request message for AssignedInventorySourceService.BulkEdit.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assigned inventory sources bulk edit inventory source groups](InventorySourceGroupAssignedInventorySourceBulkEditCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditAssignedInventorySourcesRequest {
    /// The ID of the advertiser that owns the parent inventory source group. The parent partner does not have access to these assigned inventory sources.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// The assigned inventory sources to create in bulk, specified as a list of AssignedInventorySources.
    #[serde(rename="createdAssignedInventorySources")]
    
    pub created_assigned_inventory_sources: Option<Vec<AssignedInventorySource>>,
    /// The IDs of the assigned inventory sources to delete in bulk, specified as a list of assigned_inventory_source_ids.
    #[serde(rename="deletedAssignedInventorySources")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub deleted_assigned_inventory_sources: Option<Vec<i64>>,
    /// The ID of the partner that owns the inventory source group. Only this partner has write access to these assigned inventory sources.
    #[serde(rename="partnerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partner_id: Option<i64>,
}

impl client::RequestValue for BulkEditAssignedInventorySourcesRequest {}


/// Response message for AssignedInventorySourceService.BulkEdit.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assigned inventory sources bulk edit inventory source groups](InventorySourceGroupAssignedInventorySourceBulkEditCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditAssignedInventorySourcesResponse {
    /// The list of assigned inventory sources that have been successfully created. This list will be absent if empty.
    #[serde(rename="assignedInventorySources")]
    
    pub assigned_inventory_sources: Option<Vec<AssignedInventorySource>>,
}

impl client::ResponseResult for BulkEditAssignedInventorySourcesResponse {}


/// Request message for AssignedLocationService.BulkEditAssignedLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [location lists assigned locations bulk edit advertisers](AdvertiserLocationListAssignedLocationBulkEditCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditAssignedLocationsRequest {
    /// The assigned locations to create in bulk, specified as a list of AssignedLocations.
    #[serde(rename="createdAssignedLocations")]
    
    pub created_assigned_locations: Option<Vec<AssignedLocation>>,
    /// The IDs of the assigned locations to delete in bulk, specified as a list of assigned_location_ids.
    #[serde(rename="deletedAssignedLocations")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub deleted_assigned_locations: Option<Vec<i64>>,
}

impl client::RequestValue for BulkEditAssignedLocationsRequest {}


/// Response message for AssignedLocationService.BulkEditAssignedLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [location lists assigned locations bulk edit advertisers](AdvertiserLocationListAssignedLocationBulkEditCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditAssignedLocationsResponse {
    /// The list of assigned locations that have been successfully created. This list will be absent if empty.
    #[serde(rename="assignedLocations")]
    
    pub assigned_locations: Option<Vec<AssignedLocation>>,
}

impl client::ResponseResult for BulkEditAssignedLocationsResponse {}


/// Request message for BulkEditAssignedUserRoles.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [bulk edit assigned user roles users](UserBulkEditAssignedUserRoleCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditAssignedUserRolesRequest {
    /// The assigned user roles to create in batch, specified as a list of AssignedUserRoles.
    #[serde(rename="createdAssignedUserRoles")]
    
    pub created_assigned_user_roles: Option<Vec<AssignedUserRole>>,
    /// The assigned user roles to delete in batch, specified as a list of assigned_user_role_ids. The format of assigned_user_role_id is `entityType-entityid`, for example `partner-123`.
    #[serde(rename="deletedAssignedUserRoles")]
    
    pub deleted_assigned_user_roles: Option<Vec<String>>,
}

impl client::RequestValue for BulkEditAssignedUserRolesRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [bulk edit assigned user roles users](UserBulkEditAssignedUserRoleCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditAssignedUserRolesResponse {
    /// The list of assigned user roles that have been successfully created. This list will be absent if empty.
    #[serde(rename="createdAssignedUserRoles")]
    
    pub created_assigned_user_roles: Option<Vec<AssignedUserRole>>,
}

impl client::ResponseResult for BulkEditAssignedUserRolesResponse {}


/// Request message for BulkEditLineItemAssignedTargetingOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [line items bulk edit line item assigned targeting options advertisers](AdvertiserLineItemBulkEditLineItemAssignedTargetingOptionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditLineItemAssignedTargetingOptionsRequest {
    /// The assigned targeting options to create in batch, specified as a list of `CreateAssignedTargetingOptionsRequest`.
    #[serde(rename="createRequests")]
    
    pub create_requests: Option<Vec<CreateAssignedTargetingOptionsRequest>>,
    /// The assigned targeting options to delete in batch, specified as a list of `DeleteAssignedTargetingOptionsRequest`.
    #[serde(rename="deleteRequests")]
    
    pub delete_requests: Option<Vec<DeleteAssignedTargetingOptionsRequest>>,
}

impl client::RequestValue for BulkEditLineItemAssignedTargetingOptionsRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [line items bulk edit line item assigned targeting options advertisers](AdvertiserLineItemBulkEditLineItemAssignedTargetingOptionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditLineItemAssignedTargetingOptionsResponse {
    /// The list of assigned targeting options that have been successfully created. This list will be absent if empty.
    #[serde(rename="createdAssignedTargetingOptions")]
    
    pub created_assigned_targeting_options: Option<Vec<AssignedTargetingOption>>,
}

impl client::ResponseResult for BulkEditLineItemAssignedTargetingOptionsResponse {}


/// Request message for NegativeKeywordService.BulkEditNegativeKeywords.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [negative keyword lists negative keywords bulk edit advertisers](AdvertiserNegativeKeywordListNegativeKeywordBulkEditCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditNegativeKeywordsRequest {
    /// The negative keywords to create in batch, specified as a list of NegativeKeywords.
    #[serde(rename="createdNegativeKeywords")]
    
    pub created_negative_keywords: Option<Vec<NegativeKeyword>>,
    /// The negative keywords to delete in batch, specified as a list of keyword_values.
    #[serde(rename="deletedNegativeKeywords")]
    
    pub deleted_negative_keywords: Option<Vec<String>>,
}

impl client::RequestValue for BulkEditNegativeKeywordsRequest {}


/// Response message for NegativeKeywordService.BulkEditNegativeKeywords.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [negative keyword lists negative keywords bulk edit advertisers](AdvertiserNegativeKeywordListNegativeKeywordBulkEditCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditNegativeKeywordsResponse {
    /// The list of negative keywords that have been successfully created. This list will be absent if empty.
    #[serde(rename="negativeKeywords")]
    
    pub negative_keywords: Option<Vec<NegativeKeyword>>,
}

impl client::ResponseResult for BulkEditNegativeKeywordsResponse {}


/// Request message for BulkEditPartnerAssignedTargetingOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [bulk edit partner assigned targeting options partners](PartnerBulkEditPartnerAssignedTargetingOptionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditPartnerAssignedTargetingOptionsRequest {
    /// The assigned targeting options to create in batch, specified as a list of `CreateAssignedTargetingOptionsRequest`. Supported targeting types: * `TARGETING_TYPE_CHANNEL`
    #[serde(rename="createRequests")]
    
    pub create_requests: Option<Vec<CreateAssignedTargetingOptionsRequest>>,
    /// The assigned targeting options to delete in batch, specified as a list of `DeleteAssignedTargetingOptionsRequest`. Supported targeting types: * `TARGETING_TYPE_CHANNEL`
    #[serde(rename="deleteRequests")]
    
    pub delete_requests: Option<Vec<DeleteAssignedTargetingOptionsRequest>>,
}

impl client::RequestValue for BulkEditPartnerAssignedTargetingOptionsRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [bulk edit partner assigned targeting options partners](PartnerBulkEditPartnerAssignedTargetingOptionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditPartnerAssignedTargetingOptionsResponse {
    /// The list of assigned targeting options that have been successfully created. This list will be absent if empty.
    #[serde(rename="createdAssignedTargetingOptions")]
    
    pub created_assigned_targeting_options: Option<Vec<AssignedTargetingOption>>,
}

impl client::ResponseResult for BulkEditPartnerAssignedTargetingOptionsResponse {}


/// Request message for SiteService.BulkEditSites.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channels sites bulk edit advertisers](AdvertiserChannelSiteBulkEditCall) (request)
/// * [channels sites bulk edit partners](PartnerChannelSiteBulkEditCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditSitesRequest {
    /// The ID of the advertiser that owns the parent channel.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// The sites to create in batch, specified as a list of Sites.
    #[serde(rename="createdSites")]
    
    pub created_sites: Option<Vec<Site>>,
    /// The sites to delete in batch, specified as a list of site url_or_app_ids.
    #[serde(rename="deletedSites")]
    
    pub deleted_sites: Option<Vec<String>>,
    /// The ID of the partner that owns the parent channel.
    #[serde(rename="partnerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partner_id: Option<i64>,
}

impl client::RequestValue for BulkEditSitesRequest {}


/// Response message for SiteService.BulkEditSites.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channels sites bulk edit advertisers](AdvertiserChannelSiteBulkEditCall) (response)
/// * [channels sites bulk edit partners](PartnerChannelSiteBulkEditCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkEditSitesResponse {
    /// The list of sites that have been successfully created. This list will be absent if empty.
    
    pub sites: Option<Vec<Site>>,
}

impl client::ResponseResult for BulkEditSitesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [bulk list advertiser assigned targeting options advertisers](AdvertiserBulkListAdvertiserAssignedTargetingOptionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkListAdvertiserAssignedTargetingOptionsResponse {
    /// The list of assigned targeting options. This list will be absent if empty.
    #[serde(rename="assignedTargetingOptions")]
    
    pub assigned_targeting_options: Option<Vec<AssignedTargetingOption>>,
    /// A token identifying the next page of results. This value should be specified as the pageToken in a subsequent BulkListAdvertiserAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for BulkListAdvertiserAssignedTargetingOptionsResponse {}


/// Response message for BulkListCampaignAssignedTargetingOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [campaigns bulk list campaign assigned targeting options advertisers](AdvertiserCampaignBulkListCampaignAssignedTargetingOptionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkListCampaignAssignedTargetingOptionsResponse {
    /// The list of assigned targeting options. This list will be absent if empty.
    #[serde(rename="assignedTargetingOptions")]
    
    pub assigned_targeting_options: Option<Vec<AssignedTargetingOption>>,
    /// A token identifying the next page of results. This value should be specified as the pageToken in a subsequent BulkListCampaignAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for BulkListCampaignAssignedTargetingOptionsResponse {}


/// Response message for BulkListInsertionOrderAssignedTargetingOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insertion orders bulk list insertion order assigned targeting options advertisers](AdvertiserInsertionOrderBulkListInsertionOrderAssignedTargetingOptionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkListInsertionOrderAssignedTargetingOptionsResponse {
    /// The list of assigned targeting options. This list will be absent if empty.
    #[serde(rename="assignedTargetingOptions")]
    
    pub assigned_targeting_options: Option<Vec<AssignedTargetingOption>>,
    /// A token identifying the next page of results. This value should be specified as the pageToken in a subsequent BulkListInsertionOrderAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for BulkListInsertionOrderAssignedTargetingOptionsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [line items bulk list line item assigned targeting options advertisers](AdvertiserLineItemBulkListLineItemAssignedTargetingOptionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkListLineItemAssignedTargetingOptionsResponse {
    /// The list of assigned targeting options. This list will be absent if empty.
    #[serde(rename="assignedTargetingOptions")]
    
    pub assigned_targeting_options: Option<Vec<AssignedTargetingOption>>,
    /// A token identifying the next page of results. This value should be specified as the pageToken in a subsequent BulkListLineItemAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for BulkListLineItemAssignedTargetingOptionsResponse {}


/// Details for assigned Business chain targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_BUSINESS_CHAIN`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BusinessChainAssignedTargetingOptionDetails {
    /// Output only. The display name of a business chain, e.g. "KFC", "Chase Bank".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. The radius of the area around the business chain that will be targeted. The units of the radius are specified by proximity_radius_unit. Must be 1 to 800 if unit is `DISTANCE_UNIT_KILOMETERS` and 1 to 500 if unit is `DISTANCE_UNIT_MILES`. The minimum increment for both cases is 0.1. Inputs will be rounded to the nearest acceptable value if it is too granular, e.g. 15.57 will become 15.6.
    #[serde(rename="proximityRadiusAmount")]
    
    pub proximity_radius_amount: Option<f64>,
    /// Required. The unit of distance by which the targeting radius is measured.
    #[serde(rename="proximityRadiusUnit")]
    
    pub proximity_radius_unit: Option<String>,
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_BUSINESS_CHAIN`. Accepted business chain targeting option IDs can be retrieved using SearchTargetingOptions.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for BusinessChainAssignedTargetingOptionDetails {}


/// Search terms for Business Chain targeting options. At least one of the field should be populated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BusinessChainSearchTerms {
    /// The search query for the desired business chain. The query must be the full name of the business, e.g. "KFC", "mercedes-benz".
    #[serde(rename="businessChainQuery")]
    
    pub business_chain_query: Option<String>,
    /// The search query for the desired geo region, e.g. "Seattle", "United State".
    #[serde(rename="regionQuery")]
    
    pub region_query: Option<String>,
}

impl client::Part for BusinessChainSearchTerms {}


/// Represents a targetable business chain within a geo region. This will be populated in the business_chain_details field when targeting_type is `TARGETING_TYPE_BUSINESS_CHAIN`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BusinessChainTargetingOptionDetails {
    /// Output only. The display name of the business chain, e.g. "KFC", "Chase Bank".
    #[serde(rename="businessChain")]
    
    pub business_chain: Option<String>,
    /// Output only. The display name of the geographic region, e.g. "Ontario, Canada".
    #[serde(rename="geoRegion")]
    
    pub geo_region: Option<String>,
    /// Output only. The type of the geographic region.
    #[serde(rename="geoRegionType")]
    
    pub geo_region_type: Option<String>,
}

impl client::Part for BusinessChainTargetingOptionDetails {}


/// A single campaign.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [campaigns create advertisers](AdvertiserCampaignCreateCall) (request|response)
/// * [campaigns get advertisers](AdvertiserCampaignGetCall) (response)
/// * [campaigns patch advertisers](AdvertiserCampaignPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Campaign {
    /// Output only. The unique ID of the advertiser the campaign belongs to.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// The list of budgets available to this campaign. If this field is not set, the campaign uses an unlimited budget.
    #[serde(rename="campaignBudgets")]
    
    pub campaign_budgets: Option<Vec<CampaignBudget>>,
    /// Required. The planned spend and duration of the campaign.
    #[serde(rename="campaignFlight")]
    
    pub campaign_flight: Option<CampaignFlight>,
    /// Required. The goal of the campaign.
    #[serde(rename="campaignGoal")]
    
    pub campaign_goal: Option<CampaignGoal>,
    /// Output only. The unique ID of the campaign. Assigned by the system.
    #[serde(rename="campaignId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub campaign_id: Option<i64>,
    /// Required. The display name of the campaign. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. Controls whether or not the insertion orders under this campaign can spend their budgets and bid on inventory. * Accepted values are `ENTITY_STATUS_ACTIVE`, `ENTITY_STATUS_ARCHIVED`, and `ENTITY_STATUS_PAUSED`. * For CreateCampaign method, `ENTITY_STATUS_ARCHIVED` is not allowed.
    #[serde(rename="entityStatus")]
    
    pub entity_status: Option<String>,
    /// Required. The frequency cap setting of the campaign.
    #[serde(rename="frequencyCap")]
    
    pub frequency_cap: Option<FrequencyCap>,
    /// Output only. The resource name of the campaign.
    
    pub name: Option<String>,
    /// Output only. The timestamp when the campaign was last updated. Assigned by the system.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Campaign {}
impl client::ResponseResult for Campaign {}


/// Settings that control how the campaign budget is allocated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CampaignBudget {
    /// Required. The total amount the linked insertion order segments can budget. The amount is in micros. Must be greater than 0. For example, 500000000 represents 500 standard units of the currency.
    #[serde(rename="budgetAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub budget_amount_micros: Option<i64>,
    /// The unique ID of the campaign budget. Assigned by the system. Do not set for new budgets. Must be included when updating or adding budgets to campaign_budgets. Otherwise, a new ID will be generated and assigned.
    #[serde(rename="budgetId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub budget_id: Option<i64>,
    /// Required. Immutable. Specifies whether the budget is measured in currency or impressions.
    #[serde(rename="budgetUnit")]
    
    pub budget_unit: Option<String>,
    /// Required. The date range for the campaign budget. Linked budget segments may have a different date range. They are resolved relative to the parent advertiser's time zone. Both `start_date` and `end_date` must be before the year 2037.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<DateRange>,
    /// Required. The display name of the budget. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Immutable. The ID identifying this budget to the external source. If this field is set and the invoice detail level of the corresponding billing profile is set to "Budget level PO", all impressions served against this budget will include this ID on the invoice. Must be unique under the campaign.
    #[serde(rename="externalBudgetId")]
    
    pub external_budget_id: Option<String>,
    /// Required. The external source of the budget.
    #[serde(rename="externalBudgetSource")]
    
    pub external_budget_source: Option<String>,
    /// Immutable. The ID used to group budgets to be included the same invoice. If this field is set and the invoice level of the corresponding billing profile is set to "Budget invoice grouping ID", all external_budget_id sharing the same invoice_grouping_id will be grouped in the same invoice.
    #[serde(rename="invoiceGroupingId")]
    
    pub invoice_grouping_id: Option<String>,
    /// Additional metadata for use by the Mediaocean Prisma tool. Required for Mediaocean budgets. Only applicable to prisma_enabled advertisers.
    #[serde(rename="prismaConfig")]
    
    pub prisma_config: Option<PrismaConfig>,
}

impl client::Part for CampaignBudget {}


/// Settings that track the planned spend and duration of a campaign.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CampaignFlight {
    /// Required. The dates that the campaign is expected to run. They are resolved relative to the parent advertiser's time zone. * The dates specified here will not affect serving. They are used to generate alerts and warnings. For example, if the flight date of any child insertion order is outside the range of these dates, the user interface will show a warning. * `start_date` is required and must be the current date or later. * `end_date` is optional. If specified, it must be the `start_date` or later. * Any specified date must be before the year 2037.
    #[serde(rename="plannedDates")]
    
    pub planned_dates: Option<DateRange>,
    /// The amount the campaign is expected to spend for its given planned_dates. This will not limit serving, but will be used for tracking spend in the DV360 UI. The amount is in micros. Must be greater than or equal to 0. For example, 500000000 represents 500 standard units of the currency.
    #[serde(rename="plannedSpendAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub planned_spend_amount_micros: Option<i64>,
}

impl client::Part for CampaignFlight {}


/// Settings that control the goal of a campaign.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CampaignGoal {
    /// Required. The type of the campaign goal.
    #[serde(rename="campaignGoalType")]
    
    pub campaign_goal_type: Option<String>,
    /// Required. The performance goal of the campaign. Acceptable values for performance_goal_type are: * `PERFORMANCE_GOAL_TYPE_CPM` * `PERFORMANCE_GOAL_TYPE_CPC` * `PERFORMANCE_GOAL_TYPE_CPA` * `PERFORMANCE_GOAL_TYPE_CPIAVC` * `PERFORMANCE_GOAL_TYPE_CTR` * `PERFORMANCE_GOAL_TYPE_VIEWABILITY` * `PERFORMANCE_GOAL_TYPE_OTHER`
    #[serde(rename="performanceGoal")]
    
    pub performance_goal: Option<PerformanceGoal>,
}

impl client::Part for CampaignGoal {}


/// Details for assigned carrier and ISP targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_CARRIER_AND_ISP`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CarrierAndIspAssignedTargetingOptionDetails {
    /// Output only. The display name of the carrier or ISP.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Indicates if this option is being negatively targeted. All assigned carrier and ISP targeting options on the same resource must have the same value for this field.
    
    pub negative: Option<bool>,
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_CARRIER_AND_ISP`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for CarrierAndIspAssignedTargetingOptionDetails {}


/// Represents a targetable carrier or ISP. This will be populated in the carrier_and_isp_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_CARRIER_AND_ISP`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CarrierAndIspTargetingOptionDetails {
    /// Output only. The display name of the carrier or ISP.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The type indicating if it's carrier or ISP.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for CarrierAndIspTargetingOptionDetails {}


/// Assigned category targeting option details. This will be populated in the category_details field when targeting_type is `TARGETING_TYPE_CATEGORY`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CategoryAssignedTargetingOptionDetails {
    /// Output only. The display name of the category.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Indicates if this option is being negatively targeted.
    
    pub negative: Option<bool>,
    /// Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_CATEGORY`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for CategoryAssignedTargetingOptionDetails {}


/// Represents a targetable category. This will be populated in the category_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_CATEGORY`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CategoryTargetingOptionDetails {
    /// Output only. The display name of the category.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for CategoryTargetingOptionDetails {}


/// A single channel. Channels are custom groups of related websites and apps.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channels create advertisers](AdvertiserChannelCreateCall) (request|response)
/// * [channels get advertisers](AdvertiserChannelGetCall) (response)
/// * [channels patch advertisers](AdvertiserChannelPatchCall) (request|response)
/// * [channels create partners](PartnerChannelCreateCall) (request|response)
/// * [channels get partners](PartnerChannelGetCall) (response)
/// * [channels patch partners](PartnerChannelPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    /// The ID of the advertiser that owns the channel.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Output only. The unique ID of the channel. Assigned by the system.
    #[serde(rename="channelId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub channel_id: Option<i64>,
    /// Required. The display name of the channel. Must be UTF-8 encoded with a maximum length of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The resource name of the channel.
    
    pub name: Option<String>,
    /// Output only. Number of line items that are directly targeting this channel negatively.
    #[serde(rename="negativelyTargetedLineItemCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub negatively_targeted_line_item_count: Option<i64>,
    /// The ID of the partner that owns the channel.
    #[serde(rename="partnerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partner_id: Option<i64>,
    /// Output only. Number of line items that are directly targeting this channel positively.
    #[serde(rename="positivelyTargetedLineItemCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub positively_targeted_line_item_count: Option<i64>,
}

impl client::RequestValue for Channel {}
impl client::ResponseResult for Channel {}


/// Details for assigned channel targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_CHANNEL`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelAssignedTargetingOptionDetails {
    /// Required. ID of the channel. Should refer to the channel ID field on a [Partner-owned channel](partners.channels#Channel.FIELDS.channel_id) or [advertiser-owned channel](advertisers.channels#Channel.FIELDS.channel_id) resource.
    #[serde(rename="channelId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub channel_id: Option<i64>,
    /// Indicates if this option is being negatively targeted. For advertiser level assigned targeting option, this field must be true.
    
    pub negative: Option<bool>,
}

impl client::Part for ChannelAssignedTargetingOptionDetails {}


/// Settings for advertisers that use both Campaign Manager 360 (CM360) and third-party ad servers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CmHybridConfig {
    /// Required. Immutable. Account ID of the CM360 Floodlight configuration linked with the DV360 advertiser.
    #[serde(rename="cmAccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub cm_account_id: Option<i64>,
    /// Required. Immutable. ID of the CM360 Floodlight configuration linked with the DV360 advertiser.
    #[serde(rename="cmFloodlightConfigId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub cm_floodlight_config_id: Option<i64>,
    /// Required. Immutable. By setting this field to `true`, you, on behalf of your company, authorize the sharing of information from the given Floodlight configuration to this Display & Video 360 advertiser.
    #[serde(rename="cmFloodlightLinkingAuthorized")]
    
    pub cm_floodlight_linking_authorized: Option<bool>,
    /// A list of CM360 sites whose placements will be synced to DV360 as creatives. If absent or empty in CreateAdvertiser method, the system will automatically create a CM360 site. Removing sites from this list may cause DV360 creatives synced from CM360 to be deleted. At least one site must be specified.
    #[serde(rename="cmSyncableSiteIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub cm_syncable_site_ids: Option<Vec<i64>>,
    /// Whether or not to report DV360 cost to CM360.
    #[serde(rename="dv360ToCmCostReportingEnabled")]
    
    pub dv360_to_cm_cost_reporting_enabled: Option<bool>,
    /// Whether or not to include DV360 data in CM360 data transfer reports.
    #[serde(rename="dv360ToCmDataSharingEnabled")]
    
    pub dv360_to_cm_data_sharing_enabled: Option<bool>,
}

impl client::Part for CmHybridConfig {}


/// A Campaign Manager 360 tracking ad.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CmTrackingAd {
    /// The ad ID of the campaign manager 360 tracking Ad.
    #[serde(rename="cmAdId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub cm_ad_id: Option<i64>,
    /// The creative ID of the campaign manager 360 tracking Ad.
    #[serde(rename="cmCreativeId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub cm_creative_id: Option<i64>,
    /// The placement ID of the campaign manager 360 tracking Ad.
    #[serde(rename="cmPlacementId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub cm_placement_id: Option<i64>,
}

impl client::Part for CmTrackingAd {}


/// Describes a combined audience resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get combined audiences](CombinedAudienceGetCall) (response)
/// * [list combined audiences](CombinedAudienceListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CombinedAudience {
    /// Output only. The unique ID of the combined audience. Assigned by the system.
    #[serde(rename="combinedAudienceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub combined_audience_id: Option<i64>,
    /// Output only. The display name of the combined audience. .
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The resource name of the combined audience.
    
    pub name: Option<String>,
}

impl client::Resource for CombinedAudience {}
impl client::ResponseResult for CombinedAudience {}


/// Details of combined audience group. All combined audience targeting settings are logically ‘OR’ of each other.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CombinedAudienceGroup {
    /// Required. All combined audience targeting settings in combined audience group. Repeated settings with same id will be ignored. The number of combined audience settings should be no more than five, error will be thrown otherwise.
    
    pub settings: Option<Vec<CombinedAudienceTargetingSetting>>,
}

impl client::Part for CombinedAudienceGroup {}


/// Details of combined audience targeting setting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CombinedAudienceTargetingSetting {
    /// Required. Combined audience id of combined audience targeting setting. This id is combined_audience_id.
    #[serde(rename="combinedAudienceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub combined_audience_id: Option<i64>,
}

impl client::Part for CombinedAudienceTargetingSetting {}


/// Contact information defining a Customer Match audience member.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactInfo {
    /// Country code of the member. Must also be set with the following fields: * hashed_first_name * hashed_last_name * zip_codes
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// A list of SHA256 hashed email of the member. Before hashing, remove all whitespace and make sure the string is all lowercase.
    #[serde(rename="hashedEmails")]
    
    pub hashed_emails: Option<Vec<String>>,
    /// SHA256 hashed first name of the member. Before hashing, remove all whitespace and make sure the string is all lowercase. Must also be set with the following fields: * country_code * hashed_last_name * zip_codes
    #[serde(rename="hashedFirstName")]
    
    pub hashed_first_name: Option<String>,
    /// SHA256 hashed last name of the member. Before hashing, remove all whitespace and make sure the string is all lowercase. Must also be set with the following fields: * country_code * hashed_first_name * zip_codes
    #[serde(rename="hashedLastName")]
    
    pub hashed_last_name: Option<String>,
    /// A list of SHA256 hashed phone numbers of the member. Before hashing, all phone numbers must be formatted using the [E.164 format](https://developers.google.com//en.wikipedia.org/wiki/E.164) and include the country calling code.
    #[serde(rename="hashedPhoneNumbers")]
    
    pub hashed_phone_numbers: Option<Vec<String>>,
    /// A list of zip codes of the member. Must also be set with the following fields: * country_code * hashed_first_name * hashed_last_name
    #[serde(rename="zipCodes")]
    
    pub zip_codes: Option<Vec<String>>,
}

impl client::Part for ContactInfo {}


/// Wrapper message for a list of contact information defining Customer Match audience members.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactInfoList {
    /// A list of ContactInfo objects defining Customer Match audience members. The size of members after splitting the contact_infos mustn't be greater than 500,000.
    #[serde(rename="contactInfos")]
    
    pub contact_infos: Option<Vec<ContactInfo>>,
}

impl client::Part for ContactInfoList {}


/// Details for content duration assigned targeting option. This will be populated in the content_duration_details field when targeting_type is `TARGETING_TYPE_CONTENT_DURATION`. Explicitly targeting all options is not supported. Remove all content duration targeting options to achieve this effect.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentDurationAssignedTargetingOptionDetails {
    /// Output only. The content duration.
    #[serde(rename="contentDuration")]
    
    pub content_duration: Option<String>,
    /// Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_CONTENT_DURATION`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for ContentDurationAssignedTargetingOptionDetails {}


/// Represents a targetable content duration. This will be populated in the content_duration_details field when targeting_type is `TARGETING_TYPE_CONTENT_DURATION`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentDurationTargetingOptionDetails {
    /// Output only. The content duration.
    #[serde(rename="contentDuration")]
    
    pub content_duration: Option<String>,
}

impl client::Part for ContentDurationTargetingOptionDetails {}


/// Details for content genre assigned targeting option. This will be populated in the content_genre_details field when targeting_type is `TARGETING_TYPE_CONTENT_GENRE`. Explicitly targeting all options is not supported. Remove all content genre targeting options to achieve this effect.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentGenreAssignedTargetingOptionDetails {
    /// Output only. The display name of the content genre.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Indicates if this option is being negatively targeted.
    
    pub negative: Option<bool>,
    /// Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_CONTENT_GENRE`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for ContentGenreAssignedTargetingOptionDetails {}


/// Represents a targetable content genre. This will be populated in the content_genre_details field when targeting_type is `TARGETING_TYPE_CONTENT_GENRE`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentGenreTargetingOptionDetails {
    /// Output only. The display name of the content genre
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for ContentGenreTargetingOptionDetails {}


/// Assigned content instream position targeting option details. This will be populated in the content_instream_position_details field when targeting_type is `TARGETING_TYPE_CONTENT_INSTREAM_POSITION`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentInstreamPositionAssignedTargetingOptionDetails {
    /// Output only. The ad type to target. Only applicable to insertion order targeting and new line items supporting the specified ad type will inherit this targeting option by default. Possible values are: * `AD_TYPE_VIDEO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_VIDEO_DEFAULT`. * `AD_TYPE_AUDIO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_AUDIO_DEFAULT`.
    #[serde(rename="adType")]
    
    pub ad_type: Option<String>,
    /// The content instream position for video or audio ads. Output only in v1. Required in v2.
    #[serde(rename="contentInstreamPosition")]
    
    pub content_instream_position: Option<String>,
    /// Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_CONTENT_INSTREAM_POSITION`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for ContentInstreamPositionAssignedTargetingOptionDetails {}


/// Represents a targetable content instream position, which could be used by video and audio ads. This will be populated in the content_instream_position_details field when targeting_type is `TARGETING_TYPE_CONTENT_INSTREAM_POSITION`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentInstreamPositionTargetingOptionDetails {
    /// Output only. The content instream position.
    #[serde(rename="contentInstreamPosition")]
    
    pub content_instream_position: Option<String>,
}

impl client::Part for ContentInstreamPositionTargetingOptionDetails {}


/// Assigned content outstream position targeting option details. This will be populated in the content_outstream_position_details field when targeting_type is `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentOutstreamPositionAssignedTargetingOptionDetails {
    /// Output only. The ad type to target. Only applicable to insertion order targeting and new line items supporting the specified ad type will inherit this targeting option by default. Possible values are: * `AD_TYPE_DISPLAY`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_DISPLAY_DEFAULT`. * `AD_TYPE_VIDEO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_VIDEO_DEFAULT`.
    #[serde(rename="adType")]
    
    pub ad_type: Option<String>,
    /// The content outstream position. Output only in v1. Required in v2.
    #[serde(rename="contentOutstreamPosition")]
    
    pub content_outstream_position: Option<String>,
    /// Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for ContentOutstreamPositionAssignedTargetingOptionDetails {}


/// Represents a targetable content outstream position, which could be used by display and video ads. This will be populated in the content_outstream_position_details field when targeting_type is `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentOutstreamPositionTargetingOptionDetails {
    /// Output only. The content outstream position.
    #[serde(rename="contentOutstreamPosition")]
    
    pub content_outstream_position: Option<String>,
}

impl client::Part for ContentOutstreamPositionTargetingOptionDetails {}


/// Details for content stream type assigned targeting option. This will be populated in the content_stream_type_details field when targeting_type is `TARGETING_TYPE_CONTENT_STREAM_TYPE`. Explicitly targeting all options is not supported. Remove all content stream type targeting options to achieve this effect.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentStreamTypeAssignedTargetingOptionDetails {
    /// Output only. The content stream type.
    #[serde(rename="contentStreamType")]
    
    pub content_stream_type: Option<String>,
    /// Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_CONTENT_STREAM_TYPE`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for ContentStreamTypeAssignedTargetingOptionDetails {}


/// Represents a targetable content stream type. This will be populated in the content_stream_type_details field when targeting_type is `TARGETING_TYPE_CONTENT_STREAM_TYPE`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentStreamTypeTargetingOptionDetails {
    /// Output only. The content stream type.
    #[serde(rename="contentStreamType")]
    
    pub content_stream_type: Option<String>,
}

impl client::Part for ContentStreamTypeTargetingOptionDetails {}


/// Settings that control how conversions are counted. All post-click conversions will be counted. A percentage value can be set for post-view conversions counting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConversionCountingConfig {
    /// The Floodlight activity configs used to track conversions. The number of conversions counted is the sum of all of the conversions counted by all of the Floodlight activity IDs specified in this field.
    #[serde(rename="floodlightActivityConfigs")]
    
    pub floodlight_activity_configs: Option<Vec<TrackingFloodlightActivityConfig>>,
    /// The percentage of post-view conversions to count, in millis (1/1000 of a percent). Must be between 0 and 100000 inclusive. For example, to track 50% of the post-click conversions, set a value of 50000.
    #[serde(rename="postViewCountPercentageMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub post_view_count_percentage_millis: Option<i64>,
}

impl client::Part for ConversionCountingConfig {}


/// Counter event of the creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CounterEvent {
    /// Required. The name of the counter event.
    
    pub name: Option<String>,
    /// Required. The name used to identify this counter event in reports.
    #[serde(rename="reportingName")]
    
    pub reporting_name: Option<String>,
}

impl client::Part for CounterEvent {}


/// A request message for CreateAsset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assets upload advertisers](AdvertiserAssetUploadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateAssetRequest {
    /// Required. The filename of the asset, including the file extension. The filename must be UTF-8 encoded with a maximum size of 240 bytes.
    
    pub filename: Option<String>,
}

impl client::RequestValue for CreateAssetRequest {}


/// A response message for CreateAsset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assets upload advertisers](AdvertiserAssetUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateAssetResponse {
    /// The uploaded asset, if successful.
    
    pub asset: Option<Asset>,
}

impl client::ResponseResult for CreateAssetResponse {}


/// A request listing which assigned targeting options of a given targeting type should be created and added.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateAssignedTargetingOptionsRequest {
    /// Required. The assigned targeting options to create and add.
    #[serde(rename="assignedTargetingOptions")]
    
    pub assigned_targeting_options: Option<Vec<AssignedTargetingOption>>,
    /// Required. Identifies the type of this assigned targeting option.
    #[serde(rename="targetingType")]
    
    pub targeting_type: Option<String>,
}

impl client::Part for CreateAssignedTargetingOptionsRequest {}


/// Request message for \[SdfDownloadTaskService.CreateSdfDownloadTask\].
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create sdfdownloadtasks](SdfdownloadtaskCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateSdfDownloadTaskRequest {
    /// The ID of the advertiser to download SDF for.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Filters on entities by their entity IDs.
    #[serde(rename="idFilter")]
    
    pub id_filter: Option<IdFilter>,
    /// Filters on Inventory Sources by their IDs.
    #[serde(rename="inventorySourceFilter")]
    
    pub inventory_source_filter: Option<InventorySourceFilter>,
    /// Filters on selected file types. The entities in each file are filtered by a chosen set of filter entities. The filter entities must be the same type as, or a parent type of, the selected file types.
    #[serde(rename="parentEntityFilter")]
    
    pub parent_entity_filter: Option<ParentEntityFilter>,
    /// The ID of the partner to download SDF for.
    #[serde(rename="partnerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partner_id: Option<i64>,
    /// Required. The SDF version of the downloaded file. If set to `SDF_VERSION_UNSPECIFIED`, this will default to the version specified by the advertiser or partner identified by `root_id`. An advertiser inherits its SDF version from its partner unless configured otherwise.
    
    pub version: Option<String>,
}

impl client::RequestValue for CreateSdfDownloadTaskRequest {}


/// A single Creative.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives create advertisers](AdvertiserCreativeCreateCall) (request|response)
/// * [creatives get advertisers](AdvertiserCreativeGetCall) (response)
/// * [creatives patch advertisers](AdvertiserCreativePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Creative {
    /// Additional dimensions. Applicable when creative_type is one of: * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_EXPANDABLE` * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_TEMPLATED_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_LIGHTBOX` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` * `CREATIVE_TYPE_PUBLISHER_HOSTED` If this field is specified, width_pixels and height_pixels are both required and must be greater than or equal to 0.
    #[serde(rename="additionalDimensions")]
    
    pub additional_dimensions: Option<Vec<Dimensions>>,
    /// Output only. The unique ID of the advertiser the creative belongs to.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Third-party HTML tracking tag to be appended to the creative tag.
    #[serde(rename="appendedTag")]
    
    pub appended_tag: Option<String>,
    /// Required. Assets associated to this creative. Assets can be associated to the creative in one of following roles: * `ASSET_ROLE_UNSPECIFIED` * `ASSET_ROLE_MAIN` * `ASSET_ROLE_BACKUP` * `ASSET_ROLE_POLITE_LOAD`
    
    pub assets: Option<Vec<AssetAssociation>>,
    /// Output only. The unique ID of the Campaign Manager 360 placement associated with the creative. This field is only applicable for creatives that are synced from Campaign Manager.
    #[serde(rename="cmPlacementId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub cm_placement_id: Option<i64>,
    /// The Campaign Manager 360 tracking ad associated with the creative. Optional for the following creative_type when created by an advertiser that uses both Campaign Manager 360 and third-party ad serving: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` Output only for other cases.
    #[serde(rename="cmTrackingAd")]
    
    pub cm_tracking_ad: Option<CmTrackingAd>,
    /// The IDs of companion creatives for a video creative. You can assign existing display creatives (with image or HTML5 assets) to serve surrounding the publisher's video player. Companions display around the video player while the video is playing and remain after the video has completed. Creatives contain additional dimensions can not be companion creatives. This field is only supported for following creative_type: * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_VIDEO`
    #[serde(rename="companionCreativeIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub companion_creative_ids: Option<Vec<i64>>,
    /// Counter events for a rich media creative. Counters track the number of times that a user interacts with any part of a rich media creative in a specified way (mouse-overs, mouse-outs, clicks, taps, data loading, keyboard entries, etc.). Any event that can be captured in the creative can be recorded as a counter. Leave it empty or unset for creatives containing image assets only.
    #[serde(rename="counterEvents")]
    
    pub counter_events: Option<Vec<CounterEvent>>,
    /// Output only. The timestamp when the creative was created. Assigned by the system.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. A list of attributes of the creative that is generated by the system.
    #[serde(rename="creativeAttributes")]
    
    pub creative_attributes: Option<Vec<String>>,
    /// Output only. The unique ID of the creative. Assigned by the system.
    #[serde(rename="creativeId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creative_id: Option<i64>,
    /// Required. Immutable. The type of the creative.
    #[serde(rename="creativeType")]
    
    pub creative_type: Option<String>,
    /// Required. Primary dimensions of the creative. Applicable to all creative types. The value of width_pixels and height_pixels defaults to `0` when creative_type is one of: * `CREATIVE_TYPE_VIDEO` * `CREATIVE_TYPE_TEMPLATED_APP_INSTALL_INTERSTITIAL` * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_NATIVE_VIDEO` * `CREATIVE_TYPE_TEMPLATED_APP_INSTALL_VIDEO`
    
    pub dimensions: Option<Dimensions>,
    /// Required. The display name of the creative. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Indicates whether the creative is dynamic.
    
    pub dynamic: Option<bool>,
    /// Required. Controls whether or not the creative can serve. Accepted values are: * `ENTITY_STATUS_ACTIVE` * `ENTITY_STATUS_ARCHIVED` * `ENTITY_STATUS_PAUSED`
    #[serde(rename="entityStatus")]
    
    pub entity_status: Option<String>,
    /// Required. Exit events for this creative. An exit (also known as a click tag) is any area in your creative that someone can click or tap to open an advertiser's landing page. Every creative must include at least one exit. You can add an exit to your creative in any of the following ways: * Use Google Web Designer's tap area. * Define a JavaScript variable called "clickTag". * Use the Enabler (Enabler.exit()) to track exits in rich media formats.
    #[serde(rename="exitEvents")]
    
    pub exit_events: Option<Vec<ExitEvent>>,
    /// Optional. Indicates the creative will automatically expand on hover. Optional and only valid for third-party expandable creatives. Third-party expandable creatives are creatives with following hosting source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_EXPANDABLE`
    #[serde(rename="expandOnHover")]
    
    pub expand_on_hover: Option<bool>,
    /// Optional. Specifies the expanding direction of the creative. Required and only valid for third-party expandable creatives. Third-party expandable creatives are creatives with following hosting source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_EXPANDABLE`
    #[serde(rename="expandingDirection")]
    
    pub expanding_direction: Option<String>,
    /// Required. Indicates where the creative is hosted.
    #[serde(rename="hostingSource")]
    
    pub hosting_source: Option<String>,
    /// Output only. Indicates the third-party VAST tag creative requires HTML5 Video support. Output only and only valid for third-party VAST tag creatives. Third-party VAST tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_VIDEO`
    #[serde(rename="html5Video")]
    
    pub html5_video: Option<bool>,
    /// Indicates whether Integral Ad Science (IAS) campaign monitoring is enabled. To enable this for the creative, make sure the Advertiser.creative_config.ias_client_id has been set to your IAS client ID.
    #[serde(rename="iasCampaignMonitoring")]
    
    pub ias_campaign_monitoring: Option<bool>,
    /// ID information used to link this creative to an external system. Must be UTF-8 encoded with a length of no more than 10,000 characters.
    #[serde(rename="integrationCode")]
    
    pub integration_code: Option<String>,
    /// JavaScript measurement URL from supported third-party verification providers (ComScore, DoubleVerify, IAS, Moat). HTML script tags are not supported. This field is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`
    #[serde(rename="jsTrackerUrl")]
    
    pub js_tracker_url: Option<String>,
    /// Output only. The IDs of the line items this creative is associated with. To associate a creative to a line item, use LineItem.creative_ids instead.
    #[serde(rename="lineItemIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub line_item_ids: Option<Vec<i64>>,
    /// Output only. Media duration of the creative. Applicable when creative_type is one of: * `CREATIVE_TYPE_VIDEO` * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_NATIVE_VIDEO` * `CREATIVE_TYPE_PUBLISHER_HOSTED`
    #[serde(rename="mediaDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub media_duration: Option<client::chrono::Duration>,
    /// Output only. Indicates the third-party audio creative supports MP3. Output only and only valid for third-party audio creatives. Third-party audio creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_AUDIO`
    #[serde(rename="mp3Audio")]
    
    pub mp3_audio: Option<bool>,
    /// Output only. The resource name of the creative.
    
    pub name: Option<String>,
    /// User notes for this creative. Must be UTF-8 encoded with a length of no more than 20,000 characters.
    
    pub notes: Option<String>,
    /// Specifies the OBA icon for a video creative. This field is only supported in following creative_type: * `CREATIVE_TYPE_VIDEO`
    #[serde(rename="obaIcon")]
    
    pub oba_icon: Option<ObaIcon>,
    /// Output only. Indicates the third-party audio creative supports OGG. Output only and only valid for third-party audio creatives. Third-party audio creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_AUDIO`
    #[serde(rename="oggAudio")]
    
    pub ogg_audio: Option<bool>,
    /// Amount of time to play the video before counting a view. This field is required when skippable is true. This field is only supported for the following creative_type: * `CREATIVE_TYPE_VIDEO`
    #[serde(rename="progressOffset")]
    
    pub progress_offset: Option<AudioVideoOffset>,
    /// Optional. Indicates that the creative relies on HTML5 to render properly. Optional and only valid for third-party tag creatives. Third-party tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_EXPANDABLE`
    #[serde(rename="requireHtml5")]
    
    pub require_html5: Option<bool>,
    /// Optional. Indicates that the creative requires MRAID (Mobile Rich Media Ad Interface Definitions system). Set this if the creative relies on mobile gestures for interactivity, such as swiping or tapping. Optional and only valid for third-party tag creatives. Third-party tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_EXPANDABLE`
    #[serde(rename="requireMraid")]
    
    pub require_mraid: Option<bool>,
    /// Optional. Indicates that the creative will wait for a return ping for attribution. Only valid when using a Campaign Manager 360 tracking ad with a third-party ad server parameter and the ${DC_DBM_TOKEN} macro. Optional and only valid for third-party tag creatives or third-party VAST tag creatives. Third-party tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_EXPANDABLE` Third-party VAST tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_VIDEO`
    #[serde(rename="requirePingForAttribution")]
    
    pub require_ping_for_attribution: Option<bool>,
    /// Output only. The current status of the creative review process.
    #[serde(rename="reviewStatus")]
    
    pub review_status: Option<ReviewStatusInfo>,
    /// Amount of time to play the video before the skip button appears. This field is required when skippable is true. This field is only supported for the following creative_type: * `CREATIVE_TYPE_VIDEO`
    #[serde(rename="skipOffset")]
    
    pub skip_offset: Option<AudioVideoOffset>,
    /// Whether the user can choose to skip a video creative. This field is only supported for the following creative_type: * `CREATIVE_TYPE_VIDEO`
    
    pub skippable: Option<bool>,
    /// Optional. The original third-party tag used for the creative. Required and only valid for third-party tag creatives. Third-party tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_EXPANDABLE`
    #[serde(rename="thirdPartyTag")]
    
    pub third_party_tag: Option<String>,
    /// Tracking URLs from third parties to track interactions with a video creative. This field is only supported for the following creative_type: * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_VIDEO` * `CREATIVE_TYPE_NATIVE_VIDEO`
    #[serde(rename="thirdPartyUrls")]
    
    pub third_party_urls: Option<Vec<ThirdPartyUrl>>,
    /// Timer custom events for a rich media creative. Timers track the time during which a user views and interacts with a specified part of a rich media creative. A creative can have multiple timer events, each timed independently. Leave it empty or unset for creatives containing image assets only.
    #[serde(rename="timerEvents")]
    
    pub timer_events: Option<Vec<TimerEvent>>,
    /// Tracking URLs for analytics providers or third-party ad technology vendors. The URLs must start with https (except on inventory that doesn't require SSL compliance). If using macros in your URL, use only macros supported by Display & Video 360. Standard URLs only, no IMG or SCRIPT tags. This field is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`
    #[serde(rename="trackerUrls")]
    
    pub tracker_urls: Option<Vec<String>>,
    /// Output only. Audio/Video transcodes. Display & Video 360 transcodes the main asset into a number of alternative versions that use different file formats or have different properties (resolution, audio bit rate, and video bit rate), each designed for specific video players or bandwidths. These transcodes give a publisher's system more options to choose from for each impression on your video and ensures that the appropriate file serves based on the viewer’s connection and screen size. This field is only supported in following creative_type: * `CREATIVE_TYPE_VIDEO` * `CREATIVE_TYPE_NATIVE_VIDEO` * `CREATIVE_TYPE_AUDIO`
    
    pub transcodes: Option<Vec<Transcode>>,
    /// Optional. An optional creative identifier provided by a registry that is unique across all platforms. Universal Ad ID is part of the VAST 4.0 standard. It can be modified after the creative is created. This field is only supported for the following creative_type: * `CREATIVE_TYPE_VIDEO`
    #[serde(rename="universalAdId")]
    
    pub universal_ad_id: Option<UniversalAdId>,
    /// Output only. The timestamp when the creative was last updated, either by the user or system (e.g. creative review). Assigned by the system.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The URL of the VAST tag for a third-party VAST tag creative. Required and only valid for third-party VAST tag creatives. Third-party VAST tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_VIDEO`
    #[serde(rename="vastTagUrl")]
    
    pub vast_tag_url: Option<String>,
    /// Output only. Indicates the third-party VAST tag creative requires VPAID (Digital Video Player-Ad Interface). Output only and only valid for third-party VAST tag creatives. Third-party VAST tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_VIDEO`
    
    pub vpaid: Option<bool>,
}

impl client::RequestValue for Creative {}
impl client::ResponseResult for Creative {}


/// Creative requirements configuration for the inventory source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeConfig {
    /// The type of creative that can be assigned to the inventory source. Only the following types are supported: * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_VIDEO`
    #[serde(rename="creativeType")]
    
    pub creative_type: Option<String>,
    /// The configuration for display creatives. Applicable when creative_type is `CREATIVE_TYPE_STANDARD`.
    #[serde(rename="displayCreativeConfig")]
    
    pub display_creative_config: Option<InventorySourceDisplayCreativeConfig>,
    /// The configuration for video creatives. Applicable when creative_type is `CREATIVE_TYPE_VIDEO`.
    #[serde(rename="videoCreativeConfig")]
    
    pub video_creative_config: Option<InventorySourceVideoCreativeConfig>,
}

impl client::Part for CreativeConfig {}


/// A single custom bidding algorithm.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [scripts create custom bidding algorithms](CustomBiddingAlgorithmScriptCreateCall) (none)
/// * [scripts get custom bidding algorithms](CustomBiddingAlgorithmScriptGetCall) (none)
/// * [scripts list custom bidding algorithms](CustomBiddingAlgorithmScriptListCall) (none)
/// * [create custom bidding algorithms](CustomBiddingAlgorithmCreateCall) (request|response)
/// * [get custom bidding algorithms](CustomBiddingAlgorithmGetCall) (response)
/// * [list custom bidding algorithms](CustomBiddingAlgorithmListCall) (none)
/// * [patch custom bidding algorithms](CustomBiddingAlgorithmPatchCall) (request|response)
/// * [upload script custom bidding algorithms](CustomBiddingAlgorithmUploadScriptCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomBiddingAlgorithm {
    /// Immutable. The unique ID of the advertiser that owns the custom bidding algorithm.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Output only. The unique ID of the custom bidding algorithm. Assigned by the system.
    #[serde(rename="customBiddingAlgorithmId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub custom_bidding_algorithm_id: Option<i64>,
    /// Output only. The status of custom bidding algorithm.
    #[serde(rename="customBiddingAlgorithmState")]
    
    pub custom_bidding_algorithm_state: Option<String>,
    /// Required. Immutable. The type of custom bidding algorithm.
    #[serde(rename="customBiddingAlgorithmType")]
    
    pub custom_bidding_algorithm_type: Option<String>,
    /// Required. The display name of the custom bidding algorithm. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Controls whether or not the custom bidding algorithm can be used as a bidding strategy. Accepted values are: * `ENTITY_STATUS_ACTIVE` * `ENTITY_STATUS_ARCHIVED`
    #[serde(rename="entityStatus")]
    
    pub entity_status: Option<String>,
    /// Output only. The state of custom bidding model readiness for each advertiser who has access. This field may only include the state of the queried advertiser if the algorithm [`owner`](https://developers.google.com/display-video/api/reference/rest/v1/customBiddingAlgorithms#CustomBiddingAlgorithm.FIELDS.oneof_owner) is a partner and is being retrieved using an advertiser [`accessor`](https://developers.google.com/display-video/api/reference/rest/v1/customBiddingAlgorithms/list#body.QUERY_PARAMETERS.oneof_accessor).
    #[serde(rename="modelReadiness")]
    
    pub model_readiness: Option<Vec<CustomBiddingModelReadinessState>>,
    /// Output only. The resource name of the custom bidding algorithm.
    
    pub name: Option<String>,
    /// Immutable. The unique ID of the partner that owns the custom bidding algorithm.
    #[serde(rename="partnerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partner_id: Option<i64>,
    /// The IDs of the advertisers who have access to this algorithm. If advertiser_id is set, this field will only consist of that value. This field will not be set if the algorithm [`owner`](https://developers.google.com/display-video/api/reference/rest/v1/customBiddingAlgorithms#CustomBiddingAlgorithm.FIELDS.oneof_owner) is a partner and is being retrieved using an advertiser [`accessor`](https://developers.google.com/display-video/api/reference/rest/v1/customBiddingAlgorithms/list#body.QUERY_PARAMETERS.oneof_accessor).
    #[serde(rename="sharedAdvertiserIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub shared_advertiser_ids: Option<Vec<i64>>,
}

impl client::RequestValue for CustomBiddingAlgorithm {}
impl client::Resource for CustomBiddingAlgorithm {}
impl client::ResponseResult for CustomBiddingAlgorithm {}


/// The custom bidding algorithm model readiness state for a single shared advertiser.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomBiddingModelReadinessState {
    /// The unique ID of the relevant advertiser.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// The readiness state of custom bidding model.
    #[serde(rename="readinessState")]
    
    pub readiness_state: Option<String>,
}

impl client::Part for CustomBiddingModelReadinessState {}


/// A single custom bidding script.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [scripts create custom bidding algorithms](CustomBiddingAlgorithmScriptCreateCall) (request|response)
/// * [scripts get custom bidding algorithms](CustomBiddingAlgorithmScriptGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomBiddingScript {
    /// Output only. Whether the script is currently being used for scoring by the parent algorithm.
    
    pub active: Option<bool>,
    /// Output only. The time when the script was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The unique ID of the custom bidding algorithm the script belongs to.
    #[serde(rename="customBiddingAlgorithmId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub custom_bidding_algorithm_id: Option<i64>,
    /// Output only. The unique ID of the custom bidding script.
    #[serde(rename="customBiddingScriptId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub custom_bidding_script_id: Option<i64>,
    /// Output only. Error details of a rejected custom bidding script. This field will only be populated when Script.state is REJECTED.
    
    pub errors: Option<Vec<ScriptError>>,
    /// Output only. The resource name of the custom bidding script.
    
    pub name: Option<String>,
    /// The reference to the uploaded script file.
    
    pub script: Option<CustomBiddingScriptRef>,
    /// Output only. The state of the custom bidding script.
    
    pub state: Option<String>,
}

impl client::RequestValue for CustomBiddingScript {}
impl client::ResponseResult for CustomBiddingScript {}


/// The reference to the uploaded custom bidding script file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [upload script custom bidding algorithms](CustomBiddingAlgorithmUploadScriptCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomBiddingScriptRef {
    /// A resource name to be used in media.download to Download the script files. Or media.upload to Upload the script files. Resource names have the format `customBiddingAlgorithms/{custom_bidding_algorithm_id}/scriptRef/{ref_id}`.
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
}

impl client::ResponseResult for CustomBiddingScriptRef {}


/// Describes a custom list entity, such as a custom affinity or custom intent audience list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get custom lists](CustomListGetCall) (response)
/// * [list custom lists](CustomListListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomList {
    /// Output only. The unique ID of the custom list. Assigned by the system.
    #[serde(rename="customListId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub custom_list_id: Option<i64>,
    /// Output only. The display name of the custom list. .
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The resource name of the custom list.
    
    pub name: Option<String>,
}

impl client::Resource for CustomList {}
impl client::ResponseResult for CustomList {}


/// Details of custom list group. All custom list targeting settings are logically ‘OR’ of each other.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomListGroup {
    /// Required. All custom list targeting settings in custom list group. Repeated settings with same id will be ignored.
    
    pub settings: Option<Vec<CustomListTargetingSetting>>,
}

impl client::Part for CustomListGroup {}


/// Details of custom list targeting setting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomListTargetingSetting {
    /// Required. Custom id of custom list targeting setting. This id is custom_list_id.
    #[serde(rename="customListId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub custom_list_id: Option<i64>,
}

impl client::Part for CustomListTargetingSetting {}


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


/// A date range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateRange {
    /// The upper bound of the date range, inclusive. Must specify a positive value for `year`, `month`, and `day`.
    #[serde(rename="endDate")]
    
    pub end_date: Option<Date>,
    /// The lower bound of the date range, inclusive. Must specify a positive value for `year`, `month`, and `day`.
    #[serde(rename="startDate")]
    
    pub start_date: Option<Date>,
}

impl client::Part for DateRange {}


/// Representation of a segment of time defined on a specific day of the week and with a start and end time. The time represented by `start_hour` must be before the time represented by `end_hour`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DayAndTimeAssignedTargetingOptionDetails {
    /// Required. The day of the week for this day and time targeting setting.
    #[serde(rename="dayOfWeek")]
    
    pub day_of_week: Option<String>,
    /// Required. The end hour for day and time targeting. Must be between 1 (1 hour after start of day) and 24 (end of day).
    #[serde(rename="endHour")]
    
    pub end_hour: Option<i32>,
    /// Required. The start hour for day and time targeting. Must be between 0 (start of day) and 23 (1 hour before end of day).
    #[serde(rename="startHour")]
    
    pub start_hour: Option<i32>,
    /// Required. The mechanism used to determine which timezone to use for this day and time targeting setting.
    #[serde(rename="timeZoneResolution")]
    
    pub time_zone_resolution: Option<String>,
}

impl client::Part for DayAndTimeAssignedTargetingOptionDetails {}


/// Request message for ManualTriggerService.DeactivateManualTrigger.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [manual triggers deactivate advertisers](AdvertiserManualTriggerDeactivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeactivateManualTriggerRequest { _never_set: Option<bool> }

impl client::RequestValue for DeactivateManualTriggerRequest {}


/// A request listing which assigned targeting options of a given targeting type should be deleted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteAssignedTargetingOptionsRequest {
    /// Required. The assigned targeting option IDs to delete.
    #[serde(rename="assignedTargetingOptionIds")]
    
    pub assigned_targeting_option_ids: Option<Vec<String>>,
    /// Required. Identifies the type of this assigned targeting option.
    #[serde(rename="targetingType")]
    
    pub targeting_type: Option<String>,
}

impl client::Part for DeleteAssignedTargetingOptionsRequest {}


/// Assigned device make and model targeting option details. This will be populated in the device_make_model_details field when targeting_type is `TARGETING_TYPE_DEVICE_MAKE_MODEL`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceMakeModelAssignedTargetingOptionDetails {
    /// Output only. The display name of the device make and model.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Indicates if this option is being negatively targeted.
    
    pub negative: Option<bool>,
    /// Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_DEVICE_MAKE_MODEL`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for DeviceMakeModelAssignedTargetingOptionDetails {}


/// Represents a targetable device make and model. This will be populated in the device_make_model_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_DEVICE_MAKE_MODEL`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceMakeModelTargetingOptionDetails {
    /// Output only. The display name of the device make and model.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for DeviceMakeModelTargetingOptionDetails {}


/// Targeting details for device type. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_DEVICE_TYPE`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceTypeAssignedTargetingOptionDetails {
    /// The display name of the device type. Output only in v1. Required in v2.
    #[serde(rename="deviceType")]
    
    pub device_type: Option<String>,
    /// Required. ID of the device type.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for DeviceTypeAssignedTargetingOptionDetails {}


/// Represents a targetable device type. This will be populated in the device_type_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_DEVICE_TYPE`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceTypeTargetingOptionDetails {
    /// Output only. The device type that is used to be targeted.
    #[serde(rename="deviceType")]
    
    pub device_type: Option<String>,
}

impl client::Part for DeviceTypeTargetingOptionDetails {}


/// Targeting details for digital content label. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DigitalContentLabelAssignedTargetingOptionDetails {
    /// Output only. The display name of the digital content label rating tier.
    #[serde(rename="contentRatingTier")]
    
    pub content_rating_tier: Option<String>,
    /// Required. ID of the digital content label to be EXCLUDED.
    #[serde(rename="excludedTargetingOptionId")]
    
    pub excluded_targeting_option_id: Option<String>,
}

impl client::Part for DigitalContentLabelAssignedTargetingOptionDetails {}


/// Represents a targetable digital content label rating tier. This will be populated in the digital_content_label_details field of the TargetingOption when targeting_type is `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DigitalContentLabelTargetingOptionDetails {
    /// Output only. An enum for the content label brand safety tiers.
    #[serde(rename="contentRatingTier")]
    
    pub content_rating_tier: Option<String>,
}

impl client::Part for DigitalContentLabelTargetingOptionDetails {}


/// Dimensions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dimensions {
    /// The height in pixels.
    #[serde(rename="heightPixels")]
    
    pub height_pixels: Option<i32>,
    /// The width in pixels.
    #[serde(rename="widthPixels")]
    
    pub width_pixels: Option<i32>,
}

impl client::Part for Dimensions {}


/// Details of DoubleVerify settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DoubleVerify {
    /// Avoid bidding on apps with the star ratings.
    #[serde(rename="appStarRating")]
    
    pub app_star_rating: Option<DoubleVerifyAppStarRating>,
    /// Avoid bidding on apps with the age rating.
    #[serde(rename="avoidedAgeRatings")]
    
    pub avoided_age_ratings: Option<Vec<String>>,
    /// DV Brand Safety Controls.
    #[serde(rename="brandSafetyCategories")]
    
    pub brand_safety_categories: Option<DoubleVerifyBrandSafetyCategories>,
    /// The custom segment ID provided by DoubleVerify. The ID must start with "51" and consist of eight digits. Custom segment ID cannot be specified along with any of the following fields: * brand_safety_categories * avoided_age_ratings * app_star_rating * fraud_invalid_traffic
    #[serde(rename="customSegmentId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub custom_segment_id: Option<i64>,
    /// Display viewability settings (applicable to display line items only).
    #[serde(rename="displayViewability")]
    
    pub display_viewability: Option<DoubleVerifyDisplayViewability>,
    /// Avoid Sites and Apps with historical Fraud & IVT Rates.
    #[serde(rename="fraudInvalidTraffic")]
    
    pub fraud_invalid_traffic: Option<DoubleVerifyFraudInvalidTraffic>,
    /// Video viewability settings (applicable to video line items only).
    #[serde(rename="videoViewability")]
    
    pub video_viewability: Option<DoubleVerifyVideoViewability>,
}

impl client::Part for DoubleVerify {}


/// Details of DoubleVerify star ratings settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DoubleVerifyAppStarRating {
    /// Avoid bidding on apps with insufficient star ratings.
    #[serde(rename="avoidInsufficientStarRating")]
    
    pub avoid_insufficient_star_rating: Option<bool>,
    /// Avoid bidding on apps with the star ratings.
    #[serde(rename="avoidedStarRating")]
    
    pub avoided_star_rating: Option<String>,
}

impl client::Part for DoubleVerifyAppStarRating {}


/// Settings for brand safety controls.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DoubleVerifyBrandSafetyCategories {
    /// Unknown or unrateable.
    #[serde(rename="avoidUnknownBrandSafetyCategory")]
    
    pub avoid_unknown_brand_safety_category: Option<bool>,
    /// Brand safety high severity avoidance categories.
    #[serde(rename="avoidedHighSeverityCategories")]
    
    pub avoided_high_severity_categories: Option<Vec<String>>,
    /// Brand safety medium severity avoidance categories.
    #[serde(rename="avoidedMediumSeverityCategories")]
    
    pub avoided_medium_severity_categories: Option<Vec<String>>,
}

impl client::Part for DoubleVerifyBrandSafetyCategories {}


/// Details of DoubleVerify display viewability settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DoubleVerifyDisplayViewability {
    /// Target web and app inventory to maximize IAB viewable rate.
    
    pub iab: Option<String>,
    /// Target web and app inventory to maximize 100% viewable duration.
    #[serde(rename="viewableDuring")]
    
    pub viewable_during: Option<String>,
}

impl client::Part for DoubleVerifyDisplayViewability {}


/// DoubleVerify Fraud & Invalid Traffic settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DoubleVerifyFraudInvalidTraffic {
    /// Insufficient Historical Fraud & IVT Stats.
    #[serde(rename="avoidInsufficientOption")]
    
    pub avoid_insufficient_option: Option<bool>,
    /// Avoid Sites and Apps with historical Fraud & IVT.
    #[serde(rename="avoidedFraudOption")]
    
    pub avoided_fraud_option: Option<String>,
}

impl client::Part for DoubleVerifyFraudInvalidTraffic {}


/// Details of DoubleVerify video viewability settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DoubleVerifyVideoViewability {
    /// Target inventory to maximize impressions with 400x300 or greater player size.
    #[serde(rename="playerImpressionRate")]
    
    pub player_impression_rate: Option<String>,
    /// Target web inventory to maximize IAB viewable rate.
    #[serde(rename="videoIab")]
    
    pub video_iab: Option<String>,
    /// Target web inventory to maximize fully viewable rate.
    #[serde(rename="videoViewableRate")]
    
    pub video_viewable_rate: Option<String>,
}

impl client::Part for DoubleVerifyVideoViewability {}


/// Request message for FirstAndThirdPartyAudienceService.EditCustomerMatchMembers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [edit customer match members first and third party audiences](FirstAndThirdPartyAudienceEditCustomerMatchMemberCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EditCustomerMatchMembersRequest {
    /// Input only. A list of contact information to define the members to be added.
    #[serde(rename="addedContactInfoList")]
    
    pub added_contact_info_list: Option<ContactInfoList>,
    /// Input only. A list of mobile device IDs to define the members to be added.
    #[serde(rename="addedMobileDeviceIdList")]
    
    pub added_mobile_device_id_list: Option<MobileDeviceIdList>,
    /// Required. The ID of the owner advertiser of the updated Customer Match FirstAndThirdPartyAudience.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
}

impl client::RequestValue for EditCustomerMatchMembersRequest {}


/// The response of FirstAndThirdPartyAudienceService.EditCustomerMatchMembers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [edit customer match members first and third party audiences](FirstAndThirdPartyAudienceEditCustomerMatchMemberCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EditCustomerMatchMembersResponse {
    /// Required. The ID of the updated Customer Match FirstAndThirdPartyAudience.
    #[serde(rename="firstAndThirdPartyAudienceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub first_and_third_party_audience_id: Option<i64>,
}

impl client::ResponseResult for EditCustomerMatchMembersResponse {}


/// Request message for GuaranteedOrderService.EditGuaranteedOrderReadAccessors.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [edit guaranteed order read accessors guaranteed orders](GuaranteedOrderEditGuaranteedOrderReadAccessorCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EditGuaranteedOrderReadAccessorsRequest {
    /// The advertisers to add as read accessors to the guaranteed order.
    #[serde(rename="addedAdvertisers")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub added_advertisers: Option<Vec<i64>>,
    /// Required. The partner context in which the change is being made.
    #[serde(rename="partnerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partner_id: Option<i64>,
    /// Whether to give all advertisers of the read/write accessor partner read access to the guaranteed order. Only applicable if read_write_partner_id is set in the guaranteed order.
    #[serde(rename="readAccessInherited")]
    
    pub read_access_inherited: Option<bool>,
    /// The advertisers to remove as read accessors to the guaranteed order.
    #[serde(rename="removedAdvertisers")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub removed_advertisers: Option<Vec<i64>>,
}

impl client::RequestValue for EditGuaranteedOrderReadAccessorsRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [edit guaranteed order read accessors guaranteed orders](GuaranteedOrderEditGuaranteedOrderReadAccessorCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EditGuaranteedOrderReadAccessorsResponse {
    /// Whether all advertisers of read_write_partner_id have read access to the guaranteed order.
    #[serde(rename="readAccessInherited")]
    
    pub read_access_inherited: Option<bool>,
    /// The IDs of advertisers with read access to the guaranteed order.
    #[serde(rename="readAdvertiserIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub read_advertiser_ids: Option<Vec<i64>>,
}

impl client::ResponseResult for EditGuaranteedOrderReadAccessorsResponse {}


/// Request message for InventorySourceService.EditInventorySourceReadWriteAccessors.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [edit inventory source read write accessors inventory sources](InventorySourceEditInventorySourceReadWriteAccessorCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EditInventorySourceReadWriteAccessorsRequest {
    /// The advertisers to add or remove from the list of advertisers that have read/write access to the inventory source. This change will remove an existing partner read/write accessor.
    #[serde(rename="advertisersUpdate")]
    
    pub advertisers_update: Option<EditInventorySourceReadWriteAccessorsRequestAdvertisersUpdate>,
    /// Set the partner context as read/write accessor of the inventory source. This will remove all other current read/write advertiser accessors.
    #[serde(rename="assignPartner")]
    
    pub assign_partner: Option<bool>,
    /// Required. The partner context by which the accessors change is being made.
    #[serde(rename="partnerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partner_id: Option<i64>,
}

impl client::RequestValue for EditInventorySourceReadWriteAccessorsRequest {}


/// Update to the list of advertisers with read/write access to the inventory source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EditInventorySourceReadWriteAccessorsRequestAdvertisersUpdate {
    /// The advertisers to add.
    #[serde(rename="addedAdvertisers")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub added_advertisers: Option<Vec<i64>>,
    /// The advertisers to remove.
    #[serde(rename="removedAdvertisers")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub removed_advertisers: Option<Vec<i64>>,
}

impl client::Part for EditInventorySourceReadWriteAccessorsRequestAdvertisersUpdate {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [campaigns delete advertisers](AdvertiserCampaignDeleteCall) (response)
/// * [channels sites delete advertisers](AdvertiserChannelSiteDeleteCall) (response)
/// * [creatives delete advertisers](AdvertiserCreativeDeleteCall) (response)
/// * [insertion orders delete advertisers](AdvertiserInsertionOrderDeleteCall) (response)
/// * [line items targeting types assigned targeting options delete advertisers](AdvertiserLineItemTargetingTypeAssignedTargetingOptionDeleteCall) (response)
/// * [line items delete advertisers](AdvertiserLineItemDeleteCall) (response)
/// * [location lists assigned locations delete advertisers](AdvertiserLocationListAssignedLocationDeleteCall) (response)
/// * [negative keyword lists negative keywords delete advertisers](AdvertiserNegativeKeywordListNegativeKeywordDeleteCall) (response)
/// * [negative keyword lists delete advertisers](AdvertiserNegativeKeywordListDeleteCall) (response)
/// * [targeting types assigned targeting options delete advertisers](AdvertiserTargetingTypeAssignedTargetingOptionDeleteCall) (response)
/// * [delete advertisers](AdvertiserDeleteCall) (response)
/// * [assigned inventory sources delete inventory source groups](InventorySourceGroupAssignedInventorySourceDeleteCall) (response)
/// * [delete inventory source groups](InventorySourceGroupDeleteCall) (response)
/// * [channels sites delete partners](PartnerChannelSiteDeleteCall) (response)
/// * [targeting types assigned targeting options delete partners](PartnerTargetingTypeAssignedTargetingOptionDeleteCall) (response)
/// * [delete users](UserDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Assigned environment targeting option details. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_ENVIRONMENT`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentAssignedTargetingOptionDetails {
    /// The serving environment. Output only in v1. Required in v2.
    
    pub environment: Option<String>,
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_ENVIRONMENT` (e.g., "508010" for targeting the `ENVIRONMENT_WEB_OPTIMIZED` option).
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for EnvironmentAssignedTargetingOptionDetails {}


/// Represents a targetable environment. This will be populated in the environment_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_ENVIRONMENT`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentTargetingOptionDetails {
    /// Output only. The serving environment.
    
    pub environment: Option<String>,
}

impl client::Part for EnvironmentTargetingOptionDetails {}


/// Details for assigned exchange targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_EXCHANGE`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExchangeAssignedTargetingOptionDetails {
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_EXCHANGE`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for ExchangeAssignedTargetingOptionDetails {}


/// Settings that control which exchanges are enabled for a partner.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExchangeConfig {
    /// All enabled exchanges in the partner. Duplicate enabled exchanges will be ignored.
    #[serde(rename="enabledExchanges")]
    
    pub enabled_exchanges: Option<Vec<ExchangeConfigEnabledExchange>>,
}

impl client::Part for ExchangeConfig {}


/// An enabled exchange in the partner.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExchangeConfigEnabledExchange {
    /// The enabled exchange.
    
    pub exchange: Option<String>,
    /// Output only. Agency ID of Google Ad Manager. The field is only relevant when Google Ad Manager is the enabled exchange.
    #[serde(rename="googleAdManagerAgencyId")]
    
    pub google_ad_manager_agency_id: Option<String>,
    /// Output only. Network ID of Google Ad Manager. The field is only relevant when Google Ad Manager is the enabled exchange.
    #[serde(rename="googleAdManagerBuyerNetworkId")]
    
    pub google_ad_manager_buyer_network_id: Option<String>,
    /// Output only. Seat ID of the enabled exchange.
    #[serde(rename="seatId")]
    
    pub seat_id: Option<String>,
}

impl client::Part for ExchangeConfigEnabledExchange {}


/// Exchange review status for the creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExchangeReviewStatus {
    /// The exchange reviewing the creative.
    
    pub exchange: Option<String>,
    /// Status of the exchange review.
    
    pub status: Option<String>,
}

impl client::Part for ExchangeReviewStatus {}


/// Represents a targetable exchange. This will be populated in the exchange_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_EXCHANGE`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExchangeTargetingOptionDetails {
    /// Output only. The type of exchange.
    
    pub exchange: Option<String>,
}

impl client::Part for ExchangeTargetingOptionDetails {}


/// Exit event of the creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExitEvent {
    /// The name of the click tag of the exit event. The name must be unique within one creative. Leave it empty or unset for creatives containing image assets only.
    
    pub name: Option<String>,
    /// The name used to identify this event in reports. Leave it empty or unset for creatives containing image assets only.
    #[serde(rename="reportingName")]
    
    pub reporting_name: Option<String>,
    /// Required. The type of the exit event.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Required. The click through URL of the exit event. This is required when type is: * `EXIT_EVENT_TYPE_DEFAULT` * `EXIT_EVENT_TYPE_BACKUP`
    
    pub url: Option<String>,
}

impl client::Part for ExitEvent {}


/// Describes a first or third party audience list used for targeting. First party audiences are created via usage of client data. Third party audiences are provided by Third Party data providers and can only be licensed to customers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create first and third party audiences](FirstAndThirdPartyAudienceCreateCall) (request|response)
/// * [edit customer match members first and third party audiences](FirstAndThirdPartyAudienceEditCustomerMatchMemberCall) (none)
/// * [get first and third party audiences](FirstAndThirdPartyAudienceGetCall) (response)
/// * [list first and third party audiences](FirstAndThirdPartyAudienceListCall) (none)
/// * [patch first and third party audiences](FirstAndThirdPartyAudiencePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirstAndThirdPartyAudience {
    /// Output only. The estimated audience size for the Display network in the past month. If the size is less than 1000, the number will be hidden and 0 will be returned due to privacy reasons. Otherwise, the number will be rounded off to two significant digits. Only returned in GET request.
    #[serde(rename="activeDisplayAudienceSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub active_display_audience_size: Option<i64>,
    /// The app_id matches with the type of the mobile_device_ids being uploaded. Only applicable to audience_type `CUSTOMER_MATCH_DEVICE_ID`
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// Output only. The source of the audience.
    #[serde(rename="audienceSource")]
    
    pub audience_source: Option<String>,
    /// The type of the audience.
    #[serde(rename="audienceType")]
    
    pub audience_type: Option<String>,
    /// Input only. A list of contact information to define the initial audience members. Only applicable to audience_type `CUSTOMER_MATCH_CONTACT_INFO`
    #[serde(rename="contactInfoList")]
    
    pub contact_info_list: Option<ContactInfoList>,
    /// The user-provided description of the audience. Only applicable to first party audiences.
    
    pub description: Option<String>,
    /// Output only. The estimated audience size for the Display network. If the size is less than 1000, the number will be hidden and 0 will be returned due to privacy reasons. Otherwise, the number will be rounded off to two significant digits. Only returned in GET request.
    #[serde(rename="displayAudienceSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub display_audience_size: Option<i64>,
    /// Output only. The estimated desktop audience size in Display network. If the size is less than 1000, the number will be hidden and 0 will be returned due to privacy reasons. Otherwise, the number will be rounded off to two significant digits. Only applicable to first party audiences. Only returned in GET request.
    #[serde(rename="displayDesktopAudienceSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub display_desktop_audience_size: Option<i64>,
    /// Output only. The estimated mobile app audience size in Display network. If the size is less than 1000, the number will be hidden and 0 will be returned due to privacy reasons. Otherwise, the number will be rounded off to two significant digits. Only applicable to first party audiences. Only returned in GET request.
    #[serde(rename="displayMobileAppAudienceSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub display_mobile_app_audience_size: Option<i64>,
    /// Output only. The estimated mobile web audience size in Display network. If the size is less than 1000, the number will be hidden and 0 will be returned due to privacy reasons. Otherwise, the number will be rounded off to two significant digits. Only applicable to first party audiences. Only returned in GET request.
    #[serde(rename="displayMobileWebAudienceSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub display_mobile_web_audience_size: Option<i64>,
    /// The display name of the first and third party audience.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The unique ID of the first and third party audience. Assigned by the system.
    #[serde(rename="firstAndThirdPartyAudienceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub first_and_third_party_audience_id: Option<i64>,
    /// Whether the audience is a first or third party audience.
    #[serde(rename="firstAndThirdPartyAudienceType")]
    
    pub first_and_third_party_audience_type: Option<String>,
    /// Output only. The estimated audience size for Gmail network. If the size is less than 1000, the number will be hidden and 0 will be returned due to privacy reasons. Otherwise, the number will be rounded off to two significant digits. Only applicable to first party audiences. Only returned in GET request.
    #[serde(rename="gmailAudienceSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub gmail_audience_size: Option<i64>,
    /// The duration in days that an entry remains in the audience after the qualifying event. If the audience has no expiration, set the value of this field to 10000. Otherwise, the set value must be greater than 0 and less than or equal to 540. Only applicable to first party audiences. This field is required if one of the following audience_type is used: * `CUSTOMER_MATCH_CONTACT_INFO` * `CUSTOMER_MATCH_DEVICE_ID`
    #[serde(rename="membershipDurationDays")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub membership_duration_days: Option<i64>,
    /// Input only. A list of mobile device IDs to define the initial audience members. Only applicable to audience_type `CUSTOMER_MATCH_DEVICE_ID`
    #[serde(rename="mobileDeviceIdList")]
    
    pub mobile_device_id_list: Option<MobileDeviceIdList>,
    /// Output only. The resource name of the first and third party audience.
    
    pub name: Option<String>,
    /// Output only. The estimated audience size for YouTube network. If the size is less than 1000, the number will be hidden and 0 will be returned due to privacy reasons. Otherwise, the number will be rounded off to two significant digits. Only applicable to first party audiences. Only returned in GET request.
    #[serde(rename="youtubeAudienceSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub youtube_audience_size: Option<i64>,
}

impl client::RequestValue for FirstAndThirdPartyAudience {}
impl client::Resource for FirstAndThirdPartyAudience {}
impl client::ResponseResult for FirstAndThirdPartyAudience {}


/// Details of first and third party audience group. All first and third party audience targeting settings are logically ‘OR’ of each other.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirstAndThirdPartyAudienceGroup {
    /// Required. All first and third party audience targeting settings in first and third party audience group. Repeated settings with same id are not allowed.
    
    pub settings: Option<Vec<FirstAndThirdPartyAudienceTargetingSetting>>,
}

impl client::Part for FirstAndThirdPartyAudienceGroup {}


/// Details of first and third party audience targeting setting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirstAndThirdPartyAudienceTargetingSetting {
    /// Required. First and third party audience id of the first and third party audience targeting setting. This id is first_and_third_party_audience_id.
    #[serde(rename="firstAndThirdPartyAudienceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub first_and_third_party_audience_id: Option<i64>,
    /// The recency of the first and third party audience targeting setting. Only applicable to first party audiences, otherwise will be ignored. For more info, refer to https://support.google.com/displayvideo/answer/2949947#recency When unspecified, no recency limit will be used.
    
    pub recency: Option<String>,
}

impl client::Part for FirstAndThirdPartyAudienceTargetingSetting {}


/// A strategy that uses a fixed bidding price.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FixedBidStrategy {
    /// The fixed bid amount, in micros of the advertiser's currency. For insertion order entity, bid_amount_micros should be set as 0. For line item entity, bid_amount_micros must be greater than or equal to billable unit of the given currency and smaller than or equal to the upper limit 1000000000. For example, 1500000 represents 1.5 standard units of the currency.
    #[serde(rename="bidAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bid_amount_micros: Option<i64>,
}

impl client::Part for FixedBidStrategy {}


/// A single Floodlight group.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get floodlight groups](FloodlightGroupGetCall) (response)
/// * [patch floodlight groups](FloodlightGroupPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FloodlightGroup {
    /// The Active View video viewability metric configuration for the Floodlight group.
    #[serde(rename="activeViewConfig")]
    
    pub active_view_config: Option<ActiveViewVideoViewabilityMetricConfig>,
    /// User-defined custom variables owned by the Floodlight group. Use custom Floodlight variables to create reporting data that is tailored to your unique business needs. Custom Floodlight variables use the keys `U1=`, `U2=`, and so on, and can take any values that you choose to pass to them. You can use them to track virtually any type of data that you collect about your customers, such as the genre of movie that a customer purchases, the country to which the item is shipped, and so on. Custom Floodlight variables may not be used to pass any data that could be used or recognized as personally identifiable information (PII). Example: `custom_variables { fields { "U1": value { number_value: 123.4 }, "U2": value { string_value: "MyVariable2" }, "U3": value { string_value: "MyVariable3" } } }` Acceptable values for keys are "U1" through "U100", inclusive. String values must be less than 64 characters long, and cannot contain the following characters: `"<>`.
    #[serde(rename="customVariables")]
    
    pub custom_variables: Option<HashMap<String, json::Value>>,
    /// Required. The display name of the Floodlight group.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The unique ID of the Floodlight group. Assigned by the system.
    #[serde(rename="floodlightGroupId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub floodlight_group_id: Option<i64>,
    /// Required. The lookback window for the Floodlight group. Both click_days and impression_days are required. Acceptable values for both are `0` to `90`, inclusive.
    #[serde(rename="lookbackWindow")]
    
    pub lookback_window: Option<LookbackWindow>,
    /// Output only. The resource name of the Floodlight group.
    
    pub name: Option<String>,
    /// Required. The web tag type enabled for the Floodlight group.
    #[serde(rename="webTagType")]
    
    pub web_tag_type: Option<String>,
}

impl client::RequestValue for FloodlightGroup {}
impl client::Resource for FloodlightGroup {}
impl client::ResponseResult for FloodlightGroup {}


/// Settings that control the number of times a user may be shown with the same ad during a given time period.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FrequencyCap {
    /// The maximum number of times a user may be shown the same ad during this period. Must be greater than 0. Required when unlimited is `false` and max_views is not set.
    #[serde(rename="maxImpressions")]
    
    pub max_impressions: Option<i32>,
    /// The time unit in which the frequency cap will be applied. Required when unlimited is `false`.
    #[serde(rename="timeUnit")]
    
    pub time_unit: Option<String>,
    /// The number of time_unit the frequency cap will last. Required when unlimited is `false`. The following restrictions apply based on the value of time_unit: * `TIME_UNIT_LIFETIME` - this field is output only and will default to 1 * `TIME_UNIT_MONTHS` - must be between 1 and 2 * `TIME_UNIT_WEEKS` - must be between 1 and 4 * `TIME_UNIT_DAYS` - must be between 1 and 6 * `TIME_UNIT_HOURS` - must be between 1 and 23 * `TIME_UNIT_MINUTES` - must be between 1 and 59
    #[serde(rename="timeUnitCount")]
    
    pub time_unit_count: Option<i32>,
    /// Whether unlimited frequency capping is applied. When this field is set to `true`, the remaining frequency cap fields are not applicable.
    
    pub unlimited: Option<bool>,
}

impl client::Part for FrequencyCap {}


/// Details for assigned gender targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_GENDER`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenderAssignedTargetingOptionDetails {
    /// The gender of the audience. Output only in v1. Required in v2.
    
    pub gender: Option<String>,
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_GENDER`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for GenderAssignedTargetingOptionDetails {}


/// Represents a targetable gender. This will be populated in the gender_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_GENDER`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenderTargetingOptionDetails {
    /// Output only. The gender of an audience.
    
    pub gender: Option<String>,
}

impl client::Part for GenderTargetingOptionDetails {}


/// Request message for LineItemService.GenerateDefaultLineItem.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [line items generate default advertisers](AdvertiserLineItemGenerateDefaultCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateDefaultLineItemRequest {
    /// Required. The display name of the line item. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. The unique ID of the insertion order that the line item belongs to.
    #[serde(rename="insertionOrderId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub insertion_order_id: Option<i64>,
    /// Required. The type of the line item.
    #[serde(rename="lineItemType")]
    
    pub line_item_type: Option<String>,
    /// The mobile app promoted by the line item. This is applicable only when line_item_type is either `LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INSTALL` or `LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INSTALL`.
    #[serde(rename="mobileApp")]
    
    pub mobile_app: Option<MobileApp>,
}

impl client::RequestValue for GenerateDefaultLineItemRequest {}


/// Details for assigned geographic region targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_GEO_REGION`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoRegionAssignedTargetingOptionDetails {
    /// Output only. The display name of the geographic region (e.g., "Ontario, Canada").
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The type of geographic region targeting.
    #[serde(rename="geoRegionType")]
    
    pub geo_region_type: Option<String>,
    /// Indicates if this option is being negatively targeted.
    
    pub negative: Option<bool>,
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_GEO_REGION`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for GeoRegionAssignedTargetingOptionDetails {}


/// Search terms for geo region targeting options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoRegionSearchTerms {
    /// The search query for the desired geo region. The query can be a prefix, e.g. "New Yor", "Seattle", "USA", etc.
    #[serde(rename="geoRegionQuery")]
    
    pub geo_region_query: Option<String>,
}

impl client::Part for GeoRegionSearchTerms {}


/// Represents a targetable geographic region. This will be populated in the geo_region_details field when targeting_type is `TARGETING_TYPE_GEO_REGION`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoRegionTargetingOptionDetails {
    /// Output only. The display name of the geographic region (e.g., "Ontario, Canada").
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The type of geographic region targeting.
    #[serde(rename="geoRegionType")]
    
    pub geo_region_type: Option<String>,
}

impl client::Part for GeoRegionTargetingOptionDetails {}


/// Describes a Google audience resource. Includes Google audience lists.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get google audiences](GoogleAudienceGetCall) (response)
/// * [list google audiences](GoogleAudienceListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAudience {
    /// Output only. The display name of the Google audience. .
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The unique ID of the Google audience. Assigned by the system.
    #[serde(rename="googleAudienceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub google_audience_id: Option<i64>,
    /// Output only. The type of Google audience. .
    #[serde(rename="googleAudienceType")]
    
    pub google_audience_type: Option<String>,
    /// Output only. The resource name of the google audience.
    
    pub name: Option<String>,
}

impl client::Resource for GoogleAudience {}
impl client::ResponseResult for GoogleAudience {}


/// Details of Google audience group. All Google audience targeting settings are logically ‘OR’ of each other.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAudienceGroup {
    /// Required. All Google audience targeting settings in Google audience group. Repeated settings with same id will be ignored.
    
    pub settings: Option<Vec<GoogleAudienceTargetingSetting>>,
}

impl client::Part for GoogleAudienceGroup {}


/// Details of Google audience targeting setting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAudienceTargetingSetting {
    /// Required. Google audience id of the Google audience targeting setting. This id is google_audience_id.
    #[serde(rename="googleAudienceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub google_audience_id: Option<i64>,
}

impl client::Part for GoogleAudienceTargetingSetting {}


/// Media resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [download media](MediaDownloadCall) (response)
/// * [upload media](MediaUploadCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleBytestreamMedia {
    /// Name of the media resource.
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
}

impl client::RequestValue for GoogleBytestreamMedia {}
impl client::ResponseResult for GoogleBytestreamMedia {}


/// A guaranteed order. Guaranteed orders are parent entity of guaranteed inventory sources. When creating a guaranteed inventory source, a guaranteed order ID must be assigned to the inventory source.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create guaranteed orders](GuaranteedOrderCreateCall) (request|response)
/// * [edit guaranteed order read accessors guaranteed orders](GuaranteedOrderEditGuaranteedOrderReadAccessorCall) (none)
/// * [get guaranteed orders](GuaranteedOrderGetCall) (response)
/// * [list guaranteed orders](GuaranteedOrderListCall) (none)
/// * [patch guaranteed orders](GuaranteedOrderPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GuaranteedOrder {
    /// Output only. The ID of default advertiser of the guaranteed order. The default advertiser is either the read_write_advertiser_id or, if that is not set, the first advertiser listed in read_advertiser_ids. Otherwise, there is no default advertiser.
    #[serde(rename="defaultAdvertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub default_advertiser_id: Option<i64>,
    /// The ID of the default campaign that is assigned to the guaranteed order. The default campaign must belong to the default advertiser.
    #[serde(rename="defaultCampaignId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub default_campaign_id: Option<i64>,
    /// Required. The display name of the guaranteed order. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. Immutable. The exchange where the guaranteed order originated.
    
    pub exchange: Option<String>,
    /// Output only. The unique identifier of the guaranteed order. The guaranteed order IDs have the format `{exchange}-{legacy_guaranteed_order_id}`.
    #[serde(rename="guaranteedOrderId")]
    
    pub guaranteed_order_id: Option<String>,
    /// Output only. The legacy ID of the guaranteed order. Assigned by the original exchange. The legacy ID is unique within one exchange, but is not guaranteed to be unique across all guaranteed orders. This ID is used in SDF and UI.
    #[serde(rename="legacyGuaranteedOrderId")]
    
    pub legacy_guaranteed_order_id: Option<String>,
    /// Output only. The resource name of the guaranteed order.
    
    pub name: Option<String>,
    /// Required. The publisher name of the guaranteed order. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="publisherName")]
    
    pub publisher_name: Option<String>,
    /// Whether all advertisers of read_write_partner_id have read access to the guaranteed order. Only applicable if read_write_partner_id is set. If True, overrides read_advertiser_ids.
    #[serde(rename="readAccessInherited")]
    
    pub read_access_inherited: Option<bool>,
    /// The IDs of advertisers with read access to the guaranteed order. This field must not include the advertiser assigned to read_write_advertiser_id if it is set. All advertisers in this field must belong to read_write_partner_id or the same partner as read_write_advertiser_id.
    #[serde(rename="readAdvertiserIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub read_advertiser_ids: Option<Vec<i64>>,
    /// The advertiser with read/write access to the guaranteed order. This is also the default advertiser of the guaranteed order.
    #[serde(rename="readWriteAdvertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub read_write_advertiser_id: Option<i64>,
    /// The partner with read/write access to the guaranteed order.
    #[serde(rename="readWritePartnerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub read_write_partner_id: Option<i64>,
    /// The status settings of the guaranteed order.
    
    pub status: Option<GuaranteedOrderStatus>,
    /// Output only. The timestamp when the guaranteed order was last updated. Assigned by the system.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GuaranteedOrder {}
impl client::Resource for GuaranteedOrder {}
impl client::ResponseResult for GuaranteedOrder {}


/// The status settings of the guaranteed order.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GuaranteedOrderStatus {
    /// Output only. The configuration status of the guaranteed order. Acceptable values are `PENDING` and `COMPLETED`. A guaranteed order must be configured (fill in the required fields, choose creatives, and select a default campaign) before it can serve. Currently the configuration action can only be performed via UI.
    #[serde(rename="configStatus")]
    
    pub config_status: Option<String>,
    /// The user-provided reason for pausing this guaranteed order. Must be UTF-8 encoded with a maximum length of 100 bytes. Only applicable when entity_status is set to `ENTITY_STATUS_PAUSED`.
    #[serde(rename="entityPauseReason")]
    
    pub entity_pause_reason: Option<String>,
    /// Whether or not the guaranteed order is servable. Acceptable values are `ENTITY_STATUS_ACTIVE`, `ENTITY_STATUS_ARCHIVED`, and `ENTITY_STATUS_PAUSED`. Default value is `ENTITY_STATUS_ACTIVE`.
    #[serde(rename="entityStatus")]
    
    pub entity_status: Option<String>,
}

impl client::Part for GuaranteedOrderStatus {}


/// Details for assigned household income targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_HOUSEHOLD_INCOME`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HouseholdIncomeAssignedTargetingOptionDetails {
    /// The household income of the audience. Output only in v1. Required in v2.
    #[serde(rename="householdIncome")]
    
    pub household_income: Option<String>,
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_HOUSEHOLD_INCOME`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for HouseholdIncomeAssignedTargetingOptionDetails {}


/// Represents a targetable household income. This will be populated in the household_income_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_HOUSEHOLD_INCOME`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HouseholdIncomeTargetingOptionDetails {
    /// Output only. The household income of an audience.
    #[serde(rename="householdIncome")]
    
    pub household_income: Option<String>,
}

impl client::Part for HouseholdIncomeTargetingOptionDetails {}


/// A filtering option that filters entities by their entity IDs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdFilter {
    /// YouTube Ads to download by ID. All IDs must belong to the same Advertiser or Partner specified in CreateSdfDownloadTaskRequest.
    #[serde(rename="adGroupAdIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub ad_group_ad_ids: Option<Vec<i64>>,
    /// YouTube Ad Groups to download by ID. All IDs must belong to the same Advertiser or Partner specified in CreateSdfDownloadTaskRequest.
    #[serde(rename="adGroupIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub ad_group_ids: Option<Vec<i64>>,
    /// Campaigns to download by ID. All IDs must belong to the same Advertiser or Partner specified in CreateSdfDownloadTaskRequest.
    #[serde(rename="campaignIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub campaign_ids: Option<Vec<i64>>,
    /// Insertion Orders to download by ID. All IDs must belong to the same Advertiser or Partner specified in CreateSdfDownloadTaskRequest.
    #[serde(rename="insertionOrderIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub insertion_order_ids: Option<Vec<i64>>,
    /// Line Items to download by ID. All IDs must belong to the same Advertiser or Partner specified in CreateSdfDownloadTaskRequest.
    #[serde(rename="lineItemIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub line_item_ids: Option<Vec<i64>>,
    /// Media Products to download by ID. All IDs must belong to the same Advertiser or Partner specified in CreateSdfDownloadTaskRequest.
    #[serde(rename="mediaProductIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub media_product_ids: Option<Vec<i64>>,
}

impl client::Part for IdFilter {}


/// A single insertion order.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insertion orders create advertisers](AdvertiserInsertionOrderCreateCall) (request|response)
/// * [insertion orders get advertisers](AdvertiserInsertionOrderGetCall) (response)
/// * [insertion orders patch advertisers](AdvertiserInsertionOrderPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertionOrder {
    /// Output only. The unique ID of the advertiser the insertion order belongs to.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// The bidding strategy of the insertion order. By default, fixed_bid is set.
    #[serde(rename="bidStrategy")]
    
    pub bid_strategy: Option<BiddingStrategy>,
    /// Immutable. The billable outcome of the insertion order.
    #[serde(rename="billableOutcome")]
    
    pub billable_outcome: Option<String>,
    /// Required. The budget allocation settings of the insertion order.
    
    pub budget: Option<InsertionOrderBudget>,
    /// Required. Immutable. The unique ID of the campaign that the insertion order belongs to.
    #[serde(rename="campaignId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub campaign_id: Option<i64>,
    /// Required. The display name of the insertion order. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. Controls whether or not the insertion order can spend its budget and bid on inventory. * For CreateInsertionOrder method, only `ENTITY_STATUS_DRAFT` is allowed. To activate an insertion order, use UpdateInsertionOrder method and update the status to `ENTITY_STATUS_ACTIVE` after creation. * An insertion order cannot be changed back to `ENTITY_STATUS_DRAFT` status from any other status. * An insertion order cannot be set to `ENTITY_STATUS_ACTIVE` if its parent campaign is not active.
    #[serde(rename="entityStatus")]
    
    pub entity_status: Option<String>,
    /// Required. The frequency capping setting of the insertion order.
    #[serde(rename="frequencyCap")]
    
    pub frequency_cap: Option<FrequencyCap>,
    /// Output only. The unique ID of the insertion order. Assigned by the system.
    #[serde(rename="insertionOrderId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub insertion_order_id: Option<i64>,
    /// The type of insertion order. If this field is unspecified in creation, the value defaults to `RTB`.
    #[serde(rename="insertionOrderType")]
    
    pub insertion_order_type: Option<String>,
    /// Additional integration details of the insertion order.
    #[serde(rename="integrationDetails")]
    
    pub integration_details: Option<IntegrationDetails>,
    /// Output only. The resource name of the insertion order.
    
    pub name: Option<String>,
    /// Required. The budget spending speed setting of the insertion order.
    
    pub pacing: Option<Pacing>,
    /// The partner costs associated with the insertion order. If absent or empty in CreateInsertionOrder method, the newly created insertion order will inherit partner costs from the partner settings.
    #[serde(rename="partnerCosts")]
    
    pub partner_costs: Option<Vec<PartnerCost>>,
    /// Required. Performance goal of the insertion order.
    #[serde(rename="performanceGoal")]
    
    pub performance_goal: Option<PerformanceGoal>,
    /// Output only. The reservation type of the insertion order.
    #[serde(rename="reservationType")]
    
    pub reservation_type: Option<String>,
    /// Output only. The timestamp when the insertion order was last updated. Assigned by the system.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for InsertionOrder {}
impl client::ResponseResult for InsertionOrder {}


/// Settings that control how insertion order budget is allocated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertionOrderBudget {
    /// The type of automation used to manage bid and budget for the insertion order. If this field is unspecified in creation, the value defaults to `INSERTION_ORDER_AUTOMATION_TYPE_NONE`.
    #[serde(rename="automationType")]
    
    pub automation_type: Option<String>,
    /// Required. The list of budget segments. Use a budget segment to specify a specific budget for a given period of time an insertion order is running.
    #[serde(rename="budgetSegments")]
    
    pub budget_segments: Option<Vec<InsertionOrderBudgetSegment>>,
    /// Required. Immutable. The budget unit specifies whether the budget is currency based or impression based.
    #[serde(rename="budgetUnit")]
    
    pub budget_unit: Option<String>,
}

impl client::Part for InsertionOrderBudget {}


/// Settings that control the budget of a single budget segment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertionOrderBudgetSegment {
    /// Required. The budget amount the insertion order will spend for the given date_range. The amount is in micros. Must be greater than 0. For example, 500000000 represents 500 standard units of the currency.
    #[serde(rename="budgetAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub budget_amount_micros: Option<i64>,
    /// The budget_id of the campaign budget that this insertion order budget segment is a part of.
    #[serde(rename="campaignBudgetId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub campaign_budget_id: Option<i64>,
    /// Required. The start and end date settings of the budget segment. They are resolved relative to the parent advertiser's time zone. * When creating a new budget segment, both `start_date` and `end_date` must be in the future. * An existing budget segment with a `start_date` in the past has a mutable `end_date` but an immutable `start_date`. * `end_date` must be the `start_date` or later, both before the year 2037.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<DateRange>,
    /// The budget segment description. It can be used to enter Purchase Order information for each budget segment and have that information printed on the invoices. Must be UTF-8 encoded.
    
    pub description: Option<String>,
}

impl client::Part for InsertionOrderBudgetSegment {}


/// Details of Integral Ad Science settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IntegralAdScience {
    /// The custom segment ID provided by Integral Ad Science. The ID must be between `1000001` and `1999999`, inclusive.
    #[serde(rename="customSegmentId")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub custom_segment_id: Option<Vec<i64>>,
    /// Display Viewability section (applicable to display line items only).
    #[serde(rename="displayViewability")]
    
    pub display_viewability: Option<String>,
    /// Brand Safety - **Unrateable**.
    #[serde(rename="excludeUnrateable")]
    
    pub exclude_unrateable: Option<bool>,
    /// Ad Fraud settings.
    #[serde(rename="excludedAdFraudRisk")]
    
    pub excluded_ad_fraud_risk: Option<String>,
    /// Brand Safety - **Adult content**.
    #[serde(rename="excludedAdultRisk")]
    
    pub excluded_adult_risk: Option<String>,
    /// Brand Safety - **Alcohol**.
    #[serde(rename="excludedAlcoholRisk")]
    
    pub excluded_alcohol_risk: Option<String>,
    /// Brand Safety - **Drugs**.
    #[serde(rename="excludedDrugsRisk")]
    
    pub excluded_drugs_risk: Option<String>,
    /// Brand Safety - **Gambling**.
    #[serde(rename="excludedGamblingRisk")]
    
    pub excluded_gambling_risk: Option<String>,
    /// Brand Safety - **Hate speech**.
    #[serde(rename="excludedHateSpeechRisk")]
    
    pub excluded_hate_speech_risk: Option<String>,
    /// Brand Safety - **Illegal downloads**.
    #[serde(rename="excludedIllegalDownloadsRisk")]
    
    pub excluded_illegal_downloads_risk: Option<String>,
    /// Brand Safety - **Offensive language**.
    #[serde(rename="excludedOffensiveLanguageRisk")]
    
    pub excluded_offensive_language_risk: Option<String>,
    /// Brand Safety - **Violence**.
    #[serde(rename="excludedViolenceRisk")]
    
    pub excluded_violence_risk: Option<String>,
    /// True advertising quality (applicable to Display line items only).
    #[serde(rename="traqScoreOption")]
    
    pub traq_score_option: Option<String>,
    /// Video Viewability Section (applicable to video line items only).
    #[serde(rename="videoViewability")]
    
    pub video_viewability: Option<String>,
}

impl client::Part for IntegralAdScience {}


/// Integration details of an entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IntegrationDetails {
    /// Additional details of the entry in string format. Must be UTF-8 encoded with a length of no more than 1000 characters.
    
    pub details: Option<String>,
    /// An external identifier to be associated with the entry. The integration code will show up together with the entry in many places in the system, for example, reporting. Must be UTF-8 encoded with a length of no more than 500 characters.
    #[serde(rename="integrationCode")]
    
    pub integration_code: Option<String>,
}

impl client::Part for IntegrationDetails {}


/// An inventory source.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create inventory sources](InventorySourceCreateCall) (request|response)
/// * [edit inventory source read write accessors inventory sources](InventorySourceEditInventorySourceReadWriteAccessorCall) (none)
/// * [get inventory sources](InventorySourceGetCall) (response)
/// * [list inventory sources](InventorySourceListCall) (none)
/// * [patch inventory sources](InventorySourcePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventorySource {
    /// Whether the inventory source has a guaranteed or non-guaranteed delivery.
    
    pub commitment: Option<String>,
    /// The creative requirements of the inventory source. Not applicable for auction packages.
    #[serde(rename="creativeConfigs")]
    
    pub creative_configs: Option<Vec<CreativeConfig>>,
    /// The ID in the exchange space that uniquely identifies the inventory source. Must be unique across buyers within each exchange but not necessarily unique across exchanges.
    #[serde(rename="dealId")]
    
    pub deal_id: Option<String>,
    /// The delivery method of the inventory source. * For non-guaranteed inventory sources, the only acceptable value is `INVENTORY_SOURCE_DELIVERY_METHOD_PROGRAMMATIC`. * For guaranteed inventory sources, acceptable values are `INVENTORY_SOURCE_DELIVERY_METHOD_TAG` and `INVENTORY_SOURCE_DELIVERY_METHOD_PROGRAMMATIC`.
    #[serde(rename="deliveryMethod")]
    
    pub delivery_method: Option<String>,
    /// The display name of the inventory source. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The exchange to which the inventory source belongs.
    
    pub exchange: Option<String>,
    /// Immutable. The ID of the guaranteed order that this inventory source belongs to. Only applicable when commitment is `INVENTORY_SOURCE_COMMITMENT_GUARANTEED`.
    #[serde(rename="guaranteedOrderId")]
    
    pub guaranteed_order_id: Option<String>,
    /// Output only. The unique ID of the inventory source. Assigned by the system.
    #[serde(rename="inventorySourceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub inventory_source_id: Option<i64>,
    /// Output only. The product type of the inventory source, denoting the way through which it sells inventory.
    #[serde(rename="inventorySourceProductType")]
    
    pub inventory_source_product_type: Option<String>,
    /// Denotes the type of the inventory source.
    #[serde(rename="inventorySourceType")]
    
    pub inventory_source_type: Option<String>,
    /// Output only. The resource name of the inventory source.
    
    pub name: Option<String>,
    /// The publisher/seller name of the inventory source.
    #[serde(rename="publisherName")]
    
    pub publisher_name: Option<String>,
    /// Required. The rate details of the inventory source.
    #[serde(rename="rateDetails")]
    
    pub rate_details: Option<RateDetails>,
    /// Output only. The IDs of advertisers with read-only access to the inventory source.
    #[serde(rename="readAdvertiserIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub read_advertiser_ids: Option<Vec<i64>>,
    /// Output only. The IDs of partners with read-only access to the inventory source. All advertisers of partners in this field inherit read-only access to the inventory source.
    #[serde(rename="readPartnerIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub read_partner_ids: Option<Vec<i64>>,
    /// The partner or advertisers that have read/write access to the inventory source. Output only when commitment is `INVENTORY_SOURCE_COMMITMENT_GUARANTEED`, in which case the read/write accessors are inherited from the parent guaranteed order. Required when commitment is `INVENTORY_SOURCE_COMMITMENT_NON_GUARANTEED`. If commitment is `INVENTORY_SOURCE_COMMITMENT_NON_GUARANTEED` and a partner is set in this field, all advertisers under this partner will automatically have read-only access to the inventory source. These advertisers will not be included in read_advertiser_ids.
    #[serde(rename="readWriteAccessors")]
    
    pub read_write_accessors: Option<InventorySourceAccessors>,
    /// The status settings of the inventory source.
    
    pub status: Option<InventorySourceStatus>,
    /// Immutable. The unique ID of the sub-site property assigned to this inventory source.
    #[serde(rename="subSitePropertyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub sub_site_property_id: Option<i64>,
    /// The time range when this inventory source starts and stops serving.
    #[serde(rename="timeRange")]
    
    pub time_range: Option<TimeRange>,
    /// Output only. The timestamp when the inventory source was last updated. Assigned by the system.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for InventorySource {}
impl client::Resource for InventorySource {}
impl client::ResponseResult for InventorySource {}


/// The partner or advertisers with access to the inventory source.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [edit inventory source read write accessors inventory sources](InventorySourceEditInventorySourceReadWriteAccessorCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventorySourceAccessors {
    /// The advertisers with access to the inventory source. All advertisers must belong to the same partner.
    
    pub advertisers: Option<InventorySourceAccessorsAdvertiserAccessors>,
    /// The partner with access to the inventory source.
    
    pub partner: Option<InventorySourceAccessorsPartnerAccessor>,
}

impl client::ResponseResult for InventorySourceAccessors {}


/// The advertisers with access to the inventory source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventorySourceAccessorsAdvertiserAccessors {
    /// The IDs of the advertisers.
    #[serde(rename="advertiserIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub advertiser_ids: Option<Vec<i64>>,
}

impl client::Part for InventorySourceAccessorsAdvertiserAccessors {}


/// The partner with access to the inventory source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventorySourceAccessorsPartnerAccessor {
    /// The ID of the partner.
    #[serde(rename="partnerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partner_id: Option<i64>,
}

impl client::Part for InventorySourceAccessorsPartnerAccessor {}


/// Targeting details for inventory source. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_INVENTORY_SOURCE`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventorySourceAssignedTargetingOptionDetails {
    /// Required. ID of the inventory source. Should refer to the inventory_source_id field of an InventorySource resource.
    #[serde(rename="inventorySourceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub inventory_source_id: Option<i64>,
}

impl client::Part for InventorySourceAssignedTargetingOptionDetails {}


/// The configuration for display creatives.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventorySourceDisplayCreativeConfig {
    /// The size requirements for display creatives that can be assigned to the inventory source.
    #[serde(rename="creativeSize")]
    
    pub creative_size: Option<Dimensions>,
}

impl client::Part for InventorySourceDisplayCreativeConfig {}


/// A filtering option for filtering on Inventory Source entities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventorySourceFilter {
    /// Inventory Sources to download by ID. All IDs must belong to the same Advertiser or Partner specified in CreateSdfDownloadTaskRequest. Leave empty to download all Inventory Sources for the selected Advertiser or Partner.
    #[serde(rename="inventorySourceIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub inventory_source_ids: Option<Vec<i64>>,
}

impl client::Part for InventorySourceFilter {}


/// A collection of targetable inventory sources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assigned inventory sources bulk edit inventory source groups](InventorySourceGroupAssignedInventorySourceBulkEditCall) (none)
/// * [assigned inventory sources create inventory source groups](InventorySourceGroupAssignedInventorySourceCreateCall) (none)
/// * [assigned inventory sources delete inventory source groups](InventorySourceGroupAssignedInventorySourceDeleteCall) (none)
/// * [assigned inventory sources list inventory source groups](InventorySourceGroupAssignedInventorySourceListCall) (none)
/// * [create inventory source groups](InventorySourceGroupCreateCall) (request|response)
/// * [delete inventory source groups](InventorySourceGroupDeleteCall) (none)
/// * [get inventory source groups](InventorySourceGroupGetCall) (response)
/// * [list inventory source groups](InventorySourceGroupListCall) (none)
/// * [patch inventory source groups](InventorySourceGroupPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventorySourceGroup {
    /// Required. The display name of the inventory source group. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The unique ID of the inventory source group. Assigned by the system.
    #[serde(rename="inventorySourceGroupId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub inventory_source_group_id: Option<i64>,
    /// Output only. The resource name of the inventory source group.
    
    pub name: Option<String>,
}

impl client::RequestValue for InventorySourceGroup {}
impl client::Resource for InventorySourceGroup {}
impl client::ResponseResult for InventorySourceGroup {}


/// Targeting details for inventory source group. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_INVENTORY_SOURCE_GROUP`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventorySourceGroupAssignedTargetingOptionDetails {
    /// Required. ID of the inventory source group. Should refer to the inventory_source_group_id field of an InventorySourceGroup resource.
    #[serde(rename="inventorySourceGroupId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub inventory_source_group_id: Option<i64>,
}

impl client::Part for InventorySourceGroupAssignedTargetingOptionDetails {}


/// The status related settings of the inventory source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventorySourceStatus {
    /// Output only. The configuration status of the inventory source. Only applicable for guaranteed inventory sources. Acceptable values are `INVENTORY_SOURCE_CONFIG_STATUS_PENDING` and `INVENTORY_SOURCE_CONFIG_STATUS_COMPLETED`. An inventory source must be configured (fill in the required fields, choose creatives, and select a default campaign) before it can serve.
    #[serde(rename="configStatus")]
    
    pub config_status: Option<String>,
    /// The user-provided reason for pausing this inventory source. Must not exceed 100 characters. Only applicable when entity_status is set to `ENTITY_STATUS_PAUSED`.
    #[serde(rename="entityPauseReason")]
    
    pub entity_pause_reason: Option<String>,
    /// Whether or not the inventory source is servable. Acceptable values are `ENTITY_STATUS_ACTIVE`, `ENTITY_STATUS_ARCHIVED`, and `ENTITY_STATUS_PAUSED`. Default value is `ENTITY_STATUS_ACTIVE`.
    #[serde(rename="entityStatus")]
    
    pub entity_status: Option<String>,
    /// Output only. The seller-provided reason for pausing this inventory source. Only applicable for inventory sources synced directly from the publishers and when seller_status is set to `ENTITY_STATUS_PAUSED`.
    #[serde(rename="sellerPauseReason")]
    
    pub seller_pause_reason: Option<String>,
    /// Output only. The status set by the seller for the inventory source. Only applicable for inventory sources synced directly from the publishers. Acceptable values are `ENTITY_STATUS_ACTIVE` and `ENTITY_STATUS_PAUSED`.
    #[serde(rename="sellerStatus")]
    
    pub seller_status: Option<String>,
}

impl client::Part for InventorySourceStatus {}


/// The configuration for video creatives.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventorySourceVideoCreativeConfig {
    /// The duration requirements for the video creatives that can be assigned to the inventory source.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
}

impl client::Part for InventorySourceVideoCreativeConfig {}


/// A single invoice.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Invoice {
    /// The budget grouping ID for this invoice. This field will only be set if the invoice level of the corresponding billing profile was set to "Budget invoice grouping ID".
    #[serde(rename="budgetInvoiceGroupingId")]
    
    pub budget_invoice_grouping_id: Option<String>,
    /// The list of summarized information for each budget associated with this invoice. This field will only be set if the invoice detail level of the corresponding billing profile was set to "Budget level PO".
    #[serde(rename="budgetSummaries")]
    
    pub budget_summaries: Option<Vec<BudgetSummary>>,
    /// The ID of the original invoice being adjusted by this invoice, if applicable. May appear on the invoice PDF as `Reference invoice number`. If replaced_invoice_ids is set, this field will be empty.
    #[serde(rename="correctedInvoiceId")]
    
    pub corrected_invoice_id: Option<String>,
    /// The currency used in the invoice in ISO 4217 format.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// The display name of the invoice.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The date when the invoice is due.
    #[serde(rename="dueDate")]
    
    pub due_date: Option<Date>,
    /// The unique ID of the invoice.
    #[serde(rename="invoiceId")]
    
    pub invoice_id: Option<String>,
    /// The type of invoice document.
    #[serde(rename="invoiceType")]
    
    pub invoice_type: Option<String>,
    /// The date when the invoice was issued.
    #[serde(rename="issueDate")]
    
    pub issue_date: Option<Date>,
    /// The resource name of the invoice.
    
    pub name: Option<String>,
    /// The total amount of costs or adjustments not tied to a particular budget, in micros of the invoice's currency. For example, if currency_code is `USD`, then 1000000 represents one US dollar.
    #[serde(rename="nonBudgetMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub non_budget_micros: Option<i64>,
    /// The ID of the payments account the invoice belongs to. Appears on the invoice PDF as `Billing Account Number`.
    #[serde(rename="paymentsAccountId")]
    
    pub payments_account_id: Option<String>,
    /// The ID of the payments profile the invoice belongs to. Appears on the invoice PDF as `Billing ID`.
    #[serde(rename="paymentsProfileId")]
    
    pub payments_profile_id: Option<String>,
    /// The URL to download a PDF copy of the invoice. This URL is user specific and requires a valid OAuth 2.0 access token to access. The access token must be provided in an `Authorization: Bearer` HTTP header and be authorized for one of the following scopes: * `https://www.googleapis.com/auth/display-video-mediaplanning` * `https://www.googleapis.com/auth/display-video` The URL will be valid for 7 days after retrieval of this invoice object or until this invoice is retrieved again.
    #[serde(rename="pdfUrl")]
    
    pub pdf_url: Option<String>,
    /// Purchase order number associated with the invoice.
    #[serde(rename="purchaseOrderNumber")]
    
    pub purchase_order_number: Option<String>,
    /// The ID(s) of any originally issued invoice that is being cancelled by this invoice, if applicable. Multiple invoices may be listed if those invoices are being consolidated into a single invoice. May appear on invoice PDF as `Replaced invoice numbers`. If corrected_invoice_id is set, this field will be empty.
    #[serde(rename="replacedInvoiceIds")]
    
    pub replaced_invoice_ids: Option<Vec<String>>,
    /// The service start and end dates which are covered by this invoice.
    #[serde(rename="serviceDateRange")]
    
    pub service_date_range: Option<DateRange>,
    /// The pre-tax subtotal amount, in micros of the invoice's currency. For example, if currency_code is `USD`, then 1000000 represents one US dollar.
    #[serde(rename="subtotalAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subtotal_amount_micros: Option<i64>,
    /// The invoice total amount, in micros of the invoice's currency. For example, if currency_code is `USD`, then 1000000 represents one US dollar.
    #[serde(rename="totalAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_amount_micros: Option<i64>,
    /// The sum of all taxes in invoice, in micros of the invoice's currency. For example, if currency_code is `USD`, then 1000000 represents one US dollar.
    #[serde(rename="totalTaxAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_tax_amount_micros: Option<i64>,
}

impl client::Part for Invoice {}


/// Details for assigned keyword targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_KEYWORD`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeywordAssignedTargetingOptionDetails {
    /// Required. The keyword, for example `car insurance`. Positive keyword cannot be offensive word. Must be UTF-8 encoded with a maximum size of 255 bytes. Maximum number of characters is 80. Maximum number of words is 10.
    
    pub keyword: Option<String>,
    /// Indicates if this option is being negatively targeted.
    
    pub negative: Option<bool>,
}

impl client::Part for KeywordAssignedTargetingOptionDetails {}


/// Details for assigned language targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_LANGUAGE`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LanguageAssignedTargetingOptionDetails {
    /// Output only. The display name of the language (e.g., "French").
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Indicates if this option is being negatively targeted. All assigned language targeting options on the same resource must have the same value for this field.
    
    pub negative: Option<bool>,
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_LANGUAGE`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for LanguageAssignedTargetingOptionDetails {}


/// Represents a targetable language. This will be populated in the language_details field when targeting_type is `TARGETING_TYPE_LANGUAGE`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LanguageTargetingOptionDetails {
    /// Output only. The display name of the language (e.g., "French").
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for LanguageTargetingOptionDetails {}


/// A single line item.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [line items create advertisers](AdvertiserLineItemCreateCall) (request|response)
/// * [line items generate default advertisers](AdvertiserLineItemGenerateDefaultCall) (response)
/// * [line items get advertisers](AdvertiserLineItemGetCall) (response)
/// * [line items patch advertisers](AdvertiserLineItemPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LineItem {
    /// Output only. The unique ID of the advertiser the line item belongs to.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Required. The bidding strategy of the line item.
    #[serde(rename="bidStrategy")]
    
    pub bid_strategy: Option<BiddingStrategy>,
    /// Required. The budget allocation setting of the line item.
    
    pub budget: Option<LineItemBudget>,
    /// Output only. The unique ID of the campaign that the line item belongs to.
    #[serde(rename="campaignId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub campaign_id: Option<i64>,
    /// The conversion tracking setting of the line item.
    #[serde(rename="conversionCounting")]
    
    pub conversion_counting: Option<ConversionCountingConfig>,
    /// The IDs of the creatives associated with the line item.
    #[serde(rename="creativeIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub creative_ids: Option<Vec<i64>>,
    /// Required. The display name of the line item. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. Controls whether or not the line item can spend its budget and bid on inventory. * For CreateLineItem method, only `ENTITY_STATUS_DRAFT` is allowed. To activate a line item, use UpdateLineItem method and update the status to `ENTITY_STATUS_ACTIVE` after creation. * A line item cannot be changed back to `ENTITY_STATUS_DRAFT` status from any other status. * If the line item's parent insertion order is not active, the line item can't spend its budget even if its own status is `ENTITY_STATUS_ACTIVE`.
    #[serde(rename="entityStatus")]
    
    pub entity_status: Option<String>,
    /// Whether to exclude new exchanges from automatically being targeted by the line item. This field is false by default.
    #[serde(rename="excludeNewExchanges")]
    
    pub exclude_new_exchanges: Option<bool>,
    /// Required. The start and end time of the line item's flight.
    
    pub flight: Option<LineItemFlight>,
    /// Required. The impression frequency cap settings of the line item. The max_impressions field in this settings object must be used if assigning a limited cap.
    #[serde(rename="frequencyCap")]
    
    pub frequency_cap: Option<FrequencyCap>,
    /// Required. Immutable. The unique ID of the insertion order that the line item belongs to.
    #[serde(rename="insertionOrderId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub insertion_order_id: Option<i64>,
    /// Integration details of the line item.
    #[serde(rename="integrationDetails")]
    
    pub integration_details: Option<IntegrationDetails>,
    /// The IDs of the private inventory sources assigned to the line item.
    #[serde(rename="inventorySourceIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub inventory_source_ids: Option<Vec<i64>>,
    /// Output only. The unique ID of the line item. Assigned by the system.
    #[serde(rename="lineItemId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub line_item_id: Option<i64>,
    /// Required. Immutable. The type of the line item.
    #[serde(rename="lineItemType")]
    
    pub line_item_type: Option<String>,
    /// The mobile app promoted by the line item. This is applicable only when line_item_type is either `LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INSTALL` or `LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INSTALL`.
    #[serde(rename="mobileApp")]
    
    pub mobile_app: Option<MobileApp>,
    /// Output only. The resource name of the line item.
    
    pub name: Option<String>,
    /// Required. The budget spending speed setting of the line item.
    
    pub pacing: Option<Pacing>,
    /// The partner costs associated with the line item. If absent or empty in CreateLineItem method, the newly created line item will inherit partner costs from its parent insertion order.
    #[serde(rename="partnerCosts")]
    
    pub partner_costs: Option<Vec<PartnerCost>>,
    /// Required. The partner revenue model setting of the line item.
    #[serde(rename="partnerRevenueModel")]
    
    pub partner_revenue_model: Option<PartnerRevenueModel>,
    /// Output only. The reservation type of the line item.
    #[serde(rename="reservationType")]
    
    pub reservation_type: Option<String>,
    /// The [targeting expansion](https://developers.google.com//support.google.com/displayvideo/answer/10191558) settings of the line item. This config is only applicable when eligible audience list targeting is assigned to the line item. Beginning November 7, 2022, these settings may represent the [optimized targeting feature](https://developers.google.com//support.google.com/displayvideo/answer/12060859) in place of targeting expansion. This feature will be rolled out to all partners by November 9, 2022.
    #[serde(rename="targetingExpansion")]
    
    pub targeting_expansion: Option<TargetingExpansionConfig>,
    /// Output only. The timestamp when the line item was last updated. Assigned by the system.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The warning messages generated by the line item. These warnings do not block saving the line item, but some may block the line item from running.
    #[serde(rename="warningMessages")]
    
    pub warning_messages: Option<Vec<String>>,
}

impl client::RequestValue for LineItem {}
impl client::ResponseResult for LineItem {}


/// Settings that control how budget is allocated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LineItemBudget {
    /// Required. The type of the budget allocation. `LINE_ITEM_BUDGET_ALLOCATION_TYPE_AUTOMATIC` is only applicable when automatic budget allocation is enabled for the parent insertion order.
    #[serde(rename="budgetAllocationType")]
    
    pub budget_allocation_type: Option<String>,
    /// Output only. The budget unit specifies whether the budget is currency based or impression based. This value is inherited from the parent insertion order.
    #[serde(rename="budgetUnit")]
    
    pub budget_unit: Option<String>,
    /// The maximum budget amount the line item will spend. Must be greater than 0. When budget_allocation_type is: * `LINE_ITEM_BUDGET_ALLOCATION_TYPE_AUTOMATIC`, this field is immutable and is set by the system. * `LINE_ITEM_BUDGET_ALLOCATION_TYPE_FIXED`, if budget_unit is: - `BUDGET_UNIT_CURRENCY`, this field represents maximum budget amount to spend, in micros of the advertiser's currency. For example, 1500000 represents 1.5 standard units of the currency. - `BUDGET_UNIT_IMPRESSIONS`, this field represents the maximum number of impressions to serve. * `LINE_ITEM_BUDGET_ALLOCATION_TYPE_UNLIMITED`, this field is not applicable and will be ignored by the system.
    #[serde(rename="maxAmount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_amount: Option<i64>,
}

impl client::Part for LineItemBudget {}


/// Settings that control the active duration of a line item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LineItemFlight {
    /// The flight start and end dates of the line item. They are resolved relative to the parent advertiser's time zone. * Required when flight_date_type is `LINE_ITEM_FLIGHT_DATE_TYPE_CUSTOM`. Output only otherwise. * When creating a new flight, both `start_date` and `end_date` must be in the future. * An existing flight with a `start_date` in the past has a mutable `end_date` but an immutable `start_date`. * `end_date` must be the `start_date` or later, both before the year 2037.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<DateRange>,
    /// Required. The type of the line item's flight dates.
    #[serde(rename="flightDateType")]
    
    pub flight_date_type: Option<String>,
    /// The ID of the manual trigger associated with the line item. * Required when flight_date_type is `LINE_ITEM_FLIGHT_DATE_TYPE_TRIGGER`. Must not be set otherwise. * When set, the line item's flight dates are inherited from its parent insertion order. * Active line items will spend when the selected trigger is activated within the parent insertion order's flight dates.
    #[serde(rename="triggerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub trigger_id: Option<i64>,
}

impl client::Part for LineItemFlight {}


/// Response message for ListAdvertiserAssignedTargetingOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [targeting types assigned targeting options list advertisers](AdvertiserTargetingTypeAssignedTargetingOptionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAdvertiserAssignedTargetingOptionsResponse {
    /// The list of assigned targeting options. This list will be absent if empty.
    #[serde(rename="assignedTargetingOptions")]
    
    pub assigned_targeting_options: Option<Vec<AssignedTargetingOption>>,
    /// A token identifying the next page of results. This value should be specified as the pageToken in a subsequent ListAdvertiserAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAdvertiserAssignedTargetingOptionsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list advertisers](AdvertiserListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAdvertisersResponse {
    /// The list of advertisers. This list will be absent if empty.
    
    pub advertisers: Option<Vec<Advertiser>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListAdvertisers` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAdvertisersResponse {}


/// Response message for AssignedInventorySourceService.ListAssignedInventorySources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assigned inventory sources list inventory source groups](InventorySourceGroupAssignedInventorySourceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAssignedInventorySourcesResponse {
    /// The list of assigned inventory sources. This list will be absent if empty.
    #[serde(rename="assignedInventorySources")]
    
    pub assigned_inventory_sources: Option<Vec<AssignedInventorySource>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListAssignedInventorySources` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAssignedInventorySourcesResponse {}


/// Response message for AssignedLocationService.ListAssignedLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [location lists assigned locations list advertisers](AdvertiserLocationListAssignedLocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAssignedLocationsResponse {
    /// The list of assigned locations. This list will be absent if empty.
    #[serde(rename="assignedLocations")]
    
    pub assigned_locations: Option<Vec<AssignedLocation>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListAssignedLocations` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAssignedLocationsResponse {}


/// Response message for ListCampaignAssignedTargetingOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [campaigns targeting types assigned targeting options list advertisers](AdvertiserCampaignTargetingTypeAssignedTargetingOptionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCampaignAssignedTargetingOptionsResponse {
    /// The list of assigned targeting options. This list will be absent if empty.
    #[serde(rename="assignedTargetingOptions")]
    
    pub assigned_targeting_options: Option<Vec<AssignedTargetingOption>>,
    /// A token identifying the next page of results. This value should be specified as the pageToken in a subsequent ListCampaignAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCampaignAssignedTargetingOptionsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [campaigns list advertisers](AdvertiserCampaignListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCampaignsResponse {
    /// The list of campaigns. This list will be absent if empty.
    
    pub campaigns: Option<Vec<Campaign>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListCampaigns` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCampaignsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channels list advertisers](AdvertiserChannelListCall) (response)
/// * [channels list partners](PartnerChannelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListChannelsResponse {
    /// The list of channels. This list will be absent if empty.
    
    pub channels: Option<Vec<Channel>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListChannels` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListChannelsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list combined audiences](CombinedAudienceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCombinedAudiencesResponse {
    /// The list of combined audiences. This list will be absent if empty.
    #[serde(rename="combinedAudiences")]
    
    pub combined_audiences: Option<Vec<CombinedAudience>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListCombinedAudiences` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCombinedAudiencesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives list advertisers](AdvertiserCreativeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCreativesResponse {
    /// The list of creatives. This list will be absent if empty.
    
    pub creatives: Option<Vec<Creative>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListCreativesRequest` method to retrieve the next page of results. If this field is null, it means this is the last page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCreativesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list custom bidding algorithms](CustomBiddingAlgorithmListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCustomBiddingAlgorithmsResponse {
    /// The list of custom bidding algorithms. This list will be absent if empty.
    #[serde(rename="customBiddingAlgorithms")]
    
    pub custom_bidding_algorithms: Option<Vec<CustomBiddingAlgorithm>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListCustomBiddingAlgorithmsRequest` method to retrieve the next page of results. If this field is null, it means this is the last page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCustomBiddingAlgorithmsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [scripts list custom bidding algorithms](CustomBiddingAlgorithmScriptListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCustomBiddingScriptsResponse {
    /// The list of custom bidding scripts. This list will be absent if empty.
    #[serde(rename="customBiddingScripts")]
    
    pub custom_bidding_scripts: Option<Vec<CustomBiddingScript>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListCustomBiddingScriptsRequest` method to retrieve the next page of results. If this field is null, it means this is the last page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCustomBiddingScriptsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list custom lists](CustomListListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCustomListsResponse {
    /// The list of custom lists. This list will be absent if empty.
    #[serde(rename="customLists")]
    
    pub custom_lists: Option<Vec<CustomList>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListCustomLists` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCustomListsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list first and third party audiences](FirstAndThirdPartyAudienceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFirstAndThirdPartyAudiencesResponse {
    /// The list of first and third party audiences. Audience size properties will not be included. This list will be absent if empty.
    #[serde(rename="firstAndThirdPartyAudiences")]
    
    pub first_and_third_party_audiences: Option<Vec<FirstAndThirdPartyAudience>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListFirstAndThirdPartyAudiences` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListFirstAndThirdPartyAudiencesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list google audiences](GoogleAudienceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGoogleAudiencesResponse {
    /// The list of Google audiences. This list will be absent if empty.
    #[serde(rename="googleAudiences")]
    
    pub google_audiences: Option<Vec<GoogleAudience>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListGoogleAudiences` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListGoogleAudiencesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list guaranteed orders](GuaranteedOrderListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGuaranteedOrdersResponse {
    /// The list of guaranteed orders. This list will be absent if empty.
    #[serde(rename="guaranteedOrders")]
    
    pub guaranteed_orders: Option<Vec<GuaranteedOrder>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListGuaranteedOrders` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListGuaranteedOrdersResponse {}


/// Response message for ListInsertionOrderAssignedTargetingOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insertion orders targeting types assigned targeting options list advertisers](AdvertiserInsertionOrderTargetingTypeAssignedTargetingOptionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInsertionOrderAssignedTargetingOptionsResponse {
    /// The list of assigned targeting options. This list will be absent if empty.
    #[serde(rename="assignedTargetingOptions")]
    
    pub assigned_targeting_options: Option<Vec<AssignedTargetingOption>>,
    /// A token identifying the next page of results. This value should be specified as the pageToken in a subsequent ListInsertionOrderAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListInsertionOrderAssignedTargetingOptionsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insertion orders list advertisers](AdvertiserInsertionOrderListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInsertionOrdersResponse {
    /// The list of insertion orders. This list will be absent if empty.
    #[serde(rename="insertionOrders")]
    
    pub insertion_orders: Option<Vec<InsertionOrder>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListInsertionOrders` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListInsertionOrdersResponse {}


/// Response message for InventorySourceGroupService.ListInventorySourceGroups.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list inventory source groups](InventorySourceGroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInventorySourceGroupsResponse {
    /// The list of inventory source groups. This list will be absent if empty.
    #[serde(rename="inventorySourceGroups")]
    
    pub inventory_source_groups: Option<Vec<InventorySourceGroup>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListInventorySourceGroups` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListInventorySourceGroupsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list inventory sources](InventorySourceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInventorySourcesResponse {
    /// The list of inventory sources. This list will be absent if empty.
    #[serde(rename="inventorySources")]
    
    pub inventory_sources: Option<Vec<InventorySource>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListInventorySources` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListInventorySourcesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [invoices list advertisers](AdvertiserInvoiceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInvoicesResponse {
    /// The list of invoices. This list will be absent if empty.
    
    pub invoices: Option<Vec<Invoice>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListInvoices` method to retrieve the next page of results. This token will be absent if there are no more invoices to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListInvoicesResponse {}


/// Response message for ListLineItemAssignedTargetingOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [line items targeting types assigned targeting options list advertisers](AdvertiserLineItemTargetingTypeAssignedTargetingOptionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLineItemAssignedTargetingOptionsResponse {
    /// The list of assigned targeting options. This list will be absent if empty.
    #[serde(rename="assignedTargetingOptions")]
    
    pub assigned_targeting_options: Option<Vec<AssignedTargetingOption>>,
    /// A token identifying the next page of results. This value should be specified as the pageToken in a subsequent ListLineItemAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLineItemAssignedTargetingOptionsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [line items list advertisers](AdvertiserLineItemListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLineItemsResponse {
    /// The list of line items. This list will be absent if empty.
    #[serde(rename="lineItems")]
    
    pub line_items: Option<Vec<LineItem>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListLineItems` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLineItemsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [location lists list advertisers](AdvertiserLocationListListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLocationListsResponse {
    /// The list of location lists. This list will be absent if empty.
    #[serde(rename="locationLists")]
    
    pub location_lists: Option<Vec<LocationList>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListLocationLists` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLocationListsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [manual triggers list advertisers](AdvertiserManualTriggerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListManualTriggersResponse {
    /// The list of manual triggers. This list will be absent if empty.
    #[serde(rename="manualTriggers")]
    
    pub manual_triggers: Option<Vec<ManualTrigger>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListManualTriggers` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListManualTriggersResponse {}


/// Response message for NegativeKeywordListService.ListNegativeKeywordLists.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [negative keyword lists list advertisers](AdvertiserNegativeKeywordListListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNegativeKeywordListsResponse {
    /// The list of negative keyword lists. This list will be absent if empty.
    #[serde(rename="negativeKeywordLists")]
    
    pub negative_keyword_lists: Option<Vec<NegativeKeywordList>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListNegativeKeywordLists` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListNegativeKeywordListsResponse {}


/// Response message for NegativeKeywordService.ListNegativeKeywords.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [negative keyword lists negative keywords list advertisers](AdvertiserNegativeKeywordListNegativeKeywordListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNegativeKeywordsResponse {
    /// The list of negative keywords. This list will be absent if empty.
    #[serde(rename="negativeKeywords")]
    
    pub negative_keywords: Option<Vec<NegativeKeyword>>,
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListNegativeKeywords` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListNegativeKeywordsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [targeting types assigned targeting options list partners](PartnerTargetingTypeAssignedTargetingOptionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPartnerAssignedTargetingOptionsResponse {
    /// The list of assigned targeting options. This list will be absent if empty.
    #[serde(rename="assignedTargetingOptions")]
    
    pub assigned_targeting_options: Option<Vec<AssignedTargetingOption>>,
    /// A token identifying the next page of results. This value should be specified as the pageToken in a subsequent ListPartnerAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListPartnerAssignedTargetingOptionsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list partners](PartnerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPartnersResponse {
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListPartners` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of partners. This list will be absent if empty.
    
    pub partners: Option<Vec<Partner>>,
}

impl client::ResponseResult for ListPartnersResponse {}


/// Response message for SiteService.ListSites.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channels sites list advertisers](AdvertiserChannelSiteListCall) (response)
/// * [channels sites list partners](PartnerChannelSiteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSitesResponse {
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListSites` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of sites. This list will be absent if empty.
    
    pub sites: Option<Vec<Site>>,
}

impl client::ResponseResult for ListSitesResponse {}


/// Response message for ListTargetingOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [targeting options list targeting types](TargetingTypeTargetingOptionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTargetingOptionsResponse {
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListTargetingOptions` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of targeting options. This list will be absent if empty.
    #[serde(rename="targetingOptions")]
    
    pub targeting_options: Option<Vec<TargetingOption>>,
}

impl client::ResponseResult for ListTargetingOptionsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list users](UserListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListUsersResponse {
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListUsers` method to retrieve the next page of results. This token will be absent if there are no more results to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of users. This list will be absent if empty.
    
    pub users: Option<Vec<User>>,
}

impl client::ResponseResult for ListUsersResponse {}


/// A list of locations used for targeting.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [location lists create advertisers](AdvertiserLocationListCreateCall) (request|response)
/// * [location lists get advertisers](AdvertiserLocationListGetCall) (response)
/// * [location lists patch advertisers](AdvertiserLocationListPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationList {
    /// Required. Immutable. The unique ID of the advertiser the location list belongs to.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Required. The display name of the location list. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The unique ID of the location list. Assigned by the system.
    #[serde(rename="locationListId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub location_list_id: Option<i64>,
    /// Required. Immutable. The type of location. All locations in the list will share this type.
    #[serde(rename="locationType")]
    
    pub location_type: Option<String>,
    /// Output only. The resource name of the location list.
    
    pub name: Option<String>,
}

impl client::RequestValue for LocationList {}
impl client::ResponseResult for LocationList {}


/// Specifies how many days into the past to look when determining whether to record a conversion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LookbackWindow {
    /// Lookback window, in days, from the last time a given user clicked on one of your ads.
    #[serde(rename="clickDays")]
    
    pub click_days: Option<i32>,
    /// Lookback window, in days, from the last time a given user viewed one of your ads.
    #[serde(rename="impressionDays")]
    
    pub impression_days: Option<i32>,
}

impl client::Part for LookbackWindow {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [invoices lookup invoice currency advertisers](AdvertiserInvoiceLookupInvoiceCurrencyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LookupInvoiceCurrencyResponse {
    /// Currency used by the advertiser in ISO 4217 format.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
}

impl client::ResponseResult for LookupInvoiceCurrencyResponse {}


/// A single manual trigger in Display & Video 360.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [manual triggers activate advertisers](AdvertiserManualTriggerActivateCall) (response)
/// * [manual triggers create advertisers](AdvertiserManualTriggerCreateCall) (request|response)
/// * [manual triggers deactivate advertisers](AdvertiserManualTriggerDeactivateCall) (response)
/// * [manual triggers get advertisers](AdvertiserManualTriggerGetCall) (response)
/// * [manual triggers patch advertisers](AdvertiserManualTriggerPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManualTrigger {
    /// Required. The maximum duration of each activation in minutes. Must be between 1 and 360 inclusive. After this duration, the trigger will be automatically deactivated.
    #[serde(rename="activationDurationMinutes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub activation_duration_minutes: Option<i64>,
    /// Required. Immutable. The unique ID of the advertiser that the manual trigger belongs to.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Required. The display name of the manual trigger. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The timestamp of the trigger's latest activation.
    #[serde(rename="latestActivationTime")]
    
    pub latest_activation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The resource name of the manual trigger.
    
    pub name: Option<String>,
    /// Output only. The state of the manual trigger. Will be set to the `INACTIVE` state upon creation.
    
    pub state: Option<String>,
    /// Output only. The unique ID of the manual trigger.
    #[serde(rename="triggerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub trigger_id: Option<i64>,
}

impl client::RequestValue for ManualTrigger {}
impl client::ResponseResult for ManualTrigger {}


/// A strategy that automatically adjusts the bid to optimize a specified performance goal while spending the full budget.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaximizeSpendBidStrategy {
    /// The ID of the Custom Bidding Algorithm used by this strategy. Only applicable when performance_goal_type is set to `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO`.
    #[serde(rename="customBiddingAlgorithmId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub custom_bidding_algorithm_id: Option<i64>,
    /// The maximum average CPM that may be bid, in micros of the advertiser's currency. Must be greater than or equal to a billable unit of the given currency. For example, 1500000 represents 1.5 standard units of the currency.
    #[serde(rename="maxAverageCpmBidAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_average_cpm_bid_amount_micros: Option<i64>,
    /// Required. The type of the performance goal that the bidding strategy tries to minimize while spending the full budget. `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM` is not supported for this strategy.
    #[serde(rename="performanceGoalType")]
    
    pub performance_goal_type: Option<String>,
    /// Whether the strategy takes deal floor prices into account.
    #[serde(rename="raiseBidForDeals")]
    
    pub raise_bid_for_deals: Option<bool>,
}

impl client::Part for MaximizeSpendBidStrategy {}


/// Measurement settings of a partner.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MeasurementConfig {
    /// Whether or not to report DV360 cost to CM360.
    #[serde(rename="dv360ToCmCostReportingEnabled")]
    
    pub dv360_to_cm_cost_reporting_enabled: Option<bool>,
    /// Whether or not to include DV360 data in CM360 data transfer reports.
    #[serde(rename="dv360ToCmDataSharingEnabled")]
    
    pub dv360_to_cm_data_sharing_enabled: Option<bool>,
}

impl client::Part for MeasurementConfig {}


/// A mobile app promoted by a mobile app install line item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MobileApp {
    /// Required. The ID of the app provided by the platform store. Android apps are identified by the bundle ID used by Android's Play store, such as `com.google.android.gm`. iOS apps are identified by a nine-digit app ID used by Apple's App store, such as `422689480`.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// Output only. The app name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The app platform.
    
    pub platform: Option<String>,
    /// Output only. The app publisher.
    
    pub publisher: Option<String>,
}

impl client::Part for MobileApp {}


/// Wrapper message for a list of mobile device IDs defining Customer Match audience members.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MobileDeviceIdList {
    /// A list of mobile device IDs defining Customer Match audience members. The size of mobile_device_ids mustn't be greater than 500,000.
    #[serde(rename="mobileDeviceIds")]
    
    pub mobile_device_ids: Option<Vec<String>>,
}

impl client::Part for MobileDeviceIdList {}


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


/// Details for native content position assigned targeting option. This will be populated in the native_content_position_details field when targeting_type is `TARGETING_TYPE_NATIVE_CONTENT_POSITION`. Explicitly targeting all options is not supported. Remove all native content position targeting options to achieve this effect.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NativeContentPositionAssignedTargetingOptionDetails {
    /// The content position. Output only in v1. Required in v2.
    #[serde(rename="contentPosition")]
    
    pub content_position: Option<String>,
    /// Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_NATIVE_CONTENT_POSITION`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for NativeContentPositionAssignedTargetingOptionDetails {}


/// Represents a targetable native content position. This will be populated in the native_content_position_details field when targeting_type is `TARGETING_TYPE_NATIVE_CONTENT_POSITION`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NativeContentPositionTargetingOptionDetails {
    /// Output only. The content position.
    #[serde(rename="contentPosition")]
    
    pub content_position: Option<String>,
}

impl client::Part for NativeContentPositionTargetingOptionDetails {}


/// A negatively targeted keyword that belongs to a negative keyword list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [negative keyword lists negative keywords create advertisers](AdvertiserNegativeKeywordListNegativeKeywordCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NegativeKeyword {
    /// Required. Immutable. The negatively targeted keyword, for example `car insurance`. Must be UTF-8 encoded with a maximum size of 255 bytes. Maximum number of characters is 80. Maximum number of words is 10. Valid characters are restricted to ASCII characters only. The only URL-escaping permitted is for representing whitespace between words. Leading or trailing whitespace is ignored.
    #[serde(rename="keywordValue")]
    
    pub keyword_value: Option<String>,
    /// Output only. The resource name of the negative keyword.
    
    pub name: Option<String>,
}

impl client::RequestValue for NegativeKeyword {}
impl client::ResponseResult for NegativeKeyword {}


/// A list of negative keywords used for targeting.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [negative keyword lists create advertisers](AdvertiserNegativeKeywordListCreateCall) (request|response)
/// * [negative keyword lists get advertisers](AdvertiserNegativeKeywordListGetCall) (response)
/// * [negative keyword lists patch advertisers](AdvertiserNegativeKeywordListPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NegativeKeywordList {
    /// Output only. The unique ID of the advertiser the negative keyword list belongs to.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Required. The display name of the negative keyword list. Must be UTF-8 encoded with a maximum size of 255 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The resource name of the negative keyword list.
    
    pub name: Option<String>,
    /// Output only. The unique ID of the negative keyword list. Assigned by the system.
    #[serde(rename="negativeKeywordListId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub negative_keyword_list_id: Option<i64>,
    /// Output only. Number of line items that are directly targeting this negative keyword list.
    #[serde(rename="targetedLineItemCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub targeted_line_item_count: Option<i64>,
}

impl client::RequestValue for NegativeKeywordList {}
impl client::ResponseResult for NegativeKeywordList {}


/// Targeting details for negative keyword list. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_NEGATIVE_KEYWORD_LIST`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NegativeKeywordListAssignedTargetingOptionDetails {
    /// Required. ID of the negative keyword list. Should refer to the negative_keyword_list_id field of a NegativeKeywordList resource.
    #[serde(rename="negativeKeywordListId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub negative_keyword_list_id: Option<i64>,
}

impl client::Part for NegativeKeywordListAssignedTargetingOptionDetails {}


/// OBA Icon for a Creative
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObaIcon {
    /// Required. The click tracking URL of the OBA icon. Only URLs of the following domains are allowed: * https://info.evidon.com * https://l.betrad.com
    #[serde(rename="clickTrackingUrl")]
    
    pub click_tracking_url: Option<String>,
    /// The dimensions of the OBA icon.
    
    pub dimensions: Option<Dimensions>,
    /// Required. The landing page URL of the OBA icon. Only URLs of the following domains are allowed: * https://info.evidon.com * https://l.betrad.com
    #[serde(rename="landingPageUrl")]
    
    pub landing_page_url: Option<String>,
    /// The position of the OBA icon on the creative.
    
    pub position: Option<String>,
    /// The program of the OBA icon. For example: “AdChoices”.
    
    pub program: Option<String>,
    /// The MIME type of the OBA icon resource.
    #[serde(rename="resourceMimeType")]
    
    pub resource_mime_type: Option<String>,
    /// The URL of the OBA icon resource.
    #[serde(rename="resourceUrl")]
    
    pub resource_url: Option<String>,
    /// Required. The view tracking URL of the OBA icon. Only URLs of the following domains are allowed: * https://info.evidon.com * https://l.betrad.com
    #[serde(rename="viewTrackingUrl")]
    
    pub view_tracking_url: Option<String>,
}

impl client::Part for ObaIcon {}


/// Represents a targetable Open Measurement enabled inventory type. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_OMID`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OmidAssignedTargetingOptionDetails {
    /// The type of Open Measurement enabled inventory. Output only in v1. Required in v2.
    
    pub omid: Option<String>,
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_OMID`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for OmidAssignedTargetingOptionDetails {}


/// Represents a targetable Open Measurement enabled inventory type. This will be populated in the omid_details field when targeting_type is `TARGETING_TYPE_OMID`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OmidTargetingOptionDetails {
    /// Output only. The type of Open Measurement enabled inventory.
    
    pub omid: Option<String>,
}

impl client::Part for OmidTargetingOptionDetails {}


/// On screen position targeting option details. This will be populated in the on_screen_position_details field when targeting_type is `TARGETING_TYPE_ON_SCREEN_POSITION`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OnScreenPositionAssignedTargetingOptionDetails {
    /// Output only. The ad type to target. Only applicable to insertion order targeting and new line items supporting the specified ad type will inherit this targeting option by default. Possible values are: * `AD_TYPE_DISPLAY`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_DISPLAY_DEFAULT`. * `AD_TYPE_VIDEO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_VIDEO_DEFAULT`.
    #[serde(rename="adType")]
    
    pub ad_type: Option<String>,
    /// Output only. The on screen position.
    #[serde(rename="onScreenPosition")]
    
    pub on_screen_position: Option<String>,
    /// Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_ON_SCREEN_POSITION`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for OnScreenPositionAssignedTargetingOptionDetails {}


/// Represents a targetable on screen position, which could be used by display and video ads. This will be populated in the on_screen_position_details field when targeting_type is `TARGETING_TYPE_ON_SCREEN_POSITION`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OnScreenPositionTargetingOptionDetails {
    /// Output only. The on screen position.
    #[serde(rename="onScreenPosition")]
    
    pub on_screen_position: Option<String>,
}

impl client::Part for OnScreenPositionTargetingOptionDetails {}


/// Assigned operating system targeting option details. This will be populated in the operating_system_details field when targeting_type is `TARGETING_TYPE_OPERATING_SYSTEM`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperatingSystemAssignedTargetingOptionDetails {
    /// Output only. The display name of the operating system.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Indicates if this option is being negatively targeted.
    
    pub negative: Option<bool>,
    /// Required. The targeting option ID populated in targeting_option_id field when targeting_type is `TARGETING_TYPE_OPERATING_SYSTEM`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for OperatingSystemAssignedTargetingOptionDetails {}


/// Represents a targetable operating system. This will be populated in the operating_system_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_OPERATING_SYSTEM`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperatingSystemTargetingOptionDetails {
    /// Output only. The display name of the operating system.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for OperatingSystemTargetingOptionDetails {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations get sdfdownloadtasks](SdfdownloadtaskOperationGetCall) (response)
/// * [create sdfdownloadtasks](SdfdownloadtaskCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// Settings that control the rate at which a budget is spent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Pacing {
    /// Maximum number of impressions to serve every day. Applicable when the budget is impression based. Must be greater than 0.
    #[serde(rename="dailyMaxImpressions")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub daily_max_impressions: Option<i64>,
    /// Maximum currency amount to spend every day in micros of advertiser's currency. Applicable when the budget is currency based. Must be greater than 0. For example, for 1.5 standard unit of the currency, set this field to 1500000. The value assigned will be rounded to whole billable units for the relevant currency by the following rules: any positive value less than a single billable unit will be rounded up to one billable unit and any value larger than a single billable unit will be rounded down to the nearest billable value. For example, if the currency's billable unit is 0.01, and this field is set to 10257770, it will round down to 10250000, a value of 10.25. If set to 505, it will round up to 10000, a value of 0.01.
    #[serde(rename="dailyMaxMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub daily_max_micros: Option<i64>,
    /// Required. The time period in which the pacing budget will be spent. When automatic budget allocation is enabled at the insertion order via auto_budget_allocation, this field is output only and defaults to `PACING_PERIOD_FLIGHT`.
    #[serde(rename="pacingPeriod")]
    
    pub pacing_period: Option<String>,
    /// Required. The type of pacing that defines how the budget amount will be spent across the pacing_period.
    #[serde(rename="pacingType")]
    
    pub pacing_type: Option<String>,
}

impl client::Part for Pacing {}


/// A filtering option that filters on selected file types belonging to a chosen set of filter entities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParentEntityFilter {
    /// Required. File types that will be returned.
    #[serde(rename="fileType")]
    
    pub file_type: Option<Vec<String>>,
    /// The IDs of the specified filter type. This is used to filter entities to fetch. If filter type is not `FILTER_TYPE_NONE`, at least one ID must be specified.
    #[serde(rename="filterIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub filter_ids: Option<Vec<i64>>,
    /// Required. Filter type used to filter fetched entities.
    #[serde(rename="filterType")]
    
    pub filter_type: Option<String>,
}

impl client::Part for ParentEntityFilter {}


/// Details for assigned parental status targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_PARENTAL_STATUS`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParentalStatusAssignedTargetingOptionDetails {
    /// The parental status of the audience. Output only in v1. Required in v2.
    #[serde(rename="parentalStatus")]
    
    pub parental_status: Option<String>,
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_PARENTAL_STATUS`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for ParentalStatusAssignedTargetingOptionDetails {}


/// Represents a targetable parental status. This will be populated in the parental_status_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_PARENTAL_STATUS`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParentalStatusTargetingOptionDetails {
    /// Output only. The parental status of an audience.
    #[serde(rename="parentalStatus")]
    
    pub parental_status: Option<String>,
}

impl client::Part for ParentalStatusTargetingOptionDetails {}


/// A single partner in Display & Video 360 (DV360).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channels sites bulk edit partners](PartnerChannelSiteBulkEditCall) (none)
/// * [channels sites create partners](PartnerChannelSiteCreateCall) (none)
/// * [channels sites delete partners](PartnerChannelSiteDeleteCall) (none)
/// * [channels sites list partners](PartnerChannelSiteListCall) (none)
/// * [channels sites replace partners](PartnerChannelSiteReplaceCall) (none)
/// * [channels create partners](PartnerChannelCreateCall) (none)
/// * [channels get partners](PartnerChannelGetCall) (none)
/// * [channels list partners](PartnerChannelListCall) (none)
/// * [channels patch partners](PartnerChannelPatchCall) (none)
/// * [targeting types assigned targeting options create partners](PartnerTargetingTypeAssignedTargetingOptionCreateCall) (none)
/// * [targeting types assigned targeting options delete partners](PartnerTargetingTypeAssignedTargetingOptionDeleteCall) (none)
/// * [targeting types assigned targeting options get partners](PartnerTargetingTypeAssignedTargetingOptionGetCall) (none)
/// * [targeting types assigned targeting options list partners](PartnerTargetingTypeAssignedTargetingOptionListCall) (none)
/// * [bulk edit partner assigned targeting options partners](PartnerBulkEditPartnerAssignedTargetingOptionCall) (none)
/// * [get partners](PartnerGetCall) (response)
/// * [list partners](PartnerListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Partner {
    /// Ad server related settings of the partner.
    #[serde(rename="adServerConfig")]
    
    pub ad_server_config: Option<PartnerAdServerConfig>,
    /// Settings that control how partner data may be accessed.
    #[serde(rename="dataAccessConfig")]
    
    pub data_access_config: Option<PartnerDataAccessConfig>,
    /// The display name of the partner. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The status of the partner.
    #[serde(rename="entityStatus")]
    
    pub entity_status: Option<String>,
    /// Settings that control which exchanges are enabled for the partner.
    #[serde(rename="exchangeConfig")]
    
    pub exchange_config: Option<ExchangeConfig>,
    /// General settings of the partner.
    #[serde(rename="generalConfig")]
    
    pub general_config: Option<PartnerGeneralConfig>,
    /// Output only. The resource name of the partner.
    
    pub name: Option<String>,
    /// Output only. The unique ID of the partner. Assigned by the system.
    #[serde(rename="partnerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partner_id: Option<i64>,
    /// Output only. The timestamp when the partner was last updated. Assigned by the system.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Resource for Partner {}
impl client::ResponseResult for Partner {}


/// Ad server related settings of a partner.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartnerAdServerConfig {
    /// Measurement settings of a partner.
    #[serde(rename="measurementConfig")]
    
    pub measurement_config: Option<MeasurementConfig>,
}

impl client::Part for PartnerAdServerConfig {}


/// Settings that control a partner cost. A partner cost is any type of expense involved in running a campaign, other than the costs of purchasing impressions (which is called the media cost) and using third-party audience segment data (data fee). Some examples of partner costs include the fees for using DV360, a third-party ad server, or a third-party ad serving verification service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartnerCost {
    /// Required. The type of the partner cost.
    #[serde(rename="costType")]
    
    pub cost_type: Option<String>,
    /// The CPM fee amount in micros of advertiser's currency. Applicable when the fee_type is `PARTNER_FEE_TYPE_CPM_FEE`. Must be greater than or equal to 0. For example, for 1.5 standard unit of the advertiser's currency, set this field to 1500000.
    #[serde(rename="feeAmount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub fee_amount: Option<i64>,
    /// The media fee percentage in millis (1/1000 of a percent). Applicable when the fee_type is `PARTNER_FEE_TYPE_MEDIA_FEE`. Must be greater than or equal to 0. For example: 100 represents 0.1%.
    #[serde(rename="feePercentageMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub fee_percentage_millis: Option<i64>,
    /// Required. The fee type for this partner cost.
    #[serde(rename="feeType")]
    
    pub fee_type: Option<String>,
    /// The invoice type for this partner cost. * Required when cost_type is one of: - `PARTNER_COST_TYPE_ADLOOX` - `PARTNER_COST_TYPE_DOUBLE_VERIFY` - `PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE`. * Output only for other types.
    #[serde(rename="invoiceType")]
    
    pub invoice_type: Option<String>,
}

impl client::Part for PartnerCost {}


/// Settings that control how partner related data may be accessed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartnerDataAccessConfig {
    /// Structured Data Files (SDF) settings for the partner. The SDF configuration for the partner.
    #[serde(rename="sdfConfig")]
    
    pub sdf_config: Option<SdfConfig>,
}

impl client::Part for PartnerDataAccessConfig {}


/// General settings of a partner.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartnerGeneralConfig {
    /// Immutable. Partner's currency in ISO 4217 format.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Immutable. The standard TZ database name of the partner's time zone. For example, `America/New_York`. See more at: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::Part for PartnerGeneralConfig {}


/// Settings that control how partner revenue is calculated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartnerRevenueModel {
    /// Required. The markup amount of the partner revenue model. Must be greater than or equal to 0. * When the markup_type is set to be `PARTNER_REVENUE_MODEL_MARKUP_TYPE_CPM`, this field represents the CPM markup in micros of advertiser's currency. For example, 1500000 represents 1.5 standard units of the currency. * When the markup_type is set to be `PARTNER_REVENUE_MODEL_MARKUP_TYPE_MEDIA_COST_MARKUP`, this field represents the media cost percent markup in millis. For example, 100 represents 0.1% (decimal 0.001). * When the markup_type is set to be `PARTNER_REVENUE_MODEL_MARKUP_TYPE_TOTAL_MEDIA_COST_MARKUP`, this field represents the total media cost percent markup in millis. For example, 100 represents 0.1% (decimal 0.001).
    #[serde(rename="markupAmount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub markup_amount: Option<i64>,
    /// Required. The markup type of the partner revenue model.
    #[serde(rename="markupType")]
    
    pub markup_type: Option<String>,
}

impl client::Part for PartnerRevenueModel {}


/// Settings that control the performance goal of a campaign or insertion order.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceGoal {
    /// The goal amount, in micros of the advertiser's currency. Applicable when performance_goal_type is one of: * `PERFORMANCE_GOAL_TYPE_CPM` * `PERFORMANCE_GOAL_TYPE_CPC` * `PERFORMANCE_GOAL_TYPE_CPA` * `PERFORMANCE_GOAL_TYPE_CPIAVC` * `PERFORMANCE_GOAL_TYPE_VCPM` For example 1500000 represents 1.5 standard units of the currency.
    #[serde(rename="performanceGoalAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub performance_goal_amount_micros: Option<i64>,
    /// The decimal representation of the goal percentage in micros. Applicable when performance_goal_type is one of: * `PERFORMANCE_GOAL_TYPE_CTR` * `PERFORMANCE_GOAL_TYPE_VIEWABILITY` * `PERFORMANCE_GOAL_TYPE_CLICK_CVR` * `PERFORMANCE_GOAL_TYPE_IMPRESSION_CVR` * `PERFORMANCE_GOAL_TYPE_VTR` * `PERFORMANCE_GOAL_TYPE_AUDIO_COMPLETION_RATE` * `PERFORMANCE_GOAL_TYPE_VIDEO_COMPLETION_RATE` For example, 70000 represents 7% (decimal 0.07).
    #[serde(rename="performanceGoalPercentageMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub performance_goal_percentage_micros: Option<i64>,
    /// A key performance indicator (KPI) string, which can be empty. Must be UTF-8 encoded with a length of no more than 100 characters. Applicable when performance_goal_type is set to `PERFORMANCE_GOAL_TYPE_OTHER`.
    #[serde(rename="performanceGoalString")]
    
    pub performance_goal_string: Option<String>,
    /// Required. The type of the performance goal.
    #[serde(rename="performanceGoalType")]
    
    pub performance_goal_type: Option<String>,
}

impl client::Part for PerformanceGoal {}


/// A strategy that automatically adjusts the bid to meet or beat a specified performance goal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceGoalBidStrategy {
    /// The ID of the Custom Bidding Algorithm used by this strategy. Only applicable when performance_goal_type is set to `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO`.
    #[serde(rename="customBiddingAlgorithmId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub custom_bidding_algorithm_id: Option<i64>,
    /// The maximum average CPM that may be bid, in micros of the advertiser's currency. Must be greater than or equal to a billable unit of the given currency. Not applicable when performance_goal_type is set to `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM`. For example, 1500000 represents 1.5 standard units of the currency.
    #[serde(rename="maxAverageCpmBidAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_average_cpm_bid_amount_micros: Option<i64>,
    /// Required. The performance goal the bidding strategy will attempt to meet or beat, in micros of the advertiser's currency or in micro of the ROAS (Return On Advertising Spend) value which is also based on advertiser's currency. Must be greater than or equal to a billable unit of the given currency and smaller or equal to upper bounds. Each performance_goal_type has its upper bound: * when performance_goal_type is `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA`, upper bound is 10000.00 USD. * when performance_goal_type is `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC`, upper bound is 1000.00 USD. * when performance_goal_type is `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM`, upper bound is 1000.00 USD. * when performance_goal_type is `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO`, upper bound is 1000.00 and lower bound is 0.01. Example: If set to `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM`, the bid price will be based on the probability that each available impression will be viewable. For example, if viewable CPM target is $2 and an impression is 40% likely to be viewable, the bid price will be $0.80 CPM (40% of $2). For example, 1500000 represents 1.5 standard units of the currency or ROAS value.
    #[serde(rename="performanceGoalAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub performance_goal_amount_micros: Option<i64>,
    /// Required. The type of the performance goal that the bidding strategy will try to meet or beat. For line item level usage, the value must be one of: * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO`.
    #[serde(rename="performanceGoalType")]
    
    pub performance_goal_type: Option<String>,
}

impl client::Part for PerformanceGoalBidStrategy {}


/// Details for assigned POI targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_POI`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PoiAssignedTargetingOptionDetails {
    /// Output only. The display name of a POI, e.g. "Times Square", "Space Needle", followed by its full address if available.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Latitude of the POI rounding to 6th decimal place.
    
    pub latitude: Option<f64>,
    /// Output only. Longitude of the POI rounding to 6th decimal place.
    
    pub longitude: Option<f64>,
    /// Required. The radius of the area around the POI that will be targeted. The units of the radius are specified by proximity_radius_unit. Must be 1 to 800 if unit is `DISTANCE_UNIT_KILOMETERS` and 1 to 500 if unit is `DISTANCE_UNIT_MILES`.
    #[serde(rename="proximityRadiusAmount")]
    
    pub proximity_radius_amount: Option<f64>,
    /// Required. The unit of distance by which the targeting radius is measured.
    #[serde(rename="proximityRadiusUnit")]
    
    pub proximity_radius_unit: Option<String>,
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_POI`. Accepted POI targeting option IDs can be retrieved using SearchTargetingOptions. If targeting a specific latitude/longitude coordinate removed from an address or POI name, you can generate the necessary targeting option ID by rounding the desired coordinate values to the 6th decimal place, removing the decimals, and concatenating the string values separated by a semicolon. For example, you can target the latitude/longitude pair of 40.7414691, -74.003387 using the targeting option ID "40741469;-74003387".
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for PoiAssignedTargetingOptionDetails {}


/// Search terms for POI targeting options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PoiSearchTerms {
    /// The search query for the desired POI name, street address, or coordinate of the desired POI. The query can be a prefix, e.g. "Times squar", "40.7505045,-73.99562", "315 W 44th St", etc.
    #[serde(rename="poiQuery")]
    
    pub poi_query: Option<String>,
}

impl client::Part for PoiSearchTerms {}


/// Represents a targetable point of interest(POI). This will be populated in the poi_details field when targeting_type is `TARGETING_TYPE_POI`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PoiTargetingOptionDetails {
    /// Output only. The display name of a POI(e.g. "Times Square", "Space Needle"), followed by its full address if available.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Latitude of the POI rounding to 6th decimal place.
    
    pub latitude: Option<f64>,
    /// Output only. Longitude of the POI rounding to 6th decimal place.
    
    pub longitude: Option<f64>,
}

impl client::Part for PoiTargetingOptionDetails {}


/// Settings specific to the Mediaocean Prisma tool.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrismaConfig {
    /// Required. Relevant client, product, and estimate codes from the Mediaocean Prisma tool.
    #[serde(rename="prismaCpeCode")]
    
    pub prisma_cpe_code: Option<PrismaCpeCode>,
    /// Required. The Prisma type.
    #[serde(rename="prismaType")]
    
    pub prisma_type: Option<String>,
    /// Required. The entity allocated this budget (DSP, site, etc.).
    
    pub supplier: Option<String>,
}

impl client::Part for PrismaConfig {}


/// Google Payments Center supports searching and filtering on the component fields of this code.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrismaCpeCode {
    /// The Prisma client code.
    #[serde(rename="prismaClientCode")]
    
    pub prisma_client_code: Option<String>,
    /// The Prisma estimate code.
    #[serde(rename="prismaEstimateCode")]
    
    pub prisma_estimate_code: Option<String>,
    /// The Prisma product code.
    #[serde(rename="prismaProductCode")]
    
    pub prisma_product_code: Option<String>,
}

impl client::Part for PrismaCpeCode {}


/// Targeting details for proximity location list. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_PROXIMITY_LOCATION_LIST`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProximityLocationListAssignedTargetingOptionDetails {
    /// Required. ID of the proximity location list. Should refer to the location_list_id field of a LocationList resource whose type is `TARGETING_LOCATION_TYPE_PROXIMITY`.
    #[serde(rename="proximityLocationListId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub proximity_location_list_id: Option<i64>,
    /// Required. Radius range for proximity location list. This represents the size of the area around a chosen location that will be targeted. `All` proximity location targeting under a single resource must have the same radius range value. Set this value to match any existing targeting. If updated, this field will change the radius range for all proximity targeting under the resource.
    #[serde(rename="proximityRadiusRange")]
    
    pub proximity_radius_range: Option<String>,
}

impl client::Part for ProximityLocationListAssignedTargetingOptionDetails {}


/// Publisher review status for the creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublisherReviewStatus {
    /// The publisher reviewing the creative.
    #[serde(rename="publisherName")]
    
    pub publisher_name: Option<String>,
    /// Status of the publisher review.
    
    pub status: Option<String>,
}

impl client::Part for PublisherReviewStatus {}


/// The rate related settings of the inventory source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RateDetails {
    /// The rate type. Acceptable values are `INVENTORY_SOURCE_RATE_TYPE_CPM_FIXED`, `INVENTORY_SOURCE_RATE_TYPE_CPM_FLOOR`, and `INVENTORY_SOURCE_RATE_TYPE_CPD`.
    #[serde(rename="inventorySourceRateType")]
    
    pub inventory_source_rate_type: Option<String>,
    /// Output only. The amount that the buyer has committed to spending on the inventory source up front. Only applicable for guaranteed inventory sources.
    #[serde(rename="minimumSpend")]
    
    pub minimum_spend: Option<Money>,
    /// The rate for the inventory source.
    
    pub rate: Option<Money>,
    /// Required for guaranteed inventory sources. The number of impressions guaranteed by the seller.
    #[serde(rename="unitsPurchased")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub units_purchased: Option<i64>,
}

impl client::Part for RateDetails {}


/// Targeting details for regional location list. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_REGIONAL_LOCATION_LIST`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegionalLocationListAssignedTargetingOptionDetails {
    /// Indicates if this option is being negatively targeted.
    
    pub negative: Option<bool>,
    /// Required. ID of the regional location list. Should refer to the location_list_id field of a LocationList resource whose type is `TARGETING_LOCATION_TYPE_REGIONAL`.
    #[serde(rename="regionalLocationListId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub regional_location_list_id: Option<i64>,
}

impl client::Part for RegionalLocationListAssignedTargetingOptionDetails {}


/// Request message for NegativeKeywordService.ReplaceNegativeKeywords.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [negative keyword lists negative keywords replace advertisers](AdvertiserNegativeKeywordListNegativeKeywordReplaceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceNegativeKeywordsRequest {
    /// The negative keywords that will replace the existing keywords in the negative keyword list, specified as a list of NegativeKeywords.
    #[serde(rename="newNegativeKeywords")]
    
    pub new_negative_keywords: Option<Vec<NegativeKeyword>>,
}

impl client::RequestValue for ReplaceNegativeKeywordsRequest {}


/// Response message for NegativeKeywordService.ReplaceNegativeKeywords.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [negative keyword lists negative keywords replace advertisers](AdvertiserNegativeKeywordListNegativeKeywordReplaceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceNegativeKeywordsResponse {
    /// The full list of negative keywords now present in the negative keyword list.
    #[serde(rename="negativeKeywords")]
    
    pub negative_keywords: Option<Vec<NegativeKeyword>>,
}

impl client::ResponseResult for ReplaceNegativeKeywordsResponse {}


/// Request message for SiteService.ReplaceSites.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channels sites replace advertisers](AdvertiserChannelSiteReplaceCall) (request)
/// * [channels sites replace partners](PartnerChannelSiteReplaceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceSitesRequest {
    /// The ID of the advertiser that owns the parent channel.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// The sites that will replace the existing sites assigned to the channel, specified as a list of Sites.
    #[serde(rename="newSites")]
    
    pub new_sites: Option<Vec<Site>>,
    /// The ID of the partner that owns the parent channel.
    #[serde(rename="partnerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partner_id: Option<i64>,
}

impl client::RequestValue for ReplaceSitesRequest {}


/// Response message for SiteService.ReplaceSites.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channels sites replace advertisers](AdvertiserChannelSiteReplaceCall) (response)
/// * [channels sites replace partners](PartnerChannelSiteReplaceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceSitesResponse {
    /// The list of sites in the channel after replacing.
    
    pub sites: Option<Vec<Site>>,
}

impl client::ResponseResult for ReplaceSitesResponse {}


/// Review statuses for the creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReviewStatusInfo {
    /// Represents the basic approval needed for a creative to begin serving. Summary of creative_and_landing_page_review_status and content_and_policy_review_status.
    #[serde(rename="approvalStatus")]
    
    pub approval_status: Option<String>,
    /// Content and policy review status for the creative.
    #[serde(rename="contentAndPolicyReviewStatus")]
    
    pub content_and_policy_review_status: Option<String>,
    /// Creative and landing page review status for the creative.
    #[serde(rename="creativeAndLandingPageReviewStatus")]
    
    pub creative_and_landing_page_review_status: Option<String>,
    /// Exchange review statuses for the creative.
    #[serde(rename="exchangeReviewStatuses")]
    
    pub exchange_review_statuses: Option<Vec<ExchangeReviewStatus>>,
    /// Publisher review statuses for the creative.
    #[serde(rename="publisherReviewStatuses")]
    
    pub publisher_review_statuses: Option<Vec<PublisherReviewStatus>>,
}

impl client::Part for ReviewStatusInfo {}


/// An error message for a custom bidding script.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScriptError {
    /// The column number in the script where the error was thrown.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub column: Option<i64>,
    /// The type of error.
    #[serde(rename="errorCode")]
    
    pub error_code: Option<String>,
    /// The detailed error message.
    #[serde(rename="errorMessage")]
    
    pub error_message: Option<String>,
    /// The line number in the script where the error was thrown.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub line: Option<i64>,
}

impl client::Part for ScriptError {}


/// Structured Data File (SDF) related settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SdfConfig {
    /// An administrator email address to which the SDF processing status reports will be sent.
    #[serde(rename="adminEmail")]
    
    pub admin_email: Option<String>,
    /// Required. The version of SDF being used.
    
    pub version: Option<String>,
}

impl client::Part for SdfConfig {}


/// Request message for SearchTargetingOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [targeting options search targeting types](TargetingTypeTargetingOptionSearchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchTargetingOptionsRequest {
    /// Required. The Advertiser this request is being made in the context of.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Search terms for Business Chain targeting options. Can only be used when targeting_type is `TARGETING_TYPE_BUSINESS_CHAIN`.
    #[serde(rename="businessChainSearchTerms")]
    
    pub business_chain_search_terms: Option<BusinessChainSearchTerms>,
    /// Search terms for geo region targeting options. Can only be used when targeting_type is `TARGETING_TYPE_GEO_REGION`.
    #[serde(rename="geoRegionSearchTerms")]
    
    pub geo_region_search_terms: Option<GeoRegionSearchTerms>,
    /// Requested page size. Must be between `1` and `200`. If unspecified will default to `100`. Returns error code `INVALID_ARGUMENT` if an invalid value is specified.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// A token identifying a page of results the server should return. Typically, this is the value of next_page_token returned from the previous call to `SearchTargetingOptions` method. If not specified, the first page of results will be returned.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Search terms for POI targeting options. Can only be used when targeting_type is `TARGETING_TYPE_POI`.
    #[serde(rename="poiSearchTerms")]
    
    pub poi_search_terms: Option<PoiSearchTerms>,
}

impl client::RequestValue for SearchTargetingOptionsRequest {}


/// Response message for SearchTargetingOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [targeting options search targeting types](TargetingTypeTargetingOptionSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchTargetingOptionsResponse {
    /// A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `SearchTargetingOptions` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of targeting options that match the search criteria. This list will be absent if empty.
    #[serde(rename="targetingOptions")]
    
    pub targeting_options: Option<Vec<TargetingOption>>,
}

impl client::ResponseResult for SearchTargetingOptionsResponse {}


/// Targeting details for sensitive category. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SensitiveCategoryAssignedTargetingOptionDetails {
    /// Required. ID of the sensitive category to be EXCLUDED.
    #[serde(rename="excludedTargetingOptionId")]
    
    pub excluded_targeting_option_id: Option<String>,
    /// Output only. An enum for the DV360 Sensitive category content classifier.
    #[serde(rename="sensitiveCategory")]
    
    pub sensitive_category: Option<String>,
}

impl client::Part for SensitiveCategoryAssignedTargetingOptionDetails {}


/// Represents a targetable sensitive category. This will be populated in the sensitive_category_details field of the TargetingOption when targeting_type is `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SensitiveCategoryTargetingOptionDetails {
    /// Output only. An enum for the DV360 Sensitive category content classifier.
    #[serde(rename="sensitiveCategory")]
    
    pub sensitive_category: Option<String>,
}

impl client::Part for SensitiveCategoryTargetingOptionDetails {}


/// A single site. Sites are apps or websites belonging to a channel.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channels sites create advertisers](AdvertiserChannelSiteCreateCall) (request|response)
/// * [channels sites create partners](PartnerChannelSiteCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Site {
    /// Output only. The resource name of the site.
    
    pub name: Option<String>,
    /// Required. The app ID or URL of the site. Must be UTF-8 encoded with a maximum length of 240 bytes.
    #[serde(rename="urlOrAppId")]
    
    pub url_or_app_id: Option<String>,
}

impl client::RequestValue for Site {}
impl client::ResponseResult for Site {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for Status {}


/// Details for assigned sub-exchange targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_SUB_EXCHANGE`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubExchangeAssignedTargetingOptionDetails {
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_SUB_EXCHANGE`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
}

impl client::Part for SubExchangeAssignedTargetingOptionDetails {}


/// Represents a targetable sub-exchange. This will be populated in the sub_exchange_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_SUB_EXCHANGE`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubExchangeTargetingOptionDetails {
    /// Output only. The display name of the sub-exchange.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for SubExchangeTargetingOptionDetails {}


/// Settings that control the targeting expansion of the line item. Targeting expansion allows the line item to reach a larger audience based on the original audience list and the targeting expansion level. Beginning November 7, 2022, these settings may represent the [optimized targeting feature](https://developers.google.com//support.google.com/displayvideo/answer/12060859) in place of targeting expansion. This feature will be rolled out to all partners by November 9, 2022.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetingExpansionConfig {
    /// Required. Whether to exclude first-party audiences from use in targeting expansion or optimized targeting. Similar audiences of the excluded first-party lists will not be excluded. Only applicable when a first-party audience is positively targeted (directly or included in a combined audience), otherwise this selection will be ignored.
    #[serde(rename="excludeFirstPartyAudience")]
    
    pub exclude_first_party_audience: Option<bool>,
    /// Required. Magnitude of expansion for applicable targeting under this line item. Beginning November 7, 2022, the behavior of this field will change in the following ways with the replacement of targeting expansion with [optimized targeting](https://developers.google.com//support.google.com/displayvideo/answer/12060859): * This field will represent the optimized targeting checkbox, with a `NO_EXPANSION` value representing optimized targeting turned off and a `LEAST_EXPANSION` value representing optimized targeting turned on. * `NO_EXPANSION` will be the default value for the field and will be automatically assigned if you do not set the field. * If you set the field to any value other than `NO_EXPANSION`, it will automatically be set to `LEAST_EXPANSION`.
    #[serde(rename="targetingExpansionLevel")]
    
    pub targeting_expansion_level: Option<String>,
}

impl client::Part for TargetingExpansionConfig {}


/// Represents a single targeting option, which is a targetable concept in DV360.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [targeting options get targeting types](TargetingTypeTargetingOptionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetingOption {
    /// Age range details.
    #[serde(rename="ageRangeDetails")]
    
    pub age_range_details: Option<AgeRangeTargetingOptionDetails>,
    /// App category details.
    #[serde(rename="appCategoryDetails")]
    
    pub app_category_details: Option<AppCategoryTargetingOptionDetails>,
    /// Audio content type details.
    #[serde(rename="audioContentTypeDetails")]
    
    pub audio_content_type_details: Option<AudioContentTypeTargetingOptionDetails>,
    /// Authorized seller status resource details.
    #[serde(rename="authorizedSellerStatusDetails")]
    
    pub authorized_seller_status_details: Option<AuthorizedSellerStatusTargetingOptionDetails>,
    /// Browser details.
    #[serde(rename="browserDetails")]
    
    pub browser_details: Option<BrowserTargetingOptionDetails>,
    /// Business chain resource details.
    #[serde(rename="businessChainDetails")]
    
    pub business_chain_details: Option<BusinessChainTargetingOptionDetails>,
    /// Carrier and ISP details.
    #[serde(rename="carrierAndIspDetails")]
    
    pub carrier_and_isp_details: Option<CarrierAndIspTargetingOptionDetails>,
    /// Category resource details.
    #[serde(rename="categoryDetails")]
    
    pub category_details: Option<CategoryTargetingOptionDetails>,
    /// Content duration resource details.
    #[serde(rename="contentDurationDetails")]
    
    pub content_duration_details: Option<ContentDurationTargetingOptionDetails>,
    /// Content genre resource details.
    #[serde(rename="contentGenreDetails")]
    
    pub content_genre_details: Option<ContentGenreTargetingOptionDetails>,
    /// Content instream position details.
    #[serde(rename="contentInstreamPositionDetails")]
    
    pub content_instream_position_details: Option<ContentInstreamPositionTargetingOptionDetails>,
    /// Content outstream position details.
    #[serde(rename="contentOutstreamPositionDetails")]
    
    pub content_outstream_position_details: Option<ContentOutstreamPositionTargetingOptionDetails>,
    /// Content stream type resource details.
    #[serde(rename="contentStreamTypeDetails")]
    
    pub content_stream_type_details: Option<ContentStreamTypeTargetingOptionDetails>,
    /// Device make and model resource details.
    #[serde(rename="deviceMakeModelDetails")]
    
    pub device_make_model_details: Option<DeviceMakeModelTargetingOptionDetails>,
    /// Device type details.
    #[serde(rename="deviceTypeDetails")]
    
    pub device_type_details: Option<DeviceTypeTargetingOptionDetails>,
    /// Digital content label details.
    #[serde(rename="digitalContentLabelDetails")]
    
    pub digital_content_label_details: Option<DigitalContentLabelTargetingOptionDetails>,
    /// Environment details.
    #[serde(rename="environmentDetails")]
    
    pub environment_details: Option<EnvironmentTargetingOptionDetails>,
    /// Exchange details.
    #[serde(rename="exchangeDetails")]
    
    pub exchange_details: Option<ExchangeTargetingOptionDetails>,
    /// Gender details.
    #[serde(rename="genderDetails")]
    
    pub gender_details: Option<GenderTargetingOptionDetails>,
    /// Geographic region resource details.
    #[serde(rename="geoRegionDetails")]
    
    pub geo_region_details: Option<GeoRegionTargetingOptionDetails>,
    /// Household income details.
    #[serde(rename="householdIncomeDetails")]
    
    pub household_income_details: Option<HouseholdIncomeTargetingOptionDetails>,
    /// Language resource details.
    #[serde(rename="languageDetails")]
    
    pub language_details: Option<LanguageTargetingOptionDetails>,
    /// Output only. The resource name for this targeting option.
    
    pub name: Option<String>,
    /// Native content position details.
    #[serde(rename="nativeContentPositionDetails")]
    
    pub native_content_position_details: Option<NativeContentPositionTargetingOptionDetails>,
    /// Open Measurement enabled inventory details.
    #[serde(rename="omidDetails")]
    
    pub omid_details: Option<OmidTargetingOptionDetails>,
    /// On screen position details.
    #[serde(rename="onScreenPositionDetails")]
    
    pub on_screen_position_details: Option<OnScreenPositionTargetingOptionDetails>,
    /// Operating system resources details.
    #[serde(rename="operatingSystemDetails")]
    
    pub operating_system_details: Option<OperatingSystemTargetingOptionDetails>,
    /// Parental status details.
    #[serde(rename="parentalStatusDetails")]
    
    pub parental_status_details: Option<ParentalStatusTargetingOptionDetails>,
    /// POI resource details.
    #[serde(rename="poiDetails")]
    
    pub poi_details: Option<PoiTargetingOptionDetails>,
    /// Sensitive Category details.
    #[serde(rename="sensitiveCategoryDetails")]
    
    pub sensitive_category_details: Option<SensitiveCategoryTargetingOptionDetails>,
    /// Sub-exchange details.
    #[serde(rename="subExchangeDetails")]
    
    pub sub_exchange_details: Option<SubExchangeTargetingOptionDetails>,
    /// Output only. A unique identifier for this targeting option. The tuple {`targeting_type`, `targeting_option_id`} will be unique.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
    /// Output only. The type of this targeting option.
    #[serde(rename="targetingType")]
    
    pub targeting_type: Option<String>,
    /// User rewarded content details.
    #[serde(rename="userRewardedContentDetails")]
    
    pub user_rewarded_content_details: Option<UserRewardedContentTargetingOptionDetails>,
    /// Video player size details.
    #[serde(rename="videoPlayerSizeDetails")]
    
    pub video_player_size_details: Option<VideoPlayerSizeTargetingOptionDetails>,
    /// Viewability resource details.
    #[serde(rename="viewabilityDetails")]
    
    pub viewability_details: Option<ViewabilityTargetingOptionDetails>,
}

impl client::ResponseResult for TargetingOption {}


/// Settings for advertisers that use third-party ad servers only.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThirdPartyOnlyConfig {
    /// Whether or not order ID reporting for pixels is enabled. This value cannot be changed once set to `true`.
    #[serde(rename="pixelOrderIdReportingEnabled")]
    
    pub pixel_order_id_reporting_enabled: Option<bool>,
}

impl client::Part for ThirdPartyOnlyConfig {}


/// Tracking URLs from third parties to track interactions with an audio or a video creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThirdPartyUrl {
    /// The type of interaction needs to be tracked by the tracking URL
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Tracking URL used to track the interaction. Provide a URL with optional path or query string, beginning with `https:`. For example, https://www.example.com/path
    
    pub url: Option<String>,
}

impl client::Part for ThirdPartyUrl {}


/// Assigned third party verifier targeting option details. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_THIRD_PARTY_VERIFIER`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThirdPartyVerifierAssignedTargetingOptionDetails {
    /// Third party brand verifier -- Adloox.
    
    pub adloox: Option<Adloox>,
    /// Third party brand verifier -- DoubleVerify.
    #[serde(rename="doubleVerify")]
    
    pub double_verify: Option<DoubleVerify>,
    /// Third party brand verifier -- Integral Ad Science.
    #[serde(rename="integralAdScience")]
    
    pub integral_ad_science: Option<IntegralAdScience>,
}

impl client::Part for ThirdPartyVerifierAssignedTargetingOptionDetails {}


/// A time range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeRange {
    /// Required. The upper bound of a time range, inclusive.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The lower bound of a time range, inclusive.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TimeRange {}


/// Timer event of the creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimerEvent {
    /// Required. The name of the timer event.
    
    pub name: Option<String>,
    /// Required. The name used to identify this timer event in reports.
    #[serde(rename="reportingName")]
    
    pub reporting_name: Option<String>,
}

impl client::Part for TimerEvent {}


/// Settings that control the behavior of a single Floodlight activity config.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrackingFloodlightActivityConfig {
    /// Required. The ID of the Floodlight activity.
    #[serde(rename="floodlightActivityId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub floodlight_activity_id: Option<i64>,
    /// Required. The number of days after an ad has been clicked in which a conversion may be counted. Must be between 0 and 90 inclusive.
    #[serde(rename="postClickLookbackWindowDays")]
    
    pub post_click_lookback_window_days: Option<i32>,
    /// Required. The number of days after an ad has been viewed in which a conversion may be counted. Must be between 0 and 90 inclusive.
    #[serde(rename="postViewLookbackWindowDays")]
    
    pub post_view_lookback_window_days: Option<i32>,
}

impl client::Part for TrackingFloodlightActivityConfig {}


/// Represents information about the transcoded audio or video file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Transcode {
    /// The bit rate for the audio stream of the transcoded video, or the bit rate for the transcoded audio, in kilobits per second.
    #[serde(rename="audioBitRateKbps")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub audio_bit_rate_kbps: Option<i64>,
    /// The sample rate for the audio stream of the transcoded video, or the sample rate for the transcoded audio, in hertz.
    #[serde(rename="audioSampleRateHz")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub audio_sample_rate_hz: Option<i64>,
    /// The transcoding bit rate of the transcoded video, in kilobits per second.
    #[serde(rename="bitRateKbps")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bit_rate_kbps: Option<i64>,
    /// The dimensions of the transcoded video.
    
    pub dimensions: Option<Dimensions>,
    /// The size of the transcoded file, in bytes.
    #[serde(rename="fileSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub file_size_bytes: Option<i64>,
    /// The frame rate of the transcoded video, in frames per second.
    #[serde(rename="frameRate")]
    
    pub frame_rate: Option<f32>,
    /// The MIME type of the transcoded file.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// The name of the transcoded file.
    
    pub name: Option<String>,
    /// Indicates if the transcoding was successful.
    
    pub transcoded: Option<bool>,
}

impl client::Part for Transcode {}


/// A creative identifier provided by a registry that is unique across all platforms. This is part of the VAST 4.0 standard.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UniversalAdId {
    /// The unique creative identifier.
    
    pub id: Option<String>,
    /// The registry provides unique creative identifiers.
    
    pub registry: Option<String>,
}

impl client::Part for UniversalAdId {}


/// Details for assigned URL targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_URL`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlAssignedTargetingOptionDetails {
    /// Indicates if this option is being negatively targeted.
    
    pub negative: Option<bool>,
    /// Required. The URL, for example `example.com`. DV360 supports two levels of subdirectory targeting, for example `www.example.com/one-subdirectory-level/second-level`, and five levels of subdomain targeting, for example `five.four.three.two.one.example.com`.
    
    pub url: Option<String>,
}

impl client::Part for UrlAssignedTargetingOptionDetails {}


/// A single user in Display & Video 360.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [bulk edit assigned user roles users](UserBulkEditAssignedUserRoleCall) (none)
/// * [create users](UserCreateCall) (request|response)
/// * [delete users](UserDeleteCall) (none)
/// * [get users](UserGetCall) (response)
/// * [list users](UserListCall) (none)
/// * [patch users](UserPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// The assigned user roles. Required in CreateUser. Output only in UpdateUser. Can only be updated through BulkEditAssignedUserRoles.
    #[serde(rename="assignedUserRoles")]
    
    pub assigned_user_roles: Option<Vec<AssignedUserRole>>,
    /// Required. The display name of the user. Must be UTF-8 encoded with a maximum size of 240 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. Immutable. The email address used to identify the user.
    
    pub email: Option<String>,
    /// Output only. The resource name of the user.
    
    pub name: Option<String>,
    /// Output only. The unique ID of the user. Assigned by the system.
    #[serde(rename="userId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub user_id: Option<i64>,
}

impl client::RequestValue for User {}
impl client::Resource for User {}
impl client::ResponseResult for User {}


/// User rewarded content targeting option details. This will be populated in the user_rewarded_content_details field when targeting_type is `TARGETING_TYPE_USER_REWARDED_CONTENT`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserRewardedContentAssignedTargetingOptionDetails {
    /// Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_USER_REWARDED_CONTENT`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
    /// Output only. User rewarded content status for video ads.
    #[serde(rename="userRewardedContent")]
    
    pub user_rewarded_content: Option<String>,
}

impl client::Part for UserRewardedContentAssignedTargetingOptionDetails {}


/// Represents a targetable user rewarded content status for video ads only. This will be populated in the user_rewarded_content_details field when targeting_type is `TARGETING_TYPE_USER_REWARDED_CONTENT`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserRewardedContentTargetingOptionDetails {
    /// Output only. User rewarded content status for video ads.
    #[serde(rename="userRewardedContent")]
    
    pub user_rewarded_content: Option<String>,
}

impl client::Part for UserRewardedContentTargetingOptionDetails {}


/// Video player size targeting option details. This will be populated in the video_player_size_details field when targeting_type is `TARGETING_TYPE_VIDEO_PLAYER_SIZE`. Explicitly targeting all options is not supported. Remove all video player size targeting options to achieve this effect.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoPlayerSizeAssignedTargetingOptionDetails {
    /// Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_VIDEO_PLAYER_SIZE`.
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
    /// The video player size. Output only in v1. Required in v2.
    #[serde(rename="videoPlayerSize")]
    
    pub video_player_size: Option<String>,
}

impl client::Part for VideoPlayerSizeAssignedTargetingOptionDetails {}


/// Represents a targetable video player size. This will be populated in the video_player_size_details field when targeting_type is `TARGETING_TYPE_VIDEO_PLAYER_SIZE`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoPlayerSizeTargetingOptionDetails {
    /// Output only. The video player size.
    #[serde(rename="videoPlayerSize")]
    
    pub video_player_size: Option<String>,
}

impl client::Part for VideoPlayerSizeTargetingOptionDetails {}


/// Assigned viewability targeting option details. This will be populated in the viewability_details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_VIEWABILITY`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ViewabilityAssignedTargetingOptionDetails {
    /// Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_VIEWABILITY` (e.g., "509010" for targeting the `VIEWABILITY_10_PERCENT_OR_MORE` option).
    #[serde(rename="targetingOptionId")]
    
    pub targeting_option_id: Option<String>,
    /// The predicted viewability percentage. Output only in v1. Required in v2.
    
    pub viewability: Option<String>,
}

impl client::Part for ViewabilityAssignedTargetingOptionDetails {}


/// Represents a targetable viewability. This will be populated in the viewability_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_VIEWABILITY`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ViewabilityTargetingOptionDetails {
    /// Output only. The predicted viewability percentage.
    
    pub viewability: Option<String>,
}

impl client::Part for ViewabilityTargetingOptionDetails {}


