use super::*;
/// Represents a targeting rule of the form: User never had {scope} before.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AcquisitionTargetingRule {
    /// Required. The scope of subscriptions this rule considers. Only allows "this subscription" and "any subscription in app".
    
    pub scope: Option<TargetingRuleScope>,
}

impl client::Part for AcquisitionTargetingRule {}


/// Request message for ActivateBasePlan.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions base plans activate monetization](MonetizationSubscriptionBasePlanActivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivateBasePlanRequest { _never_set: Option<bool> }

impl client::RequestValue for ActivateBasePlanRequest {}


/// Request message for ActivateSubscriptionOffer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions base plans offers activate monetization](MonetizationSubscriptionBasePlanOfferActivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivateSubscriptionOfferRequest { _never_set: Option<bool> }

impl client::RequestValue for ActivateSubscriptionOfferRequest {}


/// Information about an APK. The resource for ApksService.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apks upload edits](EditApkUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Apk {
    /// Information about the binary payload of this APK.
    
    pub binary: Option<ApkBinary>,
    /// The version code of the APK, as specified in the manifest file.
    #[serde(rename="versionCode")]
    
    pub version_code: Option<i32>,
}

impl client::ResponseResult for Apk {}


/// Represents the binary payload of an APK.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApkBinary {
    /// A sha1 hash of the APK payload, encoded as a hex string and matching the output of the sha1sum command.
    
    pub sha1: Option<String>,
    /// A sha256 hash of the APK payload, encoded as a hex string and matching the output of the sha256sum command.
    
    pub sha256: Option<String>,
}

impl client::Part for ApkBinary {}


/// Request to create a new externally hosted APK.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apks addexternallyhosted edits](EditApkAddexternallyhostedCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApksAddExternallyHostedRequest {
    /// The definition of the externally-hosted APK and where it is located.
    #[serde(rename="externallyHostedApk")]
    
    pub externally_hosted_apk: Option<ExternallyHostedApk>,
}

impl client::RequestValue for ApksAddExternallyHostedRequest {}


/// Response for creating a new externally hosted APK.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apks addexternallyhosted edits](EditApkAddexternallyhostedCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApksAddExternallyHostedResponse {
    /// The definition of the externally-hosted APK and where it is located.
    #[serde(rename="externallyHostedApk")]
    
    pub externally_hosted_apk: Option<ExternallyHostedApk>,
}

impl client::ResponseResult for ApksAddExternallyHostedResponse {}


/// Response listing all APKs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apks list edits](EditApkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApksListResponse {
    /// All APKs.
    
    pub apks: Option<Vec<Apk>>,
    /// The kind of this response ("androidpublisher#apksListResponse").
    
    pub kind: Option<String>,
}

impl client::ResponseResult for ApksListResponse {}


/// The app details. The resource for DetailsService.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [details get edits](EditDetailGetCall) (response)
/// * [details patch edits](EditDetailPatchCall) (request|response)
/// * [details update edits](EditDetailUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppDetails {
    /// The user-visible support email for this app.
    #[serde(rename="contactEmail")]
    
    pub contact_email: Option<String>,
    /// The user-visible support telephone number for this app.
    #[serde(rename="contactPhone")]
    
    pub contact_phone: Option<String>,
    /// The user-visible website for this app.
    #[serde(rename="contactWebsite")]
    
    pub contact_website: Option<String>,
    /// Default language code, in BCP 47 format (eg "en-US").
    #[serde(rename="defaultLanguage")]
    
    pub default_language: Option<String>,
}

impl client::RequestValue for AppDetails {}
impl client::ResponseResult for AppDetails {}


/// An app edit. The resource for EditsService.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [commit edits](EditCommitCall) (response)
/// * [get edits](EditGetCall) (response)
/// * [insert edits](EditInsertCall) (request|response)
/// * [validate edits](EditValidateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppEdit {
    /// Output only. The time (as seconds since Epoch) at which the edit will expire and will be no longer valid for use.
    #[serde(rename="expiryTimeSeconds")]
    
    pub expiry_time_seconds: Option<String>,
    /// Output only. Identifier of the edit. Can be used in subsequent API calls.
    
    pub id: Option<String>,
}

impl client::RequestValue for AppEdit {}
impl client::ResponseResult for AppEdit {}


/// Request message for ArchiveSubscription.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions archive monetization](MonetizationSubscriptionArchiveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArchiveSubscriptionRequest { _never_set: Option<bool> }

impl client::RequestValue for ArchiveSubscriptionRequest {}


/// Represents a base plan that automatically renews at the end of its subscription period.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoRenewingBasePlanType {
    /// Required. Subscription period, specified in ISO 8601 format. For a list of acceptable billing periods, refer to the help center.
    #[serde(rename="billingPeriodDuration")]
    
    pub billing_period_duration: Option<String>,
    /// Grace period of the subscription, specified in ISO 8601 format. Acceptable values are P0D (zero days), P3D (3 days), P7D (7 days), P14D (14 days), and P30D (30 days). If not specified, a default value will be used based on the recurring period duration.
    #[serde(rename="gracePeriodDuration")]
    
    pub grace_period_duration: Option<String>,
    /// Whether the renewing base plan is backward compatible. The backward compatible base plan is returned by the Google Play Billing Library deprecated method querySkuDetailsAsync(). Only one renewing base plan can be marked as legacy compatible for a given subscription.
    #[serde(rename="legacyCompatible")]
    
    pub legacy_compatible: Option<bool>,
    /// Subscription offer id which is legacy compatible. The backward compatible subscription offer is returned by the Google Play Billing Library deprecated method querySkuDetailsAsync(). Only one subscription offer can be marked as legacy compatible for a given renewing base plan. To have no Subscription offer as legacy compatible set this field as empty string.
    #[serde(rename="legacyCompatibleSubscriptionOfferId")]
    
    pub legacy_compatible_subscription_offer_id: Option<String>,
    /// The proration mode for the base plan determines what happens when a user switches to this plan from another base plan. If unspecified, defaults to CHARGE_ON_NEXT_BILLING_DATE.
    #[serde(rename="prorationMode")]
    
    pub proration_mode: Option<AutoRenewingBasePlanTypeProrationModeEnum>,
    /// Whether users should be able to resubscribe to this base plan in Google Play surfaces. Defaults to RESUBSCRIBE_STATE_ACTIVE if not specified.
    #[serde(rename="resubscribeState")]
    
    pub resubscribe_state: Option<AutoRenewingBasePlanTypeResubscribeStateEnum>,
}

impl client::Part for AutoRenewingBasePlanType {}


/// Information related to an auto renewing plan.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoRenewingPlan {
    /// If the subscription is currently set to auto-renew, e.g. the user has not canceled the subscription
    #[serde(rename="autoRenewEnabled")]
    
    pub auto_renew_enabled: Option<bool>,
    /// The information of the last price change for the item since subscription signup.
    #[serde(rename="priceChangeDetails")]
    
    pub price_change_details: Option<SubscriptionItemPriceChangeDetails>,
}

impl client::Part for AutoRenewingPlan {}


/// A single base plan for a subscription.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasePlan {
    /// Set when the base plan automatically renews at a regular interval.
    #[serde(rename="autoRenewingBasePlanType")]
    
    pub auto_renewing_base_plan_type: Option<AutoRenewingBasePlanType>,
    /// Required. Immutable. The unique identifier of this base plan. Must be unique within the subscription, and conform with RFC-1034. That is, this ID can only contain lower-case letters (a-z), numbers (0-9), and hyphens (-), and be at most 63 characters.
    #[serde(rename="basePlanId")]
    
    pub base_plan_id: Option<String>,
    /// List of up to 20 custom tags specified for this base plan, and returned to the app through the billing library. Subscription offers for this base plan will also receive these offer tags in the billing library.
    #[serde(rename="offerTags")]
    
    pub offer_tags: Option<Vec<OfferTag>>,
    /// Pricing information for any new locations Play may launch in the future. If omitted, the BasePlan will not be automatically available any new locations Play may launch in the future.
    #[serde(rename="otherRegionsConfig")]
    
    pub other_regions_config: Option<OtherRegionsBasePlanConfig>,
    /// Set when the base plan does not automatically renew at the end of the billing period.
    #[serde(rename="prepaidBasePlanType")]
    
    pub prepaid_base_plan_type: Option<PrepaidBasePlanType>,
    /// Region-specific information for this base plan.
    #[serde(rename="regionalConfigs")]
    
    pub regional_configs: Option<Vec<RegionalBasePlanConfig>>,
    /// Output only. The state of the base plan, i.e. whether it's active. Draft and inactive base plans can be activated or deleted. Active base plans can be made inactive. Inactive base plans can be canceled. This field cannot be changed by updating the resource. Use the dedicated endpoints instead.
    
    pub state: Option<BasePlanStateEnum>,
}

impl client::Part for BasePlan {}


/// Information about an app bundle. The resource for BundlesService.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [bundles upload edits](EditBundleUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bundle {
    /// A sha1 hash of the upload payload, encoded as a hex string and matching the output of the sha1sum command.
    
    pub sha1: Option<String>,
    /// A sha256 hash of the upload payload, encoded as a hex string and matching the output of the sha256sum command.
    
    pub sha256: Option<String>,
    /// The version code of the Android App Bundle, as specified in the Android App Bundle's base module APK manifest file.
    #[serde(rename="versionCode")]
    
    pub version_code: Option<i32>,
}

impl client::ResponseResult for Bundle {}


/// Response listing all app bundles.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [bundles list edits](EditBundleListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BundlesListResponse {
    /// All app bundles.
    
    pub bundles: Option<Vec<Bundle>>,
    /// The kind of this response ("androidpublisher#bundlesListResponse").
    
    pub kind: Option<String>,
}

impl client::ResponseResult for BundlesListResponse {}


/// Result of the cancel survey when the subscription was canceled by the user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelSurveyResult {
    /// The reason the user selected in the cancel survey.
    
    pub reason: Option<CancelSurveyResultReasonEnum>,
    /// Only set for CANCEL_SURVEY_REASON_OTHERS. This is the user's freeform response to the survey.
    #[serde(rename="reasonUserInput")]
    
    pub reason_user_input: Option<String>,
}

impl client::Part for CancelSurveyResult {}


/// Information specific to a subscription in canceled state.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CanceledStateContext {
    /// Subscription was canceled by the developer.
    #[serde(rename="developerInitiatedCancellation")]
    
    pub developer_initiated_cancellation: Option<DeveloperInitiatedCancellation>,
    /// Subscription was replaced by a new subscription.
    #[serde(rename="replacementCancellation")]
    
    pub replacement_cancellation: Option<ReplacementCancellation>,
    /// Subscription was canceled by the system, for example because of a billing problem.
    #[serde(rename="systemInitiatedCancellation")]
    
    pub system_initiated_cancellation: Option<SystemInitiatedCancellation>,
    /// Subscription was canceled by user.
    #[serde(rename="userInitiatedCancellation")]
    
    pub user_initiated_cancellation: Option<UserInitiatedCancellation>,
}

impl client::Part for CanceledStateContext {}


/// An entry of conversation between user and developer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    /// A comment from a developer.
    #[serde(rename="developerComment")]
    
    pub developer_comment: Option<DeveloperComment>,
    /// A comment from a user.
    #[serde(rename="userComment")]
    
    pub user_comment: Option<UserComment>,
}

impl client::Part for Comment {}


/// Request message for ConvertRegionPrices.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [convert region prices monetization](MonetizationConvertRegionPriceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConvertRegionPricesRequest {
    /// The intital price to convert other regions from. Tax exclusive.
    
    pub price: Option<Money>,
}

impl client::RequestValue for ConvertRegionPricesRequest {}


/// Response message for ConvertRegionPrices.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [convert region prices monetization](MonetizationConvertRegionPriceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConvertRegionPricesResponse {
    /// Converted other regions prices in USD and EUR, to use for countries where Play doesn't support a country's local currency.
    #[serde(rename="convertedOtherRegionsPrice")]
    
    pub converted_other_regions_price: Option<ConvertedOtherRegionsPrice>,
    /// Map from region code to converted region price.
    #[serde(rename="convertedRegionPrices")]
    
    pub converted_region_prices: Option<HashMap<String, ConvertedRegionPrice>>,
}

impl client::ResponseResult for ConvertRegionPricesResponse {}


/// Converted other regions prices.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConvertedOtherRegionsPrice {
    /// Price in EUR to use for the "Other regions" location exclusive of taxes.
    #[serde(rename="eurPrice")]
    
    pub eur_price: Option<Money>,
    /// Price in USD to use for the "Other regions" location exclusive of taxes.
    #[serde(rename="usdPrice")]
    
    pub usd_price: Option<Money>,
}

impl client::Part for ConvertedOtherRegionsPrice {}


/// A converted region price.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConvertedRegionPrice {
    /// The converted price tax inclusive.
    
    pub price: Option<Money>,
    /// The region code of the region.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
    /// The tax amount of the converted price.
    #[serde(rename="taxAmount")]
    
    pub tax_amount: Option<Money>,
}

impl client::Part for ConvertedRegionPrice {}


/// Country targeting specification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CountryTargeting {
    /// Countries to target, specified as two letter [CLDR codes](https://unicode.org/cldr/charts/latest/supplemental/territory_containment_un_m_49.html).
    
    pub countries: Option<Vec<String>>,
    /// Include "rest of world" as well as explicitly targeted countries.
    #[serde(rename="includeRestOfWorld")]
    
    pub include_rest_of_world: Option<bool>,
}

impl client::Part for CountryTargeting {}


/// Request message for DeactivateBasePlan.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions base plans deactivate monetization](MonetizationSubscriptionBasePlanDeactivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeactivateBasePlanRequest { _never_set: Option<bool> }

impl client::RequestValue for DeactivateBasePlanRequest {}


/// Request message for DeactivateSubscriptionOffer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions base plans offers deactivate monetization](MonetizationSubscriptionBasePlanOfferDeactivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeactivateSubscriptionOfferRequest { _never_set: Option<bool> }

impl client::RequestValue for DeactivateSubscriptionOfferRequest {}


/// Represents a deobfuscation file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeobfuscationFile {
    /// The type of the deobfuscation file.
    #[serde(rename="symbolType")]
    
    pub symbol_type: Option<DeobfuscationFileSymbolTypeEnum>,
}

impl client::Part for DeobfuscationFile {}


/// Responses for the upload.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deobfuscationfiles upload edits](EditDeobfuscationfileUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeobfuscationFilesUploadResponse {
    /// The uploaded Deobfuscation File configuration.
    #[serde(rename="deobfuscationFile")]
    
    pub deobfuscation_file: Option<DeobfuscationFile>,
}

impl client::ResponseResult for DeobfuscationFilesUploadResponse {}


/// Developer entry from conversation between user and developer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeveloperComment {
    /// The last time at which this comment was updated.
    #[serde(rename="lastModified")]
    
    pub last_modified: Option<Timestamp>,
    /// The content of the comment, i.e. reply body.
    
    pub text: Option<String>,
}

impl client::Part for DeveloperComment {}


/// Information specific to cancellations initiated by developers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeveloperInitiatedCancellation { _never_set: Option<bool> }

impl client::Part for DeveloperInitiatedCancellation {}


/// LINT.IfChange A group of devices. A group is defined by a set of device selectors. A device belongs to the group if it matches any selector (logical OR).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceGroup {
    /// Device selectors for this group. A device matching any of the selectors is included in this group.
    #[serde(rename="deviceSelectors")]
    
    pub device_selectors: Option<Vec<DeviceSelector>>,
    /// The name of the group.
    
    pub name: Option<String>,
}

impl client::Part for DeviceGroup {}


/// Identifier of a device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceId {
    /// Value of Build.BRAND.
    #[serde(rename="buildBrand")]
    
    pub build_brand: Option<String>,
    /// Value of Build.DEVICE.
    #[serde(rename="buildDevice")]
    
    pub build_device: Option<String>,
}

impl client::Part for DeviceId {}


/// Characteristics of the user's device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceMetadata {
    /// Device CPU make, e.g. "Qualcomm"
    #[serde(rename="cpuMake")]
    
    pub cpu_make: Option<String>,
    /// Device CPU model, e.g. "MSM8974"
    #[serde(rename="cpuModel")]
    
    pub cpu_model: Option<String>,
    /// Device class (e.g. tablet)
    #[serde(rename="deviceClass")]
    
    pub device_class: Option<String>,
    /// OpenGL version
    #[serde(rename="glEsVersion")]
    
    pub gl_es_version: Option<i32>,
    /// Device manufacturer (e.g. Motorola)
    
    pub manufacturer: Option<String>,
    /// Comma separated list of native platforms (e.g. "arm", "arm7")
    #[serde(rename="nativePlatform")]
    
    pub native_platform: Option<String>,
    /// Device model name (e.g. Droid)
    #[serde(rename="productName")]
    
    pub product_name: Option<String>,
    /// Device RAM in Megabytes, e.g. "2048"
    #[serde(rename="ramMb")]
    
    pub ram_mb: Option<i32>,
    /// Screen density in DPI
    #[serde(rename="screenDensityDpi")]
    
    pub screen_density_dpi: Option<i32>,
    /// Screen height in pixels
    #[serde(rename="screenHeightPx")]
    
    pub screen_height_px: Option<i32>,
    /// Screen width in pixels
    #[serde(rename="screenWidthPx")]
    
    pub screen_width_px: Option<i32>,
}

impl client::Part for DeviceMetadata {}


/// Conditions about a device's RAM capabilities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceRam {
    /// Maximum RAM in bytes (bound excluded).
    #[serde(rename="maxBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_bytes: Option<i64>,
    /// Minimum RAM in bytes (bound included).
    #[serde(rename="minBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_bytes: Option<i64>,
}

impl client::Part for DeviceRam {}


/// Selector for a device group. A selector consists of a set of conditions on the device that should all match (logical AND) to determine a device group eligibility. For instance, if a selector specifies RAM conditions, device model inclusion and device model exclusion, a device is considered to match if: device matches RAM conditions AND device matches one of the included device models AND device doesn't match excluded device models
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceSelector {
    /// Conditions on the device's RAM.
    #[serde(rename="deviceRam")]
    
    pub device_ram: Option<DeviceRam>,
    /// Device models excluded by this selector, even if they match all other conditions.
    #[serde(rename="excludedDeviceIds")]
    
    pub excluded_device_ids: Option<Vec<DeviceId>>,
    /// A device that has any of these system features is excluded by this selector, even if it matches all other conditions.
    #[serde(rename="forbiddenSystemFeatures")]
    
    pub forbidden_system_features: Option<Vec<SystemFeature>>,
    /// Device models included by this selector.
    #[serde(rename="includedDeviceIds")]
    
    pub included_device_ids: Option<Vec<DeviceId>>,
    /// A device needs to have all these system features to be included by the selector.
    #[serde(rename="requiredSystemFeatures")]
    
    pub required_system_features: Option<Vec<SystemFeature>>,
}

impl client::Part for DeviceSelector {}


/// The device spec used to generate a system APK.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceSpec {
    /// Screen dpi.
    #[serde(rename="screenDensity")]
    
    pub screen_density: Option<u32>,
    /// Supported ABI architectures in the order of preference. The values should be the string as reported by the platform, e.g. "armeabi-v7a", "x86_64".
    #[serde(rename="supportedAbis")]
    
    pub supported_abis: Option<Vec<String>>,
    /// All installed locales represented as BCP-47 strings, e.g. "en-US".
    #[serde(rename="supportedLocales")]
    
    pub supported_locales: Option<Vec<String>>,
}

impl client::Part for DeviceSpec {}


/// A single device tier. Devices matching any of the device groups in device_group_names are considered to match the tier.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceTier {
    /// Groups of devices included in this tier. These groups must be defined explicitly under device_groups in this configuration.
    #[serde(rename="deviceGroupNames")]
    
    pub device_group_names: Option<Vec<String>>,
    /// The priority level of the tier. Tiers are evaluated in descending order of level: the highest level tier has the highest priority. The highest tier matching a given device is selected for that device. You should use a contiguous range of levels for your tiers in a tier set; tier levels in a tier set must be unique. For instance, if your tier set has 4 tiers (including the global fallback), you should define tiers 1, 2 and 3 in this configuration. Note: tier 0 is implicitly defined as a global fallback and selected for devices that don't match any of the tiers explicitly defined here. You mustn't define level 0 explicitly in this configuration.
    
    pub level: Option<i32>,
}

impl client::Part for DeviceTier {}


/// LINT.IfChange Configuration describing device targeting criteria for the content of an app.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [device tier configs create applications](ApplicationDeviceTierConfigCreateCall) (request|response)
/// * [device tier configs get applications](ApplicationDeviceTierConfigGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceTierConfig {
    /// Definition of device groups for the app.
    #[serde(rename="deviceGroups")]
    
    pub device_groups: Option<Vec<DeviceGroup>>,
    /// Output only. The device tier config ID.
    #[serde(rename="deviceTierConfigId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub device_tier_config_id: Option<i64>,
    /// Definition of the set of device tiers for the app.
    #[serde(rename="deviceTierSet")]
    
    pub device_tier_set: Option<DeviceTierSet>,
}

impl client::RequestValue for DeviceTierConfig {}
impl client::ResponseResult for DeviceTierConfig {}


/// A set of device tiers. A tier set determines what variation of app content gets served to a specific device, for device-targeted content. You should assign a priority level to each tier, which determines the ordering by which they are evaluated by Play. See the documentation of DeviceTier.level for more details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceTierSet {
    /// Device tiers belonging to the set.
    #[serde(rename="deviceTiers")]
    
    pub device_tiers: Option<Vec<DeviceTier>>,
}

impl client::Part for DeviceTierSet {}


/// An expansion file. The resource for ExpansionFilesService.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [expansionfiles get edits](EditExpansionfileGetCall) (response)
/// * [expansionfiles patch edits](EditExpansionfilePatchCall) (request|response)
/// * [expansionfiles update edits](EditExpansionfileUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExpansionFile {
    /// If set, this field indicates that this APK has an expansion file uploaded to it: this APK does not reference another APK's expansion file. The field's value is the size of the uploaded expansion file in bytes.
    #[serde(rename="fileSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub file_size: Option<i64>,
    /// If set, this APK's expansion file references another APK's expansion file. The file_size field will not be set.
    #[serde(rename="referencesVersion")]
    
    pub references_version: Option<i32>,
}

impl client::RequestValue for ExpansionFile {}
impl client::ResponseResult for ExpansionFile {}


/// Response for uploading an expansion file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [expansionfiles upload edits](EditExpansionfileUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExpansionFilesUploadResponse {
    /// The uploaded expansion file configuration.
    #[serde(rename="expansionFile")]
    
    pub expansion_file: Option<ExpansionFile>,
}

impl client::ResponseResult for ExpansionFilesUploadResponse {}


/// User account identifier in the third-party service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExternalAccountIdentifiers {
    /// User account identifier in the third-party service. Only present if account linking happened as part of the subscription purchase flow.
    #[serde(rename="externalAccountId")]
    
    pub external_account_id: Option<String>,
    /// An obfuscated version of the id that is uniquely associated with the user's account in your app. Present for the following purchases: * If account linking happened as part of the subscription purchase flow. * It was specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedaccountid when the purchase was made.
    #[serde(rename="obfuscatedExternalAccountId")]
    
    pub obfuscated_external_account_id: Option<String>,
    /// An obfuscated version of the id that is uniquely associated with the user's profile in your app. Only present if specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedprofileid when the purchase was made.
    #[serde(rename="obfuscatedExternalProfileId")]
    
    pub obfuscated_external_profile_id: Option<String>,
}

impl client::Part for ExternalAccountIdentifiers {}


/// Defines an APK available for this application that is hosted externally and not uploaded to Google Play. This function is only available to organizations using Managed Play whose application is configured to restrict distribution to the organizations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExternallyHostedApk {
    /// The application label.
    #[serde(rename="applicationLabel")]
    
    pub application_label: Option<String>,
    /// A certificate (or array of certificates if a certificate-chain is used) used to sign this APK, represented as a base64 encoded byte array.
    #[serde(rename="certificateBase64s")]
    
    pub certificate_base64s: Option<Vec<String>>,
    /// The URL at which the APK is hosted. This must be an https URL.
    #[serde(rename="externallyHostedUrl")]
    
    pub externally_hosted_url: Option<String>,
    /// The sha1 checksum of this APK, represented as a base64 encoded byte array.
    #[serde(rename="fileSha1Base64")]
    
    pub file_sha1_base64: Option<String>,
    /// The sha256 checksum of this APK, represented as a base64 encoded byte array.
    #[serde(rename="fileSha256Base64")]
    
    pub file_sha256_base64: Option<String>,
    /// The file size in bytes of this APK.
    #[serde(rename="fileSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub file_size: Option<i64>,
    /// The icon image from the APK, as a base64 encoded byte array.
    #[serde(rename="iconBase64")]
    
    pub icon_base64: Option<String>,
    /// The maximum SDK supported by this APK (optional).
    #[serde(rename="maximumSdk")]
    
    pub maximum_sdk: Option<i32>,
    /// The minimum SDK targeted by this APK.
    #[serde(rename="minimumSdk")]
    
    pub minimum_sdk: Option<i32>,
    /// The native code environments supported by this APK (optional).
    #[serde(rename="nativeCodes")]
    
    pub native_codes: Option<Vec<String>>,
    /// The package name.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// The features required by this APK (optional).
    #[serde(rename="usesFeatures")]
    
    pub uses_features: Option<Vec<String>>,
    /// The permissions requested by this APK.
    #[serde(rename="usesPermissions")]
    
    pub uses_permissions: Option<Vec<UsesPermission>>,
    /// The version code of this APK.
    #[serde(rename="versionCode")]
    
    pub version_code: Option<i32>,
    /// The version name of this APK.
    #[serde(rename="versionName")]
    
    pub version_name: Option<String>,
}

impl client::Part for ExternallyHostedApk {}


/// Response to list generated APKs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list generatedapks](GeneratedapkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeneratedApksListResponse {
    /// All generated APKs, grouped by the APK signing key.
    #[serde(rename="generatedApks")]
    
    pub generated_apks: Option<Vec<GeneratedApksPerSigningKey>>,
}

impl client::ResponseResult for GeneratedApksListResponse {}


/// Download metadata for split, standalone and universal APKs, as well as asset pack slices, signed with a given key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeneratedApksPerSigningKey {
    /// SHA256 hash of the APK signing public key certificate.
    #[serde(rename="certificateSha256Hash")]
    
    pub certificate_sha256_hash: Option<String>,
    /// List of asset pack slices which will be served for this app bundle, signed with a key corresponding to certificate_sha256_hash.
    #[serde(rename="generatedAssetPackSlices")]
    
    pub generated_asset_pack_slices: Option<Vec<GeneratedAssetPackSlice>>,
    /// List of generated split APKs, signed with a key corresponding to certificate_sha256_hash.
    #[serde(rename="generatedSplitApks")]
    
    pub generated_split_apks: Option<Vec<GeneratedSplitApk>>,
    /// List of generated standalone APKs, signed with a key corresponding to certificate_sha256_hash.
    #[serde(rename="generatedStandaloneApks")]
    
    pub generated_standalone_apks: Option<Vec<GeneratedStandaloneApk>>,
    /// Generated universal APK, signed with a key corresponding to certificate_sha256_hash. This field is not set if no universal APK was generated for this signing key.
    #[serde(rename="generatedUniversalApk")]
    
    pub generated_universal_apk: Option<GeneratedUniversalApk>,
}

impl client::Part for GeneratedApksPerSigningKey {}


/// Download metadata for an asset pack slice.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeneratedAssetPackSlice {
    /// Download ID, which uniquely identifies the APK to download. Should be supplied to `generatedapks.download` method.
    #[serde(rename="downloadId")]
    
    pub download_id: Option<String>,
    /// Name of the module that this asset slice belongs to.
    #[serde(rename="moduleName")]
    
    pub module_name: Option<String>,
    /// Asset slice ID.
    #[serde(rename="sliceId")]
    
    pub slice_id: Option<String>,
    /// Asset module version.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
}

impl client::Part for GeneratedAssetPackSlice {}


/// Download metadata for a split APK.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeneratedSplitApk {
    /// Download ID, which uniquely identifies the APK to download. Should be supplied to `generatedapks.download` method.
    #[serde(rename="downloadId")]
    
    pub download_id: Option<String>,
    /// Name of the module that this APK belongs to.
    #[serde(rename="moduleName")]
    
    pub module_name: Option<String>,
    /// Split ID. Empty for the main split of the base module.
    #[serde(rename="splitId")]
    
    pub split_id: Option<String>,
    /// ID of the generated variant.
    #[serde(rename="variantId")]
    
    pub variant_id: Option<i32>,
}

impl client::Part for GeneratedSplitApk {}


/// Download metadata for a standalone APK.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeneratedStandaloneApk {
    /// Download ID, which uniquely identifies the APK to download. Should be supplied to `generatedapks.download` method.
    #[serde(rename="downloadId")]
    
    pub download_id: Option<String>,
    /// ID of the generated variant.
    #[serde(rename="variantId")]
    
    pub variant_id: Option<i32>,
}

impl client::Part for GeneratedStandaloneApk {}


/// Download metadata for a universal APK.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeneratedUniversalApk {
    /// Download ID, which uniquely identifies the APK to download. Should be supplied to `generatedapks.download` method.
    #[serde(rename="downloadId")]
    
    pub download_id: Option<String>,
}

impl client::Part for GeneratedUniversalApk {}


/// An access grant resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create grants](GrantCreateCall) (request|response)
/// * [delete grants](GrantDeleteCall) (none)
/// * [patch grants](GrantPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Grant {
    /// The permissions granted to the user for this app.
    #[serde(rename="appLevelPermissions")]
    
    pub app_level_permissions: Option<Vec<GrantAppLevelPermissionsEnum>>,
    /// Required. Resource name for this grant, following the pattern "developers/{developer}/users/{email}/grants/{package_name}". If this grant is for a draft app, the app ID will be used in this resource name instead of the package name.
    
    pub name: Option<String>,
    /// Immutable. The package name of the app. This will be empty for draft apps.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
}

impl client::RequestValue for Grant {}
impl client::Resource for Grant {}
impl client::ResponseResult for Grant {}


/// An uploaded image. The resource for ImagesService.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    /// A unique id representing this image.
    
    pub id: Option<String>,
    /// A sha1 hash of the image.
    
    pub sha1: Option<String>,
    /// A sha256 hash of the image.
    
    pub sha256: Option<String>,
    /// A URL that will serve a preview of the image.
    
    pub url: Option<String>,
}

impl client::Part for Image {}


/// Response for deleting all images.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [images deleteall edits](EditImageDeleteallCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImagesDeleteAllResponse {
    /// The deleted images.
    
    pub deleted: Option<Vec<Image>>,
}

impl client::ResponseResult for ImagesDeleteAllResponse {}


/// Response listing all images.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [images list edits](EditImageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImagesListResponse {
    /// All listed Images.
    
    pub images: Option<Vec<Image>>,
}

impl client::ResponseResult for ImagesListResponse {}


/// Response for uploading an image.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [images upload edits](EditImageUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImagesUploadResponse {
    /// The uploaded image.
    
    pub image: Option<Image>,
}

impl client::ResponseResult for ImagesUploadResponse {}


/// An in-app product. The resource for InappproductsService.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get inappproducts](InappproductGetCall) (response)
/// * [insert inappproducts](InappproductInsertCall) (request|response)
/// * [patch inappproducts](InappproductPatchCall) (request|response)
/// * [update inappproducts](InappproductUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InAppProduct {
    /// Default language of the localized data, as defined by BCP-47. e.g. "en-US".
    #[serde(rename="defaultLanguage")]
    
    pub default_language: Option<String>,
    /// Default price. Cannot be zero, as in-app products are never free. Always in the developer's Checkout merchant currency.
    #[serde(rename="defaultPrice")]
    
    pub default_price: Option<Price>,
    /// Grace period of the subscription, specified in ISO 8601 format. Allows developers to give their subscribers a grace period when the payment for the new recurrence period is declined. Acceptable values are P0D (zero days), P3D (three days), P7D (seven days), P14D (14 days), and P30D (30 days).
    #[serde(rename="gracePeriod")]
    
    pub grace_period: Option<String>,
    /// List of localized title and description data. Map key is the language of the localized data, as defined by BCP-47, e.g. "en-US".
    
    pub listings: Option<HashMap<String, InAppProductListing>>,
    /// Details about taxes and legal compliance. Only applicable to managed products.
    #[serde(rename="managedProductTaxesAndComplianceSettings")]
    
    pub managed_product_taxes_and_compliance_settings: Option<ManagedProductTaxAndComplianceSettings>,
    /// Package name of the parent app.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// Prices per buyer region. None of these can be zero, as in-app products are never free. Map key is region code, as defined by ISO 3166-2.
    
    pub prices: Option<HashMap<String, Price>>,
    /// The type of the product, e.g. a recurring subscription.
    #[serde(rename="purchaseType")]
    
    pub purchase_type: Option<InAppProductPurchaseTypeEnum>,
    /// Stock-keeping-unit (SKU) of the product, unique within an app.
    
    pub sku: Option<String>,
    /// The status of the product, e.g. whether it's active.
    
    pub status: Option<InAppProductStatusEnum>,
    /// Subscription period, specified in ISO 8601 format. Acceptable values are P1W (one week), P1M (one month), P3M (three months), P6M (six months), and P1Y (one year).
    #[serde(rename="subscriptionPeriod")]
    
    pub subscription_period: Option<String>,
    /// Details about taxes and legal compliance. Only applicable to subscription products.
    #[serde(rename="subscriptionTaxesAndComplianceSettings")]
    
    pub subscription_taxes_and_compliance_settings: Option<SubscriptionTaxAndComplianceSettings>,
    /// Trial period, specified in ISO 8601 format. Acceptable values are anything between P7D (seven days) and P999D (999 days).
    #[serde(rename="trialPeriod")]
    
    pub trial_period: Option<String>,
}

impl client::RequestValue for InAppProduct {}
impl client::Resource for InAppProduct {}
impl client::ResponseResult for InAppProduct {}


/// Store listing of a single in-app product.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InAppProductListing {
    /// Localized entitlement benefits for a subscription.
    
    pub benefits: Option<Vec<String>>,
    /// Description for the store listing.
    
    pub description: Option<String>,
    /// Title for the store listing.
    
    pub title: Option<String>,
}

impl client::Part for InAppProductListing {}


/// Response listing all in-app products.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list inappproducts](InappproductListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InappproductsListResponse {
    /// All in-app products.
    
    pub inappproduct: Option<Vec<InAppProduct>>,
    /// The kind of this response ("androidpublisher#inappproductsListResponse").
    
    pub kind: Option<String>,
    /// Deprecated and unset.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// Pagination token, to handle a number of products that is over one page.
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
}

impl client::ResponseResult for InappproductsListResponse {}


/// An artifact resource which gets created when uploading an APK or Android App Bundle through internal app sharing.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [uploadapk internalappsharingartifacts](InternalappsharingartifactUploadapkCall) (response)
/// * [uploadbundle internalappsharingartifacts](InternalappsharingartifactUploadbundleCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InternalAppSharingArtifact {
    /// The sha256 fingerprint of the certificate used to sign the generated artifact.
    #[serde(rename="certificateFingerprint")]
    
    pub certificate_fingerprint: Option<String>,
    /// The download URL generated for the uploaded artifact. Users that are authorized to download can follow the link to the Play Store app to install it.
    #[serde(rename="downloadUrl")]
    
    pub download_url: Option<String>,
    /// The sha256 hash of the artifact represented as a lowercase hexadecimal number, matching the output of the sha256sum command.
    
    pub sha256: Option<String>,
}

impl client::Resource for InternalAppSharingArtifact {}
impl client::ResponseResult for InternalAppSharingArtifact {}


/// Contains the introductory price information for a subscription.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IntroductoryPriceInfo {
    /// Introductory price of the subscription, not including tax. The currency is the same as price_currency_code. Price is expressed in micro-units, where 1,000,000 micro-units represents one unit of the currency. For example, if the subscription price is €1.99, price_amount_micros is 1990000.
    #[serde(rename="introductoryPriceAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub introductory_price_amount_micros: Option<i64>,
    /// ISO 4217 currency code for the introductory subscription price. For example, if the price is specified in British pounds sterling, price_currency_code is "GBP".
    #[serde(rename="introductoryPriceCurrencyCode")]
    
    pub introductory_price_currency_code: Option<String>,
    /// The number of billing period to offer introductory pricing.
    #[serde(rename="introductoryPriceCycles")]
    
    pub introductory_price_cycles: Option<i32>,
    /// Introductory price period, specified in ISO 8601 format. Common values are (but not limited to) "P1W" (one week), "P1M" (one month), "P3M" (three months), "P6M" (six months), and "P1Y" (one year).
    #[serde(rename="introductoryPricePeriod")]
    
    pub introductory_price_period: Option<String>,
}

impl client::Part for IntroductoryPriceInfo {}


/// Response listing existing device tier configs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [device tier configs list applications](ApplicationDeviceTierConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDeviceTierConfigsResponse {
    /// Device tier configs created by the developer.
    #[serde(rename="deviceTierConfigs")]
    
    pub device_tier_configs: Option<Vec<DeviceTierConfig>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDeviceTierConfigsResponse {}


/// Response message for ListSubscriptionOffers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions base plans offers list monetization](MonetizationSubscriptionBasePlanOfferListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSubscriptionOffersResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The subscription offers from the specified subscription.
    #[serde(rename="subscriptionOffers")]
    
    pub subscription_offers: Option<Vec<SubscriptionOffer>>,
}

impl client::ResponseResult for ListSubscriptionOffersResponse {}


/// Response message for ListSubscriptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions list monetization](MonetizationSubscriptionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSubscriptionsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The subscriptions from the specified app.
    
    pub subscriptions: Option<Vec<Subscription>>,
}

impl client::ResponseResult for ListSubscriptionsResponse {}


/// A response containing one or more users with access to an account.
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
    /// A token to pass to subsequent calls in order to retrieve subsequent results. This will not be set if there are no more results to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The resulting users.
    
    pub users: Option<Vec<User>>,
}

impl client::ResponseResult for ListUsersResponse {}


/// A localized store listing. The resource for ListingsService.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [listings get edits](EditListingGetCall) (response)
/// * [listings patch edits](EditListingPatchCall) (request|response)
/// * [listings update edits](EditListingUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Listing {
    /// Full description of the app.
    #[serde(rename="fullDescription")]
    
    pub full_description: Option<String>,
    /// Language localization code (a BCP-47 language tag; for example, "de-AT" for Austrian German).
    
    pub language: Option<String>,
    /// Short description of the app.
    #[serde(rename="shortDescription")]
    
    pub short_description: Option<String>,
    /// Localized title of the app.
    
    pub title: Option<String>,
    /// URL of a promotional YouTube video for the app.
    
    pub video: Option<String>,
}

impl client::RequestValue for Listing {}
impl client::ResponseResult for Listing {}


/// Response listing all localized listings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [listings list edits](EditListingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListingsListResponse {
    /// The kind of this response ("androidpublisher#listingsListResponse").
    
    pub kind: Option<String>,
    /// All localized listings.
    
    pub listings: Option<Vec<Listing>>,
}

impl client::ResponseResult for ListingsListResponse {}


/// Localized text in given language.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedText {
    /// Language localization code (a BCP-47 language tag; for example, "de-AT" for Austrian German).
    
    pub language: Option<String>,
    /// The text in the given language.
    
    pub text: Option<String>,
}

impl client::Part for LocalizedText {}


/// Details about taxation and legal compliance for managed products.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedProductTaxAndComplianceSettings {
    /// Digital content or service classification for products distributed to users in the European Economic Area (EEA). The withdrawal regime under EEA consumer laws depends on this classification. Refer to the [Help Center article](https://support.google.com/googleplay/android-developer/answer/10463498) for more information.
    #[serde(rename="eeaWithdrawalRightType")]
    
    pub eea_withdrawal_right_type: Option<ManagedProductTaxAndComplianceSettingEeaWithdrawalRightTypeEnum>,
    /// A mapping from region code to tax rate details. The keys are region codes as defined by Unicode's "CLDR".
    #[serde(rename="taxRateInfoByRegionCode")]
    
    pub tax_rate_info_by_region_code: Option<HashMap<String, RegionalTaxRateInfo>>,
}

impl client::Part for ManagedProductTaxAndComplianceSettings {}


/// Request message for MigrateBasePlanPrices.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions base plans migrate prices monetization](MonetizationSubscriptionBasePlanMigratePriceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MigrateBasePlanPricesRequest {
    /// Required. The regional prices to update.
    #[serde(rename="regionalPriceMigrations")]
    
    pub regional_price_migrations: Option<Vec<RegionalPriceMigrationConfig>>,
    /// Required. The version of the available regions being used for the regional_price_migrations.
    #[serde(rename="regionsVersion")]
    
    pub regions_version: Option<RegionsVersion>,
}

impl client::RequestValue for MigrateBasePlanPricesRequest {}


/// Response message for MigrateBasePlanPrices.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions base plans migrate prices monetization](MonetizationSubscriptionBasePlanMigratePriceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MigrateBasePlanPricesResponse { _never_set: Option<bool> }

impl client::ResponseResult for MigrateBasePlanPricesResponse {}


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


/// Offer details information related to a purchase line item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OfferDetails {
    /// The base plan ID. Present for all base plan and offers.
    #[serde(rename="basePlanId")]
    
    pub base_plan_id: Option<String>,
    /// The offer ID. Only present for discounted offers.
    #[serde(rename="offerId")]
    
    pub offer_id: Option<String>,
    /// The latest offer tags associated with the offer. It includes tags inherited from the base plan.
    #[serde(rename="offerTags")]
    
    pub offer_tags: Option<Vec<String>>,
}

impl client::Part for OfferDetails {}


/// Represents a custom tag specified for base plans and subscription offers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OfferTag {
    /// Must conform with RFC-1034. That is, this string can only contain lower-case letters (a-z), numbers (0-9), and hyphens (-), and be at most 20 characters.
    
    pub tag: Option<String>,
}

impl client::Part for OfferTag {}


/// Pricing information for any new locations Play may launch in.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OtherRegionsBasePlanConfig {
    /// Required. Price in EUR to use for any new locations Play may launch in.
    #[serde(rename="eurPrice")]
    
    pub eur_price: Option<Money>,
    /// Whether the base plan is available for new subscribers in any new locations Play may launch in. If not specified, this will default to false.
    #[serde(rename="newSubscriberAvailability")]
    
    pub new_subscriber_availability: Option<bool>,
    /// Required. Price in USD to use for any new locations Play may launch in.
    #[serde(rename="usdPrice")]
    
    pub usd_price: Option<Money>,
}

impl client::Part for OtherRegionsBasePlanConfig {}


/// Configuration for any new locations Play may launch in specified on a subscription offer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OtherRegionsSubscriptionOfferConfig {
    /// Whether the subscription offer in any new locations Play may launch in the future. If not specified, this will default to false.
    #[serde(rename="otherRegionsNewSubscriberAvailability")]
    
    pub other_regions_new_subscriber_availability: Option<bool>,
}

impl client::Part for OtherRegionsSubscriptionOfferConfig {}


/// Configuration for any new locations Play may launch in for a single offer phase.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OtherRegionsSubscriptionOfferPhaseConfig {
    /// The absolute amount of money subtracted from the base plan price prorated over the phase duration that the user pays for this offer phase. For example, if the base plan price for this region is $12 for a period of 1 year, then a $1 absolute discount for a phase of a duration of 3 months would correspond to a price of $2. The resulting price may not be smaller than the minimum price allowed for any new locations Play may launch in.
    #[serde(rename="absoluteDiscounts")]
    
    pub absolute_discounts: Option<OtherRegionsSubscriptionOfferPhasePrices>,
    /// The absolute price the user pays for this offer phase. The price must not be smaller than the minimum price allowed for any new locations Play may launch in.
    #[serde(rename="otherRegionsPrices")]
    
    pub other_regions_prices: Option<OtherRegionsSubscriptionOfferPhasePrices>,
    /// The fraction of the base plan price prorated over the phase duration that the user pays for this offer phase. For example, if the base plan price for this region is $12 for a period of 1 year, then a 50% discount for a phase of a duration of 3 months would correspond to a price of $1.50. The discount must be specified as a fraction strictly larger than 0 and strictly smaller than 1. The resulting price will be rounded to the nearest billable unit (e.g. cents for USD). The relative discount is considered invalid if the discounted price ends up being smaller than the minimum price allowed in any new locations Play may launch in.
    #[serde(rename="relativeDiscount")]
    
    pub relative_discount: Option<f64>,
}

impl client::Part for OtherRegionsSubscriptionOfferPhaseConfig {}


/// Pricing information for any new locations Play may launch in.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OtherRegionsSubscriptionOfferPhasePrices {
    /// Required. Price in EUR to use for any new locations Play may launch in.
    #[serde(rename="eurPrice")]
    
    pub eur_price: Option<Money>,
    /// Required. Price in USD to use for any new locations Play may launch in.
    #[serde(rename="usdPrice")]
    
    pub usd_price: Option<Money>,
}

impl client::Part for OtherRegionsSubscriptionOfferPhasePrices {}


/// Information about the current page. List operations that supports paging return only one "page" of results. This protocol buffer message describes the page that has been returned.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageInfo {
    /// Maximum number of results returned in one page. ! The number of results included in the API response.
    #[serde(rename="resultPerPage")]
    
    pub result_per_page: Option<i32>,
    /// Index of the first result returned in the current page.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// Total number of results available on the backend ! The total number of results in the result set.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
}

impl client::Part for PageInfo {}


/// Information specific to a subscription in paused state.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PausedStateContext {
    /// Time at which the subscription will be automatically resumed.
    #[serde(rename="autoResumeTime")]
    
    pub auto_resume_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for PausedStateContext {}


/// Represents a base plan that does not automatically renew at the end of the base plan, and must be manually renewed by the user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrepaidBasePlanType {
    /// Required. Subscription period, specified in ISO 8601 format. For a list of acceptable billing periods, refer to the help center.
    #[serde(rename="billingPeriodDuration")]
    
    pub billing_period_duration: Option<String>,
    /// Whether users should be able to extend this prepaid base plan in Google Play surfaces. Defaults to TIME_EXTENSION_ACTIVE if not specified.
    #[serde(rename="timeExtension")]
    
    pub time_extension: Option<PrepaidBasePlanTypeTimeExtensionEnum>,
}

impl client::Part for PrepaidBasePlanType {}


/// Information related to a prepaid plan.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrepaidPlan {
    /// If present, this is the time after which top up purchases are allowed for the prepaid plan. Will not be present for expired prepaid plans.
    #[serde(rename="allowExtendAfterTime")]
    
    pub allow_extend_after_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for PrepaidPlan {}


/// Definition of a price, i.e. currency and units.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Price {
    /// 3 letter Currency code, as defined by ISO 4217. See java/com/google/common/money/CurrencyCode.java
    
    pub currency: Option<String>,
    /// Price in 1/million of the currency base unit, represented as a string.
    #[serde(rename="priceMicros")]
    
    pub price_micros: Option<String>,
}

impl client::Part for Price {}


/// A ProductPurchase resource indicates the status of a user’s inapp product purchase.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [products get purchases](PurchaseProductGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductPurchase {
    /// The acknowledgement state of the inapp product. Possible values are: 0. Yet to be acknowledged 1. Acknowledged
    #[serde(rename="acknowledgementState")]
    
    pub acknowledgement_state: Option<i32>,
    /// The consumption state of the inapp product. Possible values are: 0. Yet to be consumed 1. Consumed
    #[serde(rename="consumptionState")]
    
    pub consumption_state: Option<i32>,
    /// A developer-specified string that contains supplemental information about an order.
    #[serde(rename="developerPayload")]
    
    pub developer_payload: Option<String>,
    /// This kind represents an inappPurchase object in the androidpublisher service.
    
    pub kind: Option<String>,
    /// An obfuscated version of the id that is uniquely associated with the user's account in your app. Only present if specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedaccountid when the purchase was made.
    #[serde(rename="obfuscatedExternalAccountId")]
    
    pub obfuscated_external_account_id: Option<String>,
    /// An obfuscated version of the id that is uniquely associated with the user's profile in your app. Only present if specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedprofileid when the purchase was made.
    #[serde(rename="obfuscatedExternalProfileId")]
    
    pub obfuscated_external_profile_id: Option<String>,
    /// The order id associated with the purchase of the inapp product.
    #[serde(rename="orderId")]
    
    pub order_id: Option<String>,
    /// The inapp product SKU. May not be present.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The purchase state of the order. Possible values are: 0. Purchased 1. Canceled 2. Pending
    #[serde(rename="purchaseState")]
    
    pub purchase_state: Option<i32>,
    /// The time the product was purchased, in milliseconds since the epoch (Jan 1, 1970).
    #[serde(rename="purchaseTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub purchase_time_millis: Option<i64>,
    /// The purchase token generated to identify this purchase. May not be present.
    #[serde(rename="purchaseToken")]
    
    pub purchase_token: Option<String>,
    /// The type of purchase of the inapp product. This field is only set if this purchase was not made using the standard in-app billing flow. Possible values are: 0. Test (i.e. purchased from a license testing account) 1. Promo (i.e. purchased using a promo code) 2. Rewarded (i.e. from watching a video ad instead of paying)
    #[serde(rename="purchaseType")]
    
    pub purchase_type: Option<i32>,
    /// The quantity associated with the purchase of the inapp product. If not present, the quantity is 1.
    
    pub quantity: Option<i32>,
    /// ISO 3166-1 alpha-2 billing region code of the user at the time the product was granted.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
}

impl client::ResponseResult for ProductPurchase {}


/// Request for the product.purchases.acknowledge API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [products acknowledge purchases](PurchaseProductAcknowledgeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductPurchasesAcknowledgeRequest {
    /// Payload to attach to the purchase.
    #[serde(rename="developerPayload")]
    
    pub developer_payload: Option<String>,
}

impl client::RequestValue for ProductPurchasesAcknowledgeRequest {}


/// Configuration for a base plan specific to a region.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegionalBasePlanConfig {
    /// Whether the base plan in the specified region is available for new subscribers. Existing subscribers will not have their subscription canceled if this value is set to false. If not specified, this will default to false.
    #[serde(rename="newSubscriberAvailability")]
    
    pub new_subscriber_availability: Option<bool>,
    /// The price of the base plan in the specified region. Must be set if the base plan is available to new subscribers. Must be set in the currency that is linked to the specified region.
    
    pub price: Option<Money>,
    /// Required. Region code this configuration applies to, as defined by ISO 3166-2, e.g. "US".
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
}

impl client::Part for RegionalBasePlanConfig {}


/// Configuration for a price migration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegionalPriceMigrationConfig {
    /// Required. The cutoff time for historical prices that subscribers can remain paying. Subscribers who are on a price that was created before this cutoff time will be migrated to the currently-offered price. These subscribers will receive a notification that they will be paying a different price. Subscribers who do not agree to the new price will have their subscription ended at the next renewal.
    #[serde(rename="oldestAllowedPriceVersionTime")]
    
    pub oldest_allowed_price_version_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Region code this configuration applies to, as defined by ISO 3166-2, e.g. "US".
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
}

impl client::Part for RegionalPriceMigrationConfig {}


/// Configuration for a subscription offer in a single region.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegionalSubscriptionOfferConfig {
    /// Whether the subscription offer in the specified region is available for new subscribers. Existing subscribers will not have their subscription cancelled if this value is set to false. If not specified, this will default to false.
    #[serde(rename="newSubscriberAvailability")]
    
    pub new_subscriber_availability: Option<bool>,
    /// Required. Immutable. Region code this configuration applies to, as defined by ISO 3166-2, e.g. "US".
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
}

impl client::Part for RegionalSubscriptionOfferConfig {}


/// Configuration for a single phase of a subscription offer in a single region.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegionalSubscriptionOfferPhaseConfig {
    /// The absolute amount of money subtracted from the base plan price prorated over the phase duration that the user pays for this offer phase. For example, if the base plan price for this region is $12 for a period of 1 year, then a $1 absolute discount for a phase of a duration of 3 months would correspond to a price of $2. The resulting price may not be smaller than the minimum price allowed for this region.
    #[serde(rename="absoluteDiscount")]
    
    pub absolute_discount: Option<Money>,
    /// The absolute price the user pays for this offer phase. The price must not be smaller than the minimum price allowed for this region.
    
    pub price: Option<Money>,
    /// Required. Immutable. The region to which this config applies.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
    /// The fraction of the base plan price prorated over the phase duration that the user pays for this offer phase. For example, if the base plan price for this region is $12 for a period of 1 year, then a 50% discount for a phase of a duration of 3 months would correspond to a price of $1.50. The discount must be specified as a fraction strictly larger than 0 and strictly smaller than 1. The resulting price will be rounded to the nearest billable unit (e.g. cents for USD). The relative discount is considered invalid if the discounted price ends up being smaller than the minimum price allowed in this region.
    #[serde(rename="relativeDiscount")]
    
    pub relative_discount: Option<f64>,
}

impl client::Part for RegionalSubscriptionOfferPhaseConfig {}


/// Specified details about taxation in a given geographical region.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegionalTaxRateInfo {
    /// You must tell us if your app contains streaming products to correctly charge US state and local sales tax. Field only supported in United States.
    #[serde(rename="eligibleForStreamingServiceTaxRate")]
    
    pub eligible_for_streaming_service_tax_rate: Option<bool>,
    /// To collect communications or amusement taxes in the United States, choose the appropriate tax category. [Learn more](https://support.google.com/googleplay/android-developer/answer/10463498#streaming_tax).
    #[serde(rename="streamingTaxType")]
    
    pub streaming_tax_type: Option<RegionalTaxRateInfoStreamingTaxTypeEnum>,
    /// Tax tier to specify reduced tax rate. Developers who sell digital news, magazines, newspapers, books, or audiobooks in various regions may be eligible for reduced tax rates. [Learn more](https://support.google.com/googleplay/android-developer/answer/10463498).
    #[serde(rename="taxTier")]
    
    pub tax_tier: Option<RegionalTaxRateInfoTaxTierEnum>,
}

impl client::Part for RegionalTaxRateInfo {}


/// The version of the available regions being used for the specified resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegionsVersion {
    /// Required. A string representing version of the available regions being used for the specified resource. The current version is 2022/02.
    
    pub version: Option<String>,
}

impl client::Part for RegionsVersion {}


/// Information specific to cancellations caused by subscription replacement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplacementCancellation { _never_set: Option<bool> }

impl client::Part for ReplacementCancellation {}


/// An Android app review.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get reviews](ReviewGetCall) (response)
/// * [list reviews](ReviewListCall) (none)
/// * [reply reviews](ReviewReplyCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Review {
    /// The name of the user who wrote the review.
    #[serde(rename="authorName")]
    
    pub author_name: Option<String>,
    /// A repeated field containing comments for the review.
    
    pub comments: Option<Vec<Comment>>,
    /// Unique identifier for this review.
    #[serde(rename="reviewId")]
    
    pub review_id: Option<String>,
}

impl client::Resource for Review {}
impl client::ResponseResult for Review {}


/// The result of replying/updating a reply to review.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReviewReplyResult {
    /// The time at which the reply took effect.
    #[serde(rename="lastEdited")]
    
    pub last_edited: Option<Timestamp>,
    /// The reply text that was applied.
    #[serde(rename="replyText")]
    
    pub reply_text: Option<String>,
}

impl client::Part for ReviewReplyResult {}


/// Response listing reviews.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list reviews](ReviewListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReviewsListResponse {
    /// Information about the current page.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// List of reviews.
    
    pub reviews: Option<Vec<Review>>,
    /// Pagination token, to handle a number of products that is over one page.
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
}

impl client::ResponseResult for ReviewsListResponse {}


/// Request to reply to review or update existing reply.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reply reviews](ReviewReplyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReviewsReplyRequest {
    /// The text to set as the reply. Replies of more than approximately 350 characters will be rejected. HTML tags will be stripped.
    #[serde(rename="replyText")]
    
    pub reply_text: Option<String>,
}

impl client::RequestValue for ReviewsReplyRequest {}


/// Response on status of replying to a review.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reply reviews](ReviewReplyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReviewsReplyResponse {
    /// The result of replying/updating a reply to review.
    
    pub result: Option<ReviewReplyResult>,
}

impl client::ResponseResult for ReviewsReplyResponse {}


/// Information associated with purchases made with 'Subscribe with Google'.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscribeWithGoogleInfo {
    /// The email address of the user when the subscription was purchased.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// The family name of the user when the subscription was purchased.
    #[serde(rename="familyName")]
    
    pub family_name: Option<String>,
    /// The given name of the user when the subscription was purchased.
    #[serde(rename="givenName")]
    
    pub given_name: Option<String>,
    /// The Google profile id of the user when the subscription was purchased.
    #[serde(rename="profileId")]
    
    pub profile_id: Option<String>,
    /// The profile name of the user when the subscription was purchased.
    #[serde(rename="profileName")]
    
    pub profile_name: Option<String>,
}

impl client::Part for SubscribeWithGoogleInfo {}


/// A single subscription for an app.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions base plans activate monetization](MonetizationSubscriptionBasePlanActivateCall) (response)
/// * [subscriptions base plans deactivate monetization](MonetizationSubscriptionBasePlanDeactivateCall) (response)
/// * [subscriptions archive monetization](MonetizationSubscriptionArchiveCall) (response)
/// * [subscriptions create monetization](MonetizationSubscriptionCreateCall) (request|response)
/// * [subscriptions get monetization](MonetizationSubscriptionGetCall) (response)
/// * [subscriptions patch monetization](MonetizationSubscriptionPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Subscription {
    /// Output only. Whether this subscription is archived. Archived subscriptions are not available to any subscriber any longer, cannot be updated, and are not returned in list requests unless the show archived flag is passed in.
    
    pub archived: Option<bool>,
    /// The set of base plans for this subscription. Represents the prices and duration of the subscription if no other offers apply.
    #[serde(rename="basePlans")]
    
    pub base_plans: Option<Vec<BasePlan>>,
    /// Required. List of localized listings for this subscription. Must contain at least an entry for the default language of the parent app.
    
    pub listings: Option<Vec<SubscriptionListing>>,
    /// Immutable. Package name of the parent app.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// Immutable. Unique product ID of the product. Unique within the parent app. Product IDs must be composed of lower-case letters (a-z), numbers (0-9), underscores (_) and dots (.). It must start with a lower-case letter or number, and be between 1 and 40 (inclusive) characters in length.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// Details about taxes and legal compliance.
    #[serde(rename="taxAndComplianceSettings")]
    
    pub tax_and_compliance_settings: Option<SubscriptionTaxAndComplianceSettings>,
}

impl client::RequestValue for Subscription {}
impl client::ResponseResult for Subscription {}


/// Information provided by the user when they complete the subscription cancellation flow (cancellation reason survey).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionCancelSurveyResult {
    /// The cancellation reason the user chose in the survey. Possible values are: 0. Other 1. I don't use this service enough 2. Technical issues 3. Cost-related reasons 4. I found a better app
    #[serde(rename="cancelSurveyReason")]
    
    pub cancel_survey_reason: Option<i32>,
    /// The customized input cancel reason from the user. Only present when cancelReason is 0.
    #[serde(rename="userInputCancelReason")]
    
    pub user_input_cancel_reason: Option<String>,
}

impl client::Part for SubscriptionCancelSurveyResult {}


/// A SubscriptionDeferralInfo contains the data needed to defer a subscription purchase to a future expiry time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionDeferralInfo {
    /// The desired next expiry time to assign to the subscription, in milliseconds since the Epoch. The given time must be later/greater than the current expiry time for the subscription.
    #[serde(rename="desiredExpiryTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub desired_expiry_time_millis: Option<i64>,
    /// The expected expiry time for the subscription. If the current expiry time for the subscription is not the value specified here, the deferral will not occur.
    #[serde(rename="expectedExpiryTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expected_expiry_time_millis: Option<i64>,
}

impl client::Part for SubscriptionDeferralInfo {}


/// Price change related information of a subscription item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionItemPriceChangeDetails {
    /// The renewal time at which the price change will become effective for the user. This is subject to change(to a future time) due to cases where the renewal time shifts like pause.
    #[serde(rename="expectedNewPriceChargeTime")]
    
    pub expected_new_price_charge_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// New recurring price for the subscription item.
    #[serde(rename="newPrice")]
    
    pub new_price: Option<Money>,
    /// Price change mode specifies how the subscription item price is changing.
    #[serde(rename="priceChangeMode")]
    
    pub price_change_mode: Option<SubscriptionItemPriceChangeDetailPriceChangeModeEnum>,
    /// State the price change is currently in.
    #[serde(rename="priceChangeState")]
    
    pub price_change_state: Option<SubscriptionItemPriceChangeDetailPriceChangeStateEnum>,
}

impl client::Part for SubscriptionItemPriceChangeDetails {}


/// The consumer-visible metadata of a subscription.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionListing {
    /// A list of benefits shown to the user on platforms such as the Play Store and in restoration flows in the language of this listing. Plain text. Ordered list of at most four benefits.
    
    pub benefits: Option<Vec<String>>,
    /// The description of this subscription in the language of this listing. Maximum length - 80 characters. Plain text.
    
    pub description: Option<String>,
    /// Required. The language of this listing, as defined by BCP-47, e.g. "en-US".
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Required. The title of this subscription in the language of this listing. Plain text.
    
    pub title: Option<String>,
}

impl client::Part for SubscriptionListing {}


/// A single, temporary offer
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions base plans offers activate monetization](MonetizationSubscriptionBasePlanOfferActivateCall) (response)
/// * [subscriptions base plans offers create monetization](MonetizationSubscriptionBasePlanOfferCreateCall) (request|response)
/// * [subscriptions base plans offers deactivate monetization](MonetizationSubscriptionBasePlanOfferDeactivateCall) (response)
/// * [subscriptions base plans offers get monetization](MonetizationSubscriptionBasePlanOfferGetCall) (response)
/// * [subscriptions base plans offers patch monetization](MonetizationSubscriptionBasePlanOfferPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionOffer {
    /// Required. Immutable. The ID of the base plan to which this offer is an extension.
    #[serde(rename="basePlanId")]
    
    pub base_plan_id: Option<String>,
    /// Required. Immutable. Unique ID of this subscription offer. Must be unique within the base plan.
    #[serde(rename="offerId")]
    
    pub offer_id: Option<String>,
    /// List of up to 20 custom tags specified for this offer, and returned to the app through the billing library.
    #[serde(rename="offerTags")]
    
    pub offer_tags: Option<Vec<OfferTag>>,
    /// The configuration for any new locations Play may launch in the future.
    #[serde(rename="otherRegionsConfig")]
    
    pub other_regions_config: Option<OtherRegionsSubscriptionOfferConfig>,
    /// Required. Immutable. The package name of the app the parent subscription belongs to.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// Required. The phases of this subscription offer. Must contain at least one entry, and may contain at most five. Users will always receive all these phases in the specified order. Phases may not be added, removed, or reordered after initial creation.
    
    pub phases: Option<Vec<SubscriptionOfferPhase>>,
    /// Required. Immutable. The ID of the parent subscription this offer belongs to.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// Required. The region-specific configuration of this offer. Must contain at least one entry.
    #[serde(rename="regionalConfigs")]
    
    pub regional_configs: Option<Vec<RegionalSubscriptionOfferConfig>>,
    /// Output only. The current state of this offer. Can be changed using Activate and Deactivate actions. NB: the base plan state supersedes this state, so an active offer may not be available if the base plan is not active.
    
    pub state: Option<SubscriptionOfferStateEnum>,
    /// The requirements that users need to fulfil to be eligible for this offer. Represents the requirements that Play will evaluate to decide whether an offer should be returned. Developers may further filter these offers themselves.
    
    pub targeting: Option<SubscriptionOfferTargeting>,
}

impl client::RequestValue for SubscriptionOffer {}
impl client::ResponseResult for SubscriptionOffer {}


/// A single phase of a subscription offer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionOfferPhase {
    /// Required. The duration of a single recurrence of this phase. Specified in ISO 8601 format.
    
    pub duration: Option<String>,
    /// Pricing information for any new locations Play may launch in.
    #[serde(rename="otherRegionsConfig")]
    
    pub other_regions_config: Option<OtherRegionsSubscriptionOfferPhaseConfig>,
    /// Required. The number of times this phase repeats. If this offer phase is not free, each recurrence charges the user the price of this offer phase.
    #[serde(rename="recurrenceCount")]
    
    pub recurrence_count: Option<i32>,
    /// Required. The region-specific configuration of this offer phase. This list must contain exactly one entry for each region for which the subscription offer has a regional config.
    #[serde(rename="regionalConfigs")]
    
    pub regional_configs: Option<Vec<RegionalSubscriptionOfferPhaseConfig>>,
}

impl client::Part for SubscriptionOfferPhase {}


/// Defines the rule a user needs to satisfy to receive this offer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionOfferTargeting {
    /// Offer targeting rule for new user acquisition.
    #[serde(rename="acquisitionRule")]
    
    pub acquisition_rule: Option<AcquisitionTargetingRule>,
    /// Offer targeting rule for upgrading users' existing plans.
    #[serde(rename="upgradeRule")]
    
    pub upgrade_rule: Option<UpgradeTargetingRule>,
}

impl client::Part for SubscriptionOfferTargeting {}


/// Contains the price change information for a subscription that can be used to control the user journey for the price change in the app. This can be in the form of seeking confirmation from the user or tailoring the experience for a successful conversion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionPriceChange {
    /// The new price the subscription will renew with if the price change is accepted by the user.
    #[serde(rename="newPrice")]
    
    pub new_price: Option<Price>,
    /// The current state of the price change. Possible values are: 0. Outstanding: State for a pending price change waiting for the user to agree. In this state, you can optionally seek confirmation from the user using the In-App API. 1. Accepted: State for an accepted price change that the subscription will renew with unless it's canceled. The price change takes effect on a future date when the subscription renews. Note that the change might not occur when the subscription is renewed next.
    
    pub state: Option<i32>,
}

impl client::Part for SubscriptionPriceChange {}


/// A SubscriptionPurchase resource indicates the status of a user’s subscription purchase.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions get purchases](PurchaseSubscriptionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionPurchase {
    /// The acknowledgement state of the subscription product. Possible values are: 0. Yet to be acknowledged 1. Acknowledged
    #[serde(rename="acknowledgementState")]
    
    pub acknowledgement_state: Option<i32>,
    /// Whether the subscription will automatically be renewed when it reaches its current expiry time.
    #[serde(rename="autoRenewing")]
    
    pub auto_renewing: Option<bool>,
    /// Time at which the subscription will be automatically resumed, in milliseconds since the Epoch. Only present if the user has requested to pause the subscription.
    #[serde(rename="autoResumeTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub auto_resume_time_millis: Option<i64>,
    /// The reason why a subscription was canceled or is not auto-renewing. Possible values are: 0. User canceled the subscription 1. Subscription was canceled by the system, for example because of a billing problem 2. Subscription was replaced with a new subscription 3. Subscription was canceled by the developer
    #[serde(rename="cancelReason")]
    
    pub cancel_reason: Option<i32>,
    /// Information provided by the user when they complete the subscription cancellation flow (cancellation reason survey).
    #[serde(rename="cancelSurveyResult")]
    
    pub cancel_survey_result: Option<SubscriptionCancelSurveyResult>,
    /// ISO 3166-1 alpha-2 billing country/region code of the user at the time the subscription was granted.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// A developer-specified string that contains supplemental information about an order.
    #[serde(rename="developerPayload")]
    
    pub developer_payload: Option<String>,
    /// The email address of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// Time at which the subscription will expire, in milliseconds since the Epoch.
    #[serde(rename="expiryTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expiry_time_millis: Option<i64>,
    /// User account identifier in the third-party service. Only present if account linking happened as part of the subscription purchase flow.
    #[serde(rename="externalAccountId")]
    
    pub external_account_id: Option<String>,
    /// The family name of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'.
    #[serde(rename="familyName")]
    
    pub family_name: Option<String>,
    /// The given name of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'.
    #[serde(rename="givenName")]
    
    pub given_name: Option<String>,
    /// Introductory price information of the subscription. This is only present when the subscription was purchased with an introductory price. This field does not indicate the subscription is currently in introductory price period.
    #[serde(rename="introductoryPriceInfo")]
    
    pub introductory_price_info: Option<IntroductoryPriceInfo>,
    /// This kind represents a subscriptionPurchase object in the androidpublisher service.
    
    pub kind: Option<String>,
    /// The purchase token of the originating purchase if this subscription is one of the following: 0. Re-signup of a canceled but non-lapsed subscription 1. Upgrade/downgrade from a previous subscription For example, suppose a user originally signs up and you receive purchase token X, then the user cancels and goes through the resignup flow (before their subscription lapses) and you receive purchase token Y, and finally the user upgrades their subscription and you receive purchase token Z. If you call this API with purchase token Z, this field will be set to Y. If you call this API with purchase token Y, this field will be set to X. If you call this API with purchase token X, this field will not be set.
    #[serde(rename="linkedPurchaseToken")]
    
    pub linked_purchase_token: Option<String>,
    /// An obfuscated version of the id that is uniquely associated with the user's account in your app. Present for the following purchases: * If account linking happened as part of the subscription purchase flow. * It was specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedaccountid when the purchase was made.
    #[serde(rename="obfuscatedExternalAccountId")]
    
    pub obfuscated_external_account_id: Option<String>,
    /// An obfuscated version of the id that is uniquely associated with the user's profile in your app. Only present if specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedprofileid when the purchase was made.
    #[serde(rename="obfuscatedExternalProfileId")]
    
    pub obfuscated_external_profile_id: Option<String>,
    /// The order id of the latest recurring order associated with the purchase of the subscription. If the subscription was canceled because payment was declined, this will be the order id from the payment declined order.
    #[serde(rename="orderId")]
    
    pub order_id: Option<String>,
    /// The payment state of the subscription. Possible values are: 0. Payment pending 1. Payment received 2. Free trial 3. Pending deferred upgrade/downgrade Not present for canceled, expired subscriptions.
    #[serde(rename="paymentState")]
    
    pub payment_state: Option<i32>,
    /// Price of the subscription, For tax exclusive countries, the price doesn't include tax. For tax inclusive countries, the price includes tax. Price is expressed in micro-units, where 1,000,000 micro-units represents one unit of the currency. For example, if the subscription price is €1.99, price_amount_micros is 1990000.
    #[serde(rename="priceAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub price_amount_micros: Option<i64>,
    /// The latest price change information available. This is present only when there is an upcoming price change for the subscription yet to be applied. Once the subscription renews with the new price or the subscription is canceled, no price change information will be returned.
    #[serde(rename="priceChange")]
    
    pub price_change: Option<SubscriptionPriceChange>,
    /// ISO 4217 currency code for the subscription price. For example, if the price is specified in British pounds sterling, price_currency_code is "GBP".
    #[serde(rename="priceCurrencyCode")]
    
    pub price_currency_code: Option<String>,
    /// The Google profile id of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'.
    #[serde(rename="profileId")]
    
    pub profile_id: Option<String>,
    /// The profile name of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'.
    #[serde(rename="profileName")]
    
    pub profile_name: Option<String>,
    /// The promotion code applied on this purchase. This field is only set if a vanity code promotion is applied when the subscription was purchased.
    #[serde(rename="promotionCode")]
    
    pub promotion_code: Option<String>,
    /// The type of promotion applied on this purchase. This field is only set if a promotion is applied when the subscription was purchased. Possible values are: 0. One time code 1. Vanity code
    #[serde(rename="promotionType")]
    
    pub promotion_type: Option<i32>,
    /// The type of purchase of the subscription. This field is only set if this purchase was not made using the standard in-app billing flow. Possible values are: 0. Test (i.e. purchased from a license testing account) 1. Promo (i.e. purchased using a promo code)
    #[serde(rename="purchaseType")]
    
    pub purchase_type: Option<i32>,
    /// Time at which the subscription was granted, in milliseconds since the Epoch.
    #[serde(rename="startTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time_millis: Option<i64>,
    /// The time at which the subscription was canceled by the user, in milliseconds since the epoch. Only present if cancelReason is 0.
    #[serde(rename="userCancellationTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub user_cancellation_time_millis: Option<i64>,
}

impl client::ResponseResult for SubscriptionPurchase {}


/// Item-level info for a subscription purchase.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionPurchaseLineItem {
    /// The item is auto renewing.
    #[serde(rename="autoRenewingPlan")]
    
    pub auto_renewing_plan: Option<AutoRenewingPlan>,
    /// Time at which the subscription expired or will expire unless the access is extended (ex. renews).
    #[serde(rename="expiryTime")]
    
    pub expiry_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The offer details for this item.
    #[serde(rename="offerDetails")]
    
    pub offer_details: Option<OfferDetails>,
    /// The item is prepaid.
    #[serde(rename="prepaidPlan")]
    
    pub prepaid_plan: Option<PrepaidPlan>,
    /// The purchased product ID (for example, 'monthly001').
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
}

impl client::Part for SubscriptionPurchaseLineItem {}


/// Indicates the status of a user’s subscription purchase.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptionsv2 get purchases](PurchaseSubscriptionsv2GetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionPurchaseV2 {
    /// The acknowledgement state of the subscription.
    #[serde(rename="acknowledgementState")]
    
    pub acknowledgement_state: Option<SubscriptionPurchaseV2AcknowledgementStateEnum>,
    /// Additional context around canceled subscriptions. Only present if the subscription currently has subscription_state SUBSCRIPTION_STATE_CANCELED.
    #[serde(rename="canceledStateContext")]
    
    pub canceled_state_context: Option<CanceledStateContext>,
    /// User account identifier in the third-party service.
    #[serde(rename="externalAccountIdentifiers")]
    
    pub external_account_identifiers: Option<ExternalAccountIdentifiers>,
    /// This kind represents a SubscriptionPurchaseV2 object in the androidpublisher service.
    
    pub kind: Option<String>,
    /// The order id of the latest order associated with the purchase of the subscription. For autoRenewing subscription, this is the order id of signup order if it is not renewed yet, or the last recurring order id (success, pending, or declined order). For prepaid subscription, this is the order id associated with the queried purchase token.
    #[serde(rename="latestOrderId")]
    
    pub latest_order_id: Option<String>,
    /// Item-level info for a subscription purchase. The items in the same purchase should be either all with AutoRenewingPlan or all with PrepaidPlan.
    #[serde(rename="lineItems")]
    
    pub line_items: Option<Vec<SubscriptionPurchaseLineItem>>,
    /// The purchase token of the old subscription if this subscription is one of the following: * Re-signup of a canceled but non-lapsed subscription * Upgrade/downgrade from a previous subscription. * Convert from prepaid to auto renewing subscription. * Convert from an auto renewing subscription to prepaid. * Topup a prepaid subscription.
    #[serde(rename="linkedPurchaseToken")]
    
    pub linked_purchase_token: Option<String>,
    /// Additional context around paused subscriptions. Only present if the subscription currently has subscription_state SUBSCRIPTION_STATE_PAUSED.
    #[serde(rename="pausedStateContext")]
    
    pub paused_state_context: Option<PausedStateContext>,
    /// ISO 3166-1 alpha-2 billing country/region code of the user at the time the subscription was granted.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
    /// Time at which the subscription was granted. Not set for pending subscriptions (subscription was created but awaiting payment during signup).
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// User profile associated with purchases made with 'Subscribe with Google'.
    #[serde(rename="subscribeWithGoogleInfo")]
    
    pub subscribe_with_google_info: Option<SubscribeWithGoogleInfo>,
    /// The current state of the subscription.
    #[serde(rename="subscriptionState")]
    
    pub subscription_state: Option<SubscriptionPurchaseV2SubscriptionStateEnum>,
    /// Only present if this subscription purchase is a test purchase.
    #[serde(rename="testPurchase")]
    
    pub test_purchase: Option<TestPurchase>,
}

impl client::ResponseResult for SubscriptionPurchaseV2 {}


/// Request for the purchases.subscriptions.acknowledge API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions acknowledge purchases](PurchaseSubscriptionAcknowledgeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionPurchasesAcknowledgeRequest {
    /// Payload to attach to the purchase.
    #[serde(rename="developerPayload")]
    
    pub developer_payload: Option<String>,
}

impl client::RequestValue for SubscriptionPurchasesAcknowledgeRequest {}


/// Request for the purchases.subscriptions.defer API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions defer purchases](PurchaseSubscriptionDeferCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionPurchasesDeferRequest {
    /// The information about the new desired expiry time for the subscription.
    #[serde(rename="deferralInfo")]
    
    pub deferral_info: Option<SubscriptionDeferralInfo>,
}

impl client::RequestValue for SubscriptionPurchasesDeferRequest {}


/// Response for the purchases.subscriptions.defer API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions defer purchases](PurchaseSubscriptionDeferCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionPurchasesDeferResponse {
    /// The new expiry time for the subscription in milliseconds since the Epoch.
    #[serde(rename="newExpiryTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub new_expiry_time_millis: Option<i64>,
}

impl client::ResponseResult for SubscriptionPurchasesDeferResponse {}


/// Details about taxation, Google Play policy and legal compliance for subscription products.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionTaxAndComplianceSettings {
    /// Digital content or service classification for products distributed to users in the European Economic Area (EEA). The withdrawal regime under EEA consumer laws depends on this classification. Refer to the [Help Center article](https://support.google.com/googleplay/android-developer/answer/10463498) for more information.
    #[serde(rename="eeaWithdrawalRightType")]
    
    pub eea_withdrawal_right_type: Option<SubscriptionTaxAndComplianceSettingEeaWithdrawalRightTypeEnum>,
    /// A mapping from region code to tax rate details. The keys are region codes as defined by Unicode's "CLDR".
    #[serde(rename="taxRateInfoByRegionCode")]
    
    pub tax_rate_info_by_region_code: Option<HashMap<String, RegionalTaxRateInfo>>,
}

impl client::Part for SubscriptionTaxAndComplianceSettings {}


/// Response to list previously created system APK variants.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [variants list systemapks](SystemapkVariantListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SystemApksListResponse {
    /// All system APK variants created.
    
    pub variants: Option<Vec<Variant>>,
}

impl client::ResponseResult for SystemApksListResponse {}


/// Representation of a system feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SystemFeature {
    /// The name of the feature.
    
    pub name: Option<String>,
}

impl client::Part for SystemFeature {}


/// Information specific to cancellations initiated by Google system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SystemInitiatedCancellation { _never_set: Option<bool> }

impl client::Part for SystemInitiatedCancellation {}


/// Defines the scope of subscriptions which a targeting rule can match to target offers to users based on past or current entitlement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetingRuleScope {
    /// The scope of the current targeting rule is the subscription with the specified subscription ID. Must be a subscription within the same parent app.
    #[serde(rename="specificSubscriptionInApp")]
    
    pub specific_subscription_in_app: Option<String>,
}

impl client::Part for TargetingRuleScope {}


/// Whether this subscription purchase is a test purchase.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestPurchase { _never_set: Option<bool> }

impl client::Part for TestPurchase {}


/// The testers of an app. The resource for TestersService. Note: while it is possible in the Play Console UI to add testers via email lists, email lists are not supported by this resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [testers get edits](EditTesterGetCall) (response)
/// * [testers patch edits](EditTesterPatchCall) (request|response)
/// * [testers update edits](EditTesterUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Testers {
    /// All testing Google Groups, as email addresses.
    #[serde(rename="googleGroups")]
    
    pub google_groups: Option<Vec<String>>,
}

impl client::RequestValue for Testers {}
impl client::ResponseResult for Testers {}


/// A Timestamp represents a point in time independent of any time zone or local calendar, encoded as a count of seconds and fractions of seconds at nanosecond resolution. The count is relative to an epoch at UTC midnight on January 1, 1970.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Timestamp {
    /// Non-negative fractions of a second at nanosecond resolution. Must be from 0 to 999,999,999 inclusive.
    
    pub nanos: Option<i32>,
    /// Represents seconds of UTC time since Unix epoch.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub seconds: Option<i64>,
}

impl client::Part for Timestamp {}


/// Pagination information returned by a List operation when token pagination is enabled. List operations that supports paging return only one "page" of results. This protocol buffer message describes the page that has been returned. When using token pagination, clients should use the next/previous token to get another page of the result. The presence or absence of next/previous token indicates whether a next/previous page is available and provides a mean of accessing this page. ListRequest.page_token should be set to either next_page_token or previous_page_token to access another page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TokenPagination {
    /// Tokens to pass to the standard list field 'page_token'. Whenever available, tokens are preferred over manipulating start_index.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    #[serde(rename="previousPageToken")]
    
    pub previous_page_token: Option<String>,
}

impl client::Part for TokenPagination {}


/// A track configuration. The resource for TracksService.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tracks get edits](EditTrackGetCall) (response)
/// * [tracks patch edits](EditTrackPatchCall) (request|response)
/// * [tracks update edits](EditTrackUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    /// In a read request, represents all active releases in the track. In an update request, represents desired changes.
    
    pub releases: Option<Vec<TrackRelease>>,
    /// Identifier of the track.
    
    pub track: Option<String>,
}

impl client::RequestValue for Track {}
impl client::ResponseResult for Track {}


/// Resource for per-track country availability information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [countryavailability get edits](EditCountryavailabilityGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrackCountryAvailability {
    /// A list of one or more countries where artifacts in this track are available. This list includes all countries that are targeted by the track, even if only specific carriers are targeted in that country.
    
    pub countries: Option<Vec<TrackTargetedCountry>>,
    /// Whether artifacts in this track are available to "rest of the world" countries.
    #[serde(rename="restOfWorld")]
    
    pub rest_of_world: Option<bool>,
    /// Whether this track's availability is synced with the default production track. See https://support.google.com/googleplay/android-developer/answer/7550024 for more information on syncing country availability with production. Note that if this is true, the returned "countries" and "rest_of_world" fields will reflect the values for the default production track.
    #[serde(rename="syncWithProduction")]
    
    pub sync_with_production: Option<bool>,
}

impl client::ResponseResult for TrackCountryAvailability {}


/// A release within a track.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrackRelease {
    /// Restricts a release to a specific set of countries.
    #[serde(rename="countryTargeting")]
    
    pub country_targeting: Option<CountryTargeting>,
    /// In-app update priority of the release. All newly added APKs in the release will be considered at this priority. Can take values in the range [0, 5], with 5 the highest priority. Defaults to 0. in_app_update_priority can not be updated once the release is rolled out. See https://developer.android.com/guide/playcore/in-app-updates.
    #[serde(rename="inAppUpdatePriority")]
    
    pub in_app_update_priority: Option<i32>,
    /// The release name. Not required to be unique. If not set, the name is generated from the APK's version_name. If the release contains multiple APKs, the name is generated from the date.
    
    pub name: Option<String>,
    /// A description of what is new in this release.
    #[serde(rename="releaseNotes")]
    
    pub release_notes: Option<Vec<LocalizedText>>,
    /// The status of the release.
    
    pub status: Option<TrackReleaseStatusEnum>,
    /// Fraction of users who are eligible for a staged release. 0 < fraction < 1. Can only be set when status is "inProgress" or "halted".
    #[serde(rename="userFraction")]
    
    pub user_fraction: Option<f64>,
    /// Version codes of all APKs in the release. Must include version codes to retain from previous releases.
    #[serde(rename="versionCodes")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub version_codes: Option<Vec<i64>>,
}

impl client::Part for TrackRelease {}


/// Representation of a single country where the contents of a track are available.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrackTargetedCountry {
    /// The country to target, as a two-letter CLDR code.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
}

impl client::Part for TrackTargetedCountry {}


/// Response listing all tracks.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tracks list edits](EditTrackListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TracksListResponse {
    /// The kind of this response ("androidpublisher#tracksListResponse").
    
    pub kind: Option<String>,
    /// All tracks.
    
    pub tracks: Option<Vec<Track>>,
}

impl client::ResponseResult for TracksListResponse {}


/// Represents a targeting rule of the form: User currently has {scope} [with billing period {billing_period}].
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeTargetingRule {
    /// The specific billing period duration, specified in ISO 8601 format, that a user must be currently subscribed to to be eligible for this rule. If not specified, users subscribed to any billing period are matched.
    #[serde(rename="billingPeriodDuration")]
    
    pub billing_period_duration: Option<String>,
    /// Limit this offer to only once per user. If set to true, a user can never be eligible for this offer again if they ever subscribed to this offer.
    #[serde(rename="oncePerUser")]
    
    pub once_per_user: Option<bool>,
    /// Required. The scope of subscriptions this rule considers. Only allows "this subscription" and "specific subscription in app".
    
    pub scope: Option<TargetingRuleScope>,
}

impl client::Part for UpgradeTargetingRule {}


/// A user resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create users](UserCreateCall) (request|response)
/// * [delete users](UserDeleteCall) (none)
/// * [list users](UserListCall) (none)
/// * [patch users](UserPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// Output only. The state of the user's access to the Play Console.
    #[serde(rename="accessState")]
    
    pub access_state: Option<UserAccessStateEnum>,
    /// Permissions for the user which apply across the developer account.
    #[serde(rename="developerAccountPermissions")]
    
    pub developer_account_permissions: Option<Vec<UserDeveloperAccountPermissionsEnum>>,
    /// Immutable. The user's email address.
    
    pub email: Option<String>,
    /// The time at which the user's access expires, if set. When setting this value, it must always be in the future.
    #[serde(rename="expirationTime")]
    
    pub expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Per-app permissions for the user.
    
    pub grants: Option<Vec<Grant>>,
    /// Required. Resource name for this user, following the pattern "developers/{developer}/users/{email}".
    
    pub name: Option<String>,
    /// Output only. Whether there are more permissions for the user that are not represented here. This can happen if the caller does not have permission to manage all apps in the account. This is also `true` if this user is the account owner. If this field is `true`, it should be taken as a signal that this user cannot be fully managed via the API. That is, the API caller is not be able to manage all of the permissions this user holds, either because it doesn't know about them or because the user is the account owner.
    
    pub partial: Option<bool>,
}

impl client::RequestValue for User {}
impl client::Resource for User {}
impl client::ResponseResult for User {}


/// User entry from conversation between user and developer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserComment {
    /// Integer Android SDK version of the user's device at the time the review was written, e.g. 23 is Marshmallow. May be absent.
    #[serde(rename="androidOsVersion")]
    
    pub android_os_version: Option<i32>,
    /// Integer version code of the app as installed at the time the review was written. May be absent.
    #[serde(rename="appVersionCode")]
    
    pub app_version_code: Option<i32>,
    /// String version name of the app as installed at the time the review was written. May be absent.
    #[serde(rename="appVersionName")]
    
    pub app_version_name: Option<String>,
    /// Codename for the reviewer's device, e.g. klte, flounder. May be absent.
    
    pub device: Option<String>,
    /// Information about the characteristics of the user's device.
    #[serde(rename="deviceMetadata")]
    
    pub device_metadata: Option<DeviceMetadata>,
    /// The last time at which this comment was updated.
    #[serde(rename="lastModified")]
    
    pub last_modified: Option<Timestamp>,
    /// Untranslated text of the review, where the review was translated. If the review was not translated this is left blank.
    #[serde(rename="originalText")]
    
    pub original_text: Option<String>,
    /// Language code for the reviewer. This is taken from the device settings so is not guaranteed to match the language the review is written in. May be absent.
    #[serde(rename="reviewerLanguage")]
    
    pub reviewer_language: Option<String>,
    /// The star rating associated with the review, from 1 to 5.
    #[serde(rename="starRating")]
    
    pub star_rating: Option<i32>,
    /// The content of the comment, i.e. review body. In some cases users have been able to write a review with separate title and body; in those cases the title and body are concatenated and separated by a tab character.
    
    pub text: Option<String>,
    /// Number of users who have given this review a thumbs down.
    #[serde(rename="thumbsDownCount")]
    
    pub thumbs_down_count: Option<i32>,
    /// Number of users who have given this review a thumbs up.
    #[serde(rename="thumbsUpCount")]
    
    pub thumbs_up_count: Option<i32>,
}

impl client::Part for UserComment {}


/// Information specific to cancellations initiated by users.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserInitiatedCancellation {
    /// Information provided by the user when they complete the subscription cancellation flow (cancellation reason survey).
    #[serde(rename="cancelSurveyResult")]
    
    pub cancel_survey_result: Option<CancelSurveyResult>,
    /// The time at which the subscription was canceled by the user. The user might still have access to the subscription after this time. Use line_items.expiry_time to determine if a user still has access.
    #[serde(rename="cancelTime")]
    
    pub cancel_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for UserInitiatedCancellation {}


/// A permission used by this APK.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsesPermission {
    /// Optionally, the maximum SDK version for which the permission is required.
    #[serde(rename="maxSdkVersion")]
    
    pub max_sdk_version: Option<i32>,
    /// The name of the permission requested.
    
    pub name: Option<String>,
}

impl client::Part for UsesPermission {}


/// APK that is suitable for inclusion in a system image. The resource of SystemApksService.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [variants create systemapks](SystemapkVariantCreateCall) (request|response)
/// * [variants get systemapks](SystemapkVariantGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Variant {
    /// The device spec used to generate the APK.
    #[serde(rename="deviceSpec")]
    
    pub device_spec: Option<DeviceSpec>,
    /// Output only. The ID of a previously created system APK variant.
    #[serde(rename="variantId")]
    
    pub variant_id: Option<u32>,
}

impl client::RequestValue for Variant {}
impl client::ResponseResult for Variant {}


/// A VoidedPurchase resource indicates a purchase that was either canceled/refunded/charged-back.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VoidedPurchase {
    /// This kind represents a voided purchase object in the androidpublisher service.
    
    pub kind: Option<String>,
    /// The order id which uniquely identifies a one-time purchase, subscription purchase, or subscription renewal.
    #[serde(rename="orderId")]
    
    pub order_id: Option<String>,
    /// The time at which the purchase was made, in milliseconds since the epoch (Jan 1, 1970).
    #[serde(rename="purchaseTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub purchase_time_millis: Option<i64>,
    /// The token which uniquely identifies a one-time purchase or subscription. To uniquely identify subscription renewals use order_id (available starting from version 3 of the API).
    #[serde(rename="purchaseToken")]
    
    pub purchase_token: Option<String>,
    /// The reason why the purchase was voided, possible values are: 0. Other 1. Remorse 2. Not_received 3. Defective 4. Accidental_purchase 5. Fraud 6. Friendly_fraud 7. Chargeback
    #[serde(rename="voidedReason")]
    
    pub voided_reason: Option<i32>,
    /// The initiator of voided purchase, possible values are: 0. User 1. Developer 2. Google
    #[serde(rename="voidedSource")]
    
    pub voided_source: Option<i32>,
    /// The time at which the purchase was canceled/refunded/charged-back, in milliseconds since the epoch (Jan 1, 1970).
    #[serde(rename="voidedTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub voided_time_millis: Option<i64>,
}

impl client::Part for VoidedPurchase {}


/// Response for the voidedpurchases.list API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [voidedpurchases list purchases](PurchaseVoidedpurchaseListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VoidedPurchasesListResponse {
    /// General pagination information.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// Pagination information for token pagination.
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// no description provided
    #[serde(rename="voidedPurchases")]
    
    pub voided_purchases: Option<Vec<VoidedPurchase>>,
}

impl client::ResponseResult for VoidedPurchasesListResponse {}


