use super::*;
/// Contains properties of a Campaign Manager account.
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
    /// Account permissions assigned to this account.
    #[serde(rename="accountPermissionIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub account_permission_ids: Option<Vec<i64>>,
    /// Profile for this account. This is a read-only field that can be left blank.
    #[serde(rename="accountProfile")]
    
    pub account_profile: Option<AccountAccountProfileEnum>,
    /// Whether this account is active.
    
    pub active: Option<bool>,
    /// Maximum number of active ads allowed for this account.
    #[serde(rename="activeAdsLimitTier")]
    
    pub active_ads_limit_tier: Option<AccountActiveAdsLimitTierEnum>,
    /// Whether to serve creatives with Active View tags. If disabled, viewability data will not be available for any impressions.
    #[serde(rename="activeViewOptOut")]
    
    pub active_view_opt_out: Option<bool>,
    /// User role permissions available to the user roles of this account.
    #[serde(rename="availablePermissionIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub available_permission_ids: Option<Vec<i64>>,
    /// ID of the country associated with this account.
    #[serde(rename="countryId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub country_id: Option<i64>,
    /// ID of currency associated with this account. This is a required field.
    /// Acceptable values are: 
    /// - "1" for USD 
    /// - "2" for GBP 
    /// - "3" for ESP 
    /// - "4" for SEK 
    /// - "5" for CAD 
    /// - "6" for JPY 
    /// - "7" for DEM 
    /// - "8" for AUD 
    /// - "9" for FRF 
    /// - "10" for ITL 
    /// - "11" for DKK 
    /// - "12" for NOK 
    /// - "13" for FIM 
    /// - "14" for ZAR 
    /// - "15" for IEP 
    /// - "16" for NLG 
    /// - "17" for EUR 
    /// - "18" for KRW 
    /// - "19" for TWD 
    /// - "20" for SGD 
    /// - "21" for CNY 
    /// - "22" for HKD 
    /// - "23" for NZD 
    /// - "24" for MYR 
    /// - "25" for BRL 
    /// - "26" for PTE 
    /// - "27" for MXP 
    /// - "28" for CLP 
    /// - "29" for TRY 
    /// - "30" for ARS 
    /// - "31" for PEN 
    /// - "32" for ILS 
    /// - "33" for CHF 
    /// - "34" for VEF 
    /// - "35" for COP 
    /// - "36" for GTQ 
    /// - "37" for PLN 
    /// - "39" for INR 
    /// - "40" for THB 
    /// - "41" for IDR 
    /// - "42" for CZK 
    /// - "43" for RON 
    /// - "44" for HUF 
    /// - "45" for RUB 
    /// - "46" for AED 
    /// - "47" for BGN 
    /// - "48" for HRK 
    /// - "49" for MXN 
    /// - "50" for NGN
    #[serde(rename="currencyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub currency_id: Option<i64>,
    /// Default placement dimensions for this account.
    #[serde(rename="defaultCreativeSizeId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub default_creative_size_id: Option<i64>,
    /// Description of this account.
    
    pub description: Option<String>,
    /// ID of this account. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#account".
    
    pub kind: Option<String>,
    /// Locale of this account.
    /// Acceptable values are: 
    /// - "cs" (Czech) 
    /// - "de" (German) 
    /// - "en" (English) 
    /// - "en-GB" (English United Kingdom) 
    /// - "es" (Spanish) 
    /// - "fr" (French) 
    /// - "it" (Italian) 
    /// - "ja" (Japanese) 
    /// - "ko" (Korean) 
    /// - "pl" (Polish) 
    /// - "pt-BR" (Portuguese Brazil) 
    /// - "ru" (Russian) 
    /// - "sv" (Swedish) 
    /// - "tr" (Turkish) 
    /// - "zh-CN" (Chinese Simplified) 
    /// - "zh-TW" (Chinese Traditional)
    
    pub locale: Option<String>,
    /// Maximum image size allowed for this account, in kilobytes. Value must be greater than or equal to 1.
    #[serde(rename="maximumImageSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub maximum_image_size: Option<i64>,
    /// Name of this account. This is a required field, and must be less than 128 characters long and be globally unique.
    
    pub name: Option<String>,
    /// Whether campaigns created in this account will be enabled for Nielsen OCR reach ratings by default.
    #[serde(rename="nielsenOcrEnabled")]
    
    pub nielsen_ocr_enabled: Option<bool>,
    /// Reporting configuration of this account.
    #[serde(rename="reportsConfiguration")]
    
    pub reports_configuration: Option<ReportsConfiguration>,
    /// Share Path to Conversion reports with Twitter.
    #[serde(rename="shareReportsWithTwitter")]
    
    pub share_reports_with_twitter: Option<bool>,
    /// File size limit in kilobytes of Rich Media teaser creatives. Acceptable values are 1 to 10240, inclusive.
    #[serde(rename="teaserSizeLimit")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub teaser_size_limit: Option<i64>,
}

impl client::RequestValue for Account {}
impl client::Resource for Account {}
impl client::ResponseResult for Account {}


/// Gets a summary of active ads in an account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get account active ad summaries](AccountActiveAdSummaryGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountActiveAdSummary {
    /// ID of the account.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Ads that have been activated for the account
    #[serde(rename="activeAds")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub active_ads: Option<i64>,
    /// Maximum number of active ads allowed for the account.
    #[serde(rename="activeAdsLimitTier")]
    
    pub active_ads_limit_tier: Option<AccountActiveAdSummaryActiveAdsLimitTierEnum>,
    /// Ads that can be activated for the account.
    #[serde(rename="availableAds")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub available_ads: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#accountActiveAdSummary".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for AccountActiveAdSummary {}


/// AccountPermissions contains information about a particular account permission. Some features of Campaign Manager require an account permission to be present in the account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get account permissions](AccountPermissionGetCall) (response)
/// * [list account permissions](AccountPermissionListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountPermission {
    /// Account profiles associated with this account permission.
    /// 
    /// Possible values are:
    /// - "ACCOUNT_PROFILE_BASIC"
    /// - "ACCOUNT_PROFILE_STANDARD"
    #[serde(rename="accountProfiles")]
    
    pub account_profiles: Option<Vec<AccountPermissionAccountProfilesEnum>>,
    /// ID of this account permission.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#accountPermission".
    
    pub kind: Option<String>,
    /// Administrative level required to enable this account permission.
    
    pub level: Option<AccountPermissionLevelEnum>,
    /// Name of this account permission.
    
    pub name: Option<String>,
    /// Permission group of this account permission.
    #[serde(rename="permissionGroupId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub permission_group_id: Option<i64>,
}

impl client::Resource for AccountPermission {}
impl client::ResponseResult for AccountPermission {}


/// AccountPermissionGroups contains a mapping of permission group IDs to names. A permission group is a grouping of account permissions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get account permission groups](AccountPermissionGroupGetCall) (response)
/// * [list account permission groups](AccountPermissionGroupListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountPermissionGroup {
    /// ID of this account permission group.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#accountPermissionGroup".
    
    pub kind: Option<String>,
    /// Name of this account permission group.
    
    pub name: Option<String>,
}

impl client::Resource for AccountPermissionGroup {}
impl client::ResponseResult for AccountPermissionGroup {}


/// Account Permission Group List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list account permission groups](AccountPermissionGroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountPermissionGroupsListResponse {
    /// Account permission group collection.
    #[serde(rename="accountPermissionGroups")]
    
    pub account_permission_groups: Option<Vec<AccountPermissionGroup>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#accountPermissionGroupsListResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for AccountPermissionGroupsListResponse {}


/// Account Permission List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list account permissions](AccountPermissionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountPermissionsListResponse {
    /// Account permission collection.
    #[serde(rename="accountPermissions")]
    
    pub account_permissions: Option<Vec<AccountPermission>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#accountPermissionsListResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for AccountPermissionsListResponse {}


/// AccountUserProfiles contains properties of a Campaign Manager user profile. This resource is specifically for managing user profiles, whereas UserProfiles is for accessing the API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get account user profiles](AccountUserProfileGetCall) (response)
/// * [insert account user profiles](AccountUserProfileInsertCall) (request|response)
/// * [list account user profiles](AccountUserProfileListCall) (none)
/// * [patch account user profiles](AccountUserProfilePatchCall) (request|response)
/// * [update account user profiles](AccountUserProfileUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountUserProfile {
    /// Account ID of the user profile. This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Whether this user profile is active. This defaults to false, and must be set true on insert for the user profile to be usable.
    
    pub active: Option<bool>,
    /// Filter that describes which advertisers are visible to the user profile.
    #[serde(rename="advertiserFilter")]
    
    pub advertiser_filter: Option<ObjectFilter>,
    /// Filter that describes which campaigns are visible to the user profile.
    #[serde(rename="campaignFilter")]
    
    pub campaign_filter: Option<ObjectFilter>,
    /// Comments for this user profile.
    
    pub comments: Option<String>,
    /// Email of the user profile. The email addresss must be linked to a Google Account. This field is required on insertion and is read-only after insertion.
    
    pub email: Option<String>,
    /// ID of the user profile. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#accountUserProfile".
    
    pub kind: Option<String>,
    /// Locale of the user profile. This is a required field.
    /// Acceptable values are:  
    /// - "cs" (Czech) 
    /// - "de" (German) 
    /// - "en" (English) 
    /// - "en-GB" (English United Kingdom) 
    /// - "es" (Spanish) 
    /// - "fr" (French) 
    /// - "it" (Italian) 
    /// - "ja" (Japanese) 
    /// - "ko" (Korean) 
    /// - "pl" (Polish) 
    /// - "pt-BR" (Portuguese Brazil)
    /// - "ru" (Russian) 
    /// - "sv" (Swedish) 
    /// - "tr" (Turkish) 
    /// - "zh-CN" (Chinese Simplified) 
    /// - "zh-TW" (Chinese Traditional)
    
    pub locale: Option<String>,
    /// Name of the user profile. This is a required field. Must be less than 64 characters long, must be globally unique, and cannot contain whitespace or any of the following characters: "&;"#%,".
    
    pub name: Option<String>,
    /// Filter that describes which sites are visible to the user profile.
    #[serde(rename="siteFilter")]
    
    pub site_filter: Option<ObjectFilter>,
    /// Subaccount ID of the user profile. This is a read-only field that can be left blank.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Trafficker type of this user profile. This is a read-only field.
    #[serde(rename="traffickerType")]
    
    pub trafficker_type: Option<AccountUserProfileTraffickerTypeEnum>,
    /// User type of the user profile. This is a read-only field that can be left blank.
    #[serde(rename="userAccessType")]
    
    pub user_access_type: Option<AccountUserProfileUserAccessTypeEnum>,
    /// Filter that describes which user roles are visible to the user profile.
    #[serde(rename="userRoleFilter")]
    
    pub user_role_filter: Option<ObjectFilter>,
    /// User role ID of the user profile. This is a required field.
    #[serde(rename="userRoleId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub user_role_id: Option<i64>,
}

impl client::RequestValue for AccountUserProfile {}
impl client::Resource for AccountUserProfile {}
impl client::ResponseResult for AccountUserProfile {}


/// Account User Profile List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list account user profiles](AccountUserProfileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountUserProfilesListResponse {
    /// Account user profile collection.
    #[serde(rename="accountUserProfiles")]
    
    pub account_user_profiles: Option<Vec<AccountUserProfile>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#accountUserProfilesListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for AccountUserProfilesListResponse {}


/// Account List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list accounts](AccountListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountsListResponse {
    /// Account collection.
    
    pub accounts: Option<Vec<Account>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#accountsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for AccountsListResponse {}


/// Represents an activity group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Activities {
    /// List of activity filters. The dimension values need to be all either of type "dfa:activity" or "dfa:activityGroup".
    
    pub filters: Option<Vec<DimensionValue>>,
    /// The kind of resource this is, in this case dfareporting#activities.
    
    pub kind: Option<String>,
    /// List of names of floodlight activity metrics.
    #[serde(rename="metricNames")]
    
    pub metric_names: Option<Vec<String>>,
}

impl client::Part for Activities {}


/// Contains properties of a Campaign Manager ad.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get ads](AdGetCall) (response)
/// * [insert ads](AdInsertCall) (request|response)
/// * [list ads](AdListCall) (none)
/// * [patch ads](AdPatchCall) (request|response)
/// * [update ads](AdUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Ad {
    /// Account ID of this ad. This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Whether this ad is active. When true, archived must be false.
    
    pub active: Option<bool>,
    /// Advertiser ID of this ad. This is a required field on insertion.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Dimension value for the ID of the advertiser. This is a read-only, auto-generated field.
    #[serde(rename="advertiserIdDimensionValue")]
    
    pub advertiser_id_dimension_value: Option<DimensionValue>,
    /// Whether this ad is archived. When true, active must be false.
    
    pub archived: Option<bool>,
    /// Audience segment ID that is being targeted for this ad. Applicable when type is AD_SERVING_STANDARD_AD.
    #[serde(rename="audienceSegmentId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub audience_segment_id: Option<i64>,
    /// Campaign ID of this ad. This is a required field on insertion.
    #[serde(rename="campaignId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub campaign_id: Option<i64>,
    /// Dimension value for the ID of the campaign. This is a read-only, auto-generated field.
    #[serde(rename="campaignIdDimensionValue")]
    
    pub campaign_id_dimension_value: Option<DimensionValue>,
    /// Click-through URL for this ad. This is a required field on insertion. Applicable when type is AD_SERVING_CLICK_TRACKER.
    #[serde(rename="clickThroughUrl")]
    
    pub click_through_url: Option<ClickThroughUrl>,
    /// Click-through URL suffix properties for this ad. Applies to the URL in the ad or (if overriding ad properties) the URL in the creative.
    #[serde(rename="clickThroughUrlSuffixProperties")]
    
    pub click_through_url_suffix_properties: Option<ClickThroughUrlSuffixProperties>,
    /// Comments for this ad.
    
    pub comments: Option<String>,
    /// Compatibility of this ad. Applicable when type is AD_SERVING_DEFAULT_AD. DISPLAY and DISPLAY_INTERSTITIAL refer to either rendering on desktop or on mobile devices or in mobile apps for regular or interstitial ads, respectively. APP and APP_INTERSTITIAL are only used for existing default ads. New mobile placements must be assigned DISPLAY or DISPLAY_INTERSTITIAL and default ads created for those placements will be limited to those compatibility types. IN_STREAM_VIDEO refers to rendering in-stream video ads developed with the VAST standard.
    
    pub compatibility: Option<AdCompatibilityEnum>,
    /// Information about the creation of this ad. This is a read-only field.
    #[serde(rename="createInfo")]
    
    pub create_info: Option<LastModifiedInfo>,
    /// Creative group assignments for this ad. Applicable when type is AD_SERVING_CLICK_TRACKER. Only one assignment per creative group number is allowed for a maximum of two assignments.
    #[serde(rename="creativeGroupAssignments")]
    
    pub creative_group_assignments: Option<Vec<CreativeGroupAssignment>>,
    /// Creative rotation for this ad. Applicable when type is AD_SERVING_DEFAULT_AD, AD_SERVING_STANDARD_AD, or AD_SERVING_TRACKING. When type is AD_SERVING_DEFAULT_AD, this field should have exactly one creativeAssignment.
    #[serde(rename="creativeRotation")]
    
    pub creative_rotation: Option<CreativeRotation>,
    /// Time and day targeting information for this ad. This field must be left blank if the ad is using a targeting template. Applicable when type is AD_SERVING_STANDARD_AD.
    #[serde(rename="dayPartTargeting")]
    
    pub day_part_targeting: Option<DayPartTargeting>,
    /// Default click-through event tag properties for this ad.
    #[serde(rename="defaultClickThroughEventTagProperties")]
    
    pub default_click_through_event_tag_properties: Option<DefaultClickThroughEventTagProperties>,
    /// Delivery schedule information for this ad. Applicable when type is AD_SERVING_STANDARD_AD or AD_SERVING_TRACKING. This field along with subfields priority and impressionRatio are required on insertion when type is AD_SERVING_STANDARD_AD.
    #[serde(rename="deliverySchedule")]
    
    pub delivery_schedule: Option<DeliverySchedule>,
    /// Whether this ad is a dynamic click tracker. Applicable when type is AD_SERVING_CLICK_TRACKER. This is a required field on insert, and is read-only after insert.
    #[serde(rename="dynamicClickTracker")]
    
    pub dynamic_click_tracker: Option<bool>,
    /// Date and time that this ad should stop serving. Must be later than the start time. This is a required field on insertion.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Event tag overrides for this ad.
    #[serde(rename="eventTagOverrides")]
    
    pub event_tag_overrides: Option<Vec<EventTagOverride>>,
    /// Geographical targeting information for this ad. This field must be left blank if the ad is using a targeting template. Applicable when type is AD_SERVING_STANDARD_AD.
    #[serde(rename="geoTargeting")]
    
    pub geo_targeting: Option<GeoTargeting>,
    /// ID of this ad. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Dimension value for the ID of this ad. This is a read-only, auto-generated field.
    #[serde(rename="idDimensionValue")]
    
    pub id_dimension_value: Option<DimensionValue>,
    /// Key-value targeting information for this ad. This field must be left blank if the ad is using a targeting template. Applicable when type is AD_SERVING_STANDARD_AD.
    #[serde(rename="keyValueTargetingExpression")]
    
    pub key_value_targeting_expression: Option<KeyValueTargetingExpression>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#ad".
    
    pub kind: Option<String>,
    /// Language targeting information for this ad. This field must be left blank if the ad is using a targeting template. Applicable when type is AD_SERVING_STANDARD_AD.
    #[serde(rename="languageTargeting")]
    
    pub language_targeting: Option<LanguageTargeting>,
    /// Information about the most recent modification of this ad. This is a read-only field.
    #[serde(rename="lastModifiedInfo")]
    
    pub last_modified_info: Option<LastModifiedInfo>,
    /// Name of this ad. This is a required field and must be less than 256 characters long.
    
    pub name: Option<String>,
    /// Placement assignments for this ad.
    #[serde(rename="placementAssignments")]
    
    pub placement_assignments: Option<Vec<PlacementAssignment>>,
    /// Remarketing list targeting expression for this ad. This field must be left blank if the ad is using a targeting template. Applicable when type is AD_SERVING_STANDARD_AD.
    #[serde(rename="remarketingListExpression")]
    
    pub remarketing_list_expression: Option<ListTargetingExpression>,
    /// Size of this ad. Applicable when type is AD_SERVING_DEFAULT_AD.
    
    pub size: Option<Size>,
    /// Whether this ad is ssl compliant. This is a read-only field that is auto-generated when the ad is inserted or updated.
    #[serde(rename="sslCompliant")]
    
    pub ssl_compliant: Option<bool>,
    /// Whether this ad requires ssl. This is a read-only field that is auto-generated when the ad is inserted or updated.
    #[serde(rename="sslRequired")]
    
    pub ssl_required: Option<bool>,
    /// Date and time that this ad should start serving. If creating an ad, this field must be a time in the future. This is a required field on insertion.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Subaccount ID of this ad. This is a read-only field that can be left blank.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Targeting template ID, used to apply preconfigured targeting information to this ad. This cannot be set while any of dayPartTargeting, geoTargeting, keyValueTargetingExpression, languageTargeting, remarketingListExpression, or technologyTargeting are set. Applicable when type is AD_SERVING_STANDARD_AD.
    #[serde(rename="targetingTemplateId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub targeting_template_id: Option<i64>,
    /// Technology platform targeting information for this ad. This field must be left blank if the ad is using a targeting template. Applicable when type is AD_SERVING_STANDARD_AD.
    #[serde(rename="technologyTargeting")]
    
    pub technology_targeting: Option<TechnologyTargeting>,
    /// Type of ad. This is a required field on insertion. Note that default ads (AD_SERVING_DEFAULT_AD) cannot be created directly (see Creative resource).
    #[serde(rename="type")]
    
    pub type_: Option<AdTypeEnum>,
}

impl client::RequestValue for Ad {}
impl client::Resource for Ad {}
impl client::ResponseResult for Ad {}


/// Campaign ad blocking settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdBlockingConfiguration {
    /// Click-through URL used by brand-neutral ads. This is a required field when overrideClickThroughUrl is set to true.
    #[serde(rename="clickThroughUrl")]
    
    pub click_through_url: Option<String>,
    /// ID of a creative bundle to use for this campaign. If set, brand-neutral ads will select creatives from this bundle. Otherwise, a default transparent pixel will be used.
    #[serde(rename="creativeBundleId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creative_bundle_id: Option<i64>,
    /// Whether this campaign has enabled ad blocking. When true, ad blocking is enabled for placements in the campaign, but this may be overridden by site and placement settings. When false, ad blocking is disabled for all placements under the campaign, regardless of site and placement settings.
    
    pub enabled: Option<bool>,
    /// Whether the brand-neutral ad's click-through URL comes from the campaign's creative bundle or the override URL. Must be set to true if ad blocking is enabled and no creative bundle is configured.
    #[serde(rename="overrideClickThroughUrl")]
    
    pub override_click_through_url: Option<bool>,
}

impl client::Part for AdBlockingConfiguration {}


/// Ad Slot
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdSlot {
    /// Comment for this ad slot.
    
    pub comment: Option<String>,
    /// Ad slot compatibility. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering either on desktop, mobile devices or in mobile apps for regular or interstitial ads respectively. APP and APP_INTERSTITIAL are for rendering in mobile apps. IN_STREAM_VIDEO refers to rendering in in-stream video ads developed with the VAST standard.
    
    pub compatibility: Option<AdSlotCompatibilityEnum>,
    /// Height of this ad slot.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub height: Option<i64>,
    /// ID of the placement from an external platform that is linked to this ad slot.
    #[serde(rename="linkedPlacementId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub linked_placement_id: Option<i64>,
    /// Name of this ad slot.
    
    pub name: Option<String>,
    /// Payment source type of this ad slot.
    #[serde(rename="paymentSourceType")]
    
    pub payment_source_type: Option<AdSlotPaymentSourceTypeEnum>,
    /// Primary ad slot of a roadblock inventory item.
    
    pub primary: Option<bool>,
    /// Width of this ad slot.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub width: Option<i64>,
}

impl client::Part for AdSlot {}


/// Ad List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list ads](AdListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdsListResponse {
    /// Ad collection.
    
    pub ads: Option<Vec<Ad>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#adsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for AdsListResponse {}


/// Contains properties of a Campaign Manager advertiser.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get advertisers](AdvertiserGetCall) (response)
/// * [insert advertisers](AdvertiserInsertCall) (request|response)
/// * [list advertisers](AdvertiserListCall) (none)
/// * [patch advertisers](AdvertiserPatchCall) (request|response)
/// * [update advertisers](AdvertiserUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Advertiser {
    /// Account ID of this advertiser.This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// ID of the advertiser group this advertiser belongs to. You can group advertisers for reporting purposes, allowing you to see aggregated information for all advertisers in each group.
    #[serde(rename="advertiserGroupId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_group_id: Option<i64>,
    /// Suffix added to click-through URL of ad creative associations under this advertiser. Must be less than 129 characters long.
    #[serde(rename="clickThroughUrlSuffix")]
    
    pub click_through_url_suffix: Option<String>,
    /// ID of the click-through event tag to apply by default to the landing pages of this advertiser's campaigns.
    #[serde(rename="defaultClickThroughEventTagId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub default_click_through_event_tag_id: Option<i64>,
    /// Default email address used in sender field for tag emails.
    #[serde(rename="defaultEmail")]
    
    pub default_email: Option<String>,
    /// Floodlight configuration ID of this advertiser. The floodlight configuration ID will be created automatically, so on insert this field should be left blank. This field can be set to another advertiser's floodlight configuration ID in order to share that advertiser's floodlight configuration with this advertiser, so long as: 
    /// - This advertiser's original floodlight configuration is not already associated with floodlight activities or floodlight activity groups. 
    /// - This advertiser's original floodlight configuration is not already shared with another advertiser.
    #[serde(rename="floodlightConfigurationId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub floodlight_configuration_id: Option<i64>,
    /// Dimension value for the ID of the floodlight configuration. This is a read-only, auto-generated field.
    #[serde(rename="floodlightConfigurationIdDimensionValue")]
    
    pub floodlight_configuration_id_dimension_value: Option<DimensionValue>,
    /// ID of this advertiser. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Dimension value for the ID of this advertiser. This is a read-only, auto-generated field.
    #[serde(rename="idDimensionValue")]
    
    pub id_dimension_value: Option<DimensionValue>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#advertiser".
    
    pub kind: Option<String>,
    /// Name of this advertiser. This is a required field and must be less than 256 characters long and unique among advertisers of the same account.
    
    pub name: Option<String>,
    /// Original floodlight configuration before any sharing occurred. Set the floodlightConfigurationId of this advertiser to originalFloodlightConfigurationId to unshare the advertiser's current floodlight configuration. You cannot unshare an advertiser's floodlight configuration if the shared configuration has activities associated with any campaign or placement.
    #[serde(rename="originalFloodlightConfigurationId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub original_floodlight_configuration_id: Option<i64>,
    /// Status of this advertiser.
    
    pub status: Option<AdvertiserStatusEnum>,
    /// Subaccount ID of this advertiser.This is a read-only field that can be left blank.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Suspension status of this advertiser.
    
    pub suspended: Option<bool>,
}

impl client::RequestValue for Advertiser {}
impl client::Resource for Advertiser {}
impl client::ResponseResult for Advertiser {}


/// Groups advertisers together so that reports can be generated for the entire group at once.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete advertiser groups](AdvertiserGroupDeleteCall) (none)
/// * [get advertiser groups](AdvertiserGroupGetCall) (response)
/// * [insert advertiser groups](AdvertiserGroupInsertCall) (request|response)
/// * [list advertiser groups](AdvertiserGroupListCall) (none)
/// * [patch advertiser groups](AdvertiserGroupPatchCall) (request|response)
/// * [update advertiser groups](AdvertiserGroupUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvertiserGroup {
    /// Account ID of this advertiser group. This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// ID of this advertiser group. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#advertiserGroup".
    
    pub kind: Option<String>,
    /// Name of this advertiser group. This is a required field and must be less than 256 characters long and unique among advertiser groups of the same account.
    
    pub name: Option<String>,
}

impl client::RequestValue for AdvertiserGroup {}
impl client::Resource for AdvertiserGroup {}
impl client::ResponseResult for AdvertiserGroup {}


/// Advertiser Group List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list advertiser groups](AdvertiserGroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvertiserGroupsListResponse {
    /// Advertiser group collection.
    #[serde(rename="advertiserGroups")]
    
    pub advertiser_groups: Option<Vec<AdvertiserGroup>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#advertiserGroupsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for AdvertiserGroupsListResponse {}


/// Advertiser List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list advertisers](AdvertiserListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvertisersListResponse {
    /// Advertiser collection.
    
    pub advertisers: Option<Vec<Advertiser>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#advertisersListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for AdvertisersListResponse {}


/// Audience Segment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AudienceSegment {
    /// Weight allocated to this segment. The weight assigned will be understood in proportion to the weights assigned to other segments in the same segment group. Acceptable values are 1 to 1000, inclusive.
    
    pub allocation: Option<i32>,
    /// ID of this audience segment. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Name of this audience segment. This is a required field and must be less than 65 characters long.
    
    pub name: Option<String>,
}

impl client::Part for AudienceSegment {}


/// Audience Segment Group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AudienceSegmentGroup {
    /// Audience segments assigned to this group. The number of segments must be between 2 and 100.
    #[serde(rename="audienceSegments")]
    
    pub audience_segments: Option<Vec<AudienceSegment>>,
    /// ID of this audience segment group. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Name of this audience segment group. This is a required field and must be less than 65 characters long.
    
    pub name: Option<String>,
}

impl client::Part for AudienceSegmentGroup {}


/// Contains information about a browser that can be targeted by ads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list browsers](BrowserListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Browser {
    /// ID referring to this grouping of browser and version numbers. This is the ID used for targeting.
    #[serde(rename="browserVersionId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub browser_version_id: Option<i64>,
    /// DART ID of this browser. This is the ID used when generating reports.
    #[serde(rename="dartId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub dart_id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#browser".
    
    pub kind: Option<String>,
    /// Major version number (leftmost number) of this browser. For example, for Chrome 5.0.376.86 beta, this field should be set to 5. An asterisk (*) may be used to target any version number, and a question mark (?) may be used to target cases where the version number cannot be identified. For example, Chrome *.* targets any version of Chrome: 1.2, 2.5, 3.5, and so on. Chrome 3.* targets Chrome 3.1, 3.5, but not 4.0. Firefox ?.? targets cases where the ad server knows the browser is Firefox but can't tell which version it is.
    #[serde(rename="majorVersion")]
    
    pub major_version: Option<String>,
    /// Minor version number (number after first dot on left) of this browser. For example, for Chrome 5.0.375.86 beta, this field should be set to 0. An asterisk (*) may be used to target any version number, and a question mark (?) may be used to target cases where the version number cannot be identified. For example, Chrome *.* targets any version of Chrome: 1.2, 2.5, 3.5, and so on. Chrome 3.* targets Chrome 3.1, 3.5, but not 4.0. Firefox ?.? targets cases where the ad server knows the browser is Firefox but can't tell which version it is.
    #[serde(rename="minorVersion")]
    
    pub minor_version: Option<String>,
    /// Name of this browser.
    
    pub name: Option<String>,
}

impl client::Resource for Browser {}


/// Browser List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list browsers](BrowserListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BrowsersListResponse {
    /// Browser collection.
    
    pub browsers: Option<Vec<Browser>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#browsersListResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for BrowsersListResponse {}


/// Contains properties of a Campaign Manager campaign.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get campaigns](CampaignGetCall) (response)
/// * [insert campaigns](CampaignInsertCall) (request|response)
/// * [list campaigns](CampaignListCall) (none)
/// * [patch campaigns](CampaignPatchCall) (request|response)
/// * [update campaigns](CampaignUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Campaign {
    /// Account ID of this campaign. This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Ad blocking settings for this campaign.
    #[serde(rename="adBlockingConfiguration")]
    
    pub ad_blocking_configuration: Option<AdBlockingConfiguration>,
    /// Additional creative optimization configurations for the campaign.
    #[serde(rename="additionalCreativeOptimizationConfigurations")]
    
    pub additional_creative_optimization_configurations: Option<Vec<CreativeOptimizationConfiguration>>,
    /// Advertiser group ID of the associated advertiser.
    #[serde(rename="advertiserGroupId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_group_id: Option<i64>,
    /// Advertiser ID of this campaign. This is a required field.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Dimension value for the advertiser ID of this campaign. This is a read-only, auto-generated field.
    #[serde(rename="advertiserIdDimensionValue")]
    
    pub advertiser_id_dimension_value: Option<DimensionValue>,
    /// Whether this campaign has been archived.
    
    pub archived: Option<bool>,
    /// Audience segment groups assigned to this campaign. Cannot have more than 300 segment groups.
    #[serde(rename="audienceSegmentGroups")]
    
    pub audience_segment_groups: Option<Vec<AudienceSegmentGroup>>,
    /// Billing invoice code included in the Campaign Manager client billing invoices associated with the campaign.
    #[serde(rename="billingInvoiceCode")]
    
    pub billing_invoice_code: Option<String>,
    /// Click-through URL suffix override properties for this campaign.
    #[serde(rename="clickThroughUrlSuffixProperties")]
    
    pub click_through_url_suffix_properties: Option<ClickThroughUrlSuffixProperties>,
    /// Arbitrary comments about this campaign. Must be less than 256 characters long.
    
    pub comment: Option<String>,
    /// Information about the creation of this campaign. This is a read-only field.
    #[serde(rename="createInfo")]
    
    pub create_info: Option<LastModifiedInfo>,
    /// List of creative group IDs that are assigned to the campaign.
    #[serde(rename="creativeGroupIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub creative_group_ids: Option<Vec<i64>>,
    /// Creative optimization configuration for the campaign.
    #[serde(rename="creativeOptimizationConfiguration")]
    
    pub creative_optimization_configuration: Option<CreativeOptimizationConfiguration>,
    /// Click-through event tag ID override properties for this campaign.
    #[serde(rename="defaultClickThroughEventTagProperties")]
    
    pub default_click_through_event_tag_properties: Option<DefaultClickThroughEventTagProperties>,
    /// Date on which the campaign will stop running. On insert, the end date must be today or a future date. The end date must be later than or be the same as the start date. If, for example, you set 6/25/2015 as both the start and end dates, the effective campaign run date is just that day only, 6/25/2015. The hours, minutes, and seconds of the end date should not be set, as doing so will result in an error. This is a required field.
    #[serde(rename="endDate")]
    
    pub end_date: Option<client::chrono::NaiveDate>,
    /// Overrides that can be used to activate or deactivate advertiser event tags.
    #[serde(rename="eventTagOverrides")]
    
    pub event_tag_overrides: Option<Vec<EventTagOverride>>,
    /// External ID for this campaign.
    #[serde(rename="externalId")]
    
    pub external_id: Option<String>,
    /// ID of this campaign. This is a read-only auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Dimension value for the ID of this campaign. This is a read-only, auto-generated field.
    #[serde(rename="idDimensionValue")]
    
    pub id_dimension_value: Option<DimensionValue>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#campaign".
    
    pub kind: Option<String>,
    /// Information about the most recent modification of this campaign. This is a read-only field.
    #[serde(rename="lastModifiedInfo")]
    
    pub last_modified_info: Option<LastModifiedInfo>,
    /// Lookback window settings for the campaign.
    #[serde(rename="lookbackConfiguration")]
    
    pub lookback_configuration: Option<LookbackConfiguration>,
    /// Name of this campaign. This is a required field and must be less than 256 characters long and unique among campaigns of the same advertiser.
    
    pub name: Option<String>,
    /// Whether Nielsen reports are enabled for this campaign.
    #[serde(rename="nielsenOcrEnabled")]
    
    pub nielsen_ocr_enabled: Option<bool>,
    /// Date on which the campaign starts running. The start date can be any date. The hours, minutes, and seconds of the start date should not be set, as doing so will result in an error. This is a required field.
    #[serde(rename="startDate")]
    
    pub start_date: Option<client::chrono::NaiveDate>,
    /// Subaccount ID of this campaign. This is a read-only field that can be left blank.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Campaign trafficker contact emails.
    #[serde(rename="traffickerEmails")]
    
    pub trafficker_emails: Option<Vec<String>>,
}

impl client::RequestValue for Campaign {}
impl client::Resource for Campaign {}
impl client::ResponseResult for Campaign {}


/// Identifies a creative which has been associated with a given campaign.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert campaign creative associations](CampaignCreativeAssociationInsertCall) (request|response)
/// * [list campaign creative associations](CampaignCreativeAssociationListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CampaignCreativeAssociation {
    /// ID of the creative associated with the campaign. This is a required field.
    #[serde(rename="creativeId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creative_id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#campaignCreativeAssociation".
    
    pub kind: Option<String>,
}

impl client::RequestValue for CampaignCreativeAssociation {}
impl client::Resource for CampaignCreativeAssociation {}
impl client::ResponseResult for CampaignCreativeAssociation {}


/// Campaign Creative Association List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list campaign creative associations](CampaignCreativeAssociationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CampaignCreativeAssociationsListResponse {
    /// Campaign creative association collection
    #[serde(rename="campaignCreativeAssociations")]
    
    pub campaign_creative_associations: Option<Vec<CampaignCreativeAssociation>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#campaignCreativeAssociationsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for CampaignCreativeAssociationsListResponse {}


/// Campaign List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list campaigns](CampaignListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CampaignsListResponse {
    /// Campaign collection.
    
    pub campaigns: Option<Vec<Campaign>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#campaignsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for CampaignsListResponse {}


/// Describes a change that a user has made to a resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get change logs](ChangeLogGetCall) (response)
/// * [list change logs](ChangeLogListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChangeLog {
    /// Account ID of the modified object.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Action which caused the change.
    
    pub action: Option<String>,
    /// Time when the object was modified.
    #[serde(rename="changeTime")]
    
    pub change_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Field name of the object which changed.
    #[serde(rename="fieldName")]
    
    pub field_name: Option<String>,
    /// ID of this change log.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#changeLog".
    
    pub kind: Option<String>,
    /// New value of the object field.
    #[serde(rename="newValue")]
    
    pub new_value: Option<String>,
    /// ID of the object of this change log. The object could be a campaign, placement, ad, or other type.
    #[serde(rename="objectId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub object_id: Option<i64>,
    /// Object type of the change log.
    #[serde(rename="objectType")]
    
    pub object_type: Option<String>,
    /// Old value of the object field.
    #[serde(rename="oldValue")]
    
    pub old_value: Option<String>,
    /// Subaccount ID of the modified object.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Transaction ID of this change log. When a single API call results in many changes, each change will have a separate ID in the change log but will share the same transactionId.
    #[serde(rename="transactionId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub transaction_id: Option<i64>,
    /// ID of the user who modified the object.
    #[serde(rename="userProfileId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub user_profile_id: Option<i64>,
    /// User profile name of the user who modified the object.
    #[serde(rename="userProfileName")]
    
    pub user_profile_name: Option<String>,
}

impl client::Resource for ChangeLog {}
impl client::ResponseResult for ChangeLog {}


/// Change Log List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list change logs](ChangeLogListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChangeLogsListResponse {
    /// Change log collection.
    #[serde(rename="changeLogs")]
    
    pub change_logs: Option<Vec<ChangeLog>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#changeLogsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ChangeLogsListResponse {}


/// City List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list cities](CityListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CitiesListResponse {
    /// City collection.
    
    pub cities: Option<Vec<City>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#citiesListResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for CitiesListResponse {}


/// Contains information about a city that can be targeted by ads.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct City {
    /// Country code of the country to which this city belongs.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// DART ID of the country to which this city belongs.
    #[serde(rename="countryDartId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub country_dart_id: Option<i64>,
    /// DART ID of this city. This is the ID used for targeting and generating reports.
    #[serde(rename="dartId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub dart_id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#city".
    
    pub kind: Option<String>,
    /// Metro region code of the metro region (DMA) to which this city belongs.
    #[serde(rename="metroCode")]
    
    pub metro_code: Option<String>,
    /// ID of the metro region (DMA) to which this city belongs.
    #[serde(rename="metroDmaId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub metro_dma_id: Option<i64>,
    /// Name of this city.
    
    pub name: Option<String>,
    /// Region code of the region to which this city belongs.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
    /// DART ID of the region to which this city belongs.
    #[serde(rename="regionDartId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub region_dart_id: Option<i64>,
}

impl client::Part for City {}


/// Creative Click Tag.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClickTag {
    /// Advertiser event name associated with the click tag. This field is used by DISPLAY_IMAGE_GALLERY and HTML5_BANNER creatives. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    #[serde(rename="eventName")]
    
    pub event_name: Option<String>,
    /// Parameter name for the specified click tag. For DISPLAY_IMAGE_GALLERY creative assets, this field must match the value of the creative asset's creativeAssetId.name field.
    
    pub name: Option<String>,
    /// Parameter value for the specified click tag. This field contains a click-through url.
    
    pub value: Option<String>,
}

impl client::Part for ClickTag {}


/// Click-through URL
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClickThroughUrl {
    /// Read-only convenience field representing the actual URL that will be used for this click-through. The URL is computed as follows: 
    /// - If defaultLandingPage is enabled then the campaign's default landing page URL is assigned to this field.
    /// - If defaultLandingPage is not enabled and a landingPageId is specified then that landing page's URL is assigned to this field.
    /// - If neither of the above cases apply, then the customClickThroughUrl is assigned to this field.
    #[serde(rename="computedClickThroughUrl")]
    
    pub computed_click_through_url: Option<String>,
    /// Custom click-through URL. Applicable if the defaultLandingPage field is set to false and the landingPageId field is left unset.
    #[serde(rename="customClickThroughUrl")]
    
    pub custom_click_through_url: Option<String>,
    /// Whether the campaign default landing page is used.
    #[serde(rename="defaultLandingPage")]
    
    pub default_landing_page: Option<bool>,
    /// ID of the landing page for the click-through URL. Applicable if the defaultLandingPage field is set to false.
    #[serde(rename="landingPageId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub landing_page_id: Option<i64>,
}

impl client::Part for ClickThroughUrl {}


/// Click Through URL Suffix settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClickThroughUrlSuffixProperties {
    /// Click-through URL suffix to apply to all ads in this entity's scope. Must be less than 128 characters long.
    #[serde(rename="clickThroughUrlSuffix")]
    
    pub click_through_url_suffix: Option<String>,
    /// Whether this entity should override the inherited click-through URL suffix with its own defined value.
    #[serde(rename="overrideInheritedSuffix")]
    
    pub override_inherited_suffix: Option<bool>,
}

impl client::Part for ClickThroughUrlSuffixProperties {}


/// Companion Click-through override.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompanionClickThroughOverride {
    /// Click-through URL of this companion click-through override.
    #[serde(rename="clickThroughUrl")]
    
    pub click_through_url: Option<ClickThroughUrl>,
    /// ID of the creative for this companion click-through override.
    #[serde(rename="creativeId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creative_id: Option<i64>,
}

impl client::Part for CompanionClickThroughOverride {}


/// Companion Settings
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompanionSetting {
    /// Whether companions are disabled for this placement.
    #[serde(rename="companionsDisabled")]
    
    pub companions_disabled: Option<bool>,
    /// Whitelist of companion sizes to be served to this placement. Set this list to null or empty to serve all companion sizes.
    #[serde(rename="enabledSizes")]
    
    pub enabled_sizes: Option<Vec<Size>>,
    /// Whether to serve only static images as companions.
    #[serde(rename="imageOnly")]
    
    pub image_only: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#companionSetting".
    
    pub kind: Option<String>,
}

impl client::Part for CompanionSetting {}


/// Represents a response to the queryCompatibleFields method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [compatible fields query reports](ReportCompatibleFieldQueryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompatibleFields {
    /// Contains items that are compatible to be selected for a report of type "CROSS_DIMENSION_REACH".
    #[serde(rename="crossDimensionReachReportCompatibleFields")]
    
    pub cross_dimension_reach_report_compatible_fields: Option<CrossDimensionReachReportCompatibleFields>,
    /// Contains items that are compatible to be selected for a report of type "FLOODLIGHT".
    #[serde(rename="floodlightReportCompatibleFields")]
    
    pub floodlight_report_compatible_fields: Option<FloodlightReportCompatibleFields>,
    /// The kind of resource this is, in this case dfareporting#compatibleFields.
    
    pub kind: Option<String>,
    /// Contains items that are compatible to be selected for a report of type "PATH_TO_CONVERSION".
    #[serde(rename="pathToConversionReportCompatibleFields")]
    
    pub path_to_conversion_report_compatible_fields: Option<PathToConversionReportCompatibleFields>,
    /// Contains items that are compatible to be selected for a report of type "REACH".
    #[serde(rename="reachReportCompatibleFields")]
    
    pub reach_report_compatible_fields: Option<ReachReportCompatibleFields>,
    /// Contains items that are compatible to be selected for a report of type "STANDARD".
    #[serde(rename="reportCompatibleFields")]
    
    pub report_compatible_fields: Option<ReportCompatibleFields>,
}

impl client::ResponseResult for CompatibleFields {}


/// Contains information about an internet connection type that can be targeted by ads. Clients can use the connection type to target mobile vs. broadband users.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get connection types](ConnectionTypeGetCall) (response)
/// * [list connection types](ConnectionTypeListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionType {
    /// ID of this connection type.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#connectionType".
    
    pub kind: Option<String>,
    /// Name of this connection type.
    
    pub name: Option<String>,
}

impl client::Resource for ConnectionType {}
impl client::ResponseResult for ConnectionType {}


/// Connection Type List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list connection types](ConnectionTypeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionTypesListResponse {
    /// Collection of connection types such as broadband and mobile.
    #[serde(rename="connectionTypes")]
    
    pub connection_types: Option<Vec<ConnectionType>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#connectionTypesListResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for ConnectionTypesListResponse {}


/// Content Category List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list content categories](ContentCategoryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentCategoriesListResponse {
    /// Content category collection.
    #[serde(rename="contentCategories")]
    
    pub content_categories: Option<Vec<ContentCategory>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#contentCategoriesListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ContentCategoriesListResponse {}


/// Organizes placements according to the contents of their associated webpages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get content categories](ContentCategoryGetCall) (response)
/// * [insert content categories](ContentCategoryInsertCall) (request|response)
/// * [patch content categories](ContentCategoryPatchCall) (request|response)
/// * [update content categories](ContentCategoryUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentCategory {
    /// Account ID of this content category. This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// ID of this content category. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#contentCategory".
    
    pub kind: Option<String>,
    /// Name of this content category. This is a required field and must be less than 256 characters long and unique among content categories of the same account.
    
    pub name: Option<String>,
}

impl client::RequestValue for ContentCategory {}
impl client::ResponseResult for ContentCategory {}


/// A Conversion represents when a user successfully performs a desired action after seeing an ad.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batchinsert conversions](ConversionBatchinsertCall) (none)
/// * [batchupdate conversions](ConversionBatchupdateCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Conversion {
    /// Whether the conversion was directed toward children.
    #[serde(rename="childDirectedTreatment")]
    
    pub child_directed_treatment: Option<bool>,
    /// Custom floodlight variables.
    #[serde(rename="customVariables")]
    
    pub custom_variables: Option<Vec<CustomFloodlightVariable>>,
    /// The alphanumeric encrypted user ID. When set, encryptionInfo should also be specified. This field is mutually exclusive with encryptedUserIdCandidates[], mobileDeviceId and gclid. This or encryptedUserIdCandidates[] or mobileDeviceId or gclid is a required field.
    #[serde(rename="encryptedUserId")]
    
    pub encrypted_user_id: Option<String>,
    /// A list of the alphanumeric encrypted user IDs. Any user ID with exposure prior to the conversion timestamp will be used in the inserted conversion. If no such user ID is found then the conversion will be rejected with NO_COOKIE_MATCH_FOUND error. When set, encryptionInfo should also be specified. This field may only be used when calling batchinsert; it is not supported by batchupdate. This field is mutually exclusive with encryptedUserId, mobileDeviceId and gclid. This or encryptedUserId or mobileDeviceId or gclid is a required field.
    #[serde(rename="encryptedUserIdCandidates")]
    
    pub encrypted_user_id_candidates: Option<Vec<String>>,
    /// Floodlight Activity ID of this conversion. This is a required field.
    #[serde(rename="floodlightActivityId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub floodlight_activity_id: Option<i64>,
    /// Floodlight Configuration ID of this conversion. This is a required field.
    #[serde(rename="floodlightConfigurationId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub floodlight_configuration_id: Option<i64>,
    /// The Google click ID. This field is mutually exclusive with encryptedUserId, encryptedUserIdCandidates[] and mobileDeviceId. This or encryptedUserId or encryptedUserIdCandidates[] or mobileDeviceId is a required field.
    
    pub gclid: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#conversion".
    
    pub kind: Option<String>,
    /// Whether Limit Ad Tracking is enabled. When set to true, the conversion will be used for reporting but not targeting. This will prevent remarketing.
    #[serde(rename="limitAdTracking")]
    
    pub limit_ad_tracking: Option<bool>,
    /// The mobile device ID. This field is mutually exclusive with encryptedUserId, encryptedUserIdCandidates[] and gclid. This or encryptedUserId or encryptedUserIdCandidates[] or gclid is a required field.
    #[serde(rename="mobileDeviceId")]
    
    pub mobile_device_id: Option<String>,
    /// The ordinal of the conversion. Use this field to control how conversions of the same user and day are de-duplicated. This is a required field.
    
    pub ordinal: Option<String>,
    /// The quantity of the conversion.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quantity: Option<i64>,
    /// The timestamp of conversion, in Unix epoch micros. This is a required field.
    #[serde(rename="timestampMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub timestamp_micros: Option<i64>,
    /// The value of the conversion.
    
    pub value: Option<f64>,
}

impl client::Resource for Conversion {}


/// The error code and description for a conversion that failed to insert or update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConversionError {
    /// The error code.
    
    pub code: Option<ConversionErrorCodeEnum>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#conversionError".
    
    pub kind: Option<String>,
    /// A description of the error.
    
    pub message: Option<String>,
}

impl client::Part for ConversionError {}


/// The original conversion that was inserted or updated and whether there were any errors.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConversionStatus {
    /// The original conversion that was inserted or updated.
    
    pub conversion: Option<Conversion>,
    /// A list of errors related to this conversion.
    
    pub errors: Option<Vec<ConversionError>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#conversionStatus".
    
    pub kind: Option<String>,
}

impl client::Part for ConversionStatus {}


/// Insert Conversions Request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batchinsert conversions](ConversionBatchinsertCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConversionsBatchInsertRequest {
    /// The set of conversions to insert.
    
    pub conversions: Option<Vec<Conversion>>,
    /// Describes how encryptedUserId or encryptedUserIdCandidates[] is encrypted. This is a required field if encryptedUserId or encryptedUserIdCandidates[] is used.
    #[serde(rename="encryptionInfo")]
    
    pub encryption_info: Option<EncryptionInfo>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#conversionsBatchInsertRequest".
    
    pub kind: Option<String>,
}

impl client::RequestValue for ConversionsBatchInsertRequest {}


/// Insert Conversions Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batchinsert conversions](ConversionBatchinsertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConversionsBatchInsertResponse {
    /// Indicates that some or all conversions failed to insert.
    #[serde(rename="hasFailures")]
    
    pub has_failures: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#conversionsBatchInsertResponse".
    
    pub kind: Option<String>,
    /// The insert status of each conversion. Statuses are returned in the same order that conversions are inserted.
    
    pub status: Option<Vec<ConversionStatus>>,
}

impl client::ResponseResult for ConversionsBatchInsertResponse {}


/// Update Conversions Request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batchupdate conversions](ConversionBatchupdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConversionsBatchUpdateRequest {
    /// The set of conversions to update.
    
    pub conversions: Option<Vec<Conversion>>,
    /// Describes how encryptedUserId is encrypted. This is a required field if encryptedUserId is used.
    #[serde(rename="encryptionInfo")]
    
    pub encryption_info: Option<EncryptionInfo>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#conversionsBatchUpdateRequest".
    
    pub kind: Option<String>,
}

impl client::RequestValue for ConversionsBatchUpdateRequest {}


/// Update Conversions Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batchupdate conversions](ConversionBatchupdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConversionsBatchUpdateResponse {
    /// Indicates that some or all conversions failed to update.
    #[serde(rename="hasFailures")]
    
    pub has_failures: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#conversionsBatchUpdateResponse".
    
    pub kind: Option<String>,
    /// The update status of each conversion. Statuses are returned in the same order that conversions are updated.
    
    pub status: Option<Vec<ConversionStatus>>,
}

impl client::ResponseResult for ConversionsBatchUpdateResponse {}


/// Country List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list countries](CountryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CountriesListResponse {
    /// Country collection.
    
    pub countries: Option<Vec<Country>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#countriesListResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for CountriesListResponse {}


/// Contains information about a country that can be targeted by ads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get countries](CountryGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Country {
    /// Country code.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// DART ID of this country. This is the ID used for targeting and generating reports.
    #[serde(rename="dartId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub dart_id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#country".
    
    pub kind: Option<String>,
    /// Name of this country.
    
    pub name: Option<String>,
    /// Whether ad serving supports secure servers in this country.
    #[serde(rename="sslEnabled")]
    
    pub ssl_enabled: Option<bool>,
}

impl client::ResponseResult for Country {}


/// Contains properties of a Creative.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get creatives](CreativeGetCall) (response)
/// * [insert creatives](CreativeInsertCall) (request|response)
/// * [list creatives](CreativeListCall) (none)
/// * [patch creatives](CreativePatchCall) (request|response)
/// * [update creatives](CreativeUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Creative {
    /// Account ID of this creative. This field, if left unset, will be auto-generated for both insert and update operations. Applicable to all creative types.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Whether the creative is active. Applicable to all creative types.
    
    pub active: Option<bool>,
    /// Ad parameters user for VPAID creative. This is a read-only field. Applicable to the following creative types: all VPAID.
    #[serde(rename="adParameters")]
    
    pub ad_parameters: Option<String>,
    /// Keywords for a Rich Media creative. Keywords let you customize the creative settings of a Rich Media ad running on your site without having to contact the advertiser. You can use keywords to dynamically change the look or functionality of a creative. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
    #[serde(rename="adTagKeys")]
    
    pub ad_tag_keys: Option<Vec<String>>,
    /// Advertiser ID of this creative. This is a required field. Applicable to all creative types.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Whether script access is allowed for this creative. This is a read-only and deprecated field which will automatically be set to true on update. Applicable to the following creative types: FLASH_INPAGE.
    #[serde(rename="allowScriptAccess")]
    
    pub allow_script_access: Option<bool>,
    /// Whether the creative is archived. Applicable to all creative types.
    
    pub archived: Option<bool>,
    /// Type of artwork used for the creative. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
    #[serde(rename="artworkType")]
    
    pub artwork_type: Option<CreativeArtworkTypeEnum>,
    /// Source application where creative was authored. Presently, only DBM authored creatives will have this field set. Applicable to all creative types.
    #[serde(rename="authoringSource")]
    
    pub authoring_source: Option<CreativeAuthoringSourceEnum>,
    /// Authoring tool for HTML5 banner creatives. This is a read-only field. Applicable to the following creative types: HTML5_BANNER.
    #[serde(rename="authoringTool")]
    
    pub authoring_tool: Option<CreativeAuthoringToolEnum>,
    /// Whether images are automatically advanced for image gallery creatives. Applicable to the following creative types: DISPLAY_IMAGE_GALLERY.
    #[serde(rename="autoAdvanceImages")]
    
    pub auto_advance_images: Option<bool>,
    /// The 6-character HTML color code, beginning with #, for the background of the window area where the Flash file is displayed. Default is white. Applicable to the following creative types: FLASH_INPAGE.
    #[serde(rename="backgroundColor")]
    
    pub background_color: Option<String>,
    /// Click-through URL for backup image. Applicable to the following creative types: FLASH_INPAGE, and HTML5_BANNER. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    #[serde(rename="backupImageClickThroughUrl")]
    
    pub backup_image_click_through_url: Option<String>,
    /// List of feature dependencies that will cause a backup image to be served if the browser that serves the ad does not support them. Feature dependencies are features that a browser must be able to support in order to render your HTML5 creative asset correctly. This field is initially auto-generated to contain all features detected by Campaign Manager for all the assets of this creative and can then be modified by the client. To reset this field, copy over all the creativeAssets' detected features. Applicable to the following creative types: HTML5_BANNER. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    #[serde(rename="backupImageFeatures")]
    
    pub backup_image_features: Option<Vec<CreativeBackupImageFeaturesEnum>>,
    /// Reporting label used for HTML5 banner backup image. Applicable to the following creative types: DISPLAY when the primary asset type is not HTML_IMAGE.
    #[serde(rename="backupImageReportingLabel")]
    
    pub backup_image_reporting_label: Option<String>,
    /// Target window for backup image. Applicable to the following creative types: FLASH_INPAGE and HTML5_BANNER. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    #[serde(rename="backupImageTargetWindow")]
    
    pub backup_image_target_window: Option<TargetWindow>,
    /// Click tags of the creative. For DISPLAY, FLASH_INPAGE, and HTML5_BANNER creatives, this is a subset of detected click tags for the assets associated with this creative. After creating a flash asset, detected click tags will be returned in the creativeAssetMetadata. When inserting the creative, populate the creative clickTags field using the creativeAssetMetadata.clickTags field. For DISPLAY_IMAGE_GALLERY creatives, there should be exactly one entry in this list for each image creative asset. A click tag is matched with a corresponding creative asset by matching the clickTag.name field with the creativeAsset.assetIdentifier.name field. Applicable to the following creative types: DISPLAY_IMAGE_GALLERY, FLASH_INPAGE, HTML5_BANNER. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    #[serde(rename="clickTags")]
    
    pub click_tags: Option<Vec<ClickTag>>,
    /// Industry standard ID assigned to creative for reach and frequency. Applicable to INSTREAM_VIDEO_REDIRECT creatives.
    #[serde(rename="commercialId")]
    
    pub commercial_id: Option<String>,
    /// List of companion creatives assigned to an in-Stream video creative. Acceptable values include IDs of existing flash and image creatives. Applicable to the following creative types: all VPAID and all INSTREAM_VIDEO with dynamicAssetSelection set to false.
    #[serde(rename="companionCreatives")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub companion_creatives: Option<Vec<i64>>,
    /// Compatibilities associated with this creative. This is a read-only field. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering either on desktop or on mobile devices or in mobile apps for regular or interstitial ads, respectively. APP and APP_INTERSTITIAL are for rendering in mobile apps. Only pre-existing creatives may have these compatibilities since new creatives will either be assigned DISPLAY or DISPLAY_INTERSTITIAL instead. IN_STREAM_VIDEO refers to rendering in in-stream video ads developed with the VAST standard. Applicable to all creative types.
    /// 
    /// Acceptable values are:
    /// - "APP"
    /// - "APP_INTERSTITIAL"
    /// - "IN_STREAM_VIDEO"
    /// - "DISPLAY"
    /// - "DISPLAY_INTERSTITIAL"
    
    pub compatibility: Option<Vec<CreativeCompatibilityEnum>>,
    /// Whether Flash assets associated with the creative need to be automatically converted to HTML5. This flag is enabled by default and users can choose to disable it if they don't want the system to generate and use HTML5 asset for this creative. Applicable to the following creative type: FLASH_INPAGE. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    #[serde(rename="convertFlashToHtml5")]
    
    pub convert_flash_to_html5: Option<bool>,
    /// List of counter events configured for the creative. For DISPLAY_IMAGE_GALLERY creatives, these are read-only and auto-generated from clickTags. Applicable to the following creative types: DISPLAY_IMAGE_GALLERY, all RICH_MEDIA, and all VPAID.
    #[serde(rename="counterCustomEvents")]
    
    pub counter_custom_events: Option<Vec<CreativeCustomEvent>>,
    /// Required if dynamicAssetSelection is true.
    #[serde(rename="creativeAssetSelection")]
    
    pub creative_asset_selection: Option<CreativeAssetSelection>,
    /// Assets associated with a creative. Applicable to all but the following creative types: INTERNAL_REDIRECT, INTERSTITIAL_INTERNAL_REDIRECT, and REDIRECT
    #[serde(rename="creativeAssets")]
    
    pub creative_assets: Option<Vec<CreativeAsset>>,
    /// Creative field assignments for this creative. Applicable to all creative types.
    #[serde(rename="creativeFieldAssignments")]
    
    pub creative_field_assignments: Option<Vec<CreativeFieldAssignment>>,
    /// Custom key-values for a Rich Media creative. Key-values let you customize the creative settings of a Rich Media ad running on your site without having to contact the advertiser. You can use key-values to dynamically change the look or functionality of a creative. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
    #[serde(rename="customKeyValues")]
    
    pub custom_key_values: Option<Vec<String>>,
    /// Set this to true to enable the use of rules to target individual assets in this creative. When set to true creativeAssetSelection must be set. This also controls asset-level companions. When this is true, companion creatives should be assigned to creative assets. Learn more. Applicable to INSTREAM_VIDEO creatives.
    #[serde(rename="dynamicAssetSelection")]
    
    pub dynamic_asset_selection: Option<bool>,
    /// List of exit events configured for the creative. For DISPLAY and DISPLAY_IMAGE_GALLERY creatives, these are read-only and auto-generated from clickTags, For DISPLAY, an event is also created from the backupImageReportingLabel. Applicable to the following creative types: DISPLAY_IMAGE_GALLERY, all RICH_MEDIA, and all VPAID. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    #[serde(rename="exitCustomEvents")]
    
    pub exit_custom_events: Option<Vec<CreativeCustomEvent>>,
    /// OpenWindow FSCommand of this creative. This lets the SWF file communicate with either Flash Player or the program hosting Flash Player, such as a web browser. This is only triggered if allowScriptAccess field is true. Applicable to the following creative types: FLASH_INPAGE.
    #[serde(rename="fsCommand")]
    
    pub fs_command: Option<FsCommand>,
    /// HTML code for the creative. This is a required field when applicable. This field is ignored if htmlCodeLocked is true. Applicable to the following creative types: all CUSTOM, FLASH_INPAGE, and HTML5_BANNER, and all RICH_MEDIA.
    #[serde(rename="htmlCode")]
    
    pub html_code: Option<String>,
    /// Whether HTML code is generated by Campaign Manager or manually entered. Set to true to ignore changes to htmlCode. Applicable to the following creative types: FLASH_INPAGE and HTML5_BANNER.
    #[serde(rename="htmlCodeLocked")]
    
    pub html_code_locked: Option<bool>,
    /// ID of this creative. This is a read-only, auto-generated field. Applicable to all creative types.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Dimension value for the ID of this creative. This is a read-only field. Applicable to all creative types.
    #[serde(rename="idDimensionValue")]
    
    pub id_dimension_value: Option<DimensionValue>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#creative".
    
    pub kind: Option<String>,
    /// Creative last modification information. This is a read-only field. Applicable to all creative types.
    #[serde(rename="lastModifiedInfo")]
    
    pub last_modified_info: Option<LastModifiedInfo>,
    /// Latest Studio trafficked creative ID associated with rich media and VPAID creatives. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
    #[serde(rename="latestTraffickedCreativeId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub latest_trafficked_creative_id: Option<i64>,
    /// Name of the creative. This is a required field and must be less than 256 characters long. Applicable to all creative types.
    
    pub name: Option<String>,
    /// Override CSS value for rich media creatives. Applicable to the following creative types: all RICH_MEDIA.
    #[serde(rename="overrideCss")]
    
    pub override_css: Option<String>,
    /// Amount of time to play the video before counting a view. Applicable to the following creative types: all INSTREAM_VIDEO.
    #[serde(rename="progressOffset")]
    
    pub progress_offset: Option<VideoOffset>,
    /// URL of hosted image or hosted video or another ad tag. For INSTREAM_VIDEO_REDIRECT creatives this is the in-stream video redirect URL. The standard for a VAST (Video Ad Serving Template) ad response allows for a redirect link to another VAST 2.0 or 3.0 call. This is a required field when applicable. Applicable to the following creative types: DISPLAY_REDIRECT, INTERNAL_REDIRECT, INTERSTITIAL_INTERNAL_REDIRECT, and INSTREAM_VIDEO_REDIRECT
    #[serde(rename="redirectUrl")]
    
    pub redirect_url: Option<String>,
    /// ID of current rendering version. This is a read-only field. Applicable to all creative types.
    #[serde(rename="renderingId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub rendering_id: Option<i64>,
    /// Dimension value for the rendering ID of this creative. This is a read-only field. Applicable to all creative types.
    #[serde(rename="renderingIdDimensionValue")]
    
    pub rendering_id_dimension_value: Option<DimensionValue>,
    /// The minimum required Flash plugin version for this creative. For example, 11.2.202.235. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
    #[serde(rename="requiredFlashPluginVersion")]
    
    pub required_flash_plugin_version: Option<String>,
    /// The internal Flash version for this creative as calculated by Studio. This is a read-only field. Applicable to the following creative types: FLASH_INPAGE all RICH_MEDIA, and all VPAID. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    #[serde(rename="requiredFlashVersion")]
    
    pub required_flash_version: Option<i32>,
    /// Size associated with this creative. When inserting or updating a creative either the size ID field or size width and height fields can be used. This is a required field when applicable; however for IMAGE, FLASH_INPAGE creatives, and for DISPLAY creatives with a primary asset of type HTML_IMAGE, if left blank, this field will be automatically set using the actual size of the associated image assets. Applicable to the following creative types: DISPLAY, DISPLAY_IMAGE_GALLERY, FLASH_INPAGE, HTML5_BANNER, IMAGE, and all RICH_MEDIA.
    
    pub size: Option<Size>,
    /// Amount of time to play the video before the skip button appears. Applicable to the following creative types: all INSTREAM_VIDEO.
    #[serde(rename="skipOffset")]
    
    pub skip_offset: Option<VideoOffset>,
    /// Whether the user can choose to skip the creative. Applicable to the following creative types: all INSTREAM_VIDEO and all VPAID.
    
    pub skippable: Option<bool>,
    /// Whether the creative is SSL-compliant. This is a read-only field. Applicable to all creative types.
    #[serde(rename="sslCompliant")]
    
    pub ssl_compliant: Option<bool>,
    /// Whether creative should be treated as SSL compliant even if the system scan shows it's not. Applicable to all creative types.
    #[serde(rename="sslOverride")]
    
    pub ssl_override: Option<bool>,
    /// Studio advertiser ID associated with rich media and VPAID creatives. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
    #[serde(rename="studioAdvertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub studio_advertiser_id: Option<i64>,
    /// Studio creative ID associated with rich media and VPAID creatives. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
    #[serde(rename="studioCreativeId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub studio_creative_id: Option<i64>,
    /// Studio trafficked creative ID associated with rich media and VPAID creatives. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
    #[serde(rename="studioTraffickedCreativeId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub studio_trafficked_creative_id: Option<i64>,
    /// Subaccount ID of this creative. This field, if left unset, will be auto-generated for both insert and update operations. Applicable to all creative types.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Third-party URL used to record backup image impressions. Applicable to the following creative types: all RICH_MEDIA.
    #[serde(rename="thirdPartyBackupImageImpressionsUrl")]
    
    pub third_party_backup_image_impressions_url: Option<String>,
    /// Third-party URL used to record rich media impressions. Applicable to the following creative types: all RICH_MEDIA.
    #[serde(rename="thirdPartyRichMediaImpressionsUrl")]
    
    pub third_party_rich_media_impressions_url: Option<String>,
    /// Third-party URLs for tracking in-stream video creative events. Applicable to the following creative types: all INSTREAM_VIDEO and all VPAID.
    #[serde(rename="thirdPartyUrls")]
    
    pub third_party_urls: Option<Vec<ThirdPartyTrackingUrl>>,
    /// List of timer events configured for the creative. For DISPLAY_IMAGE_GALLERY creatives, these are read-only and auto-generated from clickTags. Applicable to the following creative types: DISPLAY_IMAGE_GALLERY, all RICH_MEDIA, and all VPAID. Applicable to DISPLAY when the primary asset is not HTML_IMAGE.
    #[serde(rename="timerCustomEvents")]
    
    pub timer_custom_events: Option<Vec<CreativeCustomEvent>>,
    /// Combined size of all creative assets. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
    #[serde(rename="totalFileSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_file_size: Option<i64>,
    /// Type of this creative. This is a required field. Applicable to all creative types.
    /// 
    /// Note: FLASH_INPAGE, HTML5_BANNER, and IMAGE are only used for existing creatives. New creatives should use DISPLAY as a replacement for these types.
    #[serde(rename="type")]
    
    pub type_: Option<CreativeTypeEnum>,
    /// A Universal Ad ID as per the VAST 4.0 spec. Applicable to the following creative types: INSTREAM_VIDEO and VPAID.
    #[serde(rename="universalAdId")]
    
    pub universal_ad_id: Option<UniversalAdId>,
    /// The version number helps you keep track of multiple versions of your creative in your reports. The version number will always be auto-generated during insert operations to start at 1. For tracking creatives the version cannot be incremented and will always remain at 1. For all other creative types the version can be incremented only by 1 during update operations. In addition, the version will be automatically incremented by 1 when undergoing Rich Media creative merging. Applicable to all creative types.
    
    pub version: Option<i32>,
    /// Description of the video ad. Applicable to the following creative types: all INSTREAM_VIDEO and all VPAID.
    #[serde(rename="videoDescription")]
    
    pub video_description: Option<String>,
    /// Creative video duration in seconds. This is a read-only field. Applicable to the following creative types: INSTREAM_VIDEO, all RICH_MEDIA, and all VPAID.
    #[serde(rename="videoDuration")]
    
    pub video_duration: Option<f32>,
}

impl client::RequestValue for Creative {}
impl client::Resource for Creative {}
impl client::ResponseResult for Creative {}


/// Creative Asset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert creative assets](CreativeAssetInsertCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeAsset {
    /// Whether ActionScript3 is enabled for the flash asset. This is a read-only field. Applicable to the following creative type: FLASH_INPAGE. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    #[serde(rename="actionScript3")]
    
    pub action_script3: Option<bool>,
    /// Whether the video asset is active. This is a read-only field for VPAID_NON_LINEAR_VIDEO assets. Applicable to the following creative types: INSTREAM_VIDEO and all VPAID.
    
    pub active: Option<bool>,
    /// Possible alignments for an asset. This is a read-only field. Applicable to the following creative types: RICH_MEDIA_DISPLAY_MULTI_FLOATING_INTERSTITIAL.
    
    pub alignment: Option<CreativeAssetAlignmentEnum>,
    /// Artwork type of rich media creative. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA.
    #[serde(rename="artworkType")]
    
    pub artwork_type: Option<CreativeAssetArtworkTypeEnum>,
    /// Identifier of this asset. This is the same identifier returned during creative asset insert operation. This is a required field. Applicable to all but the following creative types: all REDIRECT and TRACKING_TEXT.
    #[serde(rename="assetIdentifier")]
    
    pub asset_identifier: Option<CreativeAssetId>,
    /// Exit event configured for the backup image. Applicable to the following creative types: all RICH_MEDIA.
    #[serde(rename="backupImageExit")]
    
    pub backup_image_exit: Option<CreativeCustomEvent>,
    /// Detected bit-rate for video asset. This is a read-only field. Applicable to the following creative types: INSTREAM_VIDEO and all VPAID.
    #[serde(rename="bitRate")]
    
    pub bit_rate: Option<i32>,
    /// Rich media child asset type. This is a read-only field. Applicable to the following creative types: all VPAID.
    #[serde(rename="childAssetType")]
    
    pub child_asset_type: Option<CreativeAssetChildAssetTypeEnum>,
    /// Size of an asset when collapsed. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA and all VPAID. Additionally, applicable to assets whose displayType is ASSET_DISPLAY_TYPE_EXPANDING or ASSET_DISPLAY_TYPE_PEEL_DOWN.
    #[serde(rename="collapsedSize")]
    
    pub collapsed_size: Option<Size>,
    /// List of companion creatives assigned to an in-stream video creative asset. Acceptable values include IDs of existing flash and image creatives. Applicable to INSTREAM_VIDEO creative type with dynamicAssetSelection set to true.
    #[serde(rename="companionCreativeIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub companion_creative_ids: Option<Vec<i64>>,
    /// Custom start time in seconds for making the asset visible. Applicable to the following creative types: all RICH_MEDIA. Value must be greater than or equal to 0.
    #[serde(rename="customStartTimeValue")]
    
    pub custom_start_time_value: Option<i32>,
    /// List of feature dependencies for the creative asset that are detected by Campaign Manager. Feature dependencies are features that a browser must be able to support in order to render your HTML5 creative correctly. This is a read-only, auto-generated field. Applicable to the following creative types: HTML5_BANNER. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    #[serde(rename="detectedFeatures")]
    
    pub detected_features: Option<Vec<CreativeAssetDetectedFeaturesEnum>>,
    /// Type of rich media asset. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA.
    #[serde(rename="displayType")]
    
    pub display_type: Option<CreativeAssetDisplayTypeEnum>,
    /// Duration in seconds for which an asset will be displayed. Applicable to the following creative types: INSTREAM_VIDEO and VPAID_LINEAR_VIDEO. Value must be greater than or equal to 1.
    
    pub duration: Option<i32>,
    /// Duration type for which an asset will be displayed. Applicable to the following creative types: all RICH_MEDIA.
    #[serde(rename="durationType")]
    
    pub duration_type: Option<CreativeAssetDurationTypeEnum>,
    /// Detected expanded dimension for video asset. This is a read-only field. Applicable to the following creative types: INSTREAM_VIDEO and all VPAID.
    #[serde(rename="expandedDimension")]
    
    pub expanded_dimension: Option<Size>,
    /// File size associated with this creative asset. This is a read-only field. Applicable to all but the following creative types: all REDIRECT and TRACKING_TEXT.
    #[serde(rename="fileSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub file_size: Option<i64>,
    /// Flash version of the asset. This is a read-only field. Applicable to the following creative types: FLASH_INPAGE, all RICH_MEDIA, and all VPAID. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    #[serde(rename="flashVersion")]
    
    pub flash_version: Option<i32>,
    /// Whether to hide Flash objects flag for an asset. Applicable to the following creative types: all RICH_MEDIA.
    #[serde(rename="hideFlashObjects")]
    
    pub hide_flash_objects: Option<bool>,
    /// Whether to hide selection boxes flag for an asset. Applicable to the following creative types: all RICH_MEDIA.
    #[serde(rename="hideSelectionBoxes")]
    
    pub hide_selection_boxes: Option<bool>,
    /// Whether the asset is horizontally locked. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA.
    #[serde(rename="horizontallyLocked")]
    
    pub horizontally_locked: Option<bool>,
    /// Numeric ID of this creative asset. This is a required field and should not be modified. Applicable to all but the following creative types: all REDIRECT and TRACKING_TEXT.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Dimension value for the ID of the asset. This is a read-only, auto-generated field.
    #[serde(rename="idDimensionValue")]
    
    pub id_dimension_value: Option<DimensionValue>,
    /// Detected MIME type for video asset. This is a read-only field. Applicable to the following creative types: INSTREAM_VIDEO and all VPAID.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Offset position for an asset in collapsed mode. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA and all VPAID. Additionally, only applicable to assets whose displayType is ASSET_DISPLAY_TYPE_EXPANDING or ASSET_DISPLAY_TYPE_PEEL_DOWN.
    
    pub offset: Option<OffsetPosition>,
    /// Whether the backup asset is original or changed by the user in Campaign Manager. Applicable to the following creative types: all RICH_MEDIA.
    #[serde(rename="originalBackup")]
    
    pub original_backup: Option<bool>,
    /// Offset position for an asset. Applicable to the following creative types: all RICH_MEDIA.
    
    pub position: Option<OffsetPosition>,
    /// Offset left unit for an asset. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA.
    #[serde(rename="positionLeftUnit")]
    
    pub position_left_unit: Option<CreativeAssetPositionLeftUnitEnum>,
    /// Offset top unit for an asset. This is a read-only field if the asset displayType is ASSET_DISPLAY_TYPE_OVERLAY. Applicable to the following creative types: all RICH_MEDIA.
    #[serde(rename="positionTopUnit")]
    
    pub position_top_unit: Option<CreativeAssetPositionTopUnitEnum>,
    /// Progressive URL for video asset. This is a read-only field. Applicable to the following creative types: INSTREAM_VIDEO and all VPAID.
    #[serde(rename="progressiveServingUrl")]
    
    pub progressive_serving_url: Option<String>,
    /// Whether the asset pushes down other content. Applicable to the following creative types: all RICH_MEDIA. Additionally, only applicable when the asset offsets are 0, the collapsedSize.width matches size.width, and the collapsedSize.height is less than size.height.
    
    pub pushdown: Option<bool>,
    /// Pushdown duration in seconds for an asset. Applicable to the following creative types: all RICH_MEDIA.Additionally, only applicable when the asset pushdown field is true, the offsets are 0, the collapsedSize.width matches size.width, and the collapsedSize.height is less than size.height. Acceptable values are 0 to 9.99, inclusive.
    #[serde(rename="pushdownDuration")]
    
    pub pushdown_duration: Option<f32>,
    /// Role of the asset in relation to creative. Applicable to all but the following creative types: all REDIRECT and TRACKING_TEXT. This is a required field.
    /// PRIMARY applies to DISPLAY, FLASH_INPAGE, HTML5_BANNER, IMAGE, DISPLAY_IMAGE_GALLERY, all RICH_MEDIA (which may contain multiple primary assets), and all VPAID creatives.
    /// BACKUP_IMAGE applies to FLASH_INPAGE, HTML5_BANNER, all RICH_MEDIA, and all VPAID creatives. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    /// ADDITIONAL_IMAGE and ADDITIONAL_FLASH apply to FLASH_INPAGE creatives.
    /// OTHER refers to assets from sources other than Campaign Manager, such as Studio uploaded assets, applicable to all RICH_MEDIA and all VPAID creatives.
    /// PARENT_VIDEO refers to videos uploaded by the user in Campaign Manager and is applicable to INSTREAM_VIDEO and VPAID_LINEAR_VIDEO creatives.
    /// TRANSCODED_VIDEO refers to videos transcoded by Campaign Manager from PARENT_VIDEO assets and is applicable to INSTREAM_VIDEO and VPAID_LINEAR_VIDEO creatives.
    /// ALTERNATE_VIDEO refers to the Campaign Manager representation of child asset videos from Studio, and is applicable to VPAID_LINEAR_VIDEO creatives. These cannot be added or removed within Campaign Manager.
    /// For VPAID_LINEAR_VIDEO creatives, PARENT_VIDEO, TRANSCODED_VIDEO and ALTERNATE_VIDEO assets that are marked active serve as backup in case the VPAID creative cannot be served. Only PARENT_VIDEO assets can be added or removed for an INSTREAM_VIDEO or VPAID_LINEAR_VIDEO creative.
    
    pub role: Option<CreativeAssetRoleEnum>,
    /// Size associated with this creative asset. This is a required field when applicable; however for IMAGE and FLASH_INPAGE, creatives if left blank, this field will be automatically set using the actual size of the associated image asset. Applicable to the following creative types: DISPLAY_IMAGE_GALLERY, FLASH_INPAGE, HTML5_BANNER, IMAGE, and all RICH_MEDIA. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    
    pub size: Option<Size>,
    /// Whether the asset is SSL-compliant. This is a read-only field. Applicable to all but the following creative types: all REDIRECT and TRACKING_TEXT.
    #[serde(rename="sslCompliant")]
    
    pub ssl_compliant: Option<bool>,
    /// Initial wait time type before making the asset visible. Applicable to the following creative types: all RICH_MEDIA.
    #[serde(rename="startTimeType")]
    
    pub start_time_type: Option<CreativeAssetStartTimeTypeEnum>,
    /// Streaming URL for video asset. This is a read-only field. Applicable to the following creative types: INSTREAM_VIDEO and all VPAID.
    #[serde(rename="streamingServingUrl")]
    
    pub streaming_serving_url: Option<String>,
    /// Whether the asset is transparent. Applicable to the following creative types: all RICH_MEDIA. Additionally, only applicable to HTML5 assets.
    
    pub transparency: Option<bool>,
    /// Whether the asset is vertically locked. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA.
    #[serde(rename="verticallyLocked")]
    
    pub vertically_locked: Option<bool>,
    /// Detected video duration for video asset. This is a read-only field. Applicable to the following creative types: INSTREAM_VIDEO and all VPAID.
    #[serde(rename="videoDuration")]
    
    pub video_duration: Option<f32>,
    /// Window mode options for flash assets. Applicable to the following creative types: FLASH_INPAGE, RICH_MEDIA_DISPLAY_EXPANDING, RICH_MEDIA_IM_EXPAND, RICH_MEDIA_DISPLAY_BANNER, and RICH_MEDIA_INPAGE_FLOATING.
    #[serde(rename="windowMode")]
    
    pub window_mode: Option<CreativeAssetWindowModeEnum>,
    /// zIndex value of an asset. Applicable to the following creative types: all RICH_MEDIA.Additionally, only applicable to assets whose displayType is NOT one of the following types: ASSET_DISPLAY_TYPE_INPAGE or ASSET_DISPLAY_TYPE_OVERLAY. Acceptable values are -999999999 to 999999999, inclusive.
    #[serde(rename="zIndex")]
    
    pub z_index: Option<i32>,
    /// File name of zip file. This is a read-only field. Applicable to the following creative types: HTML5_BANNER.
    #[serde(rename="zipFilename")]
    
    pub zip_filename: Option<String>,
    /// Size of zip file. This is a read-only field. Applicable to the following creative types: HTML5_BANNER.
    #[serde(rename="zipFilesize")]
    
    pub zip_filesize: Option<String>,
}

impl client::Resource for CreativeAsset {}


/// Creative Asset ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeAssetId {
    /// Name of the creative asset. This is a required field while inserting an asset. After insertion, this assetIdentifier is used to identify the uploaded asset. Characters in the name must be alphanumeric or one of the following: ".-_ ". Spaces are allowed.
    
    pub name: Option<String>,
    /// Type of asset to upload. This is a required field. FLASH and IMAGE are no longer supported for new uploads. All image assets should use HTML_IMAGE.
    #[serde(rename="type")]
    
    pub type_: Option<CreativeAssetIdTypeEnum>,
}

impl client::Part for CreativeAssetId {}


/// CreativeAssets contains properties of a creative asset file which will be uploaded or has already been uploaded. Refer to the creative sample code for how to upload assets and insert a creative.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert creative assets](CreativeAssetInsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeAssetMetadata {
    /// ID of the creative asset. This is a required field.
    #[serde(rename="assetIdentifier")]
    
    pub asset_identifier: Option<CreativeAssetId>,
    /// List of detected click tags for assets. This is a read-only auto-generated field.
    #[serde(rename="clickTags")]
    
    pub click_tags: Option<Vec<ClickTag>>,
    /// List of feature dependencies for the creative asset that are detected by Campaign Manager. Feature dependencies are features that a browser must be able to support in order to render your HTML5 creative correctly. This is a read-only, auto-generated field.
    #[serde(rename="detectedFeatures")]
    
    pub detected_features: Option<Vec<CreativeAssetMetadataDetectedFeaturesEnum>>,
    /// Numeric ID of the asset. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Dimension value for the numeric ID of the asset. This is a read-only, auto-generated field.
    #[serde(rename="idDimensionValue")]
    
    pub id_dimension_value: Option<DimensionValue>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#creativeAssetMetadata".
    
    pub kind: Option<String>,
    /// Rules validated during code generation that generated a warning. This is a read-only, auto-generated field.
    /// 
    /// Possible values are:
    /// - "ADMOB_REFERENCED"
    /// - "ASSET_FORMAT_UNSUPPORTED_DCM"
    /// - "ASSET_INVALID"
    /// - "CLICK_TAG_HARD_CODED"
    /// - "CLICK_TAG_INVALID"
    /// - "CLICK_TAG_IN_GWD"
    /// - "CLICK_TAG_MISSING"
    /// - "CLICK_TAG_MORE_THAN_ONE"
    /// - "CLICK_TAG_NON_TOP_LEVEL"
    /// - "COMPONENT_UNSUPPORTED_DCM"
    /// - "ENABLER_UNSUPPORTED_METHOD_DCM"
    /// - "EXTERNAL_FILE_REFERENCED"
    /// - "FILE_DETAIL_EMPTY"
    /// - "FILE_TYPE_INVALID"
    /// - "GWD_PROPERTIES_INVALID"
    /// - "HTML5_FEATURE_UNSUPPORTED"
    /// - "LINKED_FILE_NOT_FOUND"
    /// - "MAX_FLASH_VERSION_11"
    /// - "MRAID_REFERENCED"
    /// - "NOT_SSL_COMPLIANT"
    /// - "ORPHANED_ASSET"
    /// - "PRIMARY_HTML_MISSING"
    /// - "SVG_INVALID"
    /// - "ZIP_INVALID"
    #[serde(rename="warnedValidationRules")]
    
    pub warned_validation_rules: Option<Vec<CreativeAssetMetadataWarnedValidationRulesEnum>>,
}

impl client::RequestValue for CreativeAssetMetadata {}
impl client::ResponseResult for CreativeAssetMetadata {}


/// Encapsulates the list of rules for asset selection and a default asset in case none of the rules match. Applicable to INSTREAM_VIDEO creatives.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeAssetSelection {
    /// A creativeAssets[].id. This should refer to one of the parent assets in this creative, and will be served if none of the rules match. This is a required field.
    #[serde(rename="defaultAssetId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub default_asset_id: Option<i64>,
    /// Rules determine which asset will be served to a viewer. Rules will be evaluated in the order in which they are stored in this list. This list must contain at least one rule. Applicable to INSTREAM_VIDEO creatives.
    
    pub rules: Option<Vec<Rule>>,
}

impl client::Part for CreativeAssetSelection {}


/// Creative Assignment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeAssignment {
    /// Whether this creative assignment is active. When true, the creative will be included in the ad's rotation.
    
    pub active: Option<bool>,
    /// Whether applicable event tags should fire when this creative assignment is rendered. If this value is unset when the ad is inserted or updated, it will default to true for all creative types EXCEPT for INTERNAL_REDIRECT, INTERSTITIAL_INTERNAL_REDIRECT, and INSTREAM_VIDEO.
    #[serde(rename="applyEventTags")]
    
    pub apply_event_tags: Option<bool>,
    /// Click-through URL of the creative assignment.
    #[serde(rename="clickThroughUrl")]
    
    pub click_through_url: Option<ClickThroughUrl>,
    /// Companion creative overrides for this creative assignment. Applicable to video ads.
    #[serde(rename="companionCreativeOverrides")]
    
    pub companion_creative_overrides: Option<Vec<CompanionClickThroughOverride>>,
    /// Creative group assignments for this creative assignment. Only one assignment per creative group number is allowed for a maximum of two assignments.
    #[serde(rename="creativeGroupAssignments")]
    
    pub creative_group_assignments: Option<Vec<CreativeGroupAssignment>>,
    /// ID of the creative to be assigned. This is a required field.
    #[serde(rename="creativeId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creative_id: Option<i64>,
    /// Dimension value for the ID of the creative. This is a read-only, auto-generated field.
    #[serde(rename="creativeIdDimensionValue")]
    
    pub creative_id_dimension_value: Option<DimensionValue>,
    /// Date and time that the assigned creative should stop serving. Must be later than the start time.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Rich media exit overrides for this creative assignment.
    /// Applicable when the creative type is any of the following: 
    /// - DISPLAY
    /// - RICH_MEDIA_INPAGE
    /// - RICH_MEDIA_INPAGE_FLOATING
    /// - RICH_MEDIA_IM_EXPAND
    /// - RICH_MEDIA_EXPANDING
    /// - RICH_MEDIA_INTERSTITIAL_FLOAT
    /// - RICH_MEDIA_MOBILE_IN_APP
    /// - RICH_MEDIA_MULTI_FLOATING
    /// - RICH_MEDIA_PEEL_DOWN
    /// - VPAID_LINEAR
    /// - VPAID_NON_LINEAR
    #[serde(rename="richMediaExitOverrides")]
    
    pub rich_media_exit_overrides: Option<Vec<RichMediaExitOverride>>,
    /// Sequence number of the creative assignment, applicable when the rotation type is CREATIVE_ROTATION_TYPE_SEQUENTIAL. Acceptable values are 1 to 65535, inclusive.
    
    pub sequence: Option<i32>,
    /// Whether the creative to be assigned is SSL-compliant. This is a read-only field that is auto-generated when the ad is inserted or updated.
    #[serde(rename="sslCompliant")]
    
    pub ssl_compliant: Option<bool>,
    /// Date and time that the assigned creative should start serving.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Weight of the creative assignment, applicable when the rotation type is CREATIVE_ROTATION_TYPE_RANDOM. Value must be greater than or equal to 1.
    
    pub weight: Option<i32>,
}

impl client::Part for CreativeAssignment {}


/// Creative Custom Event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeCustomEvent {
    /// Unique ID of this event used by Reporting and Data Transfer. This is a read-only field.
    #[serde(rename="advertiserCustomEventId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_custom_event_id: Option<i64>,
    /// User-entered name for the event.
    #[serde(rename="advertiserCustomEventName")]
    
    pub advertiser_custom_event_name: Option<String>,
    /// Type of the event. This is a read-only field.
    #[serde(rename="advertiserCustomEventType")]
    
    pub advertiser_custom_event_type: Option<CreativeCustomEventAdvertiserCustomEventTypeEnum>,
    /// Artwork label column, used to link events in Campaign Manager back to events in Studio. This is a required field and should not be modified after insertion.
    #[serde(rename="artworkLabel")]
    
    pub artwork_label: Option<String>,
    /// Artwork type used by the creative.This is a read-only field.
    #[serde(rename="artworkType")]
    
    pub artwork_type: Option<CreativeCustomEventArtworkTypeEnum>,
    /// Exit URL of the event. This field is used only for exit events.
    #[serde(rename="exitUrl")]
    
    pub exit_url: Option<String>,
    /// ID of this event. This is a required field and should not be modified after insertion.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Properties for rich media popup windows. This field is used only for exit events.
    #[serde(rename="popupWindowProperties")]
    
    pub popup_window_properties: Option<PopupWindowProperties>,
    /// Target type used by the event.
    #[serde(rename="targetType")]
    
    pub target_type: Option<CreativeCustomEventTargetTypeEnum>,
    /// Video reporting ID, used to differentiate multiple videos in a single creative. This is a read-only field.
    #[serde(rename="videoReportingId")]
    
    pub video_reporting_id: Option<String>,
}

impl client::Part for CreativeCustomEvent {}


/// Contains properties of a creative field.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete creative fields](CreativeFieldDeleteCall) (none)
/// * [get creative fields](CreativeFieldGetCall) (response)
/// * [insert creative fields](CreativeFieldInsertCall) (request|response)
/// * [list creative fields](CreativeFieldListCall) (none)
/// * [patch creative fields](CreativeFieldPatchCall) (request|response)
/// * [update creative fields](CreativeFieldUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeField {
    /// Account ID of this creative field. This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Advertiser ID of this creative field. This is a required field on insertion.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Dimension value for the ID of the advertiser. This is a read-only, auto-generated field.
    #[serde(rename="advertiserIdDimensionValue")]
    
    pub advertiser_id_dimension_value: Option<DimensionValue>,
    /// ID of this creative field. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#creativeField".
    
    pub kind: Option<String>,
    /// Name of this creative field. This is a required field and must be less than 256 characters long and unique among creative fields of the same advertiser.
    
    pub name: Option<String>,
    /// Subaccount ID of this creative field. This is a read-only field that can be left blank.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
}

impl client::RequestValue for CreativeField {}
impl client::Resource for CreativeField {}
impl client::ResponseResult for CreativeField {}


/// Creative Field Assignment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeFieldAssignment {
    /// ID of the creative field.
    #[serde(rename="creativeFieldId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creative_field_id: Option<i64>,
    /// ID of the creative field value.
    #[serde(rename="creativeFieldValueId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creative_field_value_id: Option<i64>,
}

impl client::Part for CreativeFieldAssignment {}


/// Contains properties of a creative field value.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete creative field values](CreativeFieldValueDeleteCall) (none)
/// * [get creative field values](CreativeFieldValueGetCall) (response)
/// * [insert creative field values](CreativeFieldValueInsertCall) (request|response)
/// * [list creative field values](CreativeFieldValueListCall) (none)
/// * [patch creative field values](CreativeFieldValuePatchCall) (request|response)
/// * [update creative field values](CreativeFieldValueUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeFieldValue {
    /// ID of this creative field value. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#creativeFieldValue".
    
    pub kind: Option<String>,
    /// Value of this creative field value. It needs to be less than 256 characters in length and unique per creative field.
    
    pub value: Option<String>,
}

impl client::RequestValue for CreativeFieldValue {}
impl client::Resource for CreativeFieldValue {}
impl client::ResponseResult for CreativeFieldValue {}


/// Creative Field Value List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list creative field values](CreativeFieldValueListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeFieldValuesListResponse {
    /// Creative field value collection.
    #[serde(rename="creativeFieldValues")]
    
    pub creative_field_values: Option<Vec<CreativeFieldValue>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#creativeFieldValuesListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for CreativeFieldValuesListResponse {}


/// Creative Field List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list creative fields](CreativeFieldListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeFieldsListResponse {
    /// Creative field collection.
    #[serde(rename="creativeFields")]
    
    pub creative_fields: Option<Vec<CreativeField>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#creativeFieldsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for CreativeFieldsListResponse {}


/// Contains properties of a creative group.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get creative groups](CreativeGroupGetCall) (response)
/// * [insert creative groups](CreativeGroupInsertCall) (request|response)
/// * [list creative groups](CreativeGroupListCall) (none)
/// * [patch creative groups](CreativeGroupPatchCall) (request|response)
/// * [update creative groups](CreativeGroupUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeGroup {
    /// Account ID of this creative group. This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Advertiser ID of this creative group. This is a required field on insertion.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Dimension value for the ID of the advertiser. This is a read-only, auto-generated field.
    #[serde(rename="advertiserIdDimensionValue")]
    
    pub advertiser_id_dimension_value: Option<DimensionValue>,
    /// Subgroup of the creative group. Assign your creative groups to a subgroup in order to filter or manage them more easily. This field is required on insertion and is read-only after insertion. Acceptable values are 1 to 2, inclusive.
    #[serde(rename="groupNumber")]
    
    pub group_number: Option<i32>,
    /// ID of this creative group. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#creativeGroup".
    
    pub kind: Option<String>,
    /// Name of this creative group. This is a required field and must be less than 256 characters long and unique among creative groups of the same advertiser.
    
    pub name: Option<String>,
    /// Subaccount ID of this creative group. This is a read-only field that can be left blank.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
}

impl client::RequestValue for CreativeGroup {}
impl client::Resource for CreativeGroup {}
impl client::ResponseResult for CreativeGroup {}


/// Creative Group Assignment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeGroupAssignment {
    /// ID of the creative group to be assigned.
    #[serde(rename="creativeGroupId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creative_group_id: Option<i64>,
    /// Creative group number of the creative group assignment.
    #[serde(rename="creativeGroupNumber")]
    
    pub creative_group_number: Option<CreativeGroupAssignmentCreativeGroupNumberEnum>,
}

impl client::Part for CreativeGroupAssignment {}


/// Creative Group List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list creative groups](CreativeGroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeGroupsListResponse {
    /// Creative group collection.
    #[serde(rename="creativeGroups")]
    
    pub creative_groups: Option<Vec<CreativeGroup>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#creativeGroupsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for CreativeGroupsListResponse {}


/// Creative optimization settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeOptimizationConfiguration {
    /// ID of this creative optimization config. This field is auto-generated when the campaign is inserted or updated. It can be null for existing campaigns.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Name of this creative optimization config. This is a required field and must be less than 129 characters long.
    
    pub name: Option<String>,
    /// List of optimization activities associated with this configuration.
    #[serde(rename="optimizationActivitys")]
    
    pub optimization_activitys: Option<Vec<OptimizationActivity>>,
    /// Optimization model for this configuration.
    #[serde(rename="optimizationModel")]
    
    pub optimization_model: Option<CreativeOptimizationConfigurationOptimizationModelEnum>,
}

impl client::Part for CreativeOptimizationConfiguration {}


/// Creative Rotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeRotation {
    /// Creative assignments in this creative rotation.
    #[serde(rename="creativeAssignments")]
    
    pub creative_assignments: Option<Vec<CreativeAssignment>>,
    /// Creative optimization configuration that is used by this ad. It should refer to one of the existing optimization configurations in the ad's campaign. If it is unset or set to 0, then the campaign's default optimization configuration will be used for this ad.
    #[serde(rename="creativeOptimizationConfigurationId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creative_optimization_configuration_id: Option<i64>,
    /// Type of creative rotation. Can be used to specify whether to use sequential or random rotation.
    #[serde(rename="type")]
    
    pub type_: Option<CreativeRotationTypeEnum>,
    /// Strategy for calculating weights. Used with CREATIVE_ROTATION_TYPE_RANDOM.
    #[serde(rename="weightCalculationStrategy")]
    
    pub weight_calculation_strategy: Option<CreativeRotationWeightCalculationStrategyEnum>,
}

impl client::Part for CreativeRotation {}


/// Creative Settings
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeSettings {
    /// Header text for iFrames for this site. Must be less than or equal to 2000 characters long.
    #[serde(rename="iFrameFooter")]
    
    pub i_frame_footer: Option<String>,
    /// Header text for iFrames for this site. Must be less than or equal to 2000 characters long.
    #[serde(rename="iFrameHeader")]
    
    pub i_frame_header: Option<String>,
}

impl client::Part for CreativeSettings {}


/// Creative List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list creatives](CreativeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativesListResponse {
    /// Creative collection.
    
    pub creatives: Option<Vec<Creative>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#creativesListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for CreativesListResponse {}


/// Represents fields that are compatible to be selected for a report of type "CROSS_DIMENSION_REACH".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CrossDimensionReachReportCompatibleFields {
    /// Dimensions which are compatible to be selected in the "breakdown" section of the report.
    
    pub breakdown: Option<Vec<Dimension>>,
    /// Dimensions which are compatible to be selected in the "dimensionFilters" section of the report.
    #[serde(rename="dimensionFilters")]
    
    pub dimension_filters: Option<Vec<Dimension>>,
    /// The kind of resource this is, in this case dfareporting#crossDimensionReachReportCompatibleFields.
    
    pub kind: Option<String>,
    /// Metrics which are compatible to be selected in the "metricNames" section of the report.
    
    pub metrics: Option<Vec<Metric>>,
    /// Metrics which are compatible to be selected in the "overlapMetricNames" section of the report.
    #[serde(rename="overlapMetrics")]
    
    pub overlap_metrics: Option<Vec<Metric>>,
}

impl client::Part for CrossDimensionReachReportCompatibleFields {}


/// A custom floodlight variable.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomFloodlightVariable {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#customFloodlightVariable".
    
    pub kind: Option<String>,
    /// The type of custom floodlight variable to supply a value for. These map to the "u[1-20]=" in the tags.
    #[serde(rename="type")]
    
    pub type_: Option<CustomFloodlightVariableTypeEnum>,
    /// The value of the custom floodlight variable. The length of string must not exceed 50 characters.
    
    pub value: Option<String>,
}

impl client::Part for CustomFloodlightVariable {}


/// Represents a Custom Rich Media Events group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomRichMediaEvents {
    /// List of custom rich media event IDs. Dimension values must be all of type dfa:richMediaEventTypeIdAndName.
    #[serde(rename="filteredEventIds")]
    
    pub filtered_event_ids: Option<Vec<DimensionValue>>,
    /// The kind of resource this is, in this case dfareporting#customRichMediaEvents.
    
    pub kind: Option<String>,
}

impl client::Part for CustomRichMediaEvents {}


/// Represents a date range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateRange {
    /// The end date of the date range, inclusive. A string of the format: "yyyy-MM-dd".
    #[serde(rename="endDate")]
    
    pub end_date: Option<client::chrono::NaiveDate>,
    /// The kind of resource this is, in this case dfareporting#dateRange.
    
    pub kind: Option<String>,
    /// The date range relative to the date of when the report is run.
    #[serde(rename="relativeDateRange")]
    
    pub relative_date_range: Option<DateRangeRelativeDateRangeEnum>,
    /// The start date of the date range, inclusive. A string of the format: "yyyy-MM-dd".
    #[serde(rename="startDate")]
    
    pub start_date: Option<client::chrono::NaiveDate>,
}

impl client::Part for DateRange {}


/// Day Part Targeting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DayPartTargeting {
    /// Days of the week when the ad will serve.
    /// 
    /// Acceptable values are:
    /// - "SUNDAY"
    /// - "MONDAY"
    /// - "TUESDAY"
    /// - "WEDNESDAY"
    /// - "THURSDAY"
    /// - "FRIDAY"
    /// - "SATURDAY"
    #[serde(rename="daysOfWeek")]
    
    pub days_of_week: Option<Vec<DayPartTargetingDaysOfWeekEnum>>,
    /// Hours of the day when the ad will serve, where 0 is midnight to 1 AM and 23 is 11 PM to midnight. Can be specified with days of week, in which case the ad would serve during these hours on the specified days. For example if Monday, Wednesday, Friday are the days of week specified and 9-10am, 3-5pm (hours 9, 15, and 16) is specified, the ad would serve Monday, Wednesdays, and Fridays at 9-10am and 3-5pm. Acceptable values are 0 to 23, inclusive.
    #[serde(rename="hoursOfDay")]
    
    pub hours_of_day: Option<Vec<i32>>,
    /// Whether or not to use the user's local time. If false, the America/New York time zone applies.
    #[serde(rename="userLocalTime")]
    
    pub user_local_time: Option<bool>,
}

impl client::Part for DayPartTargeting {}


/// Properties of inheriting and overriding the default click-through event tag. A campaign may override the event tag defined at the advertiser level, and an ad may also override the campaign's setting further.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DefaultClickThroughEventTagProperties {
    /// ID of the click-through event tag to apply to all ads in this entity's scope.
    #[serde(rename="defaultClickThroughEventTagId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub default_click_through_event_tag_id: Option<i64>,
    /// Whether this entity should override the inherited default click-through event tag with its own defined value.
    #[serde(rename="overrideInheritedEventTag")]
    
    pub override_inherited_event_tag: Option<bool>,
}

impl client::Part for DefaultClickThroughEventTagProperties {}


/// Delivery Schedule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeliverySchedule {
    /// Limit on the number of times an individual user can be served the ad within a specified period of time.
    #[serde(rename="frequencyCap")]
    
    pub frequency_cap: Option<FrequencyCap>,
    /// Whether or not hard cutoff is enabled. If true, the ad will not serve after the end date and time. Otherwise the ad will continue to be served until it has reached its delivery goals.
    #[serde(rename="hardCutoff")]
    
    pub hard_cutoff: Option<bool>,
    /// Impression ratio for this ad. This ratio determines how often each ad is served relative to the others. For example, if ad A has an impression ratio of 1 and ad B has an impression ratio of 3, then Campaign Manager will serve ad B three times as often as ad A. Acceptable values are 1 to 10, inclusive.
    #[serde(rename="impressionRatio")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub impression_ratio: Option<i64>,
    /// Serving priority of an ad, with respect to other ads. The lower the priority number, the greater the priority with which it is served.
    
    pub priority: Option<DeliverySchedulePriorityEnum>,
}

impl client::Part for DeliverySchedule {}


/// Google Ad Manager Settings
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DfpSettings {
    /// Ad Manager network code for this directory site.
    #[serde(rename="dfpNetworkCode")]
    
    pub dfp_network_code: Option<String>,
    /// Ad Manager network name for this directory site.
    #[serde(rename="dfpNetworkName")]
    
    pub dfp_network_name: Option<String>,
    /// Whether this directory site accepts programmatic placements.
    #[serde(rename="programmaticPlacementAccepted")]
    
    pub programmatic_placement_accepted: Option<bool>,
    /// Whether this directory site accepts publisher-paid tags.
    #[serde(rename="pubPaidPlacementAccepted")]
    
    pub pub_paid_placement_accepted: Option<bool>,
    /// Whether this directory site is available only via Publisher Portal.
    #[serde(rename="publisherPortalOnly")]
    
    pub publisher_portal_only: Option<bool>,
}

impl client::Part for DfpSettings {}


/// Represents a dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dimension {
    /// The kind of resource this is, in this case dfareporting#dimension.
    
    pub kind: Option<String>,
    /// The dimension name, e.g. dfa:advertiser
    
    pub name: Option<String>,
}

impl client::Part for Dimension {}


/// Represents a dimension filter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionFilter {
    /// The name of the dimension to filter.
    #[serde(rename="dimensionName")]
    
    pub dimension_name: Option<String>,
    /// The kind of resource this is, in this case dfareporting#dimensionFilter.
    
    pub kind: Option<String>,
    /// The value of the dimension to filter.
    
    pub value: Option<String>,
}

impl client::Part for DimensionFilter {}


/// Represents a DimensionValue resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query dimension values](DimensionValueQueryCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionValue {
    /// The name of the dimension.
    #[serde(rename="dimensionName")]
    
    pub dimension_name: Option<String>,
    /// The eTag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The ID associated with the value if available.
    
    pub id: Option<String>,
    /// The kind of resource this is, in this case dfareporting#dimensionValue.
    
    pub kind: Option<String>,
    /// Determines how the 'value' field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, '*' is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions ('dfa:paidSearch*') allow a matchType other than EXACT.
    #[serde(rename="matchType")]
    
    pub match_type: Option<DimensionValueMatchTypeEnum>,
    /// The value of the dimension.
    
    pub value: Option<String>,
}

impl client::Resource for DimensionValue {}


/// Represents the list of DimensionValue resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query dimension values](DimensionValueQueryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionValueList {
    /// The eTag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The dimension values returned in this response.
    
    pub items: Option<Vec<DimensionValue>>,
    /// The kind of list this is, in this case dfareporting#dimensionValueList.
    
    pub kind: Option<String>,
    /// Continuation token used to page through dimension values. To retrieve the next page of results, set the next request's "pageToken" to the value of this field. The page token is only valid for a limited amount of time and should not be persisted.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for DimensionValueList {}


/// Represents a DimensionValuesRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query dimension values](DimensionValueQueryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionValueRequest {
    /// The name of the dimension for which values should be requested.
    #[serde(rename="dimensionName")]
    
    pub dimension_name: Option<String>,
    /// The end date of the date range for which to retrieve dimension values. A string of the format "yyyy-MM-dd".
    #[serde(rename="endDate")]
    
    pub end_date: Option<client::chrono::NaiveDate>,
    /// The list of filters by which to filter values. The filters are ANDed.
    
    pub filters: Option<Vec<DimensionFilter>>,
    /// The kind of request this is, in this case dfareporting#dimensionValueRequest.
    
    pub kind: Option<String>,
    /// The start date of the date range for which to retrieve dimension values. A string of the format "yyyy-MM-dd".
    #[serde(rename="startDate")]
    
    pub start_date: Option<client::chrono::NaiveDate>,
}

impl client::RequestValue for DimensionValueRequest {}


/// DirectorySites contains properties of a website from the Site Directory. Sites need to be added to an account via the Sites resource before they can be assigned to a placement.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get directory sites](DirectorySiteGetCall) (response)
/// * [insert directory sites](DirectorySiteInsertCall) (request|response)
/// * [list directory sites](DirectorySiteListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DirectorySite {
    /// Whether this directory site is active.
    
    pub active: Option<bool>,
    /// Directory site contacts.
    #[serde(rename="contactAssignments")]
    
    pub contact_assignments: Option<Vec<DirectorySiteContactAssignment>>,
    /// Country ID of this directory site. This is a read-only field.
    #[serde(rename="countryId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub country_id: Option<i64>,
    /// Currency ID of this directory site. This is a read-only field.
    /// Possible values are: 
    /// - "1" for USD 
    /// - "2" for GBP 
    /// - "3" for ESP 
    /// - "4" for SEK 
    /// - "5" for CAD 
    /// - "6" for JPY 
    /// - "7" for DEM 
    /// - "8" for AUD 
    /// - "9" for FRF 
    /// - "10" for ITL 
    /// - "11" for DKK 
    /// - "12" for NOK 
    /// - "13" for FIM 
    /// - "14" for ZAR 
    /// - "15" for IEP 
    /// - "16" for NLG 
    /// - "17" for EUR 
    /// - "18" for KRW 
    /// - "19" for TWD 
    /// - "20" for SGD 
    /// - "21" for CNY 
    /// - "22" for HKD 
    /// - "23" for NZD 
    /// - "24" for MYR 
    /// - "25" for BRL 
    /// - "26" for PTE 
    /// - "27" for MXP 
    /// - "28" for CLP 
    /// - "29" for TRY 
    /// - "30" for ARS 
    /// - "31" for PEN 
    /// - "32" for ILS 
    /// - "33" for CHF 
    /// - "34" for VEF 
    /// - "35" for COP 
    /// - "36" for GTQ 
    /// - "37" for PLN 
    /// - "39" for INR 
    /// - "40" for THB 
    /// - "41" for IDR 
    /// - "42" for CZK 
    /// - "43" for RON 
    /// - "44" for HUF 
    /// - "45" for RUB 
    /// - "46" for AED 
    /// - "47" for BGN 
    /// - "48" for HRK 
    /// - "49" for MXN 
    /// - "50" for NGN
    #[serde(rename="currencyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub currency_id: Option<i64>,
    /// Description of this directory site. This is a read-only field.
    
    pub description: Option<String>,
    /// ID of this directory site. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Dimension value for the ID of this directory site. This is a read-only, auto-generated field.
    #[serde(rename="idDimensionValue")]
    
    pub id_dimension_value: Option<DimensionValue>,
    /// Tag types for regular placements.
    /// 
    /// Acceptable values are:
    /// - "STANDARD"
    /// - "IFRAME_JAVASCRIPT_INPAGE"
    /// - "INTERNAL_REDIRECT_INPAGE"
    /// - "JAVASCRIPT_INPAGE"
    #[serde(rename="inpageTagFormats")]
    
    pub inpage_tag_formats: Option<Vec<DirectorySiteInpageTagFormatsEnum>>,
    /// Tag types for interstitial placements.
    /// 
    /// Acceptable values are:
    /// - "IFRAME_JAVASCRIPT_INTERSTITIAL"
    /// - "INTERNAL_REDIRECT_INTERSTITIAL"
    /// - "JAVASCRIPT_INTERSTITIAL"
    #[serde(rename="interstitialTagFormats")]
    
    pub interstitial_tag_formats: Option<Vec<DirectorySiteInterstitialTagFormatsEnum>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#directorySite".
    
    pub kind: Option<String>,
    /// Name of this directory site.
    
    pub name: Option<String>,
    /// Parent directory site ID.
    #[serde(rename="parentId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub parent_id: Option<i64>,
    /// Directory site settings.
    
    pub settings: Option<DirectorySiteSettings>,
    /// URL of this directory site.
    
    pub url: Option<String>,
}

impl client::RequestValue for DirectorySite {}
impl client::Resource for DirectorySite {}
impl client::ResponseResult for DirectorySite {}


/// Contains properties of a Site Directory contact.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get directory site contacts](DirectorySiteContactGetCall) (response)
/// * [list directory site contacts](DirectorySiteContactListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DirectorySiteContact {
    /// Address of this directory site contact.
    
    pub address: Option<String>,
    /// Email address of this directory site contact.
    
    pub email: Option<String>,
    /// First name of this directory site contact.
    #[serde(rename="firstName")]
    
    pub first_name: Option<String>,
    /// ID of this directory site contact. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#directorySiteContact".
    
    pub kind: Option<String>,
    /// Last name of this directory site contact.
    #[serde(rename="lastName")]
    
    pub last_name: Option<String>,
    /// Phone number of this directory site contact.
    
    pub phone: Option<String>,
    /// Directory site contact role.
    
    pub role: Option<DirectorySiteContactRoleEnum>,
    /// Title or designation of this directory site contact.
    
    pub title: Option<String>,
    /// Directory site contact type.
    #[serde(rename="type")]
    
    pub type_: Option<DirectorySiteContactTypeEnum>,
}

impl client::Resource for DirectorySiteContact {}
impl client::ResponseResult for DirectorySiteContact {}


/// Directory Site Contact Assignment
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DirectorySiteContactAssignment {
    /// ID of this directory site contact. This is a read-only, auto-generated field.
    #[serde(rename="contactId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub contact_id: Option<i64>,
    /// Visibility of this directory site contact assignment. When set to PUBLIC this contact assignment is visible to all account and agency users; when set to PRIVATE it is visible only to the site.
    
    pub visibility: Option<DirectorySiteContactAssignmentVisibilityEnum>,
}

impl client::Part for DirectorySiteContactAssignment {}


/// Directory Site Contact List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list directory site contacts](DirectorySiteContactListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DirectorySiteContactsListResponse {
    /// Directory site contact collection
    #[serde(rename="directorySiteContacts")]
    
    pub directory_site_contacts: Option<Vec<DirectorySiteContact>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#directorySiteContactsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for DirectorySiteContactsListResponse {}


/// Directory Site Settings
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DirectorySiteSettings {
    /// Whether this directory site has disabled active view creatives.
    #[serde(rename="activeViewOptOut")]
    
    pub active_view_opt_out: Option<bool>,
    /// Directory site Ad Manager settings.
    #[serde(rename="dfpSettings")]
    
    pub dfp_settings: Option<DfpSettings>,
    /// Whether this site accepts in-stream video ads.
    #[serde(rename="instreamVideoPlacementAccepted")]
    
    pub instream_video_placement_accepted: Option<bool>,
    /// Whether this site accepts interstitial ads.
    #[serde(rename="interstitialPlacementAccepted")]
    
    pub interstitial_placement_accepted: Option<bool>,
    /// Whether this directory site has disabled Nielsen OCR reach ratings.
    #[serde(rename="nielsenOcrOptOut")]
    
    pub nielsen_ocr_opt_out: Option<bool>,
    /// Whether this directory site has disabled generation of Verification ins tags.
    #[serde(rename="verificationTagOptOut")]
    
    pub verification_tag_opt_out: Option<bool>,
    /// Whether this directory site has disabled active view for in-stream video creatives. This is a read-only field.
    #[serde(rename="videoActiveViewOptOut")]
    
    pub video_active_view_opt_out: Option<bool>,
}

impl client::Part for DirectorySiteSettings {}


/// Directory Site List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list directory sites](DirectorySiteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DirectorySitesListResponse {
    /// Directory site collection.
    #[serde(rename="directorySites")]
    
    pub directory_sites: Option<Vec<DirectorySite>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#directorySitesListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for DirectorySitesListResponse {}


/// Contains properties of a dynamic targeting key. Dynamic targeting keys are unique, user-friendly labels, created at the advertiser level in DCM, that can be assigned to ads, creatives, and placements and used for targeting with Studio dynamic creatives. Use these labels instead of numeric Campaign Manager IDs (such as placement IDs) to save time and avoid errors in your dynamic feeds.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete dynamic targeting keys](DynamicTargetingKeyDeleteCall) (none)
/// * [insert dynamic targeting keys](DynamicTargetingKeyInsertCall) (request|response)
/// * [list dynamic targeting keys](DynamicTargetingKeyListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicTargetingKey {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#dynamicTargetingKey".
    
    pub kind: Option<String>,
    /// Name of this dynamic targeting key. This is a required field. Must be less than 256 characters long and cannot contain commas. All characters are converted to lowercase.
    
    pub name: Option<String>,
    /// ID of the object of this dynamic targeting key. This is a required field.
    #[serde(rename="objectId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub object_id: Option<i64>,
    /// Type of the object of this dynamic targeting key. This is a required field.
    #[serde(rename="objectType")]
    
    pub object_type: Option<DynamicTargetingKeyObjectTypeEnum>,
}

impl client::RequestValue for DynamicTargetingKey {}
impl client::Resource for DynamicTargetingKey {}
impl client::ResponseResult for DynamicTargetingKey {}


/// Dynamic Targeting Key List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list dynamic targeting keys](DynamicTargetingKeyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicTargetingKeysListResponse {
    /// Dynamic targeting key collection.
    #[serde(rename="dynamicTargetingKeys")]
    
    pub dynamic_targeting_keys: Option<Vec<DynamicTargetingKey>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#dynamicTargetingKeysListResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for DynamicTargetingKeysListResponse {}


/// A description of how user IDs are encrypted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EncryptionInfo {
    /// The encryption entity ID. This should match the encryption configuration for ad serving or Data Transfer.
    #[serde(rename="encryptionEntityId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub encryption_entity_id: Option<i64>,
    /// The encryption entity type. This should match the encryption configuration for ad serving or Data Transfer.
    #[serde(rename="encryptionEntityType")]
    
    pub encryption_entity_type: Option<EncryptionInfoEncryptionEntityTypeEnum>,
    /// Describes whether the encrypted cookie was received from ad serving (the %m macro) or from Data Transfer.
    #[serde(rename="encryptionSource")]
    
    pub encryption_source: Option<EncryptionInfoEncryptionSourceEnum>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#encryptionInfo".
    
    pub kind: Option<String>,
}

impl client::Part for EncryptionInfo {}


/// Contains properties of an event tag.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete event tags](EventTagDeleteCall) (none)
/// * [get event tags](EventTagGetCall) (response)
/// * [insert event tags](EventTagInsertCall) (request|response)
/// * [list event tags](EventTagListCall) (none)
/// * [patch event tags](EventTagPatchCall) (request|response)
/// * [update event tags](EventTagUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventTag {
    /// Account ID of this event tag. This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Advertiser ID of this event tag. This field or the campaignId field is required on insertion.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Dimension value for the ID of the advertiser. This is a read-only, auto-generated field.
    #[serde(rename="advertiserIdDimensionValue")]
    
    pub advertiser_id_dimension_value: Option<DimensionValue>,
    /// Campaign ID of this event tag. This field or the advertiserId field is required on insertion.
    #[serde(rename="campaignId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub campaign_id: Option<i64>,
    /// Dimension value for the ID of the campaign. This is a read-only, auto-generated field.
    #[serde(rename="campaignIdDimensionValue")]
    
    pub campaign_id_dimension_value: Option<DimensionValue>,
    /// Whether this event tag should be automatically enabled for all of the advertiser's campaigns and ads.
    #[serde(rename="enabledByDefault")]
    
    pub enabled_by_default: Option<bool>,
    /// Whether to remove this event tag from ads that are trafficked through Display & Video 360 to Ad Exchange. This may be useful if the event tag uses a pixel that is unapproved for Ad Exchange bids on one or more networks, such as the Google Display Network.
    #[serde(rename="excludeFromAdxRequests")]
    
    pub exclude_from_adx_requests: Option<bool>,
    /// ID of this event tag. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#eventTag".
    
    pub kind: Option<String>,
    /// Name of this event tag. This is a required field and must be less than 256 characters long.
    
    pub name: Option<String>,
    /// Site filter type for this event tag. If no type is specified then the event tag will be applied to all sites.
    #[serde(rename="siteFilterType")]
    
    pub site_filter_type: Option<EventTagSiteFilterTypeEnum>,
    /// Filter list of site IDs associated with this event tag. The siteFilterType determines whether this is a whitelist or blacklist filter.
    #[serde(rename="siteIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub site_ids: Option<Vec<i64>>,
    /// Whether this tag is SSL-compliant or not. This is a read-only field.
    #[serde(rename="sslCompliant")]
    
    pub ssl_compliant: Option<bool>,
    /// Status of this event tag. Must be ENABLED for this event tag to fire. This is a required field.
    
    pub status: Option<EventTagStatusEnum>,
    /// Subaccount ID of this event tag. This is a read-only field that can be left blank.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Event tag type. Can be used to specify whether to use a third-party pixel, a third-party JavaScript URL, or a third-party click-through URL for either impression or click tracking. This is a required field.
    #[serde(rename="type")]
    
    pub type_: Option<EventTagTypeEnum>,
    /// Payload URL for this event tag. The URL on a click-through event tag should have a landing page URL appended to the end of it. This field is required on insertion.
    
    pub url: Option<String>,
    /// Number of times the landing page URL should be URL-escaped before being appended to the click-through event tag URL. Only applies to click-through event tags as specified by the event tag type.
    #[serde(rename="urlEscapeLevels")]
    
    pub url_escape_levels: Option<i32>,
}

impl client::RequestValue for EventTag {}
impl client::Resource for EventTag {}
impl client::ResponseResult for EventTag {}


/// Event tag override information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventTagOverride {
    /// Whether this override is enabled.
    
    pub enabled: Option<bool>,
    /// ID of this event tag override. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
}

impl client::Part for EventTagOverride {}


/// Event Tag List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list event tags](EventTagListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventTagsListResponse {
    /// Event tag collection.
    #[serde(rename="eventTags")]
    
    pub event_tags: Option<Vec<EventTag>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#eventTagsListResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for EventTagsListResponse {}


/// Represents a File resource. A file contains the metadata for a report run. It shows the status of the run and holds the URLs to the generated report data if the run is finished and the status is “REPORT_AVAILABLE”.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get files](FileGetCall) (response)
/// * [list files](FileListCall) (none)
/// * [files get reports](ReportFileGetCall) (response)
/// * [run reports](ReportRunCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct File {
    /// The date range for which the file has report data. The date range will always be the absolute date range for which the report is run.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<DateRange>,
    /// The eTag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The filename of the file.
    #[serde(rename="fileName")]
    
    pub file_name: Option<String>,
    /// The output format of the report. Only available once the file is available.
    
    pub format: Option<FileFormatEnum>,
    /// The unique ID of this report file.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// The kind of resource this is, in this case dfareporting#file.
    
    pub kind: Option<String>,
    /// The timestamp in milliseconds since epoch when this file was last modified.
    #[serde(rename="lastModifiedTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_time: Option<i64>,
    /// The ID of the report this file was generated from.
    #[serde(rename="reportId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub report_id: Option<i64>,
    /// The status of the report file.
    
    pub status: Option<FileStatusEnum>,
    /// The URLs where the completed report file can be downloaded.
    
    pub urls: Option<FileUrls>,
}

impl client::Resource for File {}
impl client::ResponseResult for File {}


/// Represents the list of File resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list files](FileListCall) (response)
/// * [files list reports](ReportFileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileList {
    /// The eTag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The files returned in this response.
    
    pub items: Option<Vec<File>>,
    /// The kind of list this is, in this case dfareporting#fileList.
    
    pub kind: Option<String>,
    /// Continuation token used to page through files. To retrieve the next page of results, set the next request's "pageToken" to the value of this field. The page token is only valid for a limited amount of time and should not be persisted.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for FileList {}


/// Flight
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Flight {
    /// Inventory item flight end date.
    #[serde(rename="endDate")]
    
    pub end_date: Option<client::chrono::NaiveDate>,
    /// Rate or cost of this flight.
    #[serde(rename="rateOrCost")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub rate_or_cost: Option<i64>,
    /// Inventory item flight start date.
    #[serde(rename="startDate")]
    
    pub start_date: Option<client::chrono::NaiveDate>,
    /// Units of this flight.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub units: Option<i64>,
}

impl client::Part for Flight {}


/// Floodlight Activity GenerateTag Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generatetag floodlight activities](FloodlightActivityGeneratetagCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FloodlightActivitiesGenerateTagResponse {
    /// Generated tag for this Floodlight activity. For global site tags, this is the event snippet.
    #[serde(rename="floodlightActivityTag")]
    
    pub floodlight_activity_tag: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#floodlightActivitiesGenerateTagResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for FloodlightActivitiesGenerateTagResponse {}


/// Floodlight Activity List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list floodlight activities](FloodlightActivityListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FloodlightActivitiesListResponse {
    /// Floodlight activity collection.
    #[serde(rename="floodlightActivities")]
    
    pub floodlight_activities: Option<Vec<FloodlightActivity>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#floodlightActivitiesListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for FloodlightActivitiesListResponse {}


/// Contains properties of a Floodlight activity.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get floodlight activities](FloodlightActivityGetCall) (response)
/// * [insert floodlight activities](FloodlightActivityInsertCall) (request|response)
/// * [patch floodlight activities](FloodlightActivityPatchCall) (request|response)
/// * [update floodlight activities](FloodlightActivityUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FloodlightActivity {
    /// Account ID of this floodlight activity. This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Advertiser ID of this floodlight activity. If this field is left blank, the value will be copied over either from the activity group's advertiser or the existing activity's advertiser.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Dimension value for the ID of the advertiser. This is a read-only, auto-generated field.
    #[serde(rename="advertiserIdDimensionValue")]
    
    pub advertiser_id_dimension_value: Option<DimensionValue>,
    /// Code type used for cache busting in the generated tag. Applicable only when floodlightActivityGroupType is COUNTER and countingMethod is STANDARD_COUNTING or UNIQUE_COUNTING.
    #[serde(rename="cacheBustingType")]
    
    pub cache_busting_type: Option<FloodlightActivityCacheBustingTypeEnum>,
    /// Counting method for conversions for this floodlight activity. This is a required field.
    #[serde(rename="countingMethod")]
    
    pub counting_method: Option<FloodlightActivityCountingMethodEnum>,
    /// Dynamic floodlight tags.
    #[serde(rename="defaultTags")]
    
    pub default_tags: Option<Vec<FloodlightActivityDynamicTag>>,
    /// URL where this tag will be deployed. If specified, must be less than 256 characters long.
    #[serde(rename="expectedUrl")]
    
    pub expected_url: Option<String>,
    /// Floodlight activity group ID of this floodlight activity. This is a required field.
    #[serde(rename="floodlightActivityGroupId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub floodlight_activity_group_id: Option<i64>,
    /// Name of the associated floodlight activity group. This is a read-only field.
    #[serde(rename="floodlightActivityGroupName")]
    
    pub floodlight_activity_group_name: Option<String>,
    /// Tag string of the associated floodlight activity group. This is a read-only field.
    #[serde(rename="floodlightActivityGroupTagString")]
    
    pub floodlight_activity_group_tag_string: Option<String>,
    /// Type of the associated floodlight activity group. This is a read-only field.
    #[serde(rename="floodlightActivityGroupType")]
    
    pub floodlight_activity_group_type: Option<FloodlightActivityFloodlightActivityGroupTypeEnum>,
    /// Floodlight configuration ID of this floodlight activity. If this field is left blank, the value will be copied over either from the activity group's floodlight configuration or from the existing activity's floodlight configuration.
    #[serde(rename="floodlightConfigurationId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub floodlight_configuration_id: Option<i64>,
    /// Dimension value for the ID of the floodlight configuration. This is a read-only, auto-generated field.
    #[serde(rename="floodlightConfigurationIdDimensionValue")]
    
    pub floodlight_configuration_id_dimension_value: Option<DimensionValue>,
    /// Whether this activity is archived.
    
    pub hidden: Option<bool>,
    /// ID of this floodlight activity. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Dimension value for the ID of this floodlight activity. This is a read-only, auto-generated field.
    #[serde(rename="idDimensionValue")]
    
    pub id_dimension_value: Option<DimensionValue>,
    /// Whether the image tag is enabled for this activity.
    #[serde(rename="imageTagEnabled")]
    
    pub image_tag_enabled: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#floodlightActivity".
    
    pub kind: Option<String>,
    /// Name of this floodlight activity. This is a required field. Must be less than 129 characters long and cannot contain quotes.
    
    pub name: Option<String>,
    /// General notes or implementation instructions for the tag.
    
    pub notes: Option<String>,
    /// Publisher dynamic floodlight tags.
    #[serde(rename="publisherTags")]
    
    pub publisher_tags: Option<Vec<FloodlightActivityPublisherDynamicTag>>,
    /// Whether this tag should use SSL.
    
    pub secure: Option<bool>,
    /// Whether the floodlight activity is SSL-compliant. This is a read-only field, its value detected by the system from the floodlight tags.
    #[serde(rename="sslCompliant")]
    
    pub ssl_compliant: Option<bool>,
    /// Whether this floodlight activity must be SSL-compliant.
    #[serde(rename="sslRequired")]
    
    pub ssl_required: Option<bool>,
    /// Subaccount ID of this floodlight activity. This is a read-only field that can be left blank.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Tag format type for the floodlight activity. If left blank, the tag format will default to HTML.
    #[serde(rename="tagFormat")]
    
    pub tag_format: Option<FloodlightActivityTagFormatEnum>,
    /// Value of the cat= parameter in the floodlight tag, which the ad servers use to identify the activity. This is optional: if empty, a new tag string will be generated for you. This string must be 1 to 8 characters long, with valid characters being [a-z][A-Z][0-9][-][ _ ]. This tag string must also be unique among activities of the same activity group. This field is read-only after insertion.
    #[serde(rename="tagString")]
    
    pub tag_string: Option<String>,
    /// List of the user-defined variables used by this conversion tag. These map to the "u[1-100]=" in the tags. Each of these can have a user defined type.
    /// Acceptable values are U1 to U100, inclusive.
    #[serde(rename="userDefinedVariableTypes")]
    
    pub user_defined_variable_types: Option<Vec<FloodlightActivityUserDefinedVariableTypesEnum>>,
}

impl client::RequestValue for FloodlightActivity {}
impl client::ResponseResult for FloodlightActivity {}


/// Dynamic Tag
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FloodlightActivityDynamicTag {
    /// ID of this dynamic tag. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Name of this tag.
    
    pub name: Option<String>,
    /// Tag code.
    
    pub tag: Option<String>,
}

impl client::Part for FloodlightActivityDynamicTag {}


/// Contains properties of a Floodlight activity group.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get floodlight activity groups](FloodlightActivityGroupGetCall) (response)
/// * [insert floodlight activity groups](FloodlightActivityGroupInsertCall) (request|response)
/// * [list floodlight activity groups](FloodlightActivityGroupListCall) (none)
/// * [patch floodlight activity groups](FloodlightActivityGroupPatchCall) (request|response)
/// * [update floodlight activity groups](FloodlightActivityGroupUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FloodlightActivityGroup {
    /// Account ID of this floodlight activity group. This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Advertiser ID of this floodlight activity group. If this field is left blank, the value will be copied over either from the floodlight configuration's advertiser or from the existing activity group's advertiser.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Dimension value for the ID of the advertiser. This is a read-only, auto-generated field.
    #[serde(rename="advertiserIdDimensionValue")]
    
    pub advertiser_id_dimension_value: Option<DimensionValue>,
    /// Floodlight configuration ID of this floodlight activity group. This is a required field.
    #[serde(rename="floodlightConfigurationId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub floodlight_configuration_id: Option<i64>,
    /// Dimension value for the ID of the floodlight configuration. This is a read-only, auto-generated field.
    #[serde(rename="floodlightConfigurationIdDimensionValue")]
    
    pub floodlight_configuration_id_dimension_value: Option<DimensionValue>,
    /// ID of this floodlight activity group. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Dimension value for the ID of this floodlight activity group. This is a read-only, auto-generated field.
    #[serde(rename="idDimensionValue")]
    
    pub id_dimension_value: Option<DimensionValue>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#floodlightActivityGroup".
    
    pub kind: Option<String>,
    /// Name of this floodlight activity group. This is a required field. Must be less than 65 characters long and cannot contain quotes.
    
    pub name: Option<String>,
    /// Subaccount ID of this floodlight activity group. This is a read-only field that can be left blank.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Value of the type= parameter in the floodlight tag, which the ad servers use to identify the activity group that the activity belongs to. This is optional: if empty, a new tag string will be generated for you. This string must be 1 to 8 characters long, with valid characters being [a-z][A-Z][0-9][-][ _ ]. This tag string must also be unique among activity groups of the same floodlight configuration. This field is read-only after insertion.
    #[serde(rename="tagString")]
    
    pub tag_string: Option<String>,
    /// Type of the floodlight activity group. This is a required field that is read-only after insertion.
    #[serde(rename="type")]
    
    pub type_: Option<FloodlightActivityGroupTypeEnum>,
}

impl client::RequestValue for FloodlightActivityGroup {}
impl client::Resource for FloodlightActivityGroup {}
impl client::ResponseResult for FloodlightActivityGroup {}


/// Floodlight Activity Group List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list floodlight activity groups](FloodlightActivityGroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FloodlightActivityGroupsListResponse {
    /// Floodlight activity group collection.
    #[serde(rename="floodlightActivityGroups")]
    
    pub floodlight_activity_groups: Option<Vec<FloodlightActivityGroup>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#floodlightActivityGroupsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for FloodlightActivityGroupsListResponse {}


/// Publisher Dynamic Tag
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FloodlightActivityPublisherDynamicTag {
    /// Whether this tag is applicable only for click-throughs.
    #[serde(rename="clickThrough")]
    
    pub click_through: Option<bool>,
    /// Directory site ID of this dynamic tag. This is a write-only field that can be used as an alternative to the siteId field. When this resource is retrieved, only the siteId field will be populated.
    #[serde(rename="directorySiteId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub directory_site_id: Option<i64>,
    /// Dynamic floodlight tag.
    #[serde(rename="dynamicTag")]
    
    pub dynamic_tag: Option<FloodlightActivityDynamicTag>,
    /// Site ID of this dynamic tag.
    #[serde(rename="siteId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub site_id: Option<i64>,
    /// Dimension value for the ID of the site. This is a read-only, auto-generated field.
    #[serde(rename="siteIdDimensionValue")]
    
    pub site_id_dimension_value: Option<DimensionValue>,
    /// Whether this tag is applicable only for view-throughs.
    #[serde(rename="viewThrough")]
    
    pub view_through: Option<bool>,
}

impl client::Part for FloodlightActivityPublisherDynamicTag {}


/// Contains properties of a Floodlight configuration.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get floodlight configurations](FloodlightConfigurationGetCall) (response)
/// * [list floodlight configurations](FloodlightConfigurationListCall) (none)
/// * [patch floodlight configurations](FloodlightConfigurationPatchCall) (request|response)
/// * [update floodlight configurations](FloodlightConfigurationUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FloodlightConfiguration {
    /// Account ID of this floodlight configuration. This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Advertiser ID of the parent advertiser of this floodlight configuration.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Dimension value for the ID of the advertiser. This is a read-only, auto-generated field.
    #[serde(rename="advertiserIdDimensionValue")]
    
    pub advertiser_id_dimension_value: Option<DimensionValue>,
    /// Whether advertiser data is shared with Google Analytics.
    #[serde(rename="analyticsDataSharingEnabled")]
    
    pub analytics_data_sharing_enabled: Option<bool>,
    /// Whether the exposure-to-conversion report is enabled. This report shows detailed pathway information on up to 10 of the most recent ad exposures seen by a user before converting.
    #[serde(rename="exposureToConversionEnabled")]
    
    pub exposure_to_conversion_enabled: Option<bool>,
    /// Day that will be counted as the first day of the week in reports. This is a required field.
    #[serde(rename="firstDayOfWeek")]
    
    pub first_day_of_week: Option<FloodlightConfigurationFirstDayOfWeekEnum>,
    /// ID of this floodlight configuration. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Dimension value for the ID of this floodlight configuration. This is a read-only, auto-generated field.
    #[serde(rename="idDimensionValue")]
    
    pub id_dimension_value: Option<DimensionValue>,
    /// Whether in-app attribution tracking is enabled.
    #[serde(rename="inAppAttributionTrackingEnabled")]
    
    pub in_app_attribution_tracking_enabled: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#floodlightConfiguration".
    
    pub kind: Option<String>,
    /// Lookback window settings for this floodlight configuration.
    #[serde(rename="lookbackConfiguration")]
    
    pub lookback_configuration: Option<LookbackConfiguration>,
    /// Types of attribution options for natural search conversions.
    #[serde(rename="naturalSearchConversionAttributionOption")]
    
    pub natural_search_conversion_attribution_option: Option<FloodlightConfigurationNaturalSearchConversionAttributionOptionEnum>,
    /// Settings for Campaign Manager Omniture integration.
    #[serde(rename="omnitureSettings")]
    
    pub omniture_settings: Option<OmnitureSettings>,
    /// Subaccount ID of this floodlight configuration. This is a read-only field that can be left blank.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Configuration settings for dynamic and image floodlight tags.
    #[serde(rename="tagSettings")]
    
    pub tag_settings: Option<TagSettings>,
    /// List of third-party authentication tokens enabled for this configuration.
    #[serde(rename="thirdPartyAuthenticationTokens")]
    
    pub third_party_authentication_tokens: Option<Vec<ThirdPartyAuthenticationToken>>,
    /// List of user defined variables enabled for this configuration.
    #[serde(rename="userDefinedVariableConfigurations")]
    
    pub user_defined_variable_configurations: Option<Vec<UserDefinedVariableConfiguration>>,
}

impl client::RequestValue for FloodlightConfiguration {}
impl client::Resource for FloodlightConfiguration {}
impl client::ResponseResult for FloodlightConfiguration {}


/// Floodlight Configuration List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list floodlight configurations](FloodlightConfigurationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FloodlightConfigurationsListResponse {
    /// Floodlight configuration collection.
    #[serde(rename="floodlightConfigurations")]
    
    pub floodlight_configurations: Option<Vec<FloodlightConfiguration>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#floodlightConfigurationsListResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for FloodlightConfigurationsListResponse {}


/// Represents fields that are compatible to be selected for a report of type "FlOODLIGHT".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FloodlightReportCompatibleFields {
    /// Dimensions which are compatible to be selected in the "dimensionFilters" section of the report.
    #[serde(rename="dimensionFilters")]
    
    pub dimension_filters: Option<Vec<Dimension>>,
    /// Dimensions which are compatible to be selected in the "dimensions" section of the report.
    
    pub dimensions: Option<Vec<Dimension>>,
    /// The kind of resource this is, in this case dfareporting#floodlightReportCompatibleFields.
    
    pub kind: Option<String>,
    /// Metrics which are compatible to be selected in the "metricNames" section of the report.
    
    pub metrics: Option<Vec<Metric>>,
}

impl client::Part for FloodlightReportCompatibleFields {}


/// Frequency Cap.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FrequencyCap {
    /// Duration of time, in seconds, for this frequency cap. The maximum duration is 90 days. Acceptable values are 1 to 7776000, inclusive.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub duration: Option<i64>,
    /// Number of times an individual user can be served the ad within the specified duration. Acceptable values are 1 to 15, inclusive.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub impressions: Option<i64>,
}

impl client::Part for FrequencyCap {}


/// FsCommand.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FsCommand {
    /// Distance from the left of the browser.Applicable when positionOption is DISTANCE_FROM_TOP_LEFT_CORNER.
    
    pub left: Option<i32>,
    /// Position in the browser where the window will open.
    #[serde(rename="positionOption")]
    
    pub position_option: Option<FsCommandPositionOptionEnum>,
    /// Distance from the top of the browser. Applicable when positionOption is DISTANCE_FROM_TOP_LEFT_CORNER.
    
    pub top: Option<i32>,
    /// Height of the window.
    #[serde(rename="windowHeight")]
    
    pub window_height: Option<i32>,
    /// Width of the window.
    #[serde(rename="windowWidth")]
    
    pub window_width: Option<i32>,
}

impl client::Part for FsCommand {}


/// Geographical Targeting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoTargeting {
    /// Cities to be targeted. For each city only dartId is required. The other fields are populated automatically when the ad is inserted or updated. If targeting a city, do not target or exclude the country of the city, and do not target the metro or region of the city.
    
    pub cities: Option<Vec<City>>,
    /// Countries to be targeted or excluded from targeting, depending on the setting of the excludeCountries field. For each country only dartId is required. The other fields are populated automatically when the ad is inserted or updated. If targeting or excluding a country, do not target regions, cities, metros, or postal codes in the same country.
    
    pub countries: Option<Vec<Country>>,
    /// Whether or not to exclude the countries in the countries field from targeting. If false, the countries field refers to countries which will be targeted by the ad.
    #[serde(rename="excludeCountries")]
    
    pub exclude_countries: Option<bool>,
    /// Metros to be targeted. For each metro only dmaId is required. The other fields are populated automatically when the ad is inserted or updated. If targeting a metro, do not target or exclude the country of the metro.
    
    pub metros: Option<Vec<Metro>>,
    /// Postal codes to be targeted. For each postal code only id is required. The other fields are populated automatically when the ad is inserted or updated. If targeting a postal code, do not target or exclude the country of the postal code.
    #[serde(rename="postalCodes")]
    
    pub postal_codes: Option<Vec<PostalCode>>,
    /// Regions to be targeted. For each region only dartId is required. The other fields are populated automatically when the ad is inserted or updated. If targeting a region, do not target or exclude the country of the region.
    
    pub regions: Option<Vec<Region>>,
}

impl client::Part for GeoTargeting {}


/// Represents a buy from the Planning inventory store.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get inventory items](InventoryItemGetCall) (response)
/// * [list inventory items](InventoryItemListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventoryItem {
    /// Account ID of this inventory item.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Ad slots of this inventory item. If this inventory item represents a standalone placement, there will be exactly one ad slot. If this inventory item represents a placement group, there will be more than one ad slot, each representing one child placement in that placement group.
    #[serde(rename="adSlots")]
    
    pub ad_slots: Option<Vec<AdSlot>>,
    /// Advertiser ID of this inventory item.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Content category ID of this inventory item.
    #[serde(rename="contentCategoryId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub content_category_id: Option<i64>,
    /// Estimated click-through rate of this inventory item.
    #[serde(rename="estimatedClickThroughRate")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub estimated_click_through_rate: Option<i64>,
    /// Estimated conversion rate of this inventory item.
    #[serde(rename="estimatedConversionRate")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub estimated_conversion_rate: Option<i64>,
    /// ID of this inventory item.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Whether this inventory item is in plan.
    #[serde(rename="inPlan")]
    
    pub in_plan: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#inventoryItem".
    
    pub kind: Option<String>,
    /// Information about the most recent modification of this inventory item.
    #[serde(rename="lastModifiedInfo")]
    
    pub last_modified_info: Option<LastModifiedInfo>,
    /// Name of this inventory item. For standalone inventory items, this is the same name as that of its only ad slot. For group inventory items, this can differ from the name of any of its ad slots.
    
    pub name: Option<String>,
    /// Negotiation channel ID of this inventory item.
    #[serde(rename="negotiationChannelId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub negotiation_channel_id: Option<i64>,
    /// Order ID of this inventory item.
    #[serde(rename="orderId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub order_id: Option<i64>,
    /// Placement strategy ID of this inventory item.
    #[serde(rename="placementStrategyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub placement_strategy_id: Option<i64>,
    /// Pricing of this inventory item.
    
    pub pricing: Option<Pricing>,
    /// Project ID of this inventory item.
    #[serde(rename="projectId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub project_id: Option<i64>,
    /// RFP ID of this inventory item.
    #[serde(rename="rfpId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub rfp_id: Option<i64>,
    /// ID of the site this inventory item is associated with.
    #[serde(rename="siteId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub site_id: Option<i64>,
    /// Subaccount ID of this inventory item.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Type of inventory item.
    #[serde(rename="type")]
    
    pub type_: Option<InventoryItemTypeEnum>,
}

impl client::Resource for InventoryItem {}
impl client::ResponseResult for InventoryItem {}


/// Inventory item List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list inventory items](InventoryItemListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventoryItemsListResponse {
    /// Inventory item collection
    #[serde(rename="inventoryItems")]
    
    pub inventory_items: Option<Vec<InventoryItem>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#inventoryItemsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for InventoryItemsListResponse {}


/// Key Value Targeting Expression.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyValueTargetingExpression {
    /// Keyword expression being targeted by the ad.
    
    pub expression: Option<String>,
}

impl client::Part for KeyValueTargetingExpression {}


/// Contains information about where a user’s browser is taken after the user clicks an ad.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete landing pages](LandingPageDeleteCall) (none)
/// * [get landing pages](LandingPageGetCall) (response)
/// * [insert landing pages](LandingPageInsertCall) (request|response)
/// * [list landing pages](LandingPageListCall) (none)
/// * [patch landing pages](LandingPagePatchCall) (request|response)
/// * [update landing pages](LandingPageUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LandingPage {
    /// Whether or not this landing page will be assigned to any ads or creatives that do not have a landing page assigned explicitly. Only one default landing page is allowed per campaign.
    
    pub default: Option<bool>,
    /// ID of this landing page. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#landingPage".
    
    pub kind: Option<String>,
    /// Name of this landing page. This is a required field. It must be less than 256 characters long, and must be unique among landing pages of the same campaign.
    
    pub name: Option<String>,
    /// URL of this landing page. This is a required field.
    
    pub url: Option<String>,
}

impl client::RequestValue for LandingPage {}
impl client::Resource for LandingPage {}
impl client::ResponseResult for LandingPage {}


/// Landing Page List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list landing pages](LandingPageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LandingPagesListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#landingPagesListResponse".
    
    pub kind: Option<String>,
    /// Landing page collection
    #[serde(rename="landingPages")]
    
    pub landing_pages: Option<Vec<LandingPage>>,
}

impl client::ResponseResult for LandingPagesListResponse {}


/// Contains information about a language that can be targeted by ads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list languages](LanguageListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Language {
    /// Language ID of this language. This is the ID used for targeting and generating reports.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#language".
    
    pub kind: Option<String>,
    /// Format of language code is an ISO 639 two-letter language code optionally followed by an underscore followed by an ISO 3166 code. Examples are "en" for English or "zh_CN" for Simplified Chinese.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Name of this language.
    
    pub name: Option<String>,
}

impl client::Resource for Language {}


/// Language Targeting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LanguageTargeting {
    /// Languages that this ad targets. For each language only languageId is required. The other fields are populated automatically when the ad is inserted or updated.
    
    pub languages: Option<Vec<Language>>,
}

impl client::Part for LanguageTargeting {}


/// Language List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list languages](LanguageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LanguagesListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#languagesListResponse".
    
    pub kind: Option<String>,
    /// Language collection.
    
    pub languages: Option<Vec<Language>>,
}

impl client::ResponseResult for LanguagesListResponse {}


/// Modification timestamp.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LastModifiedInfo {
    /// Timestamp of the last change in milliseconds since epoch.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub time: Option<i64>,
}

impl client::Part for LastModifiedInfo {}


/// A group clause made up of list population terms representing constraints joined by ORs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPopulationClause {
    /// Terms of this list population clause. Each clause is made up of list population terms representing constraints and are joined by ORs.
    
    pub terms: Option<Vec<ListPopulationTerm>>,
}

impl client::Part for ListPopulationClause {}


/// Remarketing List Population Rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPopulationRule {
    /// Floodlight activity ID associated with this rule. This field can be left blank.
    #[serde(rename="floodlightActivityId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub floodlight_activity_id: Option<i64>,
    /// Name of floodlight activity associated with this rule. This is a read-only, auto-generated field.
    #[serde(rename="floodlightActivityName")]
    
    pub floodlight_activity_name: Option<String>,
    /// Clauses that make up this list population rule. Clauses are joined by ANDs, and the clauses themselves are made up of list population terms which are joined by ORs.
    #[serde(rename="listPopulationClauses")]
    
    pub list_population_clauses: Option<Vec<ListPopulationClause>>,
}

impl client::Part for ListPopulationRule {}


/// Remarketing List Population Rule Term.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPopulationTerm {
    /// Will be true if the term should check if the user is in the list and false if the term should check if the user is not in the list. This field is only relevant when type is set to LIST_MEMBERSHIP_TERM. False by default.
    
    pub contains: Option<bool>,
    /// Whether to negate the comparison result of this term during rule evaluation. This field is only relevant when type is left unset or set to CUSTOM_VARIABLE_TERM or REFERRER_TERM.
    
    pub negation: Option<bool>,
    /// Comparison operator of this term. This field is only relevant when type is left unset or set to CUSTOM_VARIABLE_TERM or REFERRER_TERM.
    
    pub operator: Option<ListPopulationTermOperatorEnum>,
    /// ID of the list in question. This field is only relevant when type is set to LIST_MEMBERSHIP_TERM.
    #[serde(rename="remarketingListId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub remarketing_list_id: Option<i64>,
    /// List population term type determines the applicable fields in this object. If left unset or set to CUSTOM_VARIABLE_TERM, then variableName, variableFriendlyName, operator, value, and negation are applicable. If set to LIST_MEMBERSHIP_TERM then remarketingListId and contains are applicable. If set to REFERRER_TERM then operator, value, and negation are applicable.
    #[serde(rename="type")]
    
    pub type_: Option<ListPopulationTermTypeEnum>,
    /// Literal to compare the variable to. This field is only relevant when type is left unset or set to CUSTOM_VARIABLE_TERM or REFERRER_TERM.
    
    pub value: Option<String>,
    /// Friendly name of this term's variable. This is a read-only, auto-generated field. This field is only relevant when type is left unset or set to CUSTOM_VARIABLE_TERM.
    #[serde(rename="variableFriendlyName")]
    
    pub variable_friendly_name: Option<String>,
    /// Name of the variable (U1, U2, etc.) being compared in this term. This field is only relevant when type is set to null, CUSTOM_VARIABLE_TERM or REFERRER_TERM.
    #[serde(rename="variableName")]
    
    pub variable_name: Option<String>,
}

impl client::Part for ListPopulationTerm {}


/// Remarketing List Targeting Expression.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTargetingExpression {
    /// Expression describing which lists are being targeted by the ad.
    
    pub expression: Option<String>,
}

impl client::Part for ListTargetingExpression {}


/// Lookback configuration settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LookbackConfiguration {
    /// Lookback window, in days, from the last time a given user clicked on one of your ads. If you enter 0, clicks will not be considered as triggering events for floodlight tracking. If you leave this field blank, the default value for your account will be used. Acceptable values are 0 to 90, inclusive.
    #[serde(rename="clickDuration")]
    
    pub click_duration: Option<i32>,
    /// Lookback window, in days, from the last time a given user viewed one of your ads. If you enter 0, impressions will not be considered as triggering events for floodlight tracking. If you leave this field blank, the default value for your account will be used. Acceptable values are 0 to 90, inclusive.
    #[serde(rename="postImpressionActivitiesDuration")]
    
    pub post_impression_activities_duration: Option<i32>,
}

impl client::Part for LookbackConfiguration {}


/// Represents a metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metric {
    /// The kind of resource this is, in this case dfareporting#metric.
    
    pub kind: Option<String>,
    /// The metric name, e.g. dfa:impressions
    
    pub name: Option<String>,
}

impl client::Part for Metric {}


/// Contains information about a metro region that can be targeted by ads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list metros](MetroListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metro {
    /// Country code of the country to which this metro region belongs.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// DART ID of the country to which this metro region belongs.
    #[serde(rename="countryDartId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub country_dart_id: Option<i64>,
    /// DART ID of this metro region.
    #[serde(rename="dartId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub dart_id: Option<i64>,
    /// DMA ID of this metro region. This is the ID used for targeting and generating reports, and is equivalent to metro_code.
    #[serde(rename="dmaId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub dma_id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#metro".
    
    pub kind: Option<String>,
    /// Metro code of this metro region. This is equivalent to dma_id.
    #[serde(rename="metroCode")]
    
    pub metro_code: Option<String>,
    /// Name of this metro region.
    
    pub name: Option<String>,
}

impl client::Resource for Metro {}


/// Metro List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list metros](MetroListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetrosListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#metrosListResponse".
    
    pub kind: Option<String>,
    /// Metro collection.
    
    pub metros: Option<Vec<Metro>>,
}

impl client::ResponseResult for MetrosListResponse {}


/// Contains information about a mobile carrier that can be targeted by ads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get mobile carriers](MobileCarrierGetCall) (response)
/// * [list mobile carriers](MobileCarrierListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MobileCarrier {
    /// Country code of the country to which this mobile carrier belongs.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// DART ID of the country to which this mobile carrier belongs.
    #[serde(rename="countryDartId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub country_dart_id: Option<i64>,
    /// ID of this mobile carrier.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#mobileCarrier".
    
    pub kind: Option<String>,
    /// Name of this mobile carrier.
    
    pub name: Option<String>,
}

impl client::Resource for MobileCarrier {}
impl client::ResponseResult for MobileCarrier {}


/// Mobile Carrier List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list mobile carriers](MobileCarrierListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MobileCarriersListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#mobileCarriersListResponse".
    
    pub kind: Option<String>,
    /// Mobile carrier collection.
    #[serde(rename="mobileCarriers")]
    
    pub mobile_carriers: Option<Vec<MobileCarrier>>,
}

impl client::ResponseResult for MobileCarriersListResponse {}


/// Object Filter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectFilter {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#objectFilter".
    
    pub kind: Option<String>,
    /// Applicable when status is ASSIGNED. The user has access to objects with these object IDs.
    #[serde(rename="objectIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub object_ids: Option<Vec<i64>>,
    /// Status of the filter. NONE means the user has access to none of the objects. ALL means the user has access to all objects. ASSIGNED means the user has access to the objects with IDs in the objectIds list.
    
    pub status: Option<ObjectFilterStatusEnum>,
}

impl client::Part for ObjectFilter {}


/// Offset Position.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OffsetPosition {
    /// Offset distance from left side of an asset or a window.
    
    pub left: Option<i32>,
    /// Offset distance from top side of an asset or a window.
    
    pub top: Option<i32>,
}

impl client::Part for OffsetPosition {}


/// Omniture Integration Settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OmnitureSettings {
    /// Whether placement cost data will be sent to Omniture. This property can be enabled only if omnitureIntegrationEnabled is true.
    #[serde(rename="omnitureCostDataEnabled")]
    
    pub omniture_cost_data_enabled: Option<bool>,
    /// Whether Omniture integration is enabled. This property can be enabled only when the "Advanced Ad Serving" account setting is enabled.
    #[serde(rename="omnitureIntegrationEnabled")]
    
    pub omniture_integration_enabled: Option<bool>,
}

impl client::Part for OmnitureSettings {}


/// Contains information about an operating system that can be targeted by ads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get operating systems](OperatingSystemGetCall) (response)
/// * [list operating systems](OperatingSystemListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperatingSystem {
    /// DART ID of this operating system. This is the ID used for targeting.
    #[serde(rename="dartId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub dart_id: Option<i64>,
    /// Whether this operating system is for desktop.
    
    pub desktop: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#operatingSystem".
    
    pub kind: Option<String>,
    /// Whether this operating system is for mobile.
    
    pub mobile: Option<bool>,
    /// Name of this operating system.
    
    pub name: Option<String>,
}

impl client::Resource for OperatingSystem {}
impl client::ResponseResult for OperatingSystem {}


/// Contains information about a particular version of an operating system that can be targeted by ads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get operating system versions](OperatingSystemVersionGetCall) (response)
/// * [list operating system versions](OperatingSystemVersionListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperatingSystemVersion {
    /// ID of this operating system version.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#operatingSystemVersion".
    
    pub kind: Option<String>,
    /// Major version (leftmost number) of this operating system version.
    #[serde(rename="majorVersion")]
    
    pub major_version: Option<String>,
    /// Minor version (number after the first dot) of this operating system version.
    #[serde(rename="minorVersion")]
    
    pub minor_version: Option<String>,
    /// Name of this operating system version.
    
    pub name: Option<String>,
    /// Operating system of this operating system version.
    #[serde(rename="operatingSystem")]
    
    pub operating_system: Option<OperatingSystem>,
}

impl client::Resource for OperatingSystemVersion {}
impl client::ResponseResult for OperatingSystemVersion {}


/// Operating System Version List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list operating system versions](OperatingSystemVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperatingSystemVersionsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#operatingSystemVersionsListResponse".
    
    pub kind: Option<String>,
    /// Operating system version collection.
    #[serde(rename="operatingSystemVersions")]
    
    pub operating_system_versions: Option<Vec<OperatingSystemVersion>>,
}

impl client::ResponseResult for OperatingSystemVersionsListResponse {}


/// Operating System List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list operating systems](OperatingSystemListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperatingSystemsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#operatingSystemsListResponse".
    
    pub kind: Option<String>,
    /// Operating system collection.
    #[serde(rename="operatingSystems")]
    
    pub operating_systems: Option<Vec<OperatingSystem>>,
}

impl client::ResponseResult for OperatingSystemsListResponse {}


/// Creative optimization activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OptimizationActivity {
    /// Floodlight activity ID of this optimization activity. This is a required field.
    #[serde(rename="floodlightActivityId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub floodlight_activity_id: Option<i64>,
    /// Dimension value for the ID of the floodlight activity. This is a read-only, auto-generated field.
    #[serde(rename="floodlightActivityIdDimensionValue")]
    
    pub floodlight_activity_id_dimension_value: Option<DimensionValue>,
    /// Weight associated with this optimization. The weight assigned will be understood in proportion to the weights assigned to the other optimization activities. Value must be greater than or equal to 1.
    
    pub weight: Option<i32>,
}

impl client::Part for OptimizationActivity {}


/// Describes properties of a Planning order.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get orders](OrderGetCall) (response)
/// * [list orders](OrderListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    /// Account ID of this order.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Advertiser ID of this order.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// IDs for users that have to approve documents created for this order.
    #[serde(rename="approverUserProfileIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub approver_user_profile_ids: Option<Vec<i64>>,
    /// Buyer invoice ID associated with this order.
    #[serde(rename="buyerInvoiceId")]
    
    pub buyer_invoice_id: Option<String>,
    /// Name of the buyer organization.
    #[serde(rename="buyerOrganizationName")]
    
    pub buyer_organization_name: Option<String>,
    /// Comments in this order.
    
    pub comments: Option<String>,
    /// Contacts for this order.
    
    pub contacts: Option<Vec<OrderContact>>,
    /// ID of this order. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#order".
    
    pub kind: Option<String>,
    /// Information about the most recent modification of this order.
    #[serde(rename="lastModifiedInfo")]
    
    pub last_modified_info: Option<LastModifiedInfo>,
    /// Name of this order.
    
    pub name: Option<String>,
    /// Notes of this order.
    
    pub notes: Option<String>,
    /// ID of the terms and conditions template used in this order.
    #[serde(rename="planningTermId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub planning_term_id: Option<i64>,
    /// Project ID of this order.
    #[serde(rename="projectId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub project_id: Option<i64>,
    /// Seller order ID associated with this order.
    #[serde(rename="sellerOrderId")]
    
    pub seller_order_id: Option<String>,
    /// Name of the seller organization.
    #[serde(rename="sellerOrganizationName")]
    
    pub seller_organization_name: Option<String>,
    /// Site IDs this order is associated with.
    #[serde(rename="siteId")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub site_id: Option<Vec<i64>>,
    /// Free-form site names this order is associated with.
    #[serde(rename="siteNames")]
    
    pub site_names: Option<Vec<String>>,
    /// Subaccount ID of this order.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Terms and conditions of this order.
    #[serde(rename="termsAndConditions")]
    
    pub terms_and_conditions: Option<String>,
}

impl client::Resource for Order {}
impl client::ResponseResult for Order {}


/// Contact of an order.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderContact {
    /// Free-form information about this contact. It could be any information related to this contact in addition to type, title, name, and signature user profile ID.
    #[serde(rename="contactInfo")]
    
    pub contact_info: Option<String>,
    /// Name of this contact.
    #[serde(rename="contactName")]
    
    pub contact_name: Option<String>,
    /// Title of this contact.
    #[serde(rename="contactTitle")]
    
    pub contact_title: Option<String>,
    /// Type of this contact.
    #[serde(rename="contactType")]
    
    pub contact_type: Option<OrderContactContactTypeEnum>,
    /// ID of the user profile containing the signature that will be embedded into order documents.
    #[serde(rename="signatureUserProfileId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub signature_user_profile_id: Option<i64>,
}

impl client::Part for OrderContact {}


/// Contains properties of a Planning order document.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get order documents](OrderDocumentGetCall) (response)
/// * [list order documents](OrderDocumentListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderDocument {
    /// Account ID of this order document.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Advertiser ID of this order document.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// The amended order document ID of this order document. An order document can be created by optionally amending another order document so that the change history can be preserved.
    #[serde(rename="amendedOrderDocumentId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub amended_order_document_id: Option<i64>,
    /// IDs of users who have approved this order document.
    #[serde(rename="approvedByUserProfileIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub approved_by_user_profile_ids: Option<Vec<i64>>,
    /// Whether this order document is cancelled.
    
    pub cancelled: Option<bool>,
    /// Information about the creation of this order document.
    #[serde(rename="createdInfo")]
    
    pub created_info: Option<LastModifiedInfo>,
    /// Effective date of this order document.
    #[serde(rename="effectiveDate")]
    
    pub effective_date: Option<client::chrono::NaiveDate>,
    /// ID of this order document.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#orderDocument".
    
    pub kind: Option<String>,
    /// List of email addresses that received the last sent document.
    #[serde(rename="lastSentRecipients")]
    
    pub last_sent_recipients: Option<Vec<String>>,
    /// Timestamp of the last email sent with this order document.
    #[serde(rename="lastSentTime")]
    
    pub last_sent_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// ID of the order from which this order document is created.
    #[serde(rename="orderId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub order_id: Option<i64>,
    /// Project ID of this order document.
    #[serde(rename="projectId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub project_id: Option<i64>,
    /// Whether this order document has been signed.
    
    pub signed: Option<bool>,
    /// Subaccount ID of this order document.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Title of this order document.
    
    pub title: Option<String>,
    /// Type of this order document
    #[serde(rename="type")]
    
    pub type_: Option<OrderDocumentTypeEnum>,
}

impl client::Resource for OrderDocument {}
impl client::ResponseResult for OrderDocument {}


/// Order document List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list order documents](OrderDocumentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderDocumentsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#orderDocumentsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Order document collection
    #[serde(rename="orderDocuments")]
    
    pub order_documents: Option<Vec<OrderDocument>>,
}

impl client::ResponseResult for OrderDocumentsListResponse {}


/// Order List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list orders](OrderListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#ordersListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Order collection.
    
    pub orders: Option<Vec<Order>>,
}

impl client::ResponseResult for OrdersListResponse {}


/// Represents fields that are compatible to be selected for a report of type "PATH_TO_CONVERSION".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PathToConversionReportCompatibleFields {
    /// Conversion dimensions which are compatible to be selected in the "conversionDimensions" section of the report.
    #[serde(rename="conversionDimensions")]
    
    pub conversion_dimensions: Option<Vec<Dimension>>,
    /// Custom floodlight variables which are compatible to be selected in the "customFloodlightVariables" section of the report.
    #[serde(rename="customFloodlightVariables")]
    
    pub custom_floodlight_variables: Option<Vec<Dimension>>,
    /// The kind of resource this is, in this case dfareporting#pathToConversionReportCompatibleFields.
    
    pub kind: Option<String>,
    /// Metrics which are compatible to be selected in the "metricNames" section of the report.
    
    pub metrics: Option<Vec<Metric>>,
    /// Per-interaction dimensions which are compatible to be selected in the "perInteractionDimensions" section of the report.
    #[serde(rename="perInteractionDimensions")]
    
    pub per_interaction_dimensions: Option<Vec<Dimension>>,
}

impl client::Part for PathToConversionReportCompatibleFields {}


/// Contains properties of a placement.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generatetags placements](PlacementGeneratetagCall) (none)
/// * [get placements](PlacementGetCall) (response)
/// * [insert placements](PlacementInsertCall) (request|response)
/// * [list placements](PlacementListCall) (none)
/// * [patch placements](PlacementPatchCall) (request|response)
/// * [update placements](PlacementUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Placement {
    /// Account ID of this placement. This field can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Whether this placement opts out of ad blocking. When true, ad blocking is disabled for this placement. When false, the campaign and site settings take effect.
    #[serde(rename="adBlockingOptOut")]
    
    pub ad_blocking_opt_out: Option<bool>,
    /// Advertiser ID of this placement. This field can be left blank.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Dimension value for the ID of the advertiser. This is a read-only, auto-generated field.
    #[serde(rename="advertiserIdDimensionValue")]
    
    pub advertiser_id_dimension_value: Option<DimensionValue>,
    /// Whether this placement is archived.
    
    pub archived: Option<bool>,
    /// Campaign ID of this placement. This field is a required field on insertion.
    #[serde(rename="campaignId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub campaign_id: Option<i64>,
    /// Dimension value for the ID of the campaign. This is a read-only, auto-generated field.
    #[serde(rename="campaignIdDimensionValue")]
    
    pub campaign_id_dimension_value: Option<DimensionValue>,
    /// Comments for this placement.
    
    pub comment: Option<String>,
    /// Placement compatibility. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering on desktop, on mobile devices or in mobile apps for regular or interstitial ads respectively. APP and APP_INTERSTITIAL are no longer allowed for new placement insertions. Instead, use DISPLAY or DISPLAY_INTERSTITIAL. IN_STREAM_VIDEO refers to rendering in in-stream video ads developed with the VAST standard. This field is required on insertion.
    
    pub compatibility: Option<PlacementCompatibilityEnum>,
    /// ID of the content category assigned to this placement.
    #[serde(rename="contentCategoryId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub content_category_id: Option<i64>,
    /// Information about the creation of this placement. This is a read-only field.
    #[serde(rename="createInfo")]
    
    pub create_info: Option<LastModifiedInfo>,
    /// Directory site ID of this placement. On insert, you must set either this field or the siteId field to specify the site associated with this placement. This is a required field that is read-only after insertion.
    #[serde(rename="directorySiteId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub directory_site_id: Option<i64>,
    /// Dimension value for the ID of the directory site. This is a read-only, auto-generated field.
    #[serde(rename="directorySiteIdDimensionValue")]
    
    pub directory_site_id_dimension_value: Option<DimensionValue>,
    /// External ID for this placement.
    #[serde(rename="externalId")]
    
    pub external_id: Option<String>,
    /// ID of this placement. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Dimension value for the ID of this placement. This is a read-only, auto-generated field.
    #[serde(rename="idDimensionValue")]
    
    pub id_dimension_value: Option<DimensionValue>,
    /// Key name of this placement. This is a read-only, auto-generated field.
    #[serde(rename="keyName")]
    
    pub key_name: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#placement".
    
    pub kind: Option<String>,
    /// Information about the most recent modification of this placement. This is a read-only field.
    #[serde(rename="lastModifiedInfo")]
    
    pub last_modified_info: Option<LastModifiedInfo>,
    /// Lookback window settings for this placement.
    #[serde(rename="lookbackConfiguration")]
    
    pub lookback_configuration: Option<LookbackConfiguration>,
    /// Name of this placement.This is a required field and must be less than 256 characters long.
    
    pub name: Option<String>,
    /// Whether payment was approved for this placement. This is a read-only field relevant only to publisher-paid placements.
    #[serde(rename="paymentApproved")]
    
    pub payment_approved: Option<bool>,
    /// Payment source for this placement. This is a required field that is read-only after insertion.
    #[serde(rename="paymentSource")]
    
    pub payment_source: Option<PlacementPaymentSourceEnum>,
    /// ID of this placement's group, if applicable.
    #[serde(rename="placementGroupId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub placement_group_id: Option<i64>,
    /// Dimension value for the ID of the placement group. This is a read-only, auto-generated field.
    #[serde(rename="placementGroupIdDimensionValue")]
    
    pub placement_group_id_dimension_value: Option<DimensionValue>,
    /// ID of the placement strategy assigned to this placement.
    #[serde(rename="placementStrategyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub placement_strategy_id: Option<i64>,
    /// Pricing schedule of this placement. This field is required on insertion, specifically subfields startDate, endDate and pricingType.
    #[serde(rename="pricingSchedule")]
    
    pub pricing_schedule: Option<PricingSchedule>,
    /// Whether this placement is the primary placement of a roadblock (placement group). You cannot change this field from true to false. Setting this field to true will automatically set the primary field on the original primary placement of the roadblock to false, and it will automatically set the roadblock's primaryPlacementId field to the ID of this placement.
    
    pub primary: Option<bool>,
    /// Information about the last publisher update. This is a read-only field.
    #[serde(rename="publisherUpdateInfo")]
    
    pub publisher_update_info: Option<LastModifiedInfo>,
    /// Site ID associated with this placement. On insert, you must set either this field or the directorySiteId field to specify the site associated with this placement. This is a required field that is read-only after insertion.
    #[serde(rename="siteId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub site_id: Option<i64>,
    /// Dimension value for the ID of the site. This is a read-only, auto-generated field.
    #[serde(rename="siteIdDimensionValue")]
    
    pub site_id_dimension_value: Option<DimensionValue>,
    /// Size associated with this placement. When inserting or updating a placement, only the size ID field is used. This field is required on insertion.
    
    pub size: Option<Size>,
    /// Whether creatives assigned to this placement must be SSL-compliant.
    #[serde(rename="sslRequired")]
    
    pub ssl_required: Option<bool>,
    /// Third-party placement status.
    
    pub status: Option<PlacementStatusEnum>,
    /// Subaccount ID of this placement. This field can be left blank.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Tag formats to generate for this placement. This field is required on insertion.
    /// Acceptable values are:
    /// - "PLACEMENT_TAG_STANDARD"
    /// - "PLACEMENT_TAG_IFRAME_JAVASCRIPT"
    /// - "PLACEMENT_TAG_IFRAME_ILAYER"
    /// - "PLACEMENT_TAG_INTERNAL_REDIRECT"
    /// - "PLACEMENT_TAG_JAVASCRIPT"
    /// - "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT"
    /// - "PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT"
    /// - "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT"
    /// - "PLACEMENT_TAG_CLICK_COMMANDS"
    /// - "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH"
    /// - "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3"
    /// - "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4"
    /// - "PLACEMENT_TAG_TRACKING"
    /// - "PLACEMENT_TAG_TRACKING_IFRAME"
    /// - "PLACEMENT_TAG_TRACKING_JAVASCRIPT"
    #[serde(rename="tagFormats")]
    
    pub tag_formats: Option<Vec<PlacementTagFormatsEnum>>,
    /// Tag settings for this placement.
    #[serde(rename="tagSetting")]
    
    pub tag_setting: Option<TagSetting>,
    /// Whether Verification and ActiveView are disabled for in-stream video creatives for this placement. The same setting videoActiveViewOptOut exists on the site level -- the opt out occurs if either of these settings are true. These settings are distinct from DirectorySites.settings.activeViewOptOut or Sites.siteSettings.activeViewOptOut which only apply to display ads. However, Accounts.activeViewOptOut opts out both video traffic, as well as display ads, from Verification and ActiveView.
    #[serde(rename="videoActiveViewOptOut")]
    
    pub video_active_view_opt_out: Option<bool>,
    /// A collection of settings which affect video creatives served through this placement. Applicable to placements with IN_STREAM_VIDEO compatibility.
    #[serde(rename="videoSettings")]
    
    pub video_settings: Option<VideoSettings>,
    /// VPAID adapter setting for this placement. Controls which VPAID format the measurement adapter will use for in-stream video creatives assigned to this placement.
    /// 
    /// Note: Flash is no longer supported. This field now defaults to HTML5 when the following values are provided: FLASH, BOTH.
    #[serde(rename="vpaidAdapterChoice")]
    
    pub vpaid_adapter_choice: Option<PlacementVpaidAdapterChoiceEnum>,
}

impl client::RequestValue for Placement {}
impl client::Resource for Placement {}
impl client::ResponseResult for Placement {}


/// Placement Assignment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlacementAssignment {
    /// Whether this placement assignment is active. When true, the placement will be included in the ad's rotation.
    
    pub active: Option<bool>,
    /// ID of the placement to be assigned. This is a required field.
    #[serde(rename="placementId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub placement_id: Option<i64>,
    /// Dimension value for the ID of the placement. This is a read-only, auto-generated field.
    #[serde(rename="placementIdDimensionValue")]
    
    pub placement_id_dimension_value: Option<DimensionValue>,
    /// Whether the placement to be assigned requires SSL. This is a read-only field that is auto-generated when the ad is inserted or updated.
    #[serde(rename="sslRequired")]
    
    pub ssl_required: Option<bool>,
}

impl client::Part for PlacementAssignment {}


/// Contains properties of a package or roadblock.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get placement groups](PlacementGroupGetCall) (response)
/// * [insert placement groups](PlacementGroupInsertCall) (request|response)
/// * [list placement groups](PlacementGroupListCall) (none)
/// * [patch placement groups](PlacementGroupPatchCall) (request|response)
/// * [update placement groups](PlacementGroupUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlacementGroup {
    /// Account ID of this placement group. This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Advertiser ID of this placement group. This is a required field on insertion.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Dimension value for the ID of the advertiser. This is a read-only, auto-generated field.
    #[serde(rename="advertiserIdDimensionValue")]
    
    pub advertiser_id_dimension_value: Option<DimensionValue>,
    /// Whether this placement group is archived.
    
    pub archived: Option<bool>,
    /// Campaign ID of this placement group. This field is required on insertion.
    #[serde(rename="campaignId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub campaign_id: Option<i64>,
    /// Dimension value for the ID of the campaign. This is a read-only, auto-generated field.
    #[serde(rename="campaignIdDimensionValue")]
    
    pub campaign_id_dimension_value: Option<DimensionValue>,
    /// IDs of placements which are assigned to this placement group. This is a read-only, auto-generated field.
    #[serde(rename="childPlacementIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub child_placement_ids: Option<Vec<i64>>,
    /// Comments for this placement group.
    
    pub comment: Option<String>,
    /// ID of the content category assigned to this placement group.
    #[serde(rename="contentCategoryId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub content_category_id: Option<i64>,
    /// Information about the creation of this placement group. This is a read-only field.
    #[serde(rename="createInfo")]
    
    pub create_info: Option<LastModifiedInfo>,
    /// Directory site ID associated with this placement group. On insert, you must set either this field or the site_id field to specify the site associated with this placement group. This is a required field that is read-only after insertion.
    #[serde(rename="directorySiteId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub directory_site_id: Option<i64>,
    /// Dimension value for the ID of the directory site. This is a read-only, auto-generated field.
    #[serde(rename="directorySiteIdDimensionValue")]
    
    pub directory_site_id_dimension_value: Option<DimensionValue>,
    /// External ID for this placement.
    #[serde(rename="externalId")]
    
    pub external_id: Option<String>,
    /// ID of this placement group. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Dimension value for the ID of this placement group. This is a read-only, auto-generated field.
    #[serde(rename="idDimensionValue")]
    
    pub id_dimension_value: Option<DimensionValue>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#placementGroup".
    
    pub kind: Option<String>,
    /// Information about the most recent modification of this placement group. This is a read-only field.
    #[serde(rename="lastModifiedInfo")]
    
    pub last_modified_info: Option<LastModifiedInfo>,
    /// Name of this placement group. This is a required field and must be less than 256 characters long.
    
    pub name: Option<String>,
    /// Type of this placement group. A package is a simple group of placements that acts as a single pricing point for a group of tags. A roadblock is a group of placements that not only acts as a single pricing point, but also assumes that all the tags in it will be served at the same time. A roadblock requires one of its assigned placements to be marked as primary for reporting. This field is required on insertion.
    #[serde(rename="placementGroupType")]
    
    pub placement_group_type: Option<PlacementGroupPlacementGroupTypeEnum>,
    /// ID of the placement strategy assigned to this placement group.
    #[serde(rename="placementStrategyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub placement_strategy_id: Option<i64>,
    /// Pricing schedule of this placement group. This field is required on insertion.
    #[serde(rename="pricingSchedule")]
    
    pub pricing_schedule: Option<PricingSchedule>,
    /// ID of the primary placement, used to calculate the media cost of a roadblock (placement group). Modifying this field will automatically modify the primary field on all affected roadblock child placements.
    #[serde(rename="primaryPlacementId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub primary_placement_id: Option<i64>,
    /// Dimension value for the ID of the primary placement. This is a read-only, auto-generated field.
    #[serde(rename="primaryPlacementIdDimensionValue")]
    
    pub primary_placement_id_dimension_value: Option<DimensionValue>,
    /// Site ID associated with this placement group. On insert, you must set either this field or the directorySiteId field to specify the site associated with this placement group. This is a required field that is read-only after insertion.
    #[serde(rename="siteId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub site_id: Option<i64>,
    /// Dimension value for the ID of the site. This is a read-only, auto-generated field.
    #[serde(rename="siteIdDimensionValue")]
    
    pub site_id_dimension_value: Option<DimensionValue>,
    /// Subaccount ID of this placement group. This is a read-only field that can be left blank.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
}

impl client::RequestValue for PlacementGroup {}
impl client::Resource for PlacementGroup {}
impl client::ResponseResult for PlacementGroup {}


/// Placement Group List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list placement groups](PlacementGroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlacementGroupsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#placementGroupsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Placement group collection.
    #[serde(rename="placementGroups")]
    
    pub placement_groups: Option<Vec<PlacementGroup>>,
}

impl client::ResponseResult for PlacementGroupsListResponse {}


/// Placement Strategy List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list placement strategies](PlacementStrategyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlacementStrategiesListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#placementStrategiesListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Placement strategy collection.
    #[serde(rename="placementStrategies")]
    
    pub placement_strategies: Option<Vec<PlacementStrategy>>,
}

impl client::ResponseResult for PlacementStrategiesListResponse {}


/// Contains properties of a placement strategy.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get placement strategies](PlacementStrategyGetCall) (response)
/// * [insert placement strategies](PlacementStrategyInsertCall) (request|response)
/// * [patch placement strategies](PlacementStrategyPatchCall) (request|response)
/// * [update placement strategies](PlacementStrategyUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlacementStrategy {
    /// Account ID of this placement strategy.This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// ID of this placement strategy. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#placementStrategy".
    
    pub kind: Option<String>,
    /// Name of this placement strategy. This is a required field. It must be less than 256 characters long and unique among placement strategies of the same account.
    
    pub name: Option<String>,
}

impl client::RequestValue for PlacementStrategy {}
impl client::ResponseResult for PlacementStrategy {}


/// Placement Tag
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlacementTag {
    /// Placement ID
    #[serde(rename="placementId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub placement_id: Option<i64>,
    /// Tags generated for this placement.
    #[serde(rename="tagDatas")]
    
    pub tag_datas: Option<Vec<TagData>>,
}

impl client::Part for PlacementTag {}


/// Placement GenerateTags Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generatetags placements](PlacementGeneratetagCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlacementsGenerateTagsResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#placementsGenerateTagsResponse".
    
    pub kind: Option<String>,
    /// Set of generated tags for the specified placements.
    #[serde(rename="placementTags")]
    
    pub placement_tags: Option<Vec<PlacementTag>>,
}

impl client::ResponseResult for PlacementsGenerateTagsResponse {}


/// Placement List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list placements](PlacementListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlacementsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#placementsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Placement collection.
    
    pub placements: Option<Vec<Placement>>,
}

impl client::ResponseResult for PlacementsListResponse {}


/// Contains information about a platform type that can be targeted by ads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get platform types](PlatformTypeGetCall) (response)
/// * [list platform types](PlatformTypeListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlatformType {
    /// ID of this platform type.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#platformType".
    
    pub kind: Option<String>,
    /// Name of this platform type.
    
    pub name: Option<String>,
}

impl client::Resource for PlatformType {}
impl client::ResponseResult for PlatformType {}


/// Platform Type List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list platform types](PlatformTypeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlatformTypesListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#platformTypesListResponse".
    
    pub kind: Option<String>,
    /// Platform type collection.
    #[serde(rename="platformTypes")]
    
    pub platform_types: Option<Vec<PlatformType>>,
}

impl client::ResponseResult for PlatformTypesListResponse {}


/// Popup Window Properties.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PopupWindowProperties {
    /// Popup dimension for a creative. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA and all VPAID
    
    pub dimension: Option<Size>,
    /// Upper-left corner coordinates of the popup window. Applicable if positionType is COORDINATES.
    
    pub offset: Option<OffsetPosition>,
    /// Popup window position either centered or at specific coordinate.
    #[serde(rename="positionType")]
    
    pub position_type: Option<PopupWindowPropertyPositionTypeEnum>,
    /// Whether to display the browser address bar.
    #[serde(rename="showAddressBar")]
    
    pub show_address_bar: Option<bool>,
    /// Whether to display the browser menu bar.
    #[serde(rename="showMenuBar")]
    
    pub show_menu_bar: Option<bool>,
    /// Whether to display the browser scroll bar.
    #[serde(rename="showScrollBar")]
    
    pub show_scroll_bar: Option<bool>,
    /// Whether to display the browser status bar.
    #[serde(rename="showStatusBar")]
    
    pub show_status_bar: Option<bool>,
    /// Whether to display the browser tool bar.
    #[serde(rename="showToolBar")]
    
    pub show_tool_bar: Option<bool>,
    /// Title of popup window.
    
    pub title: Option<String>,
}

impl client::Part for PopupWindowProperties {}


/// Contains information about a postal code that can be targeted by ads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get postal codes](PostalCodeGetCall) (response)
/// * [list postal codes](PostalCodeListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostalCode {
    /// Postal code. This is equivalent to the id field.
    
    pub code: Option<String>,
    /// Country code of the country to which this postal code belongs.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// DART ID of the country to which this postal code belongs.
    #[serde(rename="countryDartId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub country_dart_id: Option<i64>,
    /// ID of this postal code.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#postalCode".
    
    pub kind: Option<String>,
}

impl client::Resource for PostalCode {}
impl client::ResponseResult for PostalCode {}


/// Postal Code List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list postal codes](PostalCodeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostalCodesListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#postalCodesListResponse".
    
    pub kind: Option<String>,
    /// Postal code collection.
    #[serde(rename="postalCodes")]
    
    pub postal_codes: Option<Vec<PostalCode>>,
}

impl client::ResponseResult for PostalCodesListResponse {}


/// Pricing Information
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Pricing {
    /// Cap cost type of this inventory item.
    #[serde(rename="capCostType")]
    
    pub cap_cost_type: Option<PricingCapCostTypeEnum>,
    /// End date of this inventory item.
    #[serde(rename="endDate")]
    
    pub end_date: Option<client::chrono::NaiveDate>,
    /// Flights of this inventory item. A flight (a.k.a. pricing period) represents the inventory item pricing information for a specific period of time.
    
    pub flights: Option<Vec<Flight>>,
    /// Group type of this inventory item if it represents a placement group. Is null otherwise. There are two type of placement groups: PLANNING_PLACEMENT_GROUP_TYPE_PACKAGE is a simple group of inventory items that acts as a single pricing point for a group of tags. PLANNING_PLACEMENT_GROUP_TYPE_ROADBLOCK is a group of inventory items that not only acts as a single pricing point, but also assumes that all the tags in it will be served at the same time. A roadblock requires one of its assigned inventory items to be marked as primary.
    #[serde(rename="groupType")]
    
    pub group_type: Option<PricingGroupTypeEnum>,
    /// Pricing type of this inventory item.
    #[serde(rename="pricingType")]
    
    pub pricing_type: Option<PricingPricingTypeEnum>,
    /// Start date of this inventory item.
    #[serde(rename="startDate")]
    
    pub start_date: Option<client::chrono::NaiveDate>,
}

impl client::Part for Pricing {}


/// Pricing Schedule
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PricingSchedule {
    /// Placement cap cost option.
    #[serde(rename="capCostOption")]
    
    pub cap_cost_option: Option<PricingScheduleCapCostOptionEnum>,
    /// Whether cap costs are ignored by ad serving.
    #[serde(rename="disregardOverdelivery")]
    
    pub disregard_overdelivery: Option<bool>,
    /// Placement end date. This date must be later than, or the same day as, the placement start date, but not later than the campaign end date. If, for example, you set 6/25/2015 as both the start and end dates, the effective placement date is just that day only, 6/25/2015. The hours, minutes, and seconds of the end date should not be set, as doing so will result in an error. This field is required on insertion.
    #[serde(rename="endDate")]
    
    pub end_date: Option<client::chrono::NaiveDate>,
    /// Whether this placement is flighted. If true, pricing periods will be computed automatically.
    
    pub flighted: Option<bool>,
    /// Floodlight activity ID associated with this placement. This field should be set when placement pricing type is set to PRICING_TYPE_CPA.
    #[serde(rename="floodlightActivityId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub floodlight_activity_id: Option<i64>,
    /// Pricing periods for this placement.
    #[serde(rename="pricingPeriods")]
    
    pub pricing_periods: Option<Vec<PricingSchedulePricingPeriod>>,
    /// Placement pricing type. This field is required on insertion.
    #[serde(rename="pricingType")]
    
    pub pricing_type: Option<PricingSchedulePricingTypeEnum>,
    /// Placement start date. This date must be later than, or the same day as, the campaign start date. The hours, minutes, and seconds of the start date should not be set, as doing so will result in an error. This field is required on insertion.
    #[serde(rename="startDate")]
    
    pub start_date: Option<client::chrono::NaiveDate>,
    /// Testing start date of this placement. The hours, minutes, and seconds of the start date should not be set, as doing so will result in an error.
    #[serde(rename="testingStartDate")]
    
    pub testing_start_date: Option<client::chrono::NaiveDate>,
}

impl client::Part for PricingSchedule {}


/// Pricing Period
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PricingSchedulePricingPeriod {
    /// Pricing period end date. This date must be later than, or the same day as, the pricing period start date, but not later than the placement end date. The period end date can be the same date as the period start date. If, for example, you set 6/25/2015 as both the start and end dates, the effective pricing period date is just that day only, 6/25/2015. The hours, minutes, and seconds of the end date should not be set, as doing so will result in an error.
    #[serde(rename="endDate")]
    
    pub end_date: Option<client::chrono::NaiveDate>,
    /// Comments for this pricing period.
    #[serde(rename="pricingComment")]
    
    pub pricing_comment: Option<String>,
    /// Rate or cost of this pricing period in nanos (i.e., multipled by 1000000000). Acceptable values are 0 to 1000000000000000000, inclusive.
    #[serde(rename="rateOrCostNanos")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub rate_or_cost_nanos: Option<i64>,
    /// Pricing period start date. This date must be later than, or the same day as, the placement start date. The hours, minutes, and seconds of the start date should not be set, as doing so will result in an error.
    #[serde(rename="startDate")]
    
    pub start_date: Option<client::chrono::NaiveDate>,
    /// Units of this pricing period. Acceptable values are 0 to 10000000000, inclusive.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub units: Option<i64>,
}

impl client::Part for PricingSchedulePricingPeriod {}


/// Contains properties of a Planning project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get projects](ProjectGetCall) (response)
/// * [list projects](ProjectListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    /// Account ID of this project.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Advertiser ID of this project.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Audience age group of this project.
    #[serde(rename="audienceAgeGroup")]
    
    pub audience_age_group: Option<ProjectAudienceAgeGroupEnum>,
    /// Audience gender of this project.
    #[serde(rename="audienceGender")]
    
    pub audience_gender: Option<ProjectAudienceGenderEnum>,
    /// Budget of this project in the currency specified by the current account. The value stored in this field represents only the non-fractional amount. For example, for USD, the smallest value that can be represented by this field is 1 US dollar.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub budget: Option<i64>,
    /// Client billing code of this project.
    #[serde(rename="clientBillingCode")]
    
    pub client_billing_code: Option<String>,
    /// Name of the project client.
    #[serde(rename="clientName")]
    
    pub client_name: Option<String>,
    /// End date of the project.
    #[serde(rename="endDate")]
    
    pub end_date: Option<client::chrono::NaiveDate>,
    /// ID of this project. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#project".
    
    pub kind: Option<String>,
    /// Information about the most recent modification of this project.
    #[serde(rename="lastModifiedInfo")]
    
    pub last_modified_info: Option<LastModifiedInfo>,
    /// Name of this project.
    
    pub name: Option<String>,
    /// Overview of this project.
    
    pub overview: Option<String>,
    /// Start date of the project.
    #[serde(rename="startDate")]
    
    pub start_date: Option<client::chrono::NaiveDate>,
    /// Subaccount ID of this project.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Number of clicks that the advertiser is targeting.
    #[serde(rename="targetClicks")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub target_clicks: Option<i64>,
    /// Number of conversions that the advertiser is targeting.
    #[serde(rename="targetConversions")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub target_conversions: Option<i64>,
    /// CPA that the advertiser is targeting.
    #[serde(rename="targetCpaNanos")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub target_cpa_nanos: Option<i64>,
    /// CPC that the advertiser is targeting.
    #[serde(rename="targetCpcNanos")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub target_cpc_nanos: Option<i64>,
    /// vCPM from Active View that the advertiser is targeting.
    #[serde(rename="targetCpmActiveViewNanos")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub target_cpm_active_view_nanos: Option<i64>,
    /// CPM that the advertiser is targeting.
    #[serde(rename="targetCpmNanos")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub target_cpm_nanos: Option<i64>,
    /// Number of impressions that the advertiser is targeting.
    #[serde(rename="targetImpressions")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub target_impressions: Option<i64>,
}

impl client::Resource for Project {}
impl client::ResponseResult for Project {}


/// Project List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list projects](ProjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#projectsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Project collection.
    
    pub projects: Option<Vec<Project>>,
}

impl client::ResponseResult for ProjectsListResponse {}


/// Represents fields that are compatible to be selected for a report of type "REACH".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReachReportCompatibleFields {
    /// Dimensions which are compatible to be selected in the "dimensionFilters" section of the report.
    #[serde(rename="dimensionFilters")]
    
    pub dimension_filters: Option<Vec<Dimension>>,
    /// Dimensions which are compatible to be selected in the "dimensions" section of the report.
    
    pub dimensions: Option<Vec<Dimension>>,
    /// The kind of resource this is, in this case dfareporting#reachReportCompatibleFields.
    
    pub kind: Option<String>,
    /// Metrics which are compatible to be selected in the "metricNames" section of the report.
    
    pub metrics: Option<Vec<Metric>>,
    /// Metrics which are compatible to be selected as activity metrics to pivot on in the "activities" section of the report.
    #[serde(rename="pivotedActivityMetrics")]
    
    pub pivoted_activity_metrics: Option<Vec<Metric>>,
    /// Metrics which are compatible to be selected in the "reachByFrequencyMetricNames" section of the report.
    #[serde(rename="reachByFrequencyMetrics")]
    
    pub reach_by_frequency_metrics: Option<Vec<Metric>>,
}

impl client::Part for ReachReportCompatibleFields {}


/// Represents a recipient.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Recipient {
    /// The delivery type for the recipient.
    #[serde(rename="deliveryType")]
    
    pub delivery_type: Option<RecipientDeliveryTypeEnum>,
    /// The email address of the recipient.
    
    pub email: Option<String>,
    /// The kind of resource this is, in this case dfareporting#recipient.
    
    pub kind: Option<String>,
}

impl client::Part for Recipient {}


/// Contains information about a region that can be targeted by ads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list regions](RegionListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Region {
    /// Country code of the country to which this region belongs.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// DART ID of the country to which this region belongs.
    #[serde(rename="countryDartId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub country_dart_id: Option<i64>,
    /// DART ID of this region.
    #[serde(rename="dartId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub dart_id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#region".
    
    pub kind: Option<String>,
    /// Name of this region.
    
    pub name: Option<String>,
    /// Region code.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
}

impl client::Resource for Region {}


/// Region List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list regions](RegionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegionsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#regionsListResponse".
    
    pub kind: Option<String>,
    /// Region collection.
    
    pub regions: Option<Vec<Region>>,
}

impl client::ResponseResult for RegionsListResponse {}


/// Contains properties of a remarketing list. Remarketing enables you to create lists of users who have performed specific actions on a site, then target ads to members of those lists. This resource can be used to manage remarketing lists that are owned by your advertisers. To see all remarketing lists that are visible to your advertisers, including those that are shared to your advertiser or account, use the TargetableRemarketingLists resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get remarketing lists](RemarketingListGetCall) (response)
/// * [insert remarketing lists](RemarketingListInsertCall) (request|response)
/// * [list remarketing lists](RemarketingListListCall) (none)
/// * [patch remarketing lists](RemarketingListPatchCall) (request|response)
/// * [update remarketing lists](RemarketingListUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemarketingList {
    /// Account ID of this remarketing list. This is a read-only, auto-generated field that is only returned in GET requests.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Whether this remarketing list is active.
    
    pub active: Option<bool>,
    /// Dimension value for the advertiser ID that owns this remarketing list. This is a required field.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Dimension value for the ID of the advertiser. This is a read-only, auto-generated field.
    #[serde(rename="advertiserIdDimensionValue")]
    
    pub advertiser_id_dimension_value: Option<DimensionValue>,
    /// Remarketing list description.
    
    pub description: Option<String>,
    /// Remarketing list ID. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#remarketingList".
    
    pub kind: Option<String>,
    /// Number of days that a user should remain in the remarketing list without an impression. Acceptable values are 1 to 540, inclusive.
    #[serde(rename="lifeSpan")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub life_span: Option<i64>,
    /// Rule used to populate the remarketing list with users.
    #[serde(rename="listPopulationRule")]
    
    pub list_population_rule: Option<ListPopulationRule>,
    /// Number of users currently in the list. This is a read-only field.
    #[serde(rename="listSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub list_size: Option<i64>,
    /// Product from which this remarketing list was originated.
    #[serde(rename="listSource")]
    
    pub list_source: Option<RemarketingListListSourceEnum>,
    /// Name of the remarketing list. This is a required field. Must be no greater than 128 characters long.
    
    pub name: Option<String>,
    /// Subaccount ID of this remarketing list. This is a read-only, auto-generated field that is only returned in GET requests.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
}

impl client::RequestValue for RemarketingList {}
impl client::Resource for RemarketingList {}
impl client::ResponseResult for RemarketingList {}


/// Contains properties of a remarketing list’s sharing information. Sharing allows other accounts or advertisers to target to your remarketing lists. This resource can be used to manage remarketing list sharing to other accounts and advertisers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get remarketing list shares](RemarketingListShareGetCall) (response)
/// * [patch remarketing list shares](RemarketingListSharePatchCall) (request|response)
/// * [update remarketing list shares](RemarketingListShareUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemarketingListShare {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#remarketingListShare".
    
    pub kind: Option<String>,
    /// Remarketing list ID. This is a read-only, auto-generated field.
    #[serde(rename="remarketingListId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub remarketing_list_id: Option<i64>,
    /// Accounts that the remarketing list is shared with.
    #[serde(rename="sharedAccountIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub shared_account_ids: Option<Vec<i64>>,
    /// Advertisers that the remarketing list is shared with.
    #[serde(rename="sharedAdvertiserIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub shared_advertiser_ids: Option<Vec<i64>>,
}

impl client::RequestValue for RemarketingListShare {}
impl client::Resource for RemarketingListShare {}
impl client::ResponseResult for RemarketingListShare {}


/// Remarketing list response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list remarketing lists](RemarketingListListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemarketingListsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#remarketingListsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Remarketing list collection.
    #[serde(rename="remarketingLists")]
    
    pub remarketing_lists: Option<Vec<RemarketingList>>,
}

impl client::ResponseResult for RemarketingListsListResponse {}


/// Represents a Report resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [compatible fields query reports](ReportCompatibleFieldQueryCall) (request)
/// * [files get reports](ReportFileGetCall) (none)
/// * [files list reports](ReportFileListCall) (none)
/// * [delete reports](ReportDeleteCall) (none)
/// * [get reports](ReportGetCall) (response)
/// * [insert reports](ReportInsertCall) (request|response)
/// * [list reports](ReportListCall) (none)
/// * [patch reports](ReportPatchCall) (request|response)
/// * [run reports](ReportRunCall) (none)
/// * [update reports](ReportUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    /// The account ID to which this report belongs.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// The report criteria for a report of type "STANDARD".
    
    pub criteria: Option<ReportCriteria>,
    /// The report criteria for a report of type "CROSS_DIMENSION_REACH".
    #[serde(rename="crossDimensionReachCriteria")]
    
    pub cross_dimension_reach_criteria: Option<ReportCrossDimensionReachCriteria>,
    /// The report's email delivery settings.
    
    pub delivery: Option<ReportDelivery>,
    /// The eTag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The filename used when generating report files for this report.
    #[serde(rename="fileName")]
    
    pub file_name: Option<String>,
    /// The report criteria for a report of type "FLOODLIGHT".
    #[serde(rename="floodlightCriteria")]
    
    pub floodlight_criteria: Option<ReportFloodlightCriteria>,
    /// The output format of the report. If not specified, default format is "CSV". Note that the actual format in the completed report file might differ if for instance the report's size exceeds the format's capabilities. "CSV" will then be the fallback format.
    
    pub format: Option<ReportFormatEnum>,
    /// The unique ID identifying this report resource.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// The kind of resource this is, in this case dfareporting#report.
    
    pub kind: Option<String>,
    /// The timestamp (in milliseconds since epoch) of when this report was last modified.
    #[serde(rename="lastModifiedTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_time: Option<u64>,
    /// The name of the report.
    
    pub name: Option<String>,
    /// The user profile id of the owner of this report.
    #[serde(rename="ownerProfileId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub owner_profile_id: Option<i64>,
    /// The report criteria for a report of type "PATH_TO_CONVERSION".
    #[serde(rename="pathToConversionCriteria")]
    
    pub path_to_conversion_criteria: Option<ReportPathToConversionCriteria>,
    /// The report criteria for a report of type "REACH".
    #[serde(rename="reachCriteria")]
    
    pub reach_criteria: Option<ReportReachCriteria>,
    /// The report's schedule. Can only be set if the report's 'dateRange' is a relative date range and the relative date range is not "TODAY".
    
    pub schedule: Option<ReportSchedule>,
    /// The subaccount ID to which this report belongs if applicable.
    #[serde(rename="subAccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub sub_account_id: Option<i64>,
    /// The type of the report.
    #[serde(rename="type")]
    
    pub type_: Option<ReportTypeEnum>,
}

impl client::RequestValue for Report {}
impl client::Resource for Report {}
impl client::ResponseResult for Report {}


/// Represents fields that are compatible to be selected for a report of type "STANDARD".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportCompatibleFields {
    /// Dimensions which are compatible to be selected in the "dimensionFilters" section of the report.
    #[serde(rename="dimensionFilters")]
    
    pub dimension_filters: Option<Vec<Dimension>>,
    /// Dimensions which are compatible to be selected in the "dimensions" section of the report.
    
    pub dimensions: Option<Vec<Dimension>>,
    /// The kind of resource this is, in this case dfareporting#reportCompatibleFields.
    
    pub kind: Option<String>,
    /// Metrics which are compatible to be selected in the "metricNames" section of the report.
    
    pub metrics: Option<Vec<Metric>>,
    /// Metrics which are compatible to be selected as activity metrics to pivot on in the "activities" section of the report.
    #[serde(rename="pivotedActivityMetrics")]
    
    pub pivoted_activity_metrics: Option<Vec<Metric>>,
}

impl client::Part for ReportCompatibleFields {}


/// Represents the list of reports.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list reports](ReportListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportList {
    /// The eTag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The reports returned in this response.
    
    pub items: Option<Vec<Report>>,
    /// The kind of list this is, in this case dfareporting#reportList.
    
    pub kind: Option<String>,
    /// Continuation token used to page through reports. To retrieve the next page of results, set the next request's "pageToken" to the value of this field. The page token is only valid for a limited amount of time and should not be persisted.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ReportList {}


/// Reporting Configuration
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportsConfiguration {
    /// Whether the exposure to conversion report is enabled. This report shows detailed pathway information on up to 10 of the most recent ad exposures seen by a user before converting.
    #[serde(rename="exposureToConversionEnabled")]
    
    pub exposure_to_conversion_enabled: Option<bool>,
    /// Default lookback windows for new advertisers in this account.
    #[serde(rename="lookbackConfiguration")]
    
    pub lookback_configuration: Option<LookbackConfiguration>,
    /// Report generation time zone ID of this account. This is a required field that can only be changed by a superuser.
    /// Acceptable values are:
    /// 
    /// - "1" for "America/New_York" 
    /// - "2" for "Europe/London" 
    /// - "3" for "Europe/Paris" 
    /// - "4" for "Africa/Johannesburg" 
    /// - "5" for "Asia/Jerusalem" 
    /// - "6" for "Asia/Shanghai" 
    /// - "7" for "Asia/Hong_Kong" 
    /// - "8" for "Asia/Tokyo" 
    /// - "9" for "Australia/Sydney" 
    /// - "10" for "Asia/Dubai" 
    /// - "11" for "America/Los_Angeles" 
    /// - "12" for "Pacific/Auckland" 
    /// - "13" for "America/Sao_Paulo"
    #[serde(rename="reportGenerationTimeZoneId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub report_generation_time_zone_id: Option<i64>,
}

impl client::Part for ReportsConfiguration {}


/// Rich Media Exit Override.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RichMediaExitOverride {
    /// Click-through URL of this rich media exit override. Applicable if the enabled field is set to true.
    #[serde(rename="clickThroughUrl")]
    
    pub click_through_url: Option<ClickThroughUrl>,
    /// Whether to use the clickThroughUrl. If false, the creative-level exit will be used.
    
    pub enabled: Option<bool>,
    /// ID for the override to refer to a specific exit in the creative.
    #[serde(rename="exitId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub exit_id: Option<i64>,
}

impl client::Part for RichMediaExitOverride {}


/// A rule associates an asset with a targeting template for asset-level targeting. Applicable to INSTREAM_VIDEO creatives.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    /// A creativeAssets[].id. This should refer to one of the parent assets in this creative. This is a required field.
    #[serde(rename="assetId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub asset_id: Option<i64>,
    /// A user-friendly name for this rule. This is a required field.
    
    pub name: Option<String>,
    /// A targeting template ID. The targeting from the targeting template will be used to determine whether this asset should be served. This is a required field.
    #[serde(rename="targetingTemplateId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub targeting_template_id: Option<i64>,
}

impl client::Part for Rule {}


/// Contains properties of a site.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get sites](SiteGetCall) (response)
/// * [insert sites](SiteInsertCall) (request|response)
/// * [list sites](SiteListCall) (none)
/// * [patch sites](SitePatchCall) (request|response)
/// * [update sites](SiteUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Site {
    /// Account ID of this site. This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Whether this site is approved.
    
    pub approved: Option<bool>,
    /// Directory site associated with this site. This is a required field that is read-only after insertion.
    #[serde(rename="directorySiteId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub directory_site_id: Option<i64>,
    /// Dimension value for the ID of the directory site. This is a read-only, auto-generated field.
    #[serde(rename="directorySiteIdDimensionValue")]
    
    pub directory_site_id_dimension_value: Option<DimensionValue>,
    /// ID of this site. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Dimension value for the ID of this site. This is a read-only, auto-generated field.
    #[serde(rename="idDimensionValue")]
    
    pub id_dimension_value: Option<DimensionValue>,
    /// Key name of this site. This is a read-only, auto-generated field.
    #[serde(rename="keyName")]
    
    pub key_name: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#site".
    
    pub kind: Option<String>,
    /// Name of this site.This is a required field. Must be less than 128 characters long. If this site is under a subaccount, the name must be unique among sites of the same subaccount. Otherwise, this site is a top-level site, and the name must be unique among top-level sites of the same account.
    
    pub name: Option<String>,
    /// Site contacts.
    #[serde(rename="siteContacts")]
    
    pub site_contacts: Option<Vec<SiteContact>>,
    /// Site-wide settings.
    #[serde(rename="siteSettings")]
    
    pub site_settings: Option<SiteSettings>,
    /// Subaccount ID of this site. This is a read-only field that can be left blank.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
}

impl client::RequestValue for Site {}
impl client::Resource for Site {}
impl client::ResponseResult for Site {}


/// Site Contact
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SiteContact {
    /// Address of this site contact.
    
    pub address: Option<String>,
    /// Site contact type.
    #[serde(rename="contactType")]
    
    pub contact_type: Option<SiteContactContactTypeEnum>,
    /// Email address of this site contact. This is a required field.
    
    pub email: Option<String>,
    /// First name of this site contact.
    #[serde(rename="firstName")]
    
    pub first_name: Option<String>,
    /// ID of this site contact. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Last name of this site contact.
    #[serde(rename="lastName")]
    
    pub last_name: Option<String>,
    /// Primary phone number of this site contact.
    
    pub phone: Option<String>,
    /// Title or designation of this site contact.
    
    pub title: Option<String>,
}

impl client::Part for SiteContact {}


/// Site Settings
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SiteSettings {
    /// Whether active view creatives are disabled for this site.
    #[serde(rename="activeViewOptOut")]
    
    pub active_view_opt_out: Option<bool>,
    /// Whether this site opts out of ad blocking. When true, ad blocking is disabled for all placements under the site, regardless of the individual placement settings. When false, the campaign and placement settings take effect.
    #[serde(rename="adBlockingOptOut")]
    
    pub ad_blocking_opt_out: Option<bool>,
    /// Site-wide creative settings.
    #[serde(rename="creativeSettings")]
    
    pub creative_settings: Option<CreativeSettings>,
    /// Whether new cookies are disabled for this site.
    #[serde(rename="disableNewCookie")]
    
    pub disable_new_cookie: Option<bool>,
    /// Lookback window settings for this site.
    #[serde(rename="lookbackConfiguration")]
    
    pub lookback_configuration: Option<LookbackConfiguration>,
    /// Configuration settings for dynamic and image floodlight tags.
    #[serde(rename="tagSetting")]
    
    pub tag_setting: Option<TagSetting>,
    /// Whether Verification and ActiveView for in-stream video creatives are disabled by default for new placements created under this site. This value will be used to populate the placement.videoActiveViewOptOut field, when no value is specified for the new placement.
    #[serde(rename="videoActiveViewOptOutTemplate")]
    
    pub video_active_view_opt_out_template: Option<bool>,
    /// Default VPAID adapter setting for new placements created under this site. This value will be used to populate the placements.vpaidAdapterChoice field, when no value is specified for the new placement. Controls which VPAID format the measurement adapter will use for in-stream video creatives assigned to the placement. The publisher's specifications will typically determine this setting. For VPAID creatives, the adapter format will match the VPAID format (HTML5 VPAID creatives use the HTML5 adapter).
    /// 
    /// Note: Flash is no longer supported. This field now defaults to HTML5 when the following values are provided: FLASH, BOTH.
    #[serde(rename="vpaidAdapterChoiceTemplate")]
    
    pub vpaid_adapter_choice_template: Option<SiteSettingVpaidAdapterChoiceTemplateEnum>,
}

impl client::Part for SiteSettings {}


/// Site List Response
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
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#sitesListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Site collection.
    
    pub sites: Option<Vec<Site>>,
}

impl client::ResponseResult for SitesListResponse {}


/// Represents the dimensions of ads, placements, creatives, or creative assets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get sizes](SizeGetCall) (response)
/// * [insert sizes](SizeInsertCall) (request|response)
/// * [list sizes](SizeListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Size {
    /// Height of this size. Acceptable values are 0 to 32767, inclusive.
    
    pub height: Option<i32>,
    /// IAB standard size. This is a read-only, auto-generated field.
    
    pub iab: Option<bool>,
    /// ID of this size. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#size".
    
    pub kind: Option<String>,
    /// Width of this size. Acceptable values are 0 to 32767, inclusive.
    
    pub width: Option<i32>,
}

impl client::RequestValue for Size {}
impl client::Resource for Size {}
impl client::ResponseResult for Size {}


/// Size List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list sizes](SizeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SizesListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#sizesListResponse".
    
    pub kind: Option<String>,
    /// Size collection.
    
    pub sizes: Option<Vec<Size>>,
}

impl client::ResponseResult for SizesListResponse {}


/// Skippable Settings
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SkippableSetting {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#skippableSetting".
    
    pub kind: Option<String>,
    /// Amount of time to play videos served to this placement before counting a view. Applicable when skippable is true.
    #[serde(rename="progressOffset")]
    
    pub progress_offset: Option<VideoOffset>,
    /// Amount of time to play videos served to this placement before the skip button should appear. Applicable when skippable is true.
    #[serde(rename="skipOffset")]
    
    pub skip_offset: Option<VideoOffset>,
    /// Whether the user can skip creatives served to this placement.
    
    pub skippable: Option<bool>,
}

impl client::Part for SkippableSetting {}


/// Represents a sorted dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SortedDimension {
    /// The kind of resource this is, in this case dfareporting#sortedDimension.
    
    pub kind: Option<String>,
    /// The name of the dimension.
    
    pub name: Option<String>,
    /// An optional sort order for the dimension column.
    #[serde(rename="sortOrder")]
    
    pub sort_order: Option<SortedDimensionSortOrderEnum>,
}

impl client::Part for SortedDimension {}


/// Contains properties of a Campaign Manager subaccount.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get subaccounts](SubaccountGetCall) (response)
/// * [insert subaccounts](SubaccountInsertCall) (request|response)
/// * [list subaccounts](SubaccountListCall) (none)
/// * [patch subaccounts](SubaccountPatchCall) (request|response)
/// * [update subaccounts](SubaccountUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Subaccount {
    /// ID of the account that contains this subaccount. This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// IDs of the available user role permissions for this subaccount.
    #[serde(rename="availablePermissionIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub available_permission_ids: Option<Vec<i64>>,
    /// ID of this subaccount. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#subaccount".
    
    pub kind: Option<String>,
    /// Name of this subaccount. This is a required field. Must be less than 128 characters long and be unique among subaccounts of the same account.
    
    pub name: Option<String>,
}

impl client::RequestValue for Subaccount {}
impl client::Resource for Subaccount {}
impl client::ResponseResult for Subaccount {}


/// Subaccount List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list subaccounts](SubaccountListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubaccountsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#subaccountsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Subaccount collection.
    
    pub subaccounts: Option<Vec<Subaccount>>,
}

impl client::ResponseResult for SubaccountsListResponse {}


/// Placement Tag Data
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TagData {
    /// Ad associated with this placement tag. Applicable only when format is PLACEMENT_TAG_TRACKING.
    #[serde(rename="adId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ad_id: Option<i64>,
    /// Tag string to record a click.
    #[serde(rename="clickTag")]
    
    pub click_tag: Option<String>,
    /// Creative associated with this placement tag. Applicable only when format is PLACEMENT_TAG_TRACKING.
    #[serde(rename="creativeId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creative_id: Option<i64>,
    /// TagData tag format of this tag.
    
    pub format: Option<TagDataFormatEnum>,
    /// Tag string for serving an ad.
    #[serde(rename="impressionTag")]
    
    pub impression_tag: Option<String>,
}

impl client::Part for TagData {}


/// Tag Settings
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TagSetting {
    /// Additional key-values to be included in tags. Each key-value pair must be of the form key=value, and pairs must be separated by a semicolon (;). Keys and values must not contain commas. For example, id=2;color=red is a valid value for this field.
    #[serde(rename="additionalKeyValues")]
    
    pub additional_key_values: Option<String>,
    /// Whether static landing page URLs should be included in the tags. This setting applies only to placements.
    #[serde(rename="includeClickThroughUrls")]
    
    pub include_click_through_urls: Option<bool>,
    /// Whether click-tracking string should be included in the tags.
    #[serde(rename="includeClickTracking")]
    
    pub include_click_tracking: Option<bool>,
    /// Option specifying how keywords are embedded in ad tags. This setting can be used to specify whether keyword placeholders are inserted in placement tags for this site. Publishers can then add keywords to those placeholders.
    #[serde(rename="keywordOption")]
    
    pub keyword_option: Option<TagSettingKeywordOptionEnum>,
}

impl client::Part for TagSetting {}


/// Dynamic and Image Tag Settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TagSettings {
    /// Whether dynamic floodlight tags are enabled.
    #[serde(rename="dynamicTagEnabled")]
    
    pub dynamic_tag_enabled: Option<bool>,
    /// Whether image tags are enabled.
    #[serde(rename="imageTagEnabled")]
    
    pub image_tag_enabled: Option<bool>,
}

impl client::Part for TagSettings {}


/// Target Window.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetWindow {
    /// User-entered value.
    #[serde(rename="customHtml")]
    
    pub custom_html: Option<String>,
    /// Type of browser window for which the backup image of the flash creative can be displayed.
    #[serde(rename="targetWindowOption")]
    
    pub target_window_option: Option<TargetWindowTargetWindowOptionEnum>,
}

impl client::Part for TargetWindow {}


/// Contains properties of a targetable remarketing list. Remarketing enables you to create lists of users who have performed specific actions on a site, then target ads to members of those lists. This resource is a read-only view of a remarketing list to be used to faciliate targeting ads to specific lists. Remarketing lists that are owned by your advertisers and those that are shared to your advertisers or account are accessible via this resource. To manage remarketing lists that are owned by your advertisers, use the RemarketingLists resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get targetable remarketing lists](TargetableRemarketingListGetCall) (response)
/// * [list targetable remarketing lists](TargetableRemarketingListListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetableRemarketingList {
    /// Account ID of this remarketing list. This is a read-only, auto-generated field that is only returned in GET requests.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Whether this targetable remarketing list is active.
    
    pub active: Option<bool>,
    /// Dimension value for the advertiser ID that owns this targetable remarketing list.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Dimension value for the ID of the advertiser.
    #[serde(rename="advertiserIdDimensionValue")]
    
    pub advertiser_id_dimension_value: Option<DimensionValue>,
    /// Targetable remarketing list description.
    
    pub description: Option<String>,
    /// Targetable remarketing list ID.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#targetableRemarketingList".
    
    pub kind: Option<String>,
    /// Number of days that a user should remain in the targetable remarketing list without an impression.
    #[serde(rename="lifeSpan")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub life_span: Option<i64>,
    /// Number of users currently in the list. This is a read-only field.
    #[serde(rename="listSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub list_size: Option<i64>,
    /// Product from which this targetable remarketing list was originated.
    #[serde(rename="listSource")]
    
    pub list_source: Option<TargetableRemarketingListListSourceEnum>,
    /// Name of the targetable remarketing list. Is no greater than 128 characters long.
    
    pub name: Option<String>,
    /// Subaccount ID of this remarketing list. This is a read-only, auto-generated field that is only returned in GET requests.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
}

impl client::Resource for TargetableRemarketingList {}
impl client::ResponseResult for TargetableRemarketingList {}


/// Targetable remarketing list response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list targetable remarketing lists](TargetableRemarketingListListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetableRemarketingListsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#targetableRemarketingListsListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Targetable remarketing list collection.
    #[serde(rename="targetableRemarketingLists")]
    
    pub targetable_remarketing_lists: Option<Vec<TargetableRemarketingList>>,
}

impl client::ResponseResult for TargetableRemarketingListsListResponse {}


/// Contains properties of a targeting template. A targeting template encapsulates targeting information which can be reused across multiple ads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get targeting templates](TargetingTemplateGetCall) (response)
/// * [insert targeting templates](TargetingTemplateInsertCall) (request|response)
/// * [list targeting templates](TargetingTemplateListCall) (none)
/// * [patch targeting templates](TargetingTemplatePatchCall) (request|response)
/// * [update targeting templates](TargetingTemplateUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetingTemplate {
    /// Account ID of this targeting template. This field, if left unset, will be auto-generated on insert and is read-only after insert.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Advertiser ID of this targeting template. This is a required field on insert and is read-only after insert.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Dimension value for the ID of the advertiser. This is a read-only, auto-generated field.
    #[serde(rename="advertiserIdDimensionValue")]
    
    pub advertiser_id_dimension_value: Option<DimensionValue>,
    /// Time and day targeting criteria.
    #[serde(rename="dayPartTargeting")]
    
    pub day_part_targeting: Option<DayPartTargeting>,
    /// Geographical targeting criteria.
    #[serde(rename="geoTargeting")]
    
    pub geo_targeting: Option<GeoTargeting>,
    /// ID of this targeting template. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Key-value targeting criteria.
    #[serde(rename="keyValueTargetingExpression")]
    
    pub key_value_targeting_expression: Option<KeyValueTargetingExpression>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#targetingTemplate".
    
    pub kind: Option<String>,
    /// Language targeting criteria.
    #[serde(rename="languageTargeting")]
    
    pub language_targeting: Option<LanguageTargeting>,
    /// Remarketing list targeting criteria.
    #[serde(rename="listTargetingExpression")]
    
    pub list_targeting_expression: Option<ListTargetingExpression>,
    /// Name of this targeting template. This field is required. It must be less than 256 characters long and unique within an advertiser.
    
    pub name: Option<String>,
    /// Subaccount ID of this targeting template. This field, if left unset, will be auto-generated on insert and is read-only after insert.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
    /// Technology platform targeting criteria.
    #[serde(rename="technologyTargeting")]
    
    pub technology_targeting: Option<TechnologyTargeting>,
}

impl client::RequestValue for TargetingTemplate {}
impl client::Resource for TargetingTemplate {}
impl client::ResponseResult for TargetingTemplate {}


/// Targeting Template List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list targeting templates](TargetingTemplateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetingTemplatesListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#targetingTemplatesListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Targeting template collection.
    #[serde(rename="targetingTemplates")]
    
    pub targeting_templates: Option<Vec<TargetingTemplate>>,
}

impl client::ResponseResult for TargetingTemplatesListResponse {}


/// Technology Targeting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TechnologyTargeting {
    /// Browsers that this ad targets. For each browser either set browserVersionId or dartId along with the version numbers. If both are specified, only browserVersionId will be used. The other fields are populated automatically when the ad is inserted or updated.
    
    pub browsers: Option<Vec<Browser>>,
    /// Connection types that this ad targets. For each connection type only id is required. The other fields are populated automatically when the ad is inserted or updated.
    #[serde(rename="connectionTypes")]
    
    pub connection_types: Option<Vec<ConnectionType>>,
    /// Mobile carriers that this ad targets. For each mobile carrier only id is required, and the other fields are populated automatically when the ad is inserted or updated. If targeting a mobile carrier, do not set targeting for any zip codes.
    #[serde(rename="mobileCarriers")]
    
    pub mobile_carriers: Option<Vec<MobileCarrier>>,
    /// Operating system versions that this ad targets. To target all versions, use operatingSystems. For each operating system version, only id is required. The other fields are populated automatically when the ad is inserted or updated. If targeting an operating system version, do not set targeting for the corresponding operating system in operatingSystems.
    #[serde(rename="operatingSystemVersions")]
    
    pub operating_system_versions: Option<Vec<OperatingSystemVersion>>,
    /// Operating systems that this ad targets. To target specific versions, use operatingSystemVersions. For each operating system only dartId is required. The other fields are populated automatically when the ad is inserted or updated. If targeting an operating system, do not set targeting for operating system versions for the same operating system.
    #[serde(rename="operatingSystems")]
    
    pub operating_systems: Option<Vec<OperatingSystem>>,
    /// Platform types that this ad targets. For example, desktop, mobile, or tablet. For each platform type, only id is required, and the other fields are populated automatically when the ad is inserted or updated.
    #[serde(rename="platformTypes")]
    
    pub platform_types: Option<Vec<PlatformType>>,
}

impl client::Part for TechnologyTargeting {}


/// Third Party Authentication Token
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThirdPartyAuthenticationToken {
    /// Name of the third-party authentication token.
    
    pub name: Option<String>,
    /// Value of the third-party authentication token. This is a read-only, auto-generated field.
    
    pub value: Option<String>,
}

impl client::Part for ThirdPartyAuthenticationToken {}


/// Third-party Tracking URL.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThirdPartyTrackingUrl {
    /// Third-party URL type for in-stream video creatives.
    #[serde(rename="thirdPartyUrlType")]
    
    pub third_party_url_type: Option<ThirdPartyTrackingUrlThirdPartyUrlTypeEnum>,
    /// URL for the specified third-party URL type.
    
    pub url: Option<String>,
}

impl client::Part for ThirdPartyTrackingUrl {}


/// Transcode Settings
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TranscodeSetting {
    /// Whitelist of video formats to be served to this placement. Set this list to null or empty to serve all video formats.
    #[serde(rename="enabledVideoFormats")]
    
    pub enabled_video_formats: Option<Vec<i32>>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#transcodeSetting".
    
    pub kind: Option<String>,
}

impl client::Part for TranscodeSetting {}


/// A Universal Ad ID as per the VAST 4.0 spec. Applicable to the following creative types: INSTREAM_VIDEO and VPAID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UniversalAdId {
    /// Registry used for the Ad ID value.
    
    pub registry: Option<UniversalAdIdRegistryEnum>,
    /// ID value for this creative. Only alphanumeric characters and the following symbols are valid: "_/\-". Maximum length is 64 characters. Read only when registry is DCM.
    
    pub value: Option<String>,
}

impl client::Part for UniversalAdId {}


/// User Defined Variable configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserDefinedVariableConfiguration {
    /// Data type for the variable. This is a required field.
    #[serde(rename="dataType")]
    
    pub data_type: Option<UserDefinedVariableConfigurationDataTypeEnum>,
    /// User-friendly name for the variable which will appear in reports. This is a required field, must be less than 64 characters long, and cannot contain the following characters: ""<>".
    #[serde(rename="reportName")]
    
    pub report_name: Option<String>,
    /// Variable name in the tag. This is a required field.
    #[serde(rename="variableType")]
    
    pub variable_type: Option<UserDefinedVariableConfigurationVariableTypeEnum>,
}

impl client::Part for UserDefinedVariableConfiguration {}


/// Represents a UserProfile resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get user profiles](UserProfileGetCall) (response)
/// * [list user profiles](UserProfileListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserProfile {
    /// The account ID to which this profile belongs.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// The account name this profile belongs to.
    #[serde(rename="accountName")]
    
    pub account_name: Option<String>,
    /// The eTag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The kind of resource this is, in this case dfareporting#userProfile.
    
    pub kind: Option<String>,
    /// The unique ID of the user profile.
    #[serde(rename="profileId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub profile_id: Option<i64>,
    /// The sub account ID this profile belongs to if applicable.
    #[serde(rename="subAccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub sub_account_id: Option<i64>,
    /// The sub account name this profile belongs to if applicable.
    #[serde(rename="subAccountName")]
    
    pub sub_account_name: Option<String>,
    /// The user name.
    #[serde(rename="userName")]
    
    pub user_name: Option<String>,
}

impl client::Resource for UserProfile {}
impl client::ResponseResult for UserProfile {}


/// Represents the list of user profiles.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list user profiles](UserProfileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserProfileList {
    /// The eTag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The user profiles returned in this response.
    
    pub items: Option<Vec<UserProfile>>,
    /// The kind of list this is, in this case dfareporting#userProfileList.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for UserProfileList {}


/// Contains properties of auser role, which is used to manage user access.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete user roles](UserRoleDeleteCall) (none)
/// * [get user roles](UserRoleGetCall) (response)
/// * [insert user roles](UserRoleInsertCall) (request|response)
/// * [list user roles](UserRoleListCall) (none)
/// * [patch user roles](UserRolePatchCall) (request|response)
/// * [update user roles](UserRoleUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserRole {
    /// Account ID of this user role. This is a read-only field that can be left blank.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Whether this is a default user role. Default user roles are created by the system for the account/subaccount and cannot be modified or deleted. Each default user role comes with a basic set of preassigned permissions.
    #[serde(rename="defaultUserRole")]
    
    pub default_user_role: Option<bool>,
    /// ID of this user role. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#userRole".
    
    pub kind: Option<String>,
    /// Name of this user role. This is a required field. Must be less than 256 characters long. If this user role is under a subaccount, the name must be unique among sites of the same subaccount. Otherwise, this user role is a top-level user role, and the name must be unique among top-level user roles of the same account.
    
    pub name: Option<String>,
    /// ID of the user role that this user role is based on or copied from. This is a required field.
    #[serde(rename="parentUserRoleId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub parent_user_role_id: Option<i64>,
    /// List of permissions associated with this user role.
    
    pub permissions: Option<Vec<UserRolePermission>>,
    /// Subaccount ID of this user role. This is a read-only field that can be left blank.
    #[serde(rename="subaccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subaccount_id: Option<i64>,
}

impl client::RequestValue for UserRole {}
impl client::Resource for UserRole {}
impl client::ResponseResult for UserRole {}


/// Contains properties of a user role permission.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get user role permissions](UserRolePermissionGetCall) (response)
/// * [list user role permissions](UserRolePermissionListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserRolePermission {
    /// Levels of availability for a user role permission.
    
    pub availability: Option<UserRolePermissionAvailabilityEnum>,
    /// ID of this user role permission.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#userRolePermission".
    
    pub kind: Option<String>,
    /// Name of this user role permission.
    
    pub name: Option<String>,
    /// ID of the permission group that this user role permission belongs to.
    #[serde(rename="permissionGroupId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub permission_group_id: Option<i64>,
}

impl client::Resource for UserRolePermission {}
impl client::ResponseResult for UserRolePermission {}


/// Represents a grouping of related user role permissions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get user role permission groups](UserRolePermissionGroupGetCall) (response)
/// * [list user role permission groups](UserRolePermissionGroupListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserRolePermissionGroup {
    /// ID of this user role permission.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#userRolePermissionGroup".
    
    pub kind: Option<String>,
    /// Name of this user role permission group.
    
    pub name: Option<String>,
}

impl client::Resource for UserRolePermissionGroup {}
impl client::ResponseResult for UserRolePermissionGroup {}


/// User Role Permission Group List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list user role permission groups](UserRolePermissionGroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserRolePermissionGroupsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#userRolePermissionGroupsListResponse".
    
    pub kind: Option<String>,
    /// User role permission group collection.
    #[serde(rename="userRolePermissionGroups")]
    
    pub user_role_permission_groups: Option<Vec<UserRolePermissionGroup>>,
}

impl client::ResponseResult for UserRolePermissionGroupsListResponse {}


/// User Role Permission List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list user role permissions](UserRolePermissionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserRolePermissionsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#userRolePermissionsListResponse".
    
    pub kind: Option<String>,
    /// User role permission collection.
    #[serde(rename="userRolePermissions")]
    
    pub user_role_permissions: Option<Vec<UserRolePermission>>,
}

impl client::ResponseResult for UserRolePermissionsListResponse {}


/// User Role List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list user roles](UserRoleListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserRolesListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#userRolesListResponse".
    
    pub kind: Option<String>,
    /// Pagination token to be used for the next list operation.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// User role collection.
    #[serde(rename="userRoles")]
    
    pub user_roles: Option<Vec<UserRole>>,
}

impl client::ResponseResult for UserRolesListResponse {}


/// Contains information about supported video formats.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get video formats](VideoFormatGetCall) (response)
/// * [list video formats](VideoFormatListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoFormat {
    /// File type of the video format.
    #[serde(rename="fileType")]
    
    pub file_type: Option<VideoFormatFileTypeEnum>,
    /// ID of the video format.
    
    pub id: Option<i32>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#videoFormat".
    
    pub kind: Option<String>,
    /// The resolution of this video format.
    
    pub resolution: Option<Size>,
    /// The target bit rate of this video format.
    #[serde(rename="targetBitRate")]
    
    pub target_bit_rate: Option<i32>,
}

impl client::Resource for VideoFormat {}
impl client::ResponseResult for VideoFormat {}


/// Video Format List Response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list video formats](VideoFormatListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoFormatsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#videoFormatsListResponse".
    
    pub kind: Option<String>,
    /// Video format collection.
    #[serde(rename="videoFormats")]
    
    pub video_formats: Option<Vec<VideoFormat>>,
}

impl client::ResponseResult for VideoFormatsListResponse {}


/// Video Offset
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoOffset {
    /// Duration, as a percentage of video duration. Do not set when offsetSeconds is set. Acceptable values are 0 to 100, inclusive.
    #[serde(rename="offsetPercentage")]
    
    pub offset_percentage: Option<i32>,
    /// Duration, in seconds. Do not set when offsetPercentage is set. Acceptable values are 0 to 86399, inclusive.
    #[serde(rename="offsetSeconds")]
    
    pub offset_seconds: Option<i32>,
}

impl client::Part for VideoOffset {}


/// Video Settings
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoSettings {
    /// Settings for the companion creatives of video creatives served to this placement.
    #[serde(rename="companionSettings")]
    
    pub companion_settings: Option<CompanionSetting>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#videoSettings".
    
    pub kind: Option<String>,
    /// Settings for the skippability of video creatives served to this placement. If this object is provided, the creative-level skippable settings will be overridden.
    #[serde(rename="skippableSettings")]
    
    pub skippable_settings: Option<SkippableSetting>,
    /// Settings for the transcodes of video creatives served to this placement. If this object is provided, the creative-level transcode settings will be overridden.
    #[serde(rename="transcodeSettings")]
    
    pub transcode_settings: Option<TranscodeSetting>,
}

impl client::Part for VideoSettings {}


/// The URLs where the completed report file can be downloaded.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileUrls {
    /// The URL for downloading the report data through the API.
    #[serde(rename="apiUrl")]
    
    pub api_url: Option<String>,
    /// The URL for downloading the report data through a browser.
    #[serde(rename="browserUrl")]
    
    pub browser_url: Option<String>,
}

impl client::NestedType for FileUrls {}
impl client::Part for FileUrls {}


/// The report criteria for a report of type "STANDARD".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportCriteria {
    /// Activity group.
    
    pub activities: Option<Activities>,
    /// Custom Rich Media Events group.
    #[serde(rename="customRichMediaEvents")]
    
    pub custom_rich_media_events: Option<CustomRichMediaEvents>,
    /// The date range for which this report should be run.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<DateRange>,
    /// The list of filters on which dimensions are filtered.
    /// Filters for different dimensions are ANDed, filters for the same dimension are grouped together and ORed.
    #[serde(rename="dimensionFilters")]
    
    pub dimension_filters: Option<Vec<DimensionValue>>,
    /// The list of standard dimensions the report should include.
    
    pub dimensions: Option<Vec<SortedDimension>>,
    /// The list of names of metrics the report should include.
    #[serde(rename="metricNames")]
    
    pub metric_names: Option<Vec<String>>,
}

impl client::NestedType for ReportCriteria {}
impl client::Part for ReportCriteria {}


/// The report criteria for a report of type "CROSS_DIMENSION_REACH".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportCrossDimensionReachCriteria {
    /// The list of dimensions the report should include.
    
    pub breakdown: Option<Vec<SortedDimension>>,
    /// The date range this report should be run for.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<DateRange>,
    /// The dimension option.
    
    pub dimension: Option<ReportCrossDimensionReachCriterionDimensionEnum>,
    /// The list of filters on which dimensions are filtered.
    #[serde(rename="dimensionFilters")]
    
    pub dimension_filters: Option<Vec<DimensionValue>>,
    /// The list of names of metrics the report should include.
    #[serde(rename="metricNames")]
    
    pub metric_names: Option<Vec<String>>,
    /// The list of names of overlap metrics the report should include.
    #[serde(rename="overlapMetricNames")]
    
    pub overlap_metric_names: Option<Vec<String>>,
    /// Whether the report is pivoted or not. Defaults to true.
    
    pub pivoted: Option<bool>,
}

impl client::NestedType for ReportCrossDimensionReachCriteria {}
impl client::Part for ReportCrossDimensionReachCriteria {}


/// The report's email delivery settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportDelivery {
    /// Whether the report should be emailed to the report owner.
    #[serde(rename="emailOwner")]
    
    pub email_owner: Option<bool>,
    /// The type of delivery for the owner to receive, if enabled.
    #[serde(rename="emailOwnerDeliveryType")]
    
    pub email_owner_delivery_type: Option<ReportDeliveryEmailOwnerDeliveryTypeEnum>,
    /// The message to be sent with each email.
    
    pub message: Option<String>,
    /// The list of recipients to which to email the report.
    
    pub recipients: Option<Vec<Recipient>>,
}

impl client::NestedType for ReportDelivery {}
impl client::Part for ReportDelivery {}


/// The report criteria for a report of type "FLOODLIGHT".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportFloodlightCriteria {
    /// The list of custom rich media events to include.
    #[serde(rename="customRichMediaEvents")]
    
    pub custom_rich_media_events: Option<Vec<DimensionValue>>,
    /// The date range this report should be run for.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<DateRange>,
    /// The list of filters on which dimensions are filtered.
    /// Filters for different dimensions are ANDed, filters for the same dimension are grouped together and ORed.
    #[serde(rename="dimensionFilters")]
    
    pub dimension_filters: Option<Vec<DimensionValue>>,
    /// The list of dimensions the report should include.
    
    pub dimensions: Option<Vec<SortedDimension>>,
    /// The floodlight ID for which to show data in this report. All advertisers associated with that ID will automatically be added. The dimension of the value needs to be 'dfa:floodlightConfigId'.
    #[serde(rename="floodlightConfigId")]
    
    pub floodlight_config_id: Option<DimensionValue>,
    /// The list of names of metrics the report should include.
    #[serde(rename="metricNames")]
    
    pub metric_names: Option<Vec<String>>,
    /// The properties of the report.
    #[serde(rename="reportProperties")]
    
    pub report_properties: Option<ReportFloodlightCriteriaReportProperties>,
}

impl client::NestedType for ReportFloodlightCriteria {}
impl client::Part for ReportFloodlightCriteria {}


/// The properties of the report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportFloodlightCriteriaReportProperties {
    /// Include conversions that have no cookie, but do have an exposure path.
    #[serde(rename="includeAttributedIPConversions")]
    
    pub include_attributed_ip_conversions: Option<bool>,
    /// Include conversions of users with a DoubleClick cookie but without an exposure. That means the user did not click or see an ad from the advertiser within the Floodlight group, or that the interaction happened outside the lookback window.
    #[serde(rename="includeUnattributedCookieConversions")]
    
    pub include_unattributed_cookie_conversions: Option<bool>,
    /// Include conversions that have no associated cookies and no exposures. It’s therefore impossible to know how the user was exposed to your ads during the lookback window prior to a conversion.
    #[serde(rename="includeUnattributedIPConversions")]
    
    pub include_unattributed_ip_conversions: Option<bool>,
}

impl client::NestedType for ReportFloodlightCriteriaReportProperties {}
impl client::Part for ReportFloodlightCriteriaReportProperties {}


/// The report criteria for a report of type "PATH_TO_CONVERSION".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportPathToConversionCriteria {
    /// The list of 'dfa:activity' values to filter on.
    #[serde(rename="activityFilters")]
    
    pub activity_filters: Option<Vec<DimensionValue>>,
    /// The list of conversion dimensions the report should include.
    #[serde(rename="conversionDimensions")]
    
    pub conversion_dimensions: Option<Vec<SortedDimension>>,
    /// The list of custom floodlight variables the report should include.
    #[serde(rename="customFloodlightVariables")]
    
    pub custom_floodlight_variables: Option<Vec<SortedDimension>>,
    /// The list of custom rich media events to include.
    #[serde(rename="customRichMediaEvents")]
    
    pub custom_rich_media_events: Option<Vec<DimensionValue>>,
    /// The date range this report should be run for.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<DateRange>,
    /// The floodlight ID for which to show data in this report. All advertisers associated with that ID will automatically be added. The dimension of the value needs to be 'dfa:floodlightConfigId'.
    #[serde(rename="floodlightConfigId")]
    
    pub floodlight_config_id: Option<DimensionValue>,
    /// The list of names of metrics the report should include.
    #[serde(rename="metricNames")]
    
    pub metric_names: Option<Vec<String>>,
    /// The list of per interaction dimensions the report should include.
    #[serde(rename="perInteractionDimensions")]
    
    pub per_interaction_dimensions: Option<Vec<SortedDimension>>,
    /// The properties of the report.
    #[serde(rename="reportProperties")]
    
    pub report_properties: Option<ReportPathToConversionCriteriaReportProperties>,
}

impl client::NestedType for ReportPathToConversionCriteria {}
impl client::Part for ReportPathToConversionCriteria {}


/// The properties of the report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportPathToConversionCriteriaReportProperties {
    /// DFA checks to see if a click interaction occurred within the specified period of time before a conversion. By default the value is pulled from Floodlight or you can manually enter a custom value. Valid values: 1-90.
    #[serde(rename="clicksLookbackWindow")]
    
    pub clicks_lookback_window: Option<i32>,
    /// DFA checks to see if an impression interaction occurred within the specified period of time before a conversion. By default the value is pulled from Floodlight or you can manually enter a custom value. Valid values: 1-90.
    #[serde(rename="impressionsLookbackWindow")]
    
    pub impressions_lookback_window: Option<i32>,
    /// Deprecated: has no effect.
    #[serde(rename="includeAttributedIPConversions")]
    
    pub include_attributed_ip_conversions: Option<bool>,
    /// Include conversions of users with a DoubleClick cookie but without an exposure. That means the user did not click or see an ad from the advertiser within the Floodlight group, or that the interaction happened outside the lookback window.
    #[serde(rename="includeUnattributedCookieConversions")]
    
    pub include_unattributed_cookie_conversions: Option<bool>,
    /// Include conversions that have no associated cookies and no exposures. It’s therefore impossible to know how the user was exposed to your ads during the lookback window prior to a conversion.
    #[serde(rename="includeUnattributedIPConversions")]
    
    pub include_unattributed_ip_conversions: Option<bool>,
    /// The maximum number of click interactions to include in the report. Advertisers currently paying for E2C reports get up to 200 (100 clicks, 100 impressions). If another advertiser in your network is paying for E2C, you can have up to 5 total exposures per report.
    #[serde(rename="maximumClickInteractions")]
    
    pub maximum_click_interactions: Option<i32>,
    /// The maximum number of click interactions to include in the report. Advertisers currently paying for E2C reports get up to 200 (100 clicks, 100 impressions). If another advertiser in your network is paying for E2C, you can have up to 5 total exposures per report.
    #[serde(rename="maximumImpressionInteractions")]
    
    pub maximum_impression_interactions: Option<i32>,
    /// The maximum amount of time that can take place between interactions (clicks or impressions) by the same user. Valid values: 1-90.
    #[serde(rename="maximumInteractionGap")]
    
    pub maximum_interaction_gap: Option<i32>,
    /// Enable pivoting on interaction path.
    #[serde(rename="pivotOnInteractionPath")]
    
    pub pivot_on_interaction_path: Option<bool>,
}

impl client::NestedType for ReportPathToConversionCriteriaReportProperties {}
impl client::Part for ReportPathToConversionCriteriaReportProperties {}


/// The report criteria for a report of type "REACH".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportReachCriteria {
    /// Activity group.
    
    pub activities: Option<Activities>,
    /// Custom Rich Media Events group.
    #[serde(rename="customRichMediaEvents")]
    
    pub custom_rich_media_events: Option<CustomRichMediaEvents>,
    /// The date range this report should be run for.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<DateRange>,
    /// The list of filters on which dimensions are filtered.
    /// Filters for different dimensions are ANDed, filters for the same dimension are grouped together and ORed.
    #[serde(rename="dimensionFilters")]
    
    pub dimension_filters: Option<Vec<DimensionValue>>,
    /// The list of dimensions the report should include.
    
    pub dimensions: Option<Vec<SortedDimension>>,
    /// Whether to enable all reach dimension combinations in the report. Defaults to false. If enabled, the date range of the report should be within the last 42 days.
    #[serde(rename="enableAllDimensionCombinations")]
    
    pub enable_all_dimension_combinations: Option<bool>,
    /// The list of names of metrics the report should include.
    #[serde(rename="metricNames")]
    
    pub metric_names: Option<Vec<String>>,
    /// The list of names of  Reach By Frequency metrics the report should include.
    #[serde(rename="reachByFrequencyMetricNames")]
    
    pub reach_by_frequency_metric_names: Option<Vec<String>>,
}

impl client::NestedType for ReportReachCriteria {}
impl client::Part for ReportReachCriteria {}


/// The report's schedule. Can only be set if the report's 'dateRange' is a relative date range and the relative date range is not "TODAY".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportSchedule {
    /// Whether the schedule is active or not. Must be set to either true or false.
    
    pub active: Option<bool>,
    /// Defines every how many days, weeks or months the report should be run. Needs to be set when "repeats" is either "DAILY", "WEEKLY" or "MONTHLY".
    
    pub every: Option<i32>,
    /// The expiration date when the scheduled report stops running.
    #[serde(rename="expirationDate")]
    
    pub expiration_date: Option<client::chrono::NaiveDate>,
    /// The interval for which the report is repeated. Note:  
    /// - "DAILY" also requires field "every" to be set. 
    /// - "WEEKLY" also requires fields "every" and "repeatsOnWeekDays" to be set. 
    /// - "MONTHLY" also requires fields "every" and "runsOnDayOfMonth" to be set.
    
    pub repeats: Option<String>,
    /// List of week days "WEEKLY" on which scheduled reports should run.
    #[serde(rename="repeatsOnWeekDays")]
    
    pub repeats_on_week_days: Option<Vec<ReportScheduleRepeatsOnWeekDaysEnum>>,
    /// Enum to define for "MONTHLY" scheduled reports whether reports should be repeated on the same day of the month as "startDate" or the same day of the week of the month.
    /// Example: If 'startDate' is Monday, April 2nd 2012 (2012-04-02), "DAY_OF_MONTH" would run subsequent reports on the 2nd of every Month, and "WEEK_OF_MONTH" would run subsequent reports on the first Monday of the month.
    #[serde(rename="runsOnDayOfMonth")]
    
    pub runs_on_day_of_month: Option<ReportScheduleRunsOnDayOfMonthEnum>,
    /// Start date of date range for which scheduled reports should be run.
    #[serde(rename="startDate")]
    
    pub start_date: Option<client::chrono::NaiveDate>,
}

impl client::NestedType for ReportSchedule {}
impl client::Part for ReportSchedule {}


