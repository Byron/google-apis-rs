use super::*;
/// There is no detailed description.
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
    /// The version code of the APK, as specified in the APK's manifest file.
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


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apklistings get edits](EditApklistingGetCall) (response)
/// * [apklistings patch edits](EditApklistingPatchCall) (request|response)
/// * [apklistings update edits](EditApklistingUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApkListing {
    /// The language code, in BCP 47 format (eg "en-US").
    
    pub language: Option<String>,
    /// Describe what's new in your APK.
    #[serde(rename="recentChanges")]
    
    pub recent_changes: Option<String>,
}

impl client::RequestValue for ApkListing {}
impl client::ResponseResult for ApkListing {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apklistings list edits](EditApklistingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApkListingsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "androidpublisher#apkListingsListResponse".
    
    pub kind: Option<String>,
    /// no description provided
    
    pub listings: Option<Vec<ApkListing>>,
}

impl client::ResponseResult for ApkListingsListResponse {}


/// There is no detailed description.
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


/// There is no detailed description.
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


/// There is no detailed description.
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
    /// no description provided
    
    pub apks: Option<Vec<Apk>>,
    /// Identifies what kind of resource this is. Value: the fixed string "androidpublisher#apksListResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for ApksListResponse {}


/// There is no detailed description.
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


/// Represents an edit of an app. An edit allows clients to make multiple changes before committing them in one operation.
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
    /// The time at which the edit will expire and will be no longer valid for use in any subsequent API calls (encoded as seconds since the Epoch).
    #[serde(rename="expiryTimeSeconds")]
    
    pub expiry_time_seconds: Option<String>,
    /// The ID of the edit that can be used in subsequent API calls.
    
    pub id: Option<String>,
}

impl client::RequestValue for AppEdit {}
impl client::ResponseResult for AppEdit {}


/// There is no detailed description.
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
    /// The version code of the Android App Bundle. As specified in the Android App Bundle's base module APK manifest file.
    #[serde(rename="versionCode")]
    
    pub version_code: Option<i32>,
}

impl client::ResponseResult for Bundle {}


/// There is no detailed description.
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
    /// no description provided
    
    pub bundles: Option<Vec<Bundle>>,
    /// Identifies what kind of resource this is. Value: the fixed string "androidpublisher#bundlesListResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for BundlesListResponse {}


/// There is no detailed description.
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


/// Represents a deobfuscation file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeobfuscationFile {
    /// The type of the deobfuscation file.
    #[serde(rename="symbolType")]
    
    pub symbol_type: Option<String>,
}

impl client::Part for DeobfuscationFile {}


/// There is no detailed description.
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
    /// no description provided
    #[serde(rename="deobfuscationFile")]
    
    pub deobfuscation_file: Option<DeobfuscationFile>,
}

impl client::ResponseResult for DeobfuscationFilesUploadResponse {}


/// There is no detailed description.
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


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceMetadata {
    /// Device CPU make e.g. "Qualcomm"
    #[serde(rename="cpuMake")]
    
    pub cpu_make: Option<String>,
    /// Device CPU model e.g. "MSM8974"
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
    /// Device RAM in Megabytes e.g. "2048"
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


/// There is no detailed description.
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
    /// If set this field indicates that this APK has an Expansion File uploaded to it: this APK does not reference another APK's Expansion File. The field's value is the size of the uploaded Expansion File in bytes.
    #[serde(rename="fileSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub file_size: Option<i64>,
    /// If set this APK's Expansion File references another APK's Expansion File. The file_size field will not be set.
    #[serde(rename="referencesVersion")]
    
    pub references_version: Option<i32>,
}

impl client::RequestValue for ExpansionFile {}
impl client::ResponseResult for ExpansionFile {}


/// There is no detailed description.
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
    /// no description provided
    #[serde(rename="expansionFile")]
    
    pub expansion_file: Option<ExpansionFile>,
}

impl client::ResponseResult for ExpansionFilesUploadResponse {}


/// Defines an APK available for this application that is hosted externally and not uploaded to Google Play. This function is only available to enterprises who are using Google Play for Work, and whos application is restricted to the enterprise private channel
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExternallyHostedApk {
    /// The application label.
    #[serde(rename="applicationLabel")]
    
    pub application_label: Option<String>,
    /// A certificate (or array of certificates if a certificate-chain is used) used to signed this APK, represented as a base64 encoded byte array.
    #[serde(rename="certificateBase64s")]
    
    pub certificate_base64s: Option<Vec<String>>,
    /// The URL at which the APK is hosted. This must be an https URL.
    #[serde(rename="externallyHostedUrl")]
    
    pub externally_hosted_url: Option<String>,
    /// The SHA1 checksum of this APK, represented as a base64 encoded byte array.
    #[serde(rename="fileSha1Base64")]
    
    pub file_sha1_base64: Option<String>,
    /// The SHA256 checksum of this APK, represented as a base64 encoded byte array.
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
    
    pub uses_permissions: Option<Vec<ExternallyHostedApkUsesPermission>>,
    /// The version code of this APK.
    #[serde(rename="versionCode")]
    
    pub version_code: Option<i32>,
    /// The version name of this APK.
    #[serde(rename="versionName")]
    
    pub version_name: Option<String>,
}

impl client::Part for ExternallyHostedApk {}


/// A permission used by this APK.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExternallyHostedApkUsesPermission {
    /// Optionally, the maximum SDK version for which the permission is required.
    #[serde(rename="maxSdkVersion")]
    
    pub max_sdk_version: Option<i32>,
    /// The name of the permission requested.
    
    pub name: Option<String>,
}

impl client::Part for ExternallyHostedApkUsesPermission {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    /// A unique id representing this image.
    
    pub id: Option<String>,
    /// A sha1 hash of the image that was uploaded.
    
    pub sha1: Option<String>,
    /// A sha256 hash of the image that was uploaded.
    
    pub sha256: Option<String>,
    /// A URL that will serve a preview of the image.
    
    pub url: Option<String>,
}

impl client::Part for Image {}


/// There is no detailed description.
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
    /// no description provided
    
    pub deleted: Option<Vec<Image>>,
}

impl client::ResponseResult for ImagesDeleteAllResponse {}


/// There is no detailed description.
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
    /// no description provided
    
    pub images: Option<Vec<Image>>,
}

impl client::ResponseResult for ImagesListResponse {}


/// There is no detailed description.
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
    /// no description provided
    
    pub image: Option<Image>,
}

impl client::ResponseResult for ImagesUploadResponse {}


/// There is no detailed description.
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
    /// The default language of the localized data, as defined by BCP 47. e.g. "en-US", "en-GB".
    #[serde(rename="defaultLanguage")]
    
    pub default_language: Option<String>,
    /// Default price cannot be zero. In-app products can never be free. Default price is always in the developer's Checkout merchant currency.
    #[serde(rename="defaultPrice")]
    
    pub default_price: Option<Price>,
    /// Grace period of the subscription, specified in ISO 8601 format. It will allow developers to give their subscribers a grace period when the payment for the new recurrence period is declined. Acceptable values = "P3D" (three days), "P7D" (seven days), "P14D" (fourteen days), and "P30D" (thirty days)
    #[serde(rename="gracePeriod")]
    
    pub grace_period: Option<String>,
    /// List of localized title and description data.
    
    pub listings: Option<HashMap<String, InAppProductListing>>,
    /// The package name of the parent app.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// Prices per buyer region. None of these prices should be zero. In-app products can never be free.
    
    pub prices: Option<HashMap<String, Price>>,
    /// Purchase type enum value. Unmodifiable after creation.
    #[serde(rename="purchaseType")]
    
    pub purchase_type: Option<String>,
    /// The stock-keeping-unit (SKU) of the product, unique within an app.
    
    pub sku: Option<String>,
    /// no description provided
    
    pub status: Option<String>,
    /// Subscription period, specified in ISO 8601 format. Acceptable values are "P1W" (one week), "P1M" (one month), "P3M" (three months), "P6M" (six months), and "P1Y" (one year).
    #[serde(rename="subscriptionPeriod")]
    
    pub subscription_period: Option<String>,
    /// Trial period, specified in ISO 8601 format. Acceptable values are anything between "P7D" (seven days) and "P999D" (999 days). Seasonal subscriptions cannot have a trial period.
    #[serde(rename="trialPeriod")]
    
    pub trial_period: Option<String>,
}

impl client::RequestValue for InAppProduct {}
impl client::Resource for InAppProduct {}
impl client::ResponseResult for InAppProduct {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InAppProductListing {
    /// no description provided
    
    pub description: Option<String>,
    /// no description provided
    
    pub title: Option<String>,
}

impl client::Part for InAppProductListing {}


/// There is no detailed description.
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
    /// no description provided
    
    pub inappproduct: Option<Vec<InAppProduct>>,
    /// Identifies what kind of resource this is. Value: the fixed string "androidpublisher#inappproductsListResponse".
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
}

impl client::ResponseResult for InappproductsListResponse {}


/// There is no detailed description.
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
    /// Full description of the app; this may be up to 4000 characters in length.
    #[serde(rename="fullDescription")]
    
    pub full_description: Option<String>,
    /// Language localization code (for example, "de-AT" for Austrian German).
    
    pub language: Option<String>,
    /// Short description of the app (previously known as promo text); this may be up to 80 characters in length.
    #[serde(rename="shortDescription")]
    
    pub short_description: Option<String>,
    /// App's localized title.
    
    pub title: Option<String>,
    /// URL of a promotional YouTube video for the app.
    
    pub video: Option<String>,
}

impl client::RequestValue for Listing {}
impl client::ResponseResult for Listing {}


/// There is no detailed description.
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
    /// Identifies what kind of resource this is. Value: the fixed string "androidpublisher#listingsListResponse".
    
    pub kind: Option<String>,
    /// no description provided
    
    pub listings: Option<Vec<Listing>>,
}

impl client::ResponseResult for ListingsListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageInfo {
    /// no description provided
    #[serde(rename="resultPerPage")]
    
    pub result_per_page: Option<i32>,
    /// no description provided
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// no description provided
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
}

impl client::Part for PageInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Price {
    /// 3 letter Currency code, as defined by ISO 4217.
    
    pub currency: Option<String>,
    /// The price in millionths of the currency base unit represented as a string.
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
    /// The consumption state of the inapp product. Possible values are:  
    /// - Yet to be consumed 
    /// - Consumed
    #[serde(rename="consumptionState")]
    
    pub consumption_state: Option<i32>,
    /// A developer-specified string that contains supplemental information about an order.
    #[serde(rename="developerPayload")]
    
    pub developer_payload: Option<String>,
    /// This kind represents an inappPurchase object in the androidpublisher service.
    
    pub kind: Option<String>,
    /// The order id associated with the purchase of the inapp product.
    #[serde(rename="orderId")]
    
    pub order_id: Option<String>,
    /// The purchase state of the order. Possible values are:  
    /// - Purchased 
    /// - Canceled 
    /// - Pending
    #[serde(rename="purchaseState")]
    
    pub purchase_state: Option<i32>,
    /// The time the product was purchased, in milliseconds since the epoch (Jan 1, 1970).
    #[serde(rename="purchaseTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub purchase_time_millis: Option<i64>,
    /// The type of purchase of the inapp product. This field is only set if this purchase was not made using the standard in-app billing flow. Possible values are:  
    /// - Test (i.e. purchased from a license testing account) 
    /// - Promo (i.e. purchased using a promo code) 
    /// - Rewarded (i.e. from watching a video ad instead of paying)
    #[serde(rename="purchaseType")]
    
    pub purchase_type: Option<i32>,
}

impl client::ResponseResult for ProductPurchase {}


/// There is no detailed description.
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


/// There is no detailed description.
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


/// There is no detailed description.
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
    /// no description provided
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// no description provided
    
    pub reviews: Option<Vec<Review>>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
}

impl client::ResponseResult for ReviewsListResponse {}


/// There is no detailed description.
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


/// There is no detailed description.
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
    /// no description provided
    
    pub result: Option<ReviewReplyResult>,
}

impl client::ResponseResult for ReviewsReplyResponse {}


/// Information provided by the user when they complete the subscription cancellation flow (cancellation reason survey).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionCancelSurveyResult {
    /// The cancellation reason the user chose in the survey. Possible values are:  
    /// - Other 
    /// - I don't use this service enough 
    /// - Technical issues 
    /// - Cost-related reasons 
    /// - I found a better app
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
    /// The current state of the price change. Possible values are:  
    /// - Outstanding: State for a pending price change waiting for the user to agree. In this state, you can optionally seek confirmation from the user using the In-App API. 
    /// - Accepted: State for an accepted price change that the subscription will renew with unless it's canceled. The price change takes effect on a future date when the subscription renews. Note that the change might not occur when the subscription is renewed next.
    
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
    /// Whether the subscription will automatically be renewed when it reaches its current expiry time.
    #[serde(rename="autoRenewing")]
    
    pub auto_renewing: Option<bool>,
    /// The reason why a subscription was canceled or is not auto-renewing. Possible values are:  
    /// - User canceled the subscription 
    /// - Subscription was canceled by the system, for example because of a billing problem 
    /// - Subscription was replaced with a new subscription 
    /// - Subscription was canceled by the developer
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
    /// The family name of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'.
    #[serde(rename="familyName")]
    
    pub family_name: Option<String>,
    /// The given name of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'.
    #[serde(rename="givenName")]
    
    pub given_name: Option<String>,
    /// This kind represents a subscriptionPurchase object in the androidpublisher service.
    
    pub kind: Option<String>,
    /// The purchase token of the originating purchase if this subscription is one of the following:  
    /// - Re-signup of a canceled but non-lapsed subscription 
    /// - Upgrade/downgrade from a previous subscription  For example, suppose a user originally signs up and you receive purchase token X, then the user cancels and goes through the resignup flow (before their subscription lapses) and you receive purchase token Y, and finally the user upgrades their subscription and you receive purchase token Z. If you call this API with purchase token Z, this field will be set to Y. If you call this API with purchase token Y, this field will be set to X. If you call this API with purchase token X, this field will not be set.
    #[serde(rename="linkedPurchaseToken")]
    
    pub linked_purchase_token: Option<String>,
    /// The order id of the latest recurring order associated with the purchase of the subscription.
    #[serde(rename="orderId")]
    
    pub order_id: Option<String>,
    /// The payment state of the subscription. Possible values are:  
    /// - Payment pending 
    /// - Payment received 
    /// - Free trial 
    /// - Pending deferred upgrade/downgrade
    #[serde(rename="paymentState")]
    
    pub payment_state: Option<i32>,
    /// Price of the subscription, not including tax. Price is expressed in micro-units, where 1,000,000 micro-units represents one unit of the currency. For example, if the subscription price is €1.99, price_amount_micros is 1990000.
    #[serde(rename="priceAmountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub price_amount_micros: Option<i64>,
    /// The latest price change information available. This is present only when there is an upcoming price change for the subscription yet to be applied.
    /// 
    /// Once the subscription renews with the new price or the subscription is canceled, no price change information will be returned.
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
    /// The type of purchase of the subscription. This field is only set if this purchase was not made using the standard in-app billing flow. Possible values are:  
    /// - Test (i.e. purchased from a license testing account) 
    /// - Promo (i.e. purchased using a promo code)
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


/// There is no detailed description.
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


/// There is no detailed description.
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


/// There is no detailed description.
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
    /// A list of all Google Groups, as email addresses, that define testers for this track.
    #[serde(rename="googleGroups")]
    
    pub google_groups: Option<Vec<String>>,
}

impl client::RequestValue for Testers {}
impl client::ResponseResult for Testers {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Timestamp {
    /// no description provided
    
    pub nanos: Option<i32>,
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub seconds: Option<i64>,
}

impl client::Part for Timestamp {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TokenPagination {
    /// no description provided
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    #[serde(rename="previousPageToken")]
    
    pub previous_page_token: Option<String>,
}

impl client::Part for TokenPagination {}


/// There is no detailed description.
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
    /// Identifier for this track.
    
    pub track: Option<String>,
    /// no description provided
    #[serde(rename="userFraction")]
    
    pub user_fraction: Option<f64>,
    /// Version codes to make active on this track. Note that this list should contain all versions you wish to be active, including those you wish to retain from previous releases.
    #[serde(rename="versionCodes")]
    
    pub version_codes: Option<Vec<i32>>,
}

impl client::RequestValue for Track {}
impl client::ResponseResult for Track {}


/// There is no detailed description.
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
    /// Identifies what kind of resource this is. Value: the fixed string "androidpublisher#tracksListResponse".
    
    pub kind: Option<String>,
    /// no description provided
    
    pub tracks: Option<Vec<Track>>,
}

impl client::ResponseResult for TracksListResponse {}


/// There is no detailed description.
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
    /// Some information about the characteristics of the user's device
    #[serde(rename="deviceMetadata")]
    
    pub device_metadata: Option<DeviceMetadata>,
    /// The last time at which this comment was updated.
    #[serde(rename="lastModified")]
    
    pub last_modified: Option<Timestamp>,
    /// Untranslated text of the review, in the case where the review has been translated. If the review has not been translated this is left blank.
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
    /// Number of users who have given this review a thumbs down
    #[serde(rename="thumbsDownCount")]
    
    pub thumbs_down_count: Option<i32>,
    /// Number of users who have given this review a thumbs up
    #[serde(rename="thumbsUpCount")]
    
    pub thumbs_up_count: Option<i32>,
}

impl client::Part for UserComment {}


/// A VoidedPurchase resource indicates a purchase that was either canceled/refunded/charged-back.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VoidedPurchase {
    /// This kind represents a voided purchase object in the androidpublisher service.
    
    pub kind: Option<String>,
    /// The time at which the purchase was made, in milliseconds since the epoch (Jan 1, 1970).
    #[serde(rename="purchaseTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub purchase_time_millis: Option<i64>,
    /// The token which uniquely identifies a one-time purchase or subscription. To uniquely identify subscription renewals use order_id (available starting from version 3 of the API).
    #[serde(rename="purchaseToken")]
    
    pub purchase_token: Option<String>,
    /// The time at which the purchase was canceled/refunded/charged-back, in milliseconds since the epoch (Jan 1, 1970).
    #[serde(rename="voidedTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub voided_time_millis: Option<i64>,
}

impl client::Part for VoidedPurchase {}


/// There is no detailed description.
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
    /// no description provided
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// no description provided
    #[serde(rename="voidedPurchases")]
    
    pub voided_purchases: Option<Vec<VoidedPurchase>>,
}

impl client::ResponseResult for VoidedPurchasesListResponse {}


