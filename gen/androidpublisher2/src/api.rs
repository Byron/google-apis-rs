use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;

use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::sleep;
use tower_service;
use serde::{Serialize, Deserialize};

use crate::{client, client::GetToken, client::serde_with};

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and manage your Google Play Developer account
    Full,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/androidpublisher",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Full
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all AndroidPublisher related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::InAppProduct;
/// use androidpublisher2::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = InAppProduct::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.inappproducts().patch(req, "packageName", "sku")
///              .auto_convert_missing_prices(true)
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
pub struct AndroidPublisher<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for AndroidPublisher<S> {}

impl<'a, S> AndroidPublisher<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> AndroidPublisher<S> {
        AndroidPublisher {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.2-beta-1".to_string(),
            _base_url: "https://www.googleapis.com/androidpublisher/v2/applications/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn edits(&'a self) -> EditMethods<'a, S> {
        EditMethods { hub: &self }
    }
    pub fn inappproducts(&'a self) -> InappproductMethods<'a, S> {
        InappproductMethods { hub: &self }
    }
    pub fn orders(&'a self) -> OrderMethods<'a, S> {
        OrderMethods { hub: &self }
    }
    pub fn purchases(&'a self) -> PurchaseMethods<'a, S> {
        PurchaseMethods { hub: &self }
    }
    pub fn reviews(&'a self) -> ReviewMethods<'a, S> {
        ReviewMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.2-beta-1`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/androidpublisher/v2/applications/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apks upload edits](EditApkUploadCall) (response)
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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


/// A ProductPurchase resource indicates the status of a user's inapp product purchase.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [products get purchases](PurchaseProductGetCall) (response)
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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


/// A SubscriptionPurchase resource indicates the status of a user's subscription purchase.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions get purchases](PurchaseSubscriptionGetCall) (response)
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *edit* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher2 as androidpublisher2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `apklistings_delete(...)`, `apklistings_deleteall(...)`, `apklistings_get(...)`, `apklistings_list(...)`, `apklistings_patch(...)`, `apklistings_update(...)`, `apks_addexternallyhosted(...)`, `apks_list(...)`, `apks_upload(...)`, `bundles_list(...)`, `bundles_upload(...)`, `commit(...)`, `delete(...)`, `deobfuscationfiles_upload(...)`, `details_get(...)`, `details_patch(...)`, `details_update(...)`, `expansionfiles_get(...)`, `expansionfiles_patch(...)`, `expansionfiles_update(...)`, `expansionfiles_upload(...)`, `get(...)`, `images_delete(...)`, `images_deleteall(...)`, `images_list(...)`, `images_upload(...)`, `insert(...)`, `listings_delete(...)`, `listings_deleteall(...)`, `listings_get(...)`, `listings_list(...)`, `listings_patch(...)`, `listings_update(...)`, `testers_get(...)`, `testers_patch(...)`, `testers_update(...)`, `tracks_get(...)`, `tracks_list(...)`, `tracks_patch(...)`, `tracks_update(...)` and `validate(...)`
/// // to build up your call.
/// let rb = hub.edits();
/// # }
/// ```
pub struct EditMethods<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for EditMethods<'a, S> {}

impl<'a, S> EditMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the APK-specific localized listing for a specified APK and language code.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The APK version code whose APK-specific listings should be read or modified.
    /// * `language` - The language code (a BCP-47 language tag) of the APK-specific localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    pub fn apklistings_delete(&self, package_name: &str, edit_id: &str, apk_version_code: i32, language: &str) -> EditApklistingDeleteCall<'a, S> {
        EditApklistingDeleteCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes all the APK-specific localized listings for a specified APK.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The APK version code whose APK-specific listings should be read or modified.
    pub fn apklistings_deleteall(&self, package_name: &str, edit_id: &str, apk_version_code: i32) -> EditApklistingDeleteallCall<'a, S> {
        EditApklistingDeleteallCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the APK-specific localized listing for a specified APK and language code.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The APK version code whose APK-specific listings should be read or modified.
    /// * `language` - The language code (a BCP-47 language tag) of the APK-specific localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    pub fn apklistings_get(&self, package_name: &str, edit_id: &str, apk_version_code: i32, language: &str) -> EditApklistingGetCall<'a, S> {
        EditApklistingGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the APK-specific localized listings for a specified APK.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The APK version code whose APK-specific listings should be read or modified.
    pub fn apklistings_list(&self, package_name: &str, edit_id: &str, apk_version_code: i32) -> EditApklistingListCall<'a, S> {
        EditApklistingListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates or creates the APK-specific localized listing for a specified APK and language code. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The APK version code whose APK-specific listings should be read or modified.
    /// * `language` - The language code (a BCP-47 language tag) of the APK-specific localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    pub fn apklistings_patch(&self, request: ApkListing, package_name: &str, edit_id: &str, apk_version_code: i32, language: &str) -> EditApklistingPatchCall<'a, S> {
        EditApklistingPatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates or creates the APK-specific localized listing for a specified APK and language code.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The APK version code whose APK-specific listings should be read or modified.
    /// * `language` - The language code (a BCP-47 language tag) of the APK-specific localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    pub fn apklistings_update(&self, request: ApkListing, package_name: &str, edit_id: &str, apk_version_code: i32, language: &str) -> EditApklistingUpdateCall<'a, S> {
        EditApklistingUpdateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new APK without uploading the APK itself to Google Play, instead hosting the APK at a specified URL. This function is only available to enterprises using Google Play for Work whose application is configured to restrict distribution to the enterprise domain.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn apks_addexternallyhosted(&self, request: ApksAddExternallyHostedRequest, package_name: &str, edit_id: &str) -> EditApkAddexternallyhostedCall<'a, S> {
        EditApkAddexternallyhostedCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn apks_list(&self, package_name: &str, edit_id: &str) -> EditApkListCall<'a, S> {
        EditApkListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn apks_upload(&self, package_name: &str, edit_id: &str) -> EditApkUploadCall<'a, S> {
        EditApkUploadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn bundles_list(&self, package_name: &str, edit_id: &str) -> EditBundleListCall<'a, S> {
        EditBundleListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a new Android App Bundle to this edit. If you are using the Google API client libraries, please increase the timeout of the http request before calling this endpoint (a timeout of 2 minutes is recommended). See: https://developers.google.com/api-client-library/java/google-api-java-client/errors for an example in java.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn bundles_upload(&self, package_name: &str, edit_id: &str) -> EditBundleUploadCall<'a, S> {
        EditBundleUploadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _ack_bundle_installation_warning: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads the deobfuscation file of the specified APK. If a deobfuscation file already exists, it will be replaced.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier of the Android app for which the deobfuscatiuon files are being uploaded; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The version code of the APK whose deobfuscation file is being uploaded.
    /// * `deobfuscationFileType` - No description provided.
    pub fn deobfuscationfiles_upload(&self, package_name: &str, edit_id: &str, apk_version_code: i32, deobfuscation_file_type: &str) -> EditDeobfuscationfileUploadCall<'a, S> {
        EditDeobfuscationfileUploadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _deobfuscation_file_type: deobfuscation_file_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches app details for this edit. This includes the default language and developer support contact information.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn details_get(&self, package_name: &str, edit_id: &str) -> EditDetailGetCall<'a, S> {
        EditDetailGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates app details for this edit. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn details_patch(&self, request: AppDetails, package_name: &str, edit_id: &str) -> EditDetailPatchCall<'a, S> {
        EditDetailPatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates app details for this edit.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn details_update(&self, request: AppDetails, package_name: &str, edit_id: &str) -> EditDetailUpdateCall<'a, S> {
        EditDetailUpdateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the Expansion File configuration for the APK specified.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The version code of the APK whose Expansion File configuration is being read or modified.
    /// * `expansionFileType` - No description provided.
    pub fn expansionfiles_get(&self, package_name: &str, edit_id: &str, apk_version_code: i32, expansion_file_type: &str) -> EditExpansionfileGetCall<'a, S> {
        EditExpansionfileGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _expansion_file_type: expansion_file_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the APK's Expansion File configuration to reference another APK's Expansion Files. To add a new Expansion File use the Upload method. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The version code of the APK whose Expansion File configuration is being read or modified.
    /// * `expansionFileType` - No description provided.
    pub fn expansionfiles_patch(&self, request: ExpansionFile, package_name: &str, edit_id: &str, apk_version_code: i32, expansion_file_type: &str) -> EditExpansionfilePatchCall<'a, S> {
        EditExpansionfilePatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _expansion_file_type: expansion_file_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the APK's Expansion File configuration to reference another APK's Expansion Files. To add a new Expansion File use the Upload method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The version code of the APK whose Expansion File configuration is being read or modified.
    /// * `expansionFileType` - No description provided.
    pub fn expansionfiles_update(&self, request: ExpansionFile, package_name: &str, edit_id: &str, apk_version_code: i32, expansion_file_type: &str) -> EditExpansionfileUpdateCall<'a, S> {
        EditExpansionfileUpdateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _expansion_file_type: expansion_file_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads and attaches a new Expansion File to the APK specified.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The version code of the APK whose Expansion File configuration is being read or modified.
    /// * `expansionFileType` - No description provided.
    pub fn expansionfiles_upload(&self, package_name: &str, edit_id: &str, apk_version_code: i32, expansion_file_type: &str) -> EditExpansionfileUploadCall<'a, S> {
        EditExpansionfileUploadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _expansion_file_type: expansion_file_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the image (specified by id) from the edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing whose images are to read or modified. For example, to select Austrian German, pass "de-AT".
    /// * `imageType` - No description provided.
    /// * `imageId` - Unique identifier an image within the set of images attached to this edit.
    pub fn images_delete(&self, package_name: &str, edit_id: &str, language: &str, image_type: &str, image_id: &str) -> EditImageDeleteCall<'a, S> {
        EditImageDeleteCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _image_type: image_type.to_string(),
            _image_id: image_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes all images for the specified language and image type.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing whose images are to read or modified. For example, to select Austrian German, pass "de-AT".
    /// * `imageType` - No description provided.
    pub fn images_deleteall(&self, package_name: &str, edit_id: &str, language: &str, image_type: &str) -> EditImageDeleteallCall<'a, S> {
        EditImageDeleteallCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _image_type: image_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all images for the specified language and image type.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing whose images are to read or modified. For example, to select Austrian German, pass "de-AT".
    /// * `imageType` - No description provided.
    pub fn images_list(&self, package_name: &str, edit_id: &str, language: &str, image_type: &str) -> EditImageListCall<'a, S> {
        EditImageListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _image_type: image_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a new image and adds it to the list of images for the specified language and image type.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing whose images are to read or modified. For example, to select Austrian German, pass "de-AT".
    /// * `imageType` - No description provided.
    pub fn images_upload(&self, package_name: &str, edit_id: &str, language: &str, image_type: &str) -> EditImageUploadCall<'a, S> {
        EditImageUploadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _image_type: image_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified localized store listing from an edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    pub fn listings_delete(&self, package_name: &str, edit_id: &str, language: &str) -> EditListingDeleteCall<'a, S> {
        EditListingDeleteCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes all localized listings from an edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn listings_deleteall(&self, package_name: &str, edit_id: &str) -> EditListingDeleteallCall<'a, S> {
        EditListingDeleteallCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches information about a localized store listing.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    pub fn listings_get(&self, package_name: &str, edit_id: &str, language: &str) -> EditListingGetCall<'a, S> {
        EditListingGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns all of the localized store listings attached to this edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn listings_list(&self, package_name: &str, edit_id: &str) -> EditListingListCall<'a, S> {
        EditListingListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates or updates a localized store listing. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    pub fn listings_patch(&self, request: Listing, package_name: &str, edit_id: &str, language: &str) -> EditListingPatchCall<'a, S> {
        EditListingPatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates or updates a localized store listing.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    pub fn listings_update(&self, request: Listing, package_name: &str, edit_id: &str, language: &str) -> EditListingUpdateCall<'a, S> {
        EditListingUpdateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `track` - The track to read or modify.
    pub fn testers_get(&self, package_name: &str, edit_id: &str, track: &str) -> EditTesterGetCall<'a, S> {
        EditTesterGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _track: track.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `track` - The track to read or modify.
    pub fn testers_patch(&self, request: Testers, package_name: &str, edit_id: &str, track: &str) -> EditTesterPatchCall<'a, S> {
        EditTesterPatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _track: track.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `track` - The track to read or modify.
    pub fn testers_update(&self, request: Testers, package_name: &str, edit_id: &str, track: &str) -> EditTesterUpdateCall<'a, S> {
        EditTesterUpdateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _track: track.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the track configuration for the specified track type. Includes the APK version codes that are in this track.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `track` - The track to read or modify.
    pub fn tracks_get(&self, package_name: &str, edit_id: &str, track: &str) -> EditTrackGetCall<'a, S> {
        EditTrackGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _track: track.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the track configurations for this edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn tracks_list(&self, package_name: &str, edit_id: &str) -> EditTrackListCall<'a, S> {
        EditTrackListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the track configuration for the specified track type. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `track` - The track to read or modify.
    pub fn tracks_patch(&self, request: Track, package_name: &str, edit_id: &str, track: &str) -> EditTrackPatchCall<'a, S> {
        EditTrackPatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _track: track.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the track configuration for the specified track type.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `track` - The track to read or modify.
    pub fn tracks_update(&self, request: Track, package_name: &str, edit_id: &str, track: &str) -> EditTrackUpdateCall<'a, S> {
        EditTrackUpdateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _track: track.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Commits/applies the changes made in this edit back to the app.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn commit(&self, package_name: &str, edit_id: &str) -> EditCommitCall<'a, S> {
        EditCommitCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an edit for an app. Creating a new edit will automatically delete any of your previous edits so this method need only be called if you want to preemptively abandon an edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn delete(&self, package_name: &str, edit_id: &str) -> EditDeleteCall<'a, S> {
        EditDeleteCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about the edit specified. Calls will fail if the edit is no long active (e.g. has been deleted, superseded or expired).
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn get(&self, package_name: &str, edit_id: &str) -> EditGetCall<'a, S> {
        EditGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new edit for an app, populated with the app's current state.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    pub fn insert(&self, request: AppEdit, package_name: &str) -> EditInsertCall<'a, S> {
        EditInsertCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Checks that the edit can be successfully committed. The edit's changes are not applied to the live app.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn validate(&self, package_name: &str, edit_id: &str) -> EditValidateCall<'a, S> {
        EditValidateCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *inappproduct* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher2 as androidpublisher2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.inappproducts();
/// # }
/// ```
pub struct InappproductMethods<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for InappproductMethods<'a, S> {}

impl<'a, S> InappproductMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete an in-app product for an app.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app with the in-app product; for example, "com.spiffygame".
    /// * `sku` - Unique identifier for the in-app product.
    pub fn delete(&self, package_name: &str, sku: &str) -> InappproductDeleteCall<'a, S> {
        InappproductDeleteCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _sku: sku.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about the in-app product specified.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - No description provided.
    /// * `sku` - Unique identifier for the in-app product.
    pub fn get(&self, package_name: &str, sku: &str) -> InappproductGetCall<'a, S> {
        InappproductGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _sku: sku.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new in-app product for an app.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app; for example, "com.spiffygame".
    pub fn insert(&self, request: InAppProduct, package_name: &str) -> InappproductInsertCall<'a, S> {
        InappproductInsertCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _auto_convert_missing_prices: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all the in-app products for an Android app, both subscriptions and managed in-app products..
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app with in-app products; for example, "com.spiffygame".
    pub fn list(&self, package_name: &str) -> InappproductListCall<'a, S> {
        InappproductListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _token: Default::default(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the details of an in-app product. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app with the in-app product; for example, "com.spiffygame".
    /// * `sku` - Unique identifier for the in-app product.
    pub fn patch(&self, request: InAppProduct, package_name: &str, sku: &str) -> InappproductPatchCall<'a, S> {
        InappproductPatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _sku: sku.to_string(),
            _auto_convert_missing_prices: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the details of an in-app product.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app with the in-app product; for example, "com.spiffygame".
    /// * `sku` - Unique identifier for the in-app product.
    pub fn update(&self, request: InAppProduct, package_name: &str, sku: &str) -> InappproductUpdateCall<'a, S> {
        InappproductUpdateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _sku: sku.to_string(),
            _auto_convert_missing_prices: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *order* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher2 as androidpublisher2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `refund(...)`
/// // to build up your call.
/// let rb = hub.orders();
/// # }
/// ```
pub struct OrderMethods<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for OrderMethods<'a, S> {}

impl<'a, S> OrderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Refund a user's subscription or in-app purchase order.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package name of the application for which this subscription or in-app item was purchased (for example, 'com.some.thing').
    /// * `orderId` - The order ID provided to the user when the subscription or in-app order was purchased.
    pub fn refund(&self, package_name: &str, order_id: &str) -> OrderRefundCall<'a, S> {
        OrderRefundCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _order_id: order_id.to_string(),
            _revoke: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *purchase* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher2 as androidpublisher2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `products_get(...)`, `subscriptions_cancel(...)`, `subscriptions_defer(...)`, `subscriptions_get(...)`, `subscriptions_refund(...)`, `subscriptions_revoke(...)` and `voidedpurchases_list(...)`
/// // to build up your call.
/// let rb = hub.purchases();
/// # }
/// ```
pub struct PurchaseMethods<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for PurchaseMethods<'a, S> {}

impl<'a, S> PurchaseMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Checks the purchase and consumption status of an inapp item.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package name of the application the inapp product was sold in (for example, 'com.some.thing').
    /// * `productId` - The inapp product SKU (for example, 'com.some.thing.inapp1').
    /// * `token` - The token provided to the user's device when the inapp product was purchased.
    pub fn products_get(&self, package_name: &str, product_id: &str, token: &str) -> PurchaseProductGetCall<'a, S> {
        PurchaseProductGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels a user's subscription purchase. The subscription remains valid until its expiration time.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    /// * `subscriptionId` - The purchased subscription ID (for example, 'monthly001').
    /// * `token` - The token provided to the user's device when the subscription was purchased.
    pub fn subscriptions_cancel(&self, package_name: &str, subscription_id: &str, token: &str) -> PurchaseSubscriptionCancelCall<'a, S> {
        PurchaseSubscriptionCancelCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _subscription_id: subscription_id.to_string(),
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Defers a user's subscription purchase until a specified future expiration time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    /// * `subscriptionId` - The purchased subscription ID (for example, 'monthly001').
    /// * `token` - The token provided to the user's device when the subscription was purchased.
    pub fn subscriptions_defer(&self, request: SubscriptionPurchasesDeferRequest, package_name: &str, subscription_id: &str, token: &str) -> PurchaseSubscriptionDeferCall<'a, S> {
        PurchaseSubscriptionDeferCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _subscription_id: subscription_id.to_string(),
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Checks whether a user's subscription purchase is valid and returns its expiry time.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    /// * `subscriptionId` - The purchased subscription ID (for example, 'monthly001').
    /// * `token` - The token provided to the user's device when the subscription was purchased.
    pub fn subscriptions_get(&self, package_name: &str, subscription_id: &str, token: &str) -> PurchaseSubscriptionGetCall<'a, S> {
        PurchaseSubscriptionGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _subscription_id: subscription_id.to_string(),
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Refunds a user's subscription purchase, but the subscription remains valid until its expiration time and it will continue to recur.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    /// * `subscriptionId` - The purchased subscription ID (for example, 'monthly001').
    /// * `token` - The token provided to the user's device when the subscription was purchased.
    pub fn subscriptions_refund(&self, package_name: &str, subscription_id: &str, token: &str) -> PurchaseSubscriptionRefundCall<'a, S> {
        PurchaseSubscriptionRefundCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _subscription_id: subscription_id.to_string(),
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Refunds and immediately revokes a user's subscription purchase. Access to the subscription will be terminated immediately and it will stop recurring.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    /// * `subscriptionId` - The purchased subscription ID (for example, 'monthly001').
    /// * `token` - The token provided to the user's device when the subscription was purchased.
    pub fn subscriptions_revoke(&self, package_name: &str, subscription_id: &str, token: &str) -> PurchaseSubscriptionRevokeCall<'a, S> {
        PurchaseSubscriptionRevokeCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _subscription_id: subscription_id.to_string(),
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the purchases that were canceled, refunded or charged-back.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package name of the application for which voided purchases need to be returned (for example, 'com.some.thing').
    pub fn voidedpurchases_list(&self, package_name: &str) -> PurchaseVoidedpurchaseListCall<'a, S> {
        PurchaseVoidedpurchaseListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _token: Default::default(),
            _start_time: Default::default(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _end_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *review* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher2 as androidpublisher2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)` and `reply(...)`
/// // to build up your call.
/// let rb = hub.reviews();
/// # }
/// ```
pub struct ReviewMethods<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for ReviewMethods<'a, S> {}

impl<'a, S> ReviewMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a single review.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app for which we want reviews; for example, "com.spiffygame".
    /// * `reviewId` - No description provided.
    pub fn get(&self, package_name: &str, review_id: &str) -> ReviewGetCall<'a, S> {
        ReviewGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _review_id: review_id.to_string(),
            _translation_language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of reviews. Only reviews from last week will be returned.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app for which we want reviews; for example, "com.spiffygame".
    pub fn list(&self, package_name: &str) -> ReviewListCall<'a, S> {
        ReviewListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _translation_language: Default::default(),
            _token: Default::default(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reply to a single review, or update an existing reply.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app for which we want reviews; for example, "com.spiffygame".
    /// * `reviewId` - No description provided.
    pub fn reply(&self, request: ReviewsReplyRequest, package_name: &str, review_id: &str) -> ReviewReplyCall<'a, S> {
        ReviewReplyCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _review_id: review_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Deletes the APK-specific localized listing for a specified APK and language code.
///
/// A builder for the *apklistings.delete* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().apklistings_delete("packageName", "editId", -62, "language")
///              .doit().await;
/// # }
/// ```
pub struct EditApklistingDeleteCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _apk_version_code: i32,
    _language: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditApklistingDeleteCall<'a, S> {}

impl<'a, S> EditApklistingDeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.apklistings.delete",
                               http_method: hyper::Method::DELETE });

        for &field in ["packageName", "editId", "apkVersionCode", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("apkVersionCode", self._apk_version_code.to_string());
        params.push("language", self._language);

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/apks/{apkVersionCode}/listings/{language}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{apkVersionCode}", "apkVersionCode"), ("{language}", "language")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["language", "apkVersionCode", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditApklistingDeleteCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditApklistingDeleteCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The APK version code whose APK-specific listings should be read or modified.
    ///
    /// Sets the *apk version code* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn apk_version_code(mut self, new_value: i32) -> EditApklistingDeleteCall<'a, S> {
        self._apk_version_code = new_value;
        self
    }
    /// The language code (a BCP-47 language tag) of the APK-specific localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    ///
    /// Sets the *language* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn language(mut self, new_value: &str) -> EditApklistingDeleteCall<'a, S> {
        self._language = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditApklistingDeleteCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditApklistingDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditApklistingDeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditApklistingDeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditApklistingDeleteCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Deletes all the APK-specific localized listings for a specified APK.
///
/// A builder for the *apklistings.deleteall* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().apklistings_deleteall("packageName", "editId", -4)
///              .doit().await;
/// # }
/// ```
pub struct EditApklistingDeleteallCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _apk_version_code: i32,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditApklistingDeleteallCall<'a, S> {}

impl<'a, S> EditApklistingDeleteallCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.apklistings.deleteall",
                               http_method: hyper::Method::DELETE });

        for &field in ["packageName", "editId", "apkVersionCode"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("apkVersionCode", self._apk_version_code.to_string());

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/apks/{apkVersionCode}/listings";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{apkVersionCode}", "apkVersionCode")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["apkVersionCode", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditApklistingDeleteallCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditApklistingDeleteallCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The APK version code whose APK-specific listings should be read or modified.
    ///
    /// Sets the *apk version code* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn apk_version_code(mut self, new_value: i32) -> EditApklistingDeleteallCall<'a, S> {
        self._apk_version_code = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditApklistingDeleteallCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditApklistingDeleteallCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditApklistingDeleteallCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditApklistingDeleteallCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditApklistingDeleteallCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Fetches the APK-specific localized listing for a specified APK and language code.
///
/// A builder for the *apklistings.get* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().apklistings_get("packageName", "editId", -88, "language")
///              .doit().await;
/// # }
/// ```
pub struct EditApklistingGetCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _apk_version_code: i32,
    _language: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditApklistingGetCall<'a, S> {}

impl<'a, S> EditApklistingGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ApkListing)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.apklistings.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "editId", "apkVersionCode", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("apkVersionCode", self._apk_version_code.to_string());
        params.push("language", self._language);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/apks/{apkVersionCode}/listings/{language}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{apkVersionCode}", "apkVersionCode"), ("{language}", "language")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["language", "apkVersionCode", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditApklistingGetCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditApklistingGetCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The APK version code whose APK-specific listings should be read or modified.
    ///
    /// Sets the *apk version code* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn apk_version_code(mut self, new_value: i32) -> EditApklistingGetCall<'a, S> {
        self._apk_version_code = new_value;
        self
    }
    /// The language code (a BCP-47 language tag) of the APK-specific localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    ///
    /// Sets the *language* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn language(mut self, new_value: &str) -> EditApklistingGetCall<'a, S> {
        self._language = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditApklistingGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditApklistingGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditApklistingGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditApklistingGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditApklistingGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Lists all the APK-specific localized listings for a specified APK.
///
/// A builder for the *apklistings.list* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().apklistings_list("packageName", "editId", -93)
///              .doit().await;
/// # }
/// ```
pub struct EditApklistingListCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _apk_version_code: i32,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditApklistingListCall<'a, S> {}

impl<'a, S> EditApklistingListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ApkListingsListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.apklistings.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "editId", "apkVersionCode"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("apkVersionCode", self._apk_version_code.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/apks/{apkVersionCode}/listings";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{apkVersionCode}", "apkVersionCode")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["apkVersionCode", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditApklistingListCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditApklistingListCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The APK version code whose APK-specific listings should be read or modified.
    ///
    /// Sets the *apk version code* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn apk_version_code(mut self, new_value: i32) -> EditApklistingListCall<'a, S> {
        self._apk_version_code = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditApklistingListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditApklistingListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditApklistingListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditApklistingListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditApklistingListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates or creates the APK-specific localized listing for a specified APK and language code. This method supports patch semantics.
///
/// A builder for the *apklistings.patch* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::ApkListing;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ApkListing::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().apklistings_patch(req, "packageName", "editId", -16, "language")
///              .doit().await;
/// # }
/// ```
pub struct EditApklistingPatchCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: ApkListing,
    _package_name: String,
    _edit_id: String,
    _apk_version_code: i32,
    _language: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditApklistingPatchCall<'a, S> {}

impl<'a, S> EditApklistingPatchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ApkListing)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.apklistings.patch",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "packageName", "editId", "apkVersionCode", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("apkVersionCode", self._apk_version_code.to_string());
        params.push("language", self._language);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/apks/{apkVersionCode}/listings/{language}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{apkVersionCode}", "apkVersionCode"), ("{language}", "language")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["language", "apkVersionCode", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ApkListing) -> EditApklistingPatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditApklistingPatchCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditApklistingPatchCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The APK version code whose APK-specific listings should be read or modified.
    ///
    /// Sets the *apk version code* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn apk_version_code(mut self, new_value: i32) -> EditApklistingPatchCall<'a, S> {
        self._apk_version_code = new_value;
        self
    }
    /// The language code (a BCP-47 language tag) of the APK-specific localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    ///
    /// Sets the *language* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn language(mut self, new_value: &str) -> EditApklistingPatchCall<'a, S> {
        self._language = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditApklistingPatchCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditApklistingPatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditApklistingPatchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditApklistingPatchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditApklistingPatchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates or creates the APK-specific localized listing for a specified APK and language code.
///
/// A builder for the *apklistings.update* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::ApkListing;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ApkListing::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().apklistings_update(req, "packageName", "editId", -7, "language")
///              .doit().await;
/// # }
/// ```
pub struct EditApklistingUpdateCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: ApkListing,
    _package_name: String,
    _edit_id: String,
    _apk_version_code: i32,
    _language: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditApklistingUpdateCall<'a, S> {}

impl<'a, S> EditApklistingUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ApkListing)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.apklistings.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "packageName", "editId", "apkVersionCode", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("apkVersionCode", self._apk_version_code.to_string());
        params.push("language", self._language);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/apks/{apkVersionCode}/listings/{language}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{apkVersionCode}", "apkVersionCode"), ("{language}", "language")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["language", "apkVersionCode", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ApkListing) -> EditApklistingUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditApklistingUpdateCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditApklistingUpdateCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The APK version code whose APK-specific listings should be read or modified.
    ///
    /// Sets the *apk version code* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn apk_version_code(mut self, new_value: i32) -> EditApklistingUpdateCall<'a, S> {
        self._apk_version_code = new_value;
        self
    }
    /// The language code (a BCP-47 language tag) of the APK-specific localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    ///
    /// Sets the *language* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn language(mut self, new_value: &str) -> EditApklistingUpdateCall<'a, S> {
        self._language = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditApklistingUpdateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditApklistingUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditApklistingUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditApklistingUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditApklistingUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Creates a new APK without uploading the APK itself to Google Play, instead hosting the APK at a specified URL. This function is only available to enterprises using Google Play for Work whose application is configured to restrict distribution to the enterprise domain.
///
/// A builder for the *apks.addexternallyhosted* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::ApksAddExternallyHostedRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ApksAddExternallyHostedRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().apks_addexternallyhosted(req, "packageName", "editId")
///              .doit().await;
/// # }
/// ```
pub struct EditApkAddexternallyhostedCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: ApksAddExternallyHostedRequest,
    _package_name: String,
    _edit_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditApkAddexternallyhostedCall<'a, S> {}

impl<'a, S> EditApkAddexternallyhostedCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ApksAddExternallyHostedResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.apks.addexternallyhosted",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "packageName", "editId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/apks/externallyHosted";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ApksAddExternallyHostedRequest) -> EditApkAddexternallyhostedCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditApkAddexternallyhostedCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditApkAddexternallyhostedCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditApkAddexternallyhostedCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditApkAddexternallyhostedCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditApkAddexternallyhostedCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditApkAddexternallyhostedCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditApkAddexternallyhostedCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// A builder for the *apks.list* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().apks_list("packageName", "editId")
///              .doit().await;
/// # }
/// ```
pub struct EditApkListCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditApkListCall<'a, S> {}

impl<'a, S> EditApkListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ApksListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.apks.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "editId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/apks";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditApkListCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditApkListCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditApkListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditApkListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditApkListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditApkListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditApkListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// A builder for the *apks.upload* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use std::fs;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().apks_upload("packageName", "editId")
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;
/// # }
/// ```
pub struct EditApkUploadCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditApkUploadCall<'a, S> {}

impl<'a, S> EditApkUploadCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    async fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: client::UploadProtocol) -> client::Result<(hyper::Response<hyper::body::Body>, Apk)>
		where RS: client::ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.apks.upload",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "packageName", "editId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let (mut url, upload_type) =
            if protocol == client::UploadProtocol::Resumable {
                (self.hub._root_url.clone() + "resumable/upload/androidpublisher/v2/applications/{packageName}/edits/{editId}/apks", "resumable")
            } else if protocol == client::UploadProtocol::Simple {
                (self.hub._root_url.clone() + "upload/androidpublisher/v2/applications/{packageName}/edits/{editId}/apks", "multipart")
            } else {
                unreachable!()
            };
        params.push("uploadType", upload_type);
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);


        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    Ok(hyper::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Location", upload_url.as_ref().unwrap().clone())
                        .body(hyper::body::Body::empty())
                        .unwrap())
                } else {
                    let client = &self.hub.client;
                    dlg.pre_request();
                    let mut req_builder = hyper::Request::builder()
                        .method(hyper::Method::POST)
                        .uri(url.as_str())
                        .header(USER_AGENT, self.hub._user_agent.clone());
    
                    if let Some(token) = token.as_ref() {
                        req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                    }
    
                    upload_url_from_server = true;
                    if protocol == client::UploadProtocol::Resumable {
                        req_builder = req_builder.header("X-Upload-Content-Type", format!("{}", reader_mime_type));
                    }
    
                            let request = if protocol == client::UploadProtocol::Simple {
                                let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                    reader.seek(io::SeekFrom::Start(0)).unwrap();
                    if size > 1073741824 {
                    	return Err(client::Error::UploadSizeLimitExceeded(size, 1073741824))
                    }
                                let mut bytes = Vec::with_capacity(size as usize);
                                reader.read_to_end(&mut bytes)?;
                                req_builder.header(CONTENT_TYPE, reader_mime_type.to_string())
                                         .header(CONTENT_LENGTH, size)
                                         .body(hyper::body::Body::from(bytes))
                            } else {
                                req_builder.body(hyper::body::Body::from(Vec::new()))
                            };
    
                    client.request(request.unwrap()).await
    
                }
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    if protocol == client::UploadProtocol::Resumable {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 1073741824 {
                        	return Err(client::Error::UploadSizeLimitExceeded(size, 1073741824))
                        }
                        let upload_result = {
                            let url_str = &res.headers().get("Location").expect("LOCATION header is part of protocol").to_str().unwrap();
                            if upload_url_from_server {
                                dlg.store_upload_url(Some(url_str));
                            }

                            client::ResumableUploadHelper {
                                client: &self.hub.client,
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &self.hub.auth,
                                user_agent: &self.hub._user_agent,
                                // TODO: Check this assumption
                                auth_header: format!("Bearer {}", token.ok_or_else(|| client::Error::MissingToken("resumable upload requires token".into()))?.as_str()),
                                url: url_str,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload().await
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(client::Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(client::Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status().is_success() {
                                    dlg.store_upload_url(None);
                                    dlg.finished(false);
                                    return Err(client::Error::Failure(res))
                                }
                            }
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }

    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *multipart*: yes
    /// * *max size*: 1GB
    /// * *valid mime types*: 'application/octet-stream' and 'application/vnd.android.package-archive'
    pub async fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, Apk)>
                where RS: client::ReadSeek {
        self.doit(resumeable_stream, mime_type, client::UploadProtocol::Resumable).await
    }
    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *multipart*: yes
    /// * *max size*: 1GB
    /// * *valid mime types*: 'application/octet-stream' and 'application/vnd.android.package-archive'
    pub async fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, Apk)>
                where RS: client::ReadSeek {
        self.doit(stream, mime_type, client::UploadProtocol::Simple).await
    }

    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditApkUploadCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditApkUploadCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditApkUploadCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditApkUploadCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditApkUploadCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditApkUploadCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditApkUploadCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// A builder for the *bundles.list* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().bundles_list("packageName", "editId")
///              .doit().await;
/// # }
/// ```
pub struct EditBundleListCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditBundleListCall<'a, S> {}

impl<'a, S> EditBundleListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BundlesListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.bundles.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "editId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/bundles";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditBundleListCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditBundleListCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditBundleListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditBundleListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditBundleListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditBundleListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditBundleListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Uploads a new Android App Bundle to this edit. If you are using the Google API client libraries, please increase the timeout of the http request before calling this endpoint (a timeout of 2 minutes is recommended). See: https://developers.google.com/api-client-library/java/google-api-java-client/errors for an example in java.
///
/// A builder for the *bundles.upload* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use std::fs;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().bundles_upload("packageName", "editId")
///              .ack_bundle_installation_warning(true)
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;
/// # }
/// ```
pub struct EditBundleUploadCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _ack_bundle_installation_warning: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditBundleUploadCall<'a, S> {}

impl<'a, S> EditBundleUploadCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    async fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: client::UploadProtocol) -> client::Result<(hyper::Response<hyper::body::Body>, Bundle)>
		where RS: client::ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.bundles.upload",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "packageName", "editId", "ackBundleInstallationWarning"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        if let Some(value) = self._ack_bundle_installation_warning.as_ref() {
            params.push("ackBundleInstallationWarning", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let (mut url, upload_type) =
            if protocol == client::UploadProtocol::Resumable {
                (self.hub._root_url.clone() + "resumable/upload/androidpublisher/v2/applications/{packageName}/edits/{editId}/bundles", "resumable")
            } else if protocol == client::UploadProtocol::Simple {
                (self.hub._root_url.clone() + "upload/androidpublisher/v2/applications/{packageName}/edits/{editId}/bundles", "multipart")
            } else {
                unreachable!()
            };
        params.push("uploadType", upload_type);
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);


        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    Ok(hyper::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Location", upload_url.as_ref().unwrap().clone())
                        .body(hyper::body::Body::empty())
                        .unwrap())
                } else {
                    let client = &self.hub.client;
                    dlg.pre_request();
                    let mut req_builder = hyper::Request::builder()
                        .method(hyper::Method::POST)
                        .uri(url.as_str())
                        .header(USER_AGENT, self.hub._user_agent.clone());
    
                    if let Some(token) = token.as_ref() {
                        req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                    }
    
                    upload_url_from_server = true;
                    if protocol == client::UploadProtocol::Resumable {
                        req_builder = req_builder.header("X-Upload-Content-Type", format!("{}", reader_mime_type));
                    }
    
                            let request = if protocol == client::UploadProtocol::Simple {
                                let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                    reader.seek(io::SeekFrom::Start(0)).unwrap();
                    if size > 2147483648 {
                    	return Err(client::Error::UploadSizeLimitExceeded(size, 2147483648))
                    }
                                let mut bytes = Vec::with_capacity(size as usize);
                                reader.read_to_end(&mut bytes)?;
                                req_builder.header(CONTENT_TYPE, reader_mime_type.to_string())
                                         .header(CONTENT_LENGTH, size)
                                         .body(hyper::body::Body::from(bytes))
                            } else {
                                req_builder.body(hyper::body::Body::from(Vec::new()))
                            };
    
                    client.request(request.unwrap()).await
    
                }
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    if protocol == client::UploadProtocol::Resumable {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 2147483648 {
                        	return Err(client::Error::UploadSizeLimitExceeded(size, 2147483648))
                        }
                        let upload_result = {
                            let url_str = &res.headers().get("Location").expect("LOCATION header is part of protocol").to_str().unwrap();
                            if upload_url_from_server {
                                dlg.store_upload_url(Some(url_str));
                            }

                            client::ResumableUploadHelper {
                                client: &self.hub.client,
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &self.hub.auth,
                                user_agent: &self.hub._user_agent,
                                // TODO: Check this assumption
                                auth_header: format!("Bearer {}", token.ok_or_else(|| client::Error::MissingToken("resumable upload requires token".into()))?.as_str()),
                                url: url_str,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload().await
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(client::Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(client::Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status().is_success() {
                                    dlg.store_upload_url(None);
                                    dlg.finished(false);
                                    return Err(client::Error::Failure(res))
                                }
                            }
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }

    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *multipart*: yes
    /// * *max size*: 2GB
    /// * *valid mime types*: 'application/octet-stream'
    pub async fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, Bundle)>
                where RS: client::ReadSeek {
        self.doit(resumeable_stream, mime_type, client::UploadProtocol::Resumable).await
    }
    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *multipart*: yes
    /// * *max size*: 2GB
    /// * *valid mime types*: 'application/octet-stream'
    pub async fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, Bundle)>
                where RS: client::ReadSeek {
        self.doit(stream, mime_type, client::UploadProtocol::Simple).await
    }

    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditBundleUploadCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditBundleUploadCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// Must be set to true if the bundle installation may trigger a warning on user devices (for example, if installation size may be over a threshold, typically 100 MB).
    ///
    /// Sets the *ack bundle installation warning* query property to the given value.
    pub fn ack_bundle_installation_warning(mut self, new_value: bool) -> EditBundleUploadCall<'a, S> {
        self._ack_bundle_installation_warning = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditBundleUploadCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditBundleUploadCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditBundleUploadCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditBundleUploadCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditBundleUploadCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Uploads the deobfuscation file of the specified APK. If a deobfuscation file already exists, it will be replaced.
///
/// A builder for the *deobfuscationfiles.upload* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use std::fs;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().deobfuscationfiles_upload("packageName", "editId", -76, "deobfuscationFileType")
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;
/// # }
/// ```
pub struct EditDeobfuscationfileUploadCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _apk_version_code: i32,
    _deobfuscation_file_type: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditDeobfuscationfileUploadCall<'a, S> {}

impl<'a, S> EditDeobfuscationfileUploadCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    async fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: client::UploadProtocol) -> client::Result<(hyper::Response<hyper::body::Body>, DeobfuscationFilesUploadResponse)>
		where RS: client::ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.deobfuscationfiles.upload",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "packageName", "editId", "apkVersionCode", "deobfuscationFileType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("apkVersionCode", self._apk_version_code.to_string());
        params.push("deobfuscationFileType", self._deobfuscation_file_type);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let (mut url, upload_type) =
            if protocol == client::UploadProtocol::Resumable {
                (self.hub._root_url.clone() + "resumable/upload/androidpublisher/v2/applications/{packageName}/edits/{editId}/apks/{apkVersionCode}/deobfuscationFiles/{deobfuscationFileType}", "resumable")
            } else if protocol == client::UploadProtocol::Simple {
                (self.hub._root_url.clone() + "upload/androidpublisher/v2/applications/{packageName}/edits/{editId}/apks/{apkVersionCode}/deobfuscationFiles/{deobfuscationFileType}", "multipart")
            } else {
                unreachable!()
            };
        params.push("uploadType", upload_type);
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{apkVersionCode}", "apkVersionCode"), ("{deobfuscationFileType}", "deobfuscationFileType")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["deobfuscationFileType", "apkVersionCode", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);


        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    Ok(hyper::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Location", upload_url.as_ref().unwrap().clone())
                        .body(hyper::body::Body::empty())
                        .unwrap())
                } else {
                    let client = &self.hub.client;
                    dlg.pre_request();
                    let mut req_builder = hyper::Request::builder()
                        .method(hyper::Method::POST)
                        .uri(url.as_str())
                        .header(USER_AGENT, self.hub._user_agent.clone());
    
                    if let Some(token) = token.as_ref() {
                        req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                    }
    
                    upload_url_from_server = true;
                    if protocol == client::UploadProtocol::Resumable {
                        req_builder = req_builder.header("X-Upload-Content-Type", format!("{}", reader_mime_type));
                    }
    
                            let request = if protocol == client::UploadProtocol::Simple {
                                let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                    reader.seek(io::SeekFrom::Start(0)).unwrap();
                    if size > 314572800 {
                    	return Err(client::Error::UploadSizeLimitExceeded(size, 314572800))
                    }
                                let mut bytes = Vec::with_capacity(size as usize);
                                reader.read_to_end(&mut bytes)?;
                                req_builder.header(CONTENT_TYPE, reader_mime_type.to_string())
                                         .header(CONTENT_LENGTH, size)
                                         .body(hyper::body::Body::from(bytes))
                            } else {
                                req_builder.body(hyper::body::Body::from(Vec::new()))
                            };
    
                    client.request(request.unwrap()).await
    
                }
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    if protocol == client::UploadProtocol::Resumable {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 314572800 {
                        	return Err(client::Error::UploadSizeLimitExceeded(size, 314572800))
                        }
                        let upload_result = {
                            let url_str = &res.headers().get("Location").expect("LOCATION header is part of protocol").to_str().unwrap();
                            if upload_url_from_server {
                                dlg.store_upload_url(Some(url_str));
                            }

                            client::ResumableUploadHelper {
                                client: &self.hub.client,
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &self.hub.auth,
                                user_agent: &self.hub._user_agent,
                                // TODO: Check this assumption
                                auth_header: format!("Bearer {}", token.ok_or_else(|| client::Error::MissingToken("resumable upload requires token".into()))?.as_str()),
                                url: url_str,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload().await
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(client::Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(client::Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status().is_success() {
                                    dlg.store_upload_url(None);
                                    dlg.finished(false);
                                    return Err(client::Error::Failure(res))
                                }
                            }
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }

    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *multipart*: yes
    /// * *max size*: 300MB
    /// * *valid mime types*: 'application/octet-stream'
    pub async fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, DeobfuscationFilesUploadResponse)>
                where RS: client::ReadSeek {
        self.doit(resumeable_stream, mime_type, client::UploadProtocol::Resumable).await
    }
    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *multipart*: yes
    /// * *max size*: 300MB
    /// * *valid mime types*: 'application/octet-stream'
    pub async fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, DeobfuscationFilesUploadResponse)>
                where RS: client::ReadSeek {
        self.doit(stream, mime_type, client::UploadProtocol::Simple).await
    }

    /// Unique identifier of the Android app for which the deobfuscatiuon files are being uploaded; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditDeobfuscationfileUploadCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditDeobfuscationfileUploadCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The version code of the APK whose deobfuscation file is being uploaded.
    ///
    /// Sets the *apk version code* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn apk_version_code(mut self, new_value: i32) -> EditDeobfuscationfileUploadCall<'a, S> {
        self._apk_version_code = new_value;
        self
    }
    ///
    /// Sets the *deobfuscation file type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn deobfuscation_file_type(mut self, new_value: &str) -> EditDeobfuscationfileUploadCall<'a, S> {
        self._deobfuscation_file_type = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditDeobfuscationfileUploadCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditDeobfuscationfileUploadCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditDeobfuscationfileUploadCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditDeobfuscationfileUploadCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditDeobfuscationfileUploadCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Fetches app details for this edit. This includes the default language and developer support contact information.
///
/// A builder for the *details.get* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().details_get("packageName", "editId")
///              .doit().await;
/// # }
/// ```
pub struct EditDetailGetCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditDetailGetCall<'a, S> {}

impl<'a, S> EditDetailGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, AppDetails)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.details.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "editId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/details";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditDetailGetCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditDetailGetCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditDetailGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditDetailGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditDetailGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditDetailGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditDetailGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates app details for this edit. This method supports patch semantics.
///
/// A builder for the *details.patch* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::AppDetails;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = AppDetails::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().details_patch(req, "packageName", "editId")
///              .doit().await;
/// # }
/// ```
pub struct EditDetailPatchCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: AppDetails,
    _package_name: String,
    _edit_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditDetailPatchCall<'a, S> {}

impl<'a, S> EditDetailPatchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, AppDetails)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.details.patch",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "packageName", "editId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/details";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: AppDetails) -> EditDetailPatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditDetailPatchCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditDetailPatchCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditDetailPatchCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditDetailPatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditDetailPatchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditDetailPatchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditDetailPatchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates app details for this edit.
///
/// A builder for the *details.update* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::AppDetails;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = AppDetails::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().details_update(req, "packageName", "editId")
///              .doit().await;
/// # }
/// ```
pub struct EditDetailUpdateCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: AppDetails,
    _package_name: String,
    _edit_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditDetailUpdateCall<'a, S> {}

impl<'a, S> EditDetailUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, AppDetails)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.details.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "packageName", "editId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/details";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: AppDetails) -> EditDetailUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditDetailUpdateCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditDetailUpdateCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditDetailUpdateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditDetailUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditDetailUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditDetailUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditDetailUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Fetches the Expansion File configuration for the APK specified.
///
/// A builder for the *expansionfiles.get* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().expansionfiles_get("packageName", "editId", -49, "expansionFileType")
///              .doit().await;
/// # }
/// ```
pub struct EditExpansionfileGetCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _apk_version_code: i32,
    _expansion_file_type: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditExpansionfileGetCall<'a, S> {}

impl<'a, S> EditExpansionfileGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ExpansionFile)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.expansionfiles.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "editId", "apkVersionCode", "expansionFileType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("apkVersionCode", self._apk_version_code.to_string());
        params.push("expansionFileType", self._expansion_file_type);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/apks/{apkVersionCode}/expansionFiles/{expansionFileType}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{apkVersionCode}", "apkVersionCode"), ("{expansionFileType}", "expansionFileType")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["expansionFileType", "apkVersionCode", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditExpansionfileGetCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditExpansionfileGetCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The version code of the APK whose Expansion File configuration is being read or modified.
    ///
    /// Sets the *apk version code* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn apk_version_code(mut self, new_value: i32) -> EditExpansionfileGetCall<'a, S> {
        self._apk_version_code = new_value;
        self
    }
    ///
    /// Sets the *expansion file type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn expansion_file_type(mut self, new_value: &str) -> EditExpansionfileGetCall<'a, S> {
        self._expansion_file_type = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditExpansionfileGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditExpansionfileGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditExpansionfileGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditExpansionfileGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditExpansionfileGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates the APK's Expansion File configuration to reference another APK's Expansion Files. To add a new Expansion File use the Upload method. This method supports patch semantics.
///
/// A builder for the *expansionfiles.patch* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::ExpansionFile;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ExpansionFile::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().expansionfiles_patch(req, "packageName", "editId", -15, "expansionFileType")
///              .doit().await;
/// # }
/// ```
pub struct EditExpansionfilePatchCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: ExpansionFile,
    _package_name: String,
    _edit_id: String,
    _apk_version_code: i32,
    _expansion_file_type: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditExpansionfilePatchCall<'a, S> {}

impl<'a, S> EditExpansionfilePatchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ExpansionFile)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.expansionfiles.patch",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "packageName", "editId", "apkVersionCode", "expansionFileType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("apkVersionCode", self._apk_version_code.to_string());
        params.push("expansionFileType", self._expansion_file_type);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/apks/{apkVersionCode}/expansionFiles/{expansionFileType}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{apkVersionCode}", "apkVersionCode"), ("{expansionFileType}", "expansionFileType")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["expansionFileType", "apkVersionCode", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ExpansionFile) -> EditExpansionfilePatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditExpansionfilePatchCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditExpansionfilePatchCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The version code of the APK whose Expansion File configuration is being read or modified.
    ///
    /// Sets the *apk version code* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn apk_version_code(mut self, new_value: i32) -> EditExpansionfilePatchCall<'a, S> {
        self._apk_version_code = new_value;
        self
    }
    ///
    /// Sets the *expansion file type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn expansion_file_type(mut self, new_value: &str) -> EditExpansionfilePatchCall<'a, S> {
        self._expansion_file_type = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditExpansionfilePatchCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditExpansionfilePatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditExpansionfilePatchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditExpansionfilePatchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditExpansionfilePatchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates the APK's Expansion File configuration to reference another APK's Expansion Files. To add a new Expansion File use the Upload method.
///
/// A builder for the *expansionfiles.update* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::ExpansionFile;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ExpansionFile::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().expansionfiles_update(req, "packageName", "editId", -76, "expansionFileType")
///              .doit().await;
/// # }
/// ```
pub struct EditExpansionfileUpdateCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: ExpansionFile,
    _package_name: String,
    _edit_id: String,
    _apk_version_code: i32,
    _expansion_file_type: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditExpansionfileUpdateCall<'a, S> {}

impl<'a, S> EditExpansionfileUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ExpansionFile)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.expansionfiles.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "packageName", "editId", "apkVersionCode", "expansionFileType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("apkVersionCode", self._apk_version_code.to_string());
        params.push("expansionFileType", self._expansion_file_type);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/apks/{apkVersionCode}/expansionFiles/{expansionFileType}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{apkVersionCode}", "apkVersionCode"), ("{expansionFileType}", "expansionFileType")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["expansionFileType", "apkVersionCode", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ExpansionFile) -> EditExpansionfileUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditExpansionfileUpdateCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditExpansionfileUpdateCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The version code of the APK whose Expansion File configuration is being read or modified.
    ///
    /// Sets the *apk version code* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn apk_version_code(mut self, new_value: i32) -> EditExpansionfileUpdateCall<'a, S> {
        self._apk_version_code = new_value;
        self
    }
    ///
    /// Sets the *expansion file type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn expansion_file_type(mut self, new_value: &str) -> EditExpansionfileUpdateCall<'a, S> {
        self._expansion_file_type = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditExpansionfileUpdateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditExpansionfileUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditExpansionfileUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditExpansionfileUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditExpansionfileUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Uploads and attaches a new Expansion File to the APK specified.
///
/// A builder for the *expansionfiles.upload* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use std::fs;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().expansionfiles_upload("packageName", "editId", -44, "expansionFileType")
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;
/// # }
/// ```
pub struct EditExpansionfileUploadCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _apk_version_code: i32,
    _expansion_file_type: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditExpansionfileUploadCall<'a, S> {}

impl<'a, S> EditExpansionfileUploadCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    async fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: client::UploadProtocol) -> client::Result<(hyper::Response<hyper::body::Body>, ExpansionFilesUploadResponse)>
		where RS: client::ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.expansionfiles.upload",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "packageName", "editId", "apkVersionCode", "expansionFileType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("apkVersionCode", self._apk_version_code.to_string());
        params.push("expansionFileType", self._expansion_file_type);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let (mut url, upload_type) =
            if protocol == client::UploadProtocol::Resumable {
                (self.hub._root_url.clone() + "resumable/upload/androidpublisher/v2/applications/{packageName}/edits/{editId}/apks/{apkVersionCode}/expansionFiles/{expansionFileType}", "resumable")
            } else if protocol == client::UploadProtocol::Simple {
                (self.hub._root_url.clone() + "upload/androidpublisher/v2/applications/{packageName}/edits/{editId}/apks/{apkVersionCode}/expansionFiles/{expansionFileType}", "multipart")
            } else {
                unreachable!()
            };
        params.push("uploadType", upload_type);
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{apkVersionCode}", "apkVersionCode"), ("{expansionFileType}", "expansionFileType")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["expansionFileType", "apkVersionCode", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);


        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    Ok(hyper::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Location", upload_url.as_ref().unwrap().clone())
                        .body(hyper::body::Body::empty())
                        .unwrap())
                } else {
                    let client = &self.hub.client;
                    dlg.pre_request();
                    let mut req_builder = hyper::Request::builder()
                        .method(hyper::Method::POST)
                        .uri(url.as_str())
                        .header(USER_AGENT, self.hub._user_agent.clone());
    
                    if let Some(token) = token.as_ref() {
                        req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                    }
    
                    upload_url_from_server = true;
                    if protocol == client::UploadProtocol::Resumable {
                        req_builder = req_builder.header("X-Upload-Content-Type", format!("{}", reader_mime_type));
                    }
    
                            let request = if protocol == client::UploadProtocol::Simple {
                                let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                    reader.seek(io::SeekFrom::Start(0)).unwrap();
                    if size > 2147483648 {
                    	return Err(client::Error::UploadSizeLimitExceeded(size, 2147483648))
                    }
                                let mut bytes = Vec::with_capacity(size as usize);
                                reader.read_to_end(&mut bytes)?;
                                req_builder.header(CONTENT_TYPE, reader_mime_type.to_string())
                                         .header(CONTENT_LENGTH, size)
                                         .body(hyper::body::Body::from(bytes))
                            } else {
                                req_builder.body(hyper::body::Body::from(Vec::new()))
                            };
    
                    client.request(request.unwrap()).await
    
                }
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    if protocol == client::UploadProtocol::Resumable {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 2147483648 {
                        	return Err(client::Error::UploadSizeLimitExceeded(size, 2147483648))
                        }
                        let upload_result = {
                            let url_str = &res.headers().get("Location").expect("LOCATION header is part of protocol").to_str().unwrap();
                            if upload_url_from_server {
                                dlg.store_upload_url(Some(url_str));
                            }

                            client::ResumableUploadHelper {
                                client: &self.hub.client,
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &self.hub.auth,
                                user_agent: &self.hub._user_agent,
                                // TODO: Check this assumption
                                auth_header: format!("Bearer {}", token.ok_or_else(|| client::Error::MissingToken("resumable upload requires token".into()))?.as_str()),
                                url: url_str,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload().await
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(client::Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(client::Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status().is_success() {
                                    dlg.store_upload_url(None);
                                    dlg.finished(false);
                                    return Err(client::Error::Failure(res))
                                }
                            }
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }

    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *multipart*: yes
    /// * *max size*: 2048MB
    /// * *valid mime types*: 'application/octet-stream'
    pub async fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, ExpansionFilesUploadResponse)>
                where RS: client::ReadSeek {
        self.doit(resumeable_stream, mime_type, client::UploadProtocol::Resumable).await
    }
    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *multipart*: yes
    /// * *max size*: 2048MB
    /// * *valid mime types*: 'application/octet-stream'
    pub async fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, ExpansionFilesUploadResponse)>
                where RS: client::ReadSeek {
        self.doit(stream, mime_type, client::UploadProtocol::Simple).await
    }

    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditExpansionfileUploadCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditExpansionfileUploadCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The version code of the APK whose Expansion File configuration is being read or modified.
    ///
    /// Sets the *apk version code* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn apk_version_code(mut self, new_value: i32) -> EditExpansionfileUploadCall<'a, S> {
        self._apk_version_code = new_value;
        self
    }
    ///
    /// Sets the *expansion file type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn expansion_file_type(mut self, new_value: &str) -> EditExpansionfileUploadCall<'a, S> {
        self._expansion_file_type = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditExpansionfileUploadCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditExpansionfileUploadCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditExpansionfileUploadCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditExpansionfileUploadCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditExpansionfileUploadCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Deletes the image (specified by id) from the edit.
///
/// A builder for the *images.delete* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().images_delete("packageName", "editId", "language", "imageType", "imageId")
///              .doit().await;
/// # }
/// ```
pub struct EditImageDeleteCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _language: String,
    _image_type: String,
    _image_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditImageDeleteCall<'a, S> {}

impl<'a, S> EditImageDeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.images.delete",
                               http_method: hyper::Method::DELETE });

        for &field in ["packageName", "editId", "language", "imageType", "imageId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("language", self._language);
        params.push("imageType", self._image_type);
        params.push("imageId", self._image_id);

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/listings/{language}/{imageType}/{imageId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{language}", "language"), ("{imageType}", "imageType"), ("{imageId}", "imageId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["imageId", "imageType", "language", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditImageDeleteCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditImageDeleteCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The language code (a BCP-47 language tag) of the localized listing whose images are to read or modified. For example, to select Austrian German, pass "de-AT".
    ///
    /// Sets the *language* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn language(mut self, new_value: &str) -> EditImageDeleteCall<'a, S> {
        self._language = new_value.to_string();
        self
    }
    ///
    /// Sets the *image type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn image_type(mut self, new_value: &str) -> EditImageDeleteCall<'a, S> {
        self._image_type = new_value.to_string();
        self
    }
    /// Unique identifier an image within the set of images attached to this edit.
    ///
    /// Sets the *image id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn image_id(mut self, new_value: &str) -> EditImageDeleteCall<'a, S> {
        self._image_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditImageDeleteCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditImageDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditImageDeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditImageDeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditImageDeleteCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Deletes all images for the specified language and image type.
///
/// A builder for the *images.deleteall* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().images_deleteall("packageName", "editId", "language", "imageType")
///              .doit().await;
/// # }
/// ```
pub struct EditImageDeleteallCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _language: String,
    _image_type: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditImageDeleteallCall<'a, S> {}

impl<'a, S> EditImageDeleteallCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ImagesDeleteAllResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.images.deleteall",
                               http_method: hyper::Method::DELETE });

        for &field in ["alt", "packageName", "editId", "language", "imageType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("language", self._language);
        params.push("imageType", self._image_type);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/listings/{language}/{imageType}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{language}", "language"), ("{imageType}", "imageType")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["imageType", "language", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditImageDeleteallCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditImageDeleteallCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The language code (a BCP-47 language tag) of the localized listing whose images are to read or modified. For example, to select Austrian German, pass "de-AT".
    ///
    /// Sets the *language* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn language(mut self, new_value: &str) -> EditImageDeleteallCall<'a, S> {
        self._language = new_value.to_string();
        self
    }
    ///
    /// Sets the *image type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn image_type(mut self, new_value: &str) -> EditImageDeleteallCall<'a, S> {
        self._image_type = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditImageDeleteallCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditImageDeleteallCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditImageDeleteallCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditImageDeleteallCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditImageDeleteallCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Lists all images for the specified language and image type.
///
/// A builder for the *images.list* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().images_list("packageName", "editId", "language", "imageType")
///              .doit().await;
/// # }
/// ```
pub struct EditImageListCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _language: String,
    _image_type: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditImageListCall<'a, S> {}

impl<'a, S> EditImageListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ImagesListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.images.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "editId", "language", "imageType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("language", self._language);
        params.push("imageType", self._image_type);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/listings/{language}/{imageType}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{language}", "language"), ("{imageType}", "imageType")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["imageType", "language", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditImageListCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditImageListCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The language code (a BCP-47 language tag) of the localized listing whose images are to read or modified. For example, to select Austrian German, pass "de-AT".
    ///
    /// Sets the *language* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn language(mut self, new_value: &str) -> EditImageListCall<'a, S> {
        self._language = new_value.to_string();
        self
    }
    ///
    /// Sets the *image type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn image_type(mut self, new_value: &str) -> EditImageListCall<'a, S> {
        self._image_type = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditImageListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditImageListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditImageListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditImageListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditImageListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Uploads a new image and adds it to the list of images for the specified language and image type.
///
/// A builder for the *images.upload* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use std::fs;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload_resumable(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().images_upload("packageName", "editId", "language", "imageType")
///              .upload_resumable(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;
/// # }
/// ```
pub struct EditImageUploadCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _language: String,
    _image_type: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditImageUploadCall<'a, S> {}

impl<'a, S> EditImageUploadCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    async fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: client::UploadProtocol) -> client::Result<(hyper::Response<hyper::body::Body>, ImagesUploadResponse)>
		where RS: client::ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.images.upload",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "packageName", "editId", "language", "imageType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("language", self._language);
        params.push("imageType", self._image_type);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let (mut url, upload_type) =
            if protocol == client::UploadProtocol::Resumable {
                (self.hub._root_url.clone() + "resumable/upload/androidpublisher/v2/applications/{packageName}/edits/{editId}/listings/{language}/{imageType}", "resumable")
            } else if protocol == client::UploadProtocol::Simple {
                (self.hub._root_url.clone() + "upload/androidpublisher/v2/applications/{packageName}/edits/{editId}/listings/{language}/{imageType}", "multipart")
            } else {
                unreachable!()
            };
        params.push("uploadType", upload_type);
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{language}", "language"), ("{imageType}", "imageType")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["imageType", "language", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);


        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    Ok(hyper::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Location", upload_url.as_ref().unwrap().clone())
                        .body(hyper::body::Body::empty())
                        .unwrap())
                } else {
                    let client = &self.hub.client;
                    dlg.pre_request();
                    let mut req_builder = hyper::Request::builder()
                        .method(hyper::Method::POST)
                        .uri(url.as_str())
                        .header(USER_AGENT, self.hub._user_agent.clone());
    
                    if let Some(token) = token.as_ref() {
                        req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                    }
    
                    upload_url_from_server = true;
                    if protocol == client::UploadProtocol::Resumable {
                        req_builder = req_builder.header("X-Upload-Content-Type", format!("{}", reader_mime_type));
                    }
    
                            let request = if protocol == client::UploadProtocol::Simple {
                                let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                    reader.seek(io::SeekFrom::Start(0)).unwrap();
                    if size > 15728640 {
                    	return Err(client::Error::UploadSizeLimitExceeded(size, 15728640))
                    }
                                let mut bytes = Vec::with_capacity(size as usize);
                                reader.read_to_end(&mut bytes)?;
                                req_builder.header(CONTENT_TYPE, reader_mime_type.to_string())
                                         .header(CONTENT_LENGTH, size)
                                         .body(hyper::body::Body::from(bytes))
                            } else {
                                req_builder.body(hyper::body::Body::from(Vec::new()))
                            };
    
                    client.request(request.unwrap()).await
    
                }
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    if protocol == client::UploadProtocol::Resumable {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 15728640 {
                        	return Err(client::Error::UploadSizeLimitExceeded(size, 15728640))
                        }
                        let upload_result = {
                            let url_str = &res.headers().get("Location").expect("LOCATION header is part of protocol").to_str().unwrap();
                            if upload_url_from_server {
                                dlg.store_upload_url(Some(url_str));
                            }

                            client::ResumableUploadHelper {
                                client: &self.hub.client,
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &self.hub.auth,
                                user_agent: &self.hub._user_agent,
                                // TODO: Check this assumption
                                auth_header: format!("Bearer {}", token.ok_or_else(|| client::Error::MissingToken("resumable upload requires token".into()))?.as_str()),
                                url: url_str,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload().await
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(client::Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(client::Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status().is_success() {
                                    dlg.store_upload_url(None);
                                    dlg.finished(false);
                                    return Err(client::Error::Failure(res))
                                }
                            }
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }

    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *multipart*: yes
    /// * *max size*: 15MB
    /// * *valid mime types*: 'image/*'
    pub async fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, ImagesUploadResponse)>
                where RS: client::ReadSeek {
        self.doit(resumeable_stream, mime_type, client::UploadProtocol::Resumable).await
    }
    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *multipart*: yes
    /// * *max size*: 15MB
    /// * *valid mime types*: 'image/*'
    pub async fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, ImagesUploadResponse)>
                where RS: client::ReadSeek {
        self.doit(stream, mime_type, client::UploadProtocol::Simple).await
    }

    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditImageUploadCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditImageUploadCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The language code (a BCP-47 language tag) of the localized listing whose images are to read or modified. For example, to select Austrian German, pass "de-AT".
    ///
    /// Sets the *language* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn language(mut self, new_value: &str) -> EditImageUploadCall<'a, S> {
        self._language = new_value.to_string();
        self
    }
    ///
    /// Sets the *image type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn image_type(mut self, new_value: &str) -> EditImageUploadCall<'a, S> {
        self._image_type = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditImageUploadCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditImageUploadCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditImageUploadCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditImageUploadCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditImageUploadCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Deletes the specified localized store listing from an edit.
///
/// A builder for the *listings.delete* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().listings_delete("packageName", "editId", "language")
///              .doit().await;
/// # }
/// ```
pub struct EditListingDeleteCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _language: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditListingDeleteCall<'a, S> {}

impl<'a, S> EditListingDeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.listings.delete",
                               http_method: hyper::Method::DELETE });

        for &field in ["packageName", "editId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("language", self._language);

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/listings/{language}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{language}", "language")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["language", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditListingDeleteCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditListingDeleteCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The language code (a BCP-47 language tag) of the localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    ///
    /// Sets the *language* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn language(mut self, new_value: &str) -> EditListingDeleteCall<'a, S> {
        self._language = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditListingDeleteCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditListingDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditListingDeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditListingDeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditListingDeleteCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Deletes all localized listings from an edit.
///
/// A builder for the *listings.deleteall* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().listings_deleteall("packageName", "editId")
///              .doit().await;
/// # }
/// ```
pub struct EditListingDeleteallCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditListingDeleteallCall<'a, S> {}

impl<'a, S> EditListingDeleteallCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.listings.deleteall",
                               http_method: hyper::Method::DELETE });

        for &field in ["packageName", "editId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/listings";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditListingDeleteallCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditListingDeleteallCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditListingDeleteallCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditListingDeleteallCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditListingDeleteallCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditListingDeleteallCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditListingDeleteallCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Fetches information about a localized store listing.
///
/// A builder for the *listings.get* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().listings_get("packageName", "editId", "language")
///              .doit().await;
/// # }
/// ```
pub struct EditListingGetCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _language: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditListingGetCall<'a, S> {}

impl<'a, S> EditListingGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Listing)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.listings.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "editId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("language", self._language);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/listings/{language}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{language}", "language")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["language", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditListingGetCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditListingGetCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The language code (a BCP-47 language tag) of the localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    ///
    /// Sets the *language* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn language(mut self, new_value: &str) -> EditListingGetCall<'a, S> {
        self._language = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditListingGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditListingGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditListingGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditListingGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditListingGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns all of the localized store listings attached to this edit.
///
/// A builder for the *listings.list* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().listings_list("packageName", "editId")
///              .doit().await;
/// # }
/// ```
pub struct EditListingListCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditListingListCall<'a, S> {}

impl<'a, S> EditListingListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListingsListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.listings.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "editId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/listings";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditListingListCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditListingListCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditListingListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditListingListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditListingListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditListingListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditListingListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Creates or updates a localized store listing. This method supports patch semantics.
///
/// A builder for the *listings.patch* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::Listing;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Listing::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().listings_patch(req, "packageName", "editId", "language")
///              .doit().await;
/// # }
/// ```
pub struct EditListingPatchCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: Listing,
    _package_name: String,
    _edit_id: String,
    _language: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditListingPatchCall<'a, S> {}

impl<'a, S> EditListingPatchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Listing)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.listings.patch",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "packageName", "editId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("language", self._language);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/listings/{language}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{language}", "language")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["language", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Listing) -> EditListingPatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditListingPatchCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditListingPatchCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The language code (a BCP-47 language tag) of the localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    ///
    /// Sets the *language* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn language(mut self, new_value: &str) -> EditListingPatchCall<'a, S> {
        self._language = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditListingPatchCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditListingPatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditListingPatchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditListingPatchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditListingPatchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Creates or updates a localized store listing.
///
/// A builder for the *listings.update* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::Listing;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Listing::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().listings_update(req, "packageName", "editId", "language")
///              .doit().await;
/// # }
/// ```
pub struct EditListingUpdateCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: Listing,
    _package_name: String,
    _edit_id: String,
    _language: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditListingUpdateCall<'a, S> {}

impl<'a, S> EditListingUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Listing)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.listings.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "packageName", "editId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("language", self._language);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/listings/{language}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{language}", "language")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["language", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Listing) -> EditListingUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditListingUpdateCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditListingUpdateCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The language code (a BCP-47 language tag) of the localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    ///
    /// Sets the *language* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn language(mut self, new_value: &str) -> EditListingUpdateCall<'a, S> {
        self._language = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditListingUpdateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditListingUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditListingUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditListingUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditListingUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// A builder for the *testers.get* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().testers_get("packageName", "editId", "track")
///              .doit().await;
/// # }
/// ```
pub struct EditTesterGetCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _track: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditTesterGetCall<'a, S> {}

impl<'a, S> EditTesterGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Testers)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.testers.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "editId", "track"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("track", self._track);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/testers/{track}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{track}", "track")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["track", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditTesterGetCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditTesterGetCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The track to read or modify.
    ///
    /// Sets the *track* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn track(mut self, new_value: &str) -> EditTesterGetCall<'a, S> {
        self._track = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditTesterGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditTesterGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditTesterGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditTesterGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditTesterGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// A builder for the *testers.patch* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::Testers;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Testers::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().testers_patch(req, "packageName", "editId", "track")
///              .doit().await;
/// # }
/// ```
pub struct EditTesterPatchCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: Testers,
    _package_name: String,
    _edit_id: String,
    _track: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditTesterPatchCall<'a, S> {}

impl<'a, S> EditTesterPatchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Testers)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.testers.patch",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "packageName", "editId", "track"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("track", self._track);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/testers/{track}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{track}", "track")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["track", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Testers) -> EditTesterPatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditTesterPatchCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditTesterPatchCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The track to read or modify.
    ///
    /// Sets the *track* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn track(mut self, new_value: &str) -> EditTesterPatchCall<'a, S> {
        self._track = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditTesterPatchCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditTesterPatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditTesterPatchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditTesterPatchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditTesterPatchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// A builder for the *testers.update* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::Testers;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Testers::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().testers_update(req, "packageName", "editId", "track")
///              .doit().await;
/// # }
/// ```
pub struct EditTesterUpdateCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: Testers,
    _package_name: String,
    _edit_id: String,
    _track: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditTesterUpdateCall<'a, S> {}

impl<'a, S> EditTesterUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Testers)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.testers.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "packageName", "editId", "track"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("track", self._track);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/testers/{track}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{track}", "track")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["track", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Testers) -> EditTesterUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditTesterUpdateCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditTesterUpdateCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The track to read or modify.
    ///
    /// Sets the *track* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn track(mut self, new_value: &str) -> EditTesterUpdateCall<'a, S> {
        self._track = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditTesterUpdateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditTesterUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditTesterUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditTesterUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditTesterUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Fetches the track configuration for the specified track type. Includes the APK version codes that are in this track.
///
/// A builder for the *tracks.get* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().tracks_get("packageName", "editId", "track")
///              .doit().await;
/// # }
/// ```
pub struct EditTrackGetCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _track: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditTrackGetCall<'a, S> {}

impl<'a, S> EditTrackGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Track)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.tracks.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "editId", "track"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("track", self._track);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/tracks/{track}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{track}", "track")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["track", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditTrackGetCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditTrackGetCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The track to read or modify.
    ///
    /// Sets the *track* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn track(mut self, new_value: &str) -> EditTrackGetCall<'a, S> {
        self._track = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditTrackGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditTrackGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditTrackGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditTrackGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditTrackGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Lists all the track configurations for this edit.
///
/// A builder for the *tracks.list* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().tracks_list("packageName", "editId")
///              .doit().await;
/// # }
/// ```
pub struct EditTrackListCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditTrackListCall<'a, S> {}

impl<'a, S> EditTrackListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TracksListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.tracks.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "editId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/tracks";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditTrackListCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditTrackListCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditTrackListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditTrackListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditTrackListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditTrackListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditTrackListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates the track configuration for the specified track type. This method supports patch semantics.
///
/// A builder for the *tracks.patch* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::Track;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Track::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().tracks_patch(req, "packageName", "editId", "track")
///              .doit().await;
/// # }
/// ```
pub struct EditTrackPatchCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: Track,
    _package_name: String,
    _edit_id: String,
    _track: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditTrackPatchCall<'a, S> {}

impl<'a, S> EditTrackPatchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Track)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.tracks.patch",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "packageName", "editId", "track"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("track", self._track);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/tracks/{track}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{track}", "track")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["track", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Track) -> EditTrackPatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditTrackPatchCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditTrackPatchCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The track to read or modify.
    ///
    /// Sets the *track* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn track(mut self, new_value: &str) -> EditTrackPatchCall<'a, S> {
        self._track = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditTrackPatchCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditTrackPatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditTrackPatchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditTrackPatchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditTrackPatchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates the track configuration for the specified track type.
///
/// A builder for the *tracks.update* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::Track;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Track::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().tracks_update(req, "packageName", "editId", "track")
///              .doit().await;
/// # }
/// ```
pub struct EditTrackUpdateCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: Track,
    _package_name: String,
    _edit_id: String,
    _track: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditTrackUpdateCall<'a, S> {}

impl<'a, S> EditTrackUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Track)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.tracks.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "packageName", "editId", "track"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);
        params.push("track", self._track);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}/tracks/{track}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId"), ("{track}", "track")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["track", "editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Track) -> EditTrackUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditTrackUpdateCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditTrackUpdateCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The track to read or modify.
    ///
    /// Sets the *track* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn track(mut self, new_value: &str) -> EditTrackUpdateCall<'a, S> {
        self._track = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditTrackUpdateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditTrackUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditTrackUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditTrackUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditTrackUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Commits/applies the changes made in this edit back to the app.
///
/// A builder for the *commit* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().commit("packageName", "editId")
///              .doit().await;
/// # }
/// ```
pub struct EditCommitCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditCommitCall<'a, S> {}

impl<'a, S> EditCommitCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, AppEdit)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.commit",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "packageName", "editId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}:commit";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditCommitCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditCommitCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditCommitCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditCommitCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditCommitCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditCommitCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditCommitCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Deletes an edit for an app. Creating a new edit will automatically delete any of your previous edits so this method need only be called if you want to preemptively abandon an edit.
///
/// A builder for the *delete* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().delete("packageName", "editId")
///              .doit().await;
/// # }
/// ```
pub struct EditDeleteCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditDeleteCall<'a, S> {}

impl<'a, S> EditDeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.delete",
                               http_method: hyper::Method::DELETE });

        for &field in ["packageName", "editId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditDeleteCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditDeleteCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditDeleteCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditDeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditDeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditDeleteCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns information about the edit specified. Calls will fail if the edit is no long active (e.g. has been deleted, superseded or expired).
///
/// A builder for the *get* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().get("packageName", "editId")
///              .doit().await;
/// # }
/// ```
pub struct EditGetCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditGetCall<'a, S> {}

impl<'a, S> EditGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, AppEdit)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "editId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditGetCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditGetCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Creates a new edit for an app, populated with the app's current state.
///
/// A builder for the *insert* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::AppEdit;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = AppEdit::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().insert(req, "packageName")
///              .doit().await;
/// # }
/// ```
pub struct EditInsertCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: AppEdit,
    _package_name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditInsertCall<'a, S> {}

impl<'a, S> EditInsertCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, AppEdit)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.insert",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "packageName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: AppEdit) -> EditInsertCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditInsertCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditInsertCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditInsertCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditInsertCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditInsertCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditInsertCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Checks that the edit can be successfully committed. The edit's changes are not applied to the live app.
///
/// A builder for the *validate* method supported by a *edit* resource.
/// It is not used directly, but through a [`EditMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.edits().validate("packageName", "editId")
///              .doit().await;
/// # }
/// ```
pub struct EditValidateCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _edit_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for EditValidateCall<'a, S> {}

impl<'a, S> EditValidateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, AppEdit)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.edits.validate",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "packageName", "editId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("editId", self._edit_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/edits/{editId}:validate";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{editId}", "editId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["editId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> EditValidateCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for this edit.
    ///
    /// Sets the *edit id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn edit_id(mut self, new_value: &str) -> EditValidateCall<'a, S> {
        self._edit_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EditValidateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> EditValidateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> EditValidateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> EditValidateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> EditValidateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Delete an in-app product for an app.
///
/// A builder for the *delete* method supported by a *inappproduct* resource.
/// It is not used directly, but through a [`InappproductMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.inappproducts().delete("packageName", "sku")
///              .doit().await;
/// # }
/// ```
pub struct InappproductDeleteCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _sku: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for InappproductDeleteCall<'a, S> {}

impl<'a, S> InappproductDeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.inappproducts.delete",
                               http_method: hyper::Method::DELETE });

        for &field in ["packageName", "sku"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("sku", self._sku);

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "{packageName}/inappproducts/{sku}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{sku}", "sku")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["sku", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app with the in-app product; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> InappproductDeleteCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for the in-app product.
    ///
    /// Sets the *sku* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn sku(mut self, new_value: &str) -> InappproductDeleteCall<'a, S> {
        self._sku = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InappproductDeleteCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> InappproductDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> InappproductDeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> InappproductDeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> InappproductDeleteCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns information about the in-app product specified.
///
/// A builder for the *get* method supported by a *inappproduct* resource.
/// It is not used directly, but through a [`InappproductMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.inappproducts().get("packageName", "sku")
///              .doit().await;
/// # }
/// ```
pub struct InappproductGetCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _sku: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for InappproductGetCall<'a, S> {}

impl<'a, S> InappproductGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, InAppProduct)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.inappproducts.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "sku"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("sku", self._sku);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/inappproducts/{sku}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{sku}", "sku")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["sku", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> InappproductGetCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for the in-app product.
    ///
    /// Sets the *sku* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn sku(mut self, new_value: &str) -> InappproductGetCall<'a, S> {
        self._sku = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InappproductGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> InappproductGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> InappproductGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> InappproductGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> InappproductGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Creates a new in-app product for an app.
///
/// A builder for the *insert* method supported by a *inappproduct* resource.
/// It is not used directly, but through a [`InappproductMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::InAppProduct;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = InAppProduct::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.inappproducts().insert(req, "packageName")
///              .auto_convert_missing_prices(false)
///              .doit().await;
/// # }
/// ```
pub struct InappproductInsertCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: InAppProduct,
    _package_name: String,
    _auto_convert_missing_prices: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for InappproductInsertCall<'a, S> {}

impl<'a, S> InappproductInsertCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, InAppProduct)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.inappproducts.insert",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "packageName", "autoConvertMissingPrices"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("packageName", self._package_name);
        if let Some(value) = self._auto_convert_missing_prices.as_ref() {
            params.push("autoConvertMissingPrices", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/inappproducts";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: InAppProduct) -> InappproductInsertCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> InappproductInsertCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// If true the prices for all regions targeted by the parent app that don't have a price specified for this in-app product will be auto converted to the target currency based on the default price. Defaults to false.
    ///
    /// Sets the *auto convert missing prices* query property to the given value.
    pub fn auto_convert_missing_prices(mut self, new_value: bool) -> InappproductInsertCall<'a, S> {
        self._auto_convert_missing_prices = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InappproductInsertCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> InappproductInsertCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> InappproductInsertCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> InappproductInsertCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> InappproductInsertCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// List all the in-app products for an Android app, both subscriptions and managed in-app products..
///
/// A builder for the *list* method supported by a *inappproduct* resource.
/// It is not used directly, but through a [`InappproductMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.inappproducts().list("packageName")
///              .token("duo")
///              .start_index(59)
///              .max_results(44)
///              .doit().await;
/// # }
/// ```
pub struct InappproductListCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _token: Option<String>,
    _start_index: Option<u32>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for InappproductListCall<'a, S> {}

impl<'a, S> InappproductListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, InappproductsListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.inappproducts.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "token", "startIndex", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        if let Some(value) = self._token.as_ref() {
            params.push("token", value);
        }
        if let Some(value) = self._start_index.as_ref() {
            params.push("startIndex", value.to_string());
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/inappproducts";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app with in-app products; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> InappproductListCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    ///
    /// Sets the *token* query property to the given value.
    pub fn token(mut self, new_value: &str) -> InappproductListCall<'a, S> {
        self._token = Some(new_value.to_string());
        self
    }
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u32) -> InappproductListCall<'a, S> {
        self._start_index = Some(new_value);
        self
    }
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> InappproductListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InappproductListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> InappproductListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> InappproductListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> InappproductListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> InappproductListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates the details of an in-app product. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *inappproduct* resource.
/// It is not used directly, but through a [`InappproductMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::InAppProduct;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = InAppProduct::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.inappproducts().patch(req, "packageName", "sku")
///              .auto_convert_missing_prices(false)
///              .doit().await;
/// # }
/// ```
pub struct InappproductPatchCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: InAppProduct,
    _package_name: String,
    _sku: String,
    _auto_convert_missing_prices: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for InappproductPatchCall<'a, S> {}

impl<'a, S> InappproductPatchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, InAppProduct)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.inappproducts.patch",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "packageName", "sku", "autoConvertMissingPrices"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("sku", self._sku);
        if let Some(value) = self._auto_convert_missing_prices.as_ref() {
            params.push("autoConvertMissingPrices", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/inappproducts/{sku}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{sku}", "sku")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["sku", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: InAppProduct) -> InappproductPatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app with the in-app product; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> InappproductPatchCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for the in-app product.
    ///
    /// Sets the *sku* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn sku(mut self, new_value: &str) -> InappproductPatchCall<'a, S> {
        self._sku = new_value.to_string();
        self
    }
    /// If true the prices for all regions targeted by the parent app that don't have a price specified for this in-app product will be auto converted to the target currency based on the default price. Defaults to false.
    ///
    /// Sets the *auto convert missing prices* query property to the given value.
    pub fn auto_convert_missing_prices(mut self, new_value: bool) -> InappproductPatchCall<'a, S> {
        self._auto_convert_missing_prices = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InappproductPatchCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> InappproductPatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> InappproductPatchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> InappproductPatchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> InappproductPatchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates the details of an in-app product.
///
/// A builder for the *update* method supported by a *inappproduct* resource.
/// It is not used directly, but through a [`InappproductMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::InAppProduct;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = InAppProduct::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.inappproducts().update(req, "packageName", "sku")
///              .auto_convert_missing_prices(true)
///              .doit().await;
/// # }
/// ```
pub struct InappproductUpdateCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: InAppProduct,
    _package_name: String,
    _sku: String,
    _auto_convert_missing_prices: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for InappproductUpdateCall<'a, S> {}

impl<'a, S> InappproductUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, InAppProduct)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.inappproducts.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "packageName", "sku", "autoConvertMissingPrices"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("sku", self._sku);
        if let Some(value) = self._auto_convert_missing_prices.as_ref() {
            params.push("autoConvertMissingPrices", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/inappproducts/{sku}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{sku}", "sku")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["sku", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: InAppProduct) -> InappproductUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app with the in-app product; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> InappproductUpdateCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// Unique identifier for the in-app product.
    ///
    /// Sets the *sku* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn sku(mut self, new_value: &str) -> InappproductUpdateCall<'a, S> {
        self._sku = new_value.to_string();
        self
    }
    /// If true the prices for all regions targeted by the parent app that don't have a price specified for this in-app product will be auto converted to the target currency based on the default price. Defaults to false.
    ///
    /// Sets the *auto convert missing prices* query property to the given value.
    pub fn auto_convert_missing_prices(mut self, new_value: bool) -> InappproductUpdateCall<'a, S> {
        self._auto_convert_missing_prices = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InappproductUpdateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> InappproductUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> InappproductUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> InappproductUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> InappproductUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Refund a user's subscription or in-app purchase order.
///
/// A builder for the *refund* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().refund("packageName", "orderId")
///              .revoke(false)
///              .doit().await;
/// # }
/// ```
pub struct OrderRefundCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _order_id: String,
    _revoke: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderRefundCall<'a, S> {}

impl<'a, S> OrderRefundCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.orders.refund",
                               http_method: hyper::Method::POST });

        for &field in ["packageName", "orderId", "revoke"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("orderId", self._order_id);
        if let Some(value) = self._revoke.as_ref() {
            params.push("revoke", value.to_string());
        }

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "{packageName}/orders/{orderId}:refund";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The package name of the application for which this subscription or in-app item was purchased (for example, 'com.some.thing').
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> OrderRefundCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// The order ID provided to the user when the subscription or in-app order was purchased.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderRefundCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// Whether to revoke the purchased item. If set to true, access to the subscription or in-app item will be terminated immediately. If the item is a recurring subscription, all future payments will also be terminated. Consumed in-app items need to be handled by developer's app. (optional)
    ///
    /// Sets the *revoke* query property to the given value.
    pub fn revoke(mut self, new_value: bool) -> OrderRefundCall<'a, S> {
        self._revoke = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderRefundCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> OrderRefundCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderRefundCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderRefundCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderRefundCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Checks the purchase and consumption status of an inapp item.
///
/// A builder for the *products.get* method supported by a *purchase* resource.
/// It is not used directly, but through a [`PurchaseMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.purchases().products_get("packageName", "productId", "token")
///              .doit().await;
/// # }
/// ```
pub struct PurchaseProductGetCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _product_id: String,
    _token: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for PurchaseProductGetCall<'a, S> {}

impl<'a, S> PurchaseProductGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ProductPurchase)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.purchases.products.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "productId", "token"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("productId", self._product_id);
        params.push("token", self._token);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/purchases/products/{productId}/tokens/{token}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{productId}", "productId"), ("{token}", "token")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["token", "productId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The package name of the application the inapp product was sold in (for example, 'com.some.thing').
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> PurchaseProductGetCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// The inapp product SKU (for example, 'com.some.thing.inapp1').
    ///
    /// Sets the *product id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn product_id(mut self, new_value: &str) -> PurchaseProductGetCall<'a, S> {
        self._product_id = new_value.to_string();
        self
    }
    /// The token provided to the user's device when the inapp product was purchased.
    ///
    /// Sets the *token* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn token(mut self, new_value: &str) -> PurchaseProductGetCall<'a, S> {
        self._token = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PurchaseProductGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PurchaseProductGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PurchaseProductGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PurchaseProductGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PurchaseProductGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Cancels a user's subscription purchase. The subscription remains valid until its expiration time.
///
/// A builder for the *subscriptions.cancel* method supported by a *purchase* resource.
/// It is not used directly, but through a [`PurchaseMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.purchases().subscriptions_cancel("packageName", "subscriptionId", "token")
///              .doit().await;
/// # }
/// ```
pub struct PurchaseSubscriptionCancelCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _subscription_id: String,
    _token: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for PurchaseSubscriptionCancelCall<'a, S> {}

impl<'a, S> PurchaseSubscriptionCancelCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.purchases.subscriptions.cancel",
                               http_method: hyper::Method::POST });

        for &field in ["packageName", "subscriptionId", "token"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("subscriptionId", self._subscription_id);
        params.push("token", self._token);

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "{packageName}/purchases/subscriptions/{subscriptionId}/tokens/{token}:cancel";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{subscriptionId}", "subscriptionId"), ("{token}", "token")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["token", "subscriptionId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> PurchaseSubscriptionCancelCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// The purchased subscription ID (for example, 'monthly001').
    ///
    /// Sets the *subscription id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn subscription_id(mut self, new_value: &str) -> PurchaseSubscriptionCancelCall<'a, S> {
        self._subscription_id = new_value.to_string();
        self
    }
    /// The token provided to the user's device when the subscription was purchased.
    ///
    /// Sets the *token* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn token(mut self, new_value: &str) -> PurchaseSubscriptionCancelCall<'a, S> {
        self._token = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PurchaseSubscriptionCancelCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PurchaseSubscriptionCancelCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PurchaseSubscriptionCancelCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PurchaseSubscriptionCancelCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PurchaseSubscriptionCancelCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Defers a user's subscription purchase until a specified future expiration time.
///
/// A builder for the *subscriptions.defer* method supported by a *purchase* resource.
/// It is not used directly, but through a [`PurchaseMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::SubscriptionPurchasesDeferRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SubscriptionPurchasesDeferRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.purchases().subscriptions_defer(req, "packageName", "subscriptionId", "token")
///              .doit().await;
/// # }
/// ```
pub struct PurchaseSubscriptionDeferCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: SubscriptionPurchasesDeferRequest,
    _package_name: String,
    _subscription_id: String,
    _token: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for PurchaseSubscriptionDeferCall<'a, S> {}

impl<'a, S> PurchaseSubscriptionDeferCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, SubscriptionPurchasesDeferResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.purchases.subscriptions.defer",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "packageName", "subscriptionId", "token"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("subscriptionId", self._subscription_id);
        params.push("token", self._token);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/purchases/subscriptions/{subscriptionId}/tokens/{token}:defer";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{subscriptionId}", "subscriptionId"), ("{token}", "token")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["token", "subscriptionId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: SubscriptionPurchasesDeferRequest) -> PurchaseSubscriptionDeferCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> PurchaseSubscriptionDeferCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// The purchased subscription ID (for example, 'monthly001').
    ///
    /// Sets the *subscription id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn subscription_id(mut self, new_value: &str) -> PurchaseSubscriptionDeferCall<'a, S> {
        self._subscription_id = new_value.to_string();
        self
    }
    /// The token provided to the user's device when the subscription was purchased.
    ///
    /// Sets the *token* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn token(mut self, new_value: &str) -> PurchaseSubscriptionDeferCall<'a, S> {
        self._token = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PurchaseSubscriptionDeferCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PurchaseSubscriptionDeferCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PurchaseSubscriptionDeferCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PurchaseSubscriptionDeferCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PurchaseSubscriptionDeferCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Checks whether a user's subscription purchase is valid and returns its expiry time.
///
/// A builder for the *subscriptions.get* method supported by a *purchase* resource.
/// It is not used directly, but through a [`PurchaseMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.purchases().subscriptions_get("packageName", "subscriptionId", "token")
///              .doit().await;
/// # }
/// ```
pub struct PurchaseSubscriptionGetCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _subscription_id: String,
    _token: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for PurchaseSubscriptionGetCall<'a, S> {}

impl<'a, S> PurchaseSubscriptionGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, SubscriptionPurchase)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.purchases.subscriptions.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "subscriptionId", "token"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("subscriptionId", self._subscription_id);
        params.push("token", self._token);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/purchases/subscriptions/{subscriptionId}/tokens/{token}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{subscriptionId}", "subscriptionId"), ("{token}", "token")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["token", "subscriptionId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> PurchaseSubscriptionGetCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// The purchased subscription ID (for example, 'monthly001').
    ///
    /// Sets the *subscription id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn subscription_id(mut self, new_value: &str) -> PurchaseSubscriptionGetCall<'a, S> {
        self._subscription_id = new_value.to_string();
        self
    }
    /// The token provided to the user's device when the subscription was purchased.
    ///
    /// Sets the *token* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn token(mut self, new_value: &str) -> PurchaseSubscriptionGetCall<'a, S> {
        self._token = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PurchaseSubscriptionGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PurchaseSubscriptionGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PurchaseSubscriptionGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PurchaseSubscriptionGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PurchaseSubscriptionGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Refunds a user's subscription purchase, but the subscription remains valid until its expiration time and it will continue to recur.
///
/// A builder for the *subscriptions.refund* method supported by a *purchase* resource.
/// It is not used directly, but through a [`PurchaseMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.purchases().subscriptions_refund("packageName", "subscriptionId", "token")
///              .doit().await;
/// # }
/// ```
pub struct PurchaseSubscriptionRefundCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _subscription_id: String,
    _token: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for PurchaseSubscriptionRefundCall<'a, S> {}

impl<'a, S> PurchaseSubscriptionRefundCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.purchases.subscriptions.refund",
                               http_method: hyper::Method::POST });

        for &field in ["packageName", "subscriptionId", "token"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("subscriptionId", self._subscription_id);
        params.push("token", self._token);

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "{packageName}/purchases/subscriptions/{subscriptionId}/tokens/{token}:refund";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{subscriptionId}", "subscriptionId"), ("{token}", "token")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["token", "subscriptionId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> PurchaseSubscriptionRefundCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// The purchased subscription ID (for example, 'monthly001').
    ///
    /// Sets the *subscription id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn subscription_id(mut self, new_value: &str) -> PurchaseSubscriptionRefundCall<'a, S> {
        self._subscription_id = new_value.to_string();
        self
    }
    /// The token provided to the user's device when the subscription was purchased.
    ///
    /// Sets the *token* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn token(mut self, new_value: &str) -> PurchaseSubscriptionRefundCall<'a, S> {
        self._token = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PurchaseSubscriptionRefundCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PurchaseSubscriptionRefundCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PurchaseSubscriptionRefundCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PurchaseSubscriptionRefundCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PurchaseSubscriptionRefundCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Refunds and immediately revokes a user's subscription purchase. Access to the subscription will be terminated immediately and it will stop recurring.
///
/// A builder for the *subscriptions.revoke* method supported by a *purchase* resource.
/// It is not used directly, but through a [`PurchaseMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.purchases().subscriptions_revoke("packageName", "subscriptionId", "token")
///              .doit().await;
/// # }
/// ```
pub struct PurchaseSubscriptionRevokeCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _subscription_id: String,
    _token: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for PurchaseSubscriptionRevokeCall<'a, S> {}

impl<'a, S> PurchaseSubscriptionRevokeCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.purchases.subscriptions.revoke",
                               http_method: hyper::Method::POST });

        for &field in ["packageName", "subscriptionId", "token"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("subscriptionId", self._subscription_id);
        params.push("token", self._token);

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "{packageName}/purchases/subscriptions/{subscriptionId}/tokens/{token}:revoke";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{subscriptionId}", "subscriptionId"), ("{token}", "token")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["token", "subscriptionId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> PurchaseSubscriptionRevokeCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    /// The purchased subscription ID (for example, 'monthly001').
    ///
    /// Sets the *subscription id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn subscription_id(mut self, new_value: &str) -> PurchaseSubscriptionRevokeCall<'a, S> {
        self._subscription_id = new_value.to_string();
        self
    }
    /// The token provided to the user's device when the subscription was purchased.
    ///
    /// Sets the *token* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn token(mut self, new_value: &str) -> PurchaseSubscriptionRevokeCall<'a, S> {
        self._token = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PurchaseSubscriptionRevokeCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PurchaseSubscriptionRevokeCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PurchaseSubscriptionRevokeCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PurchaseSubscriptionRevokeCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PurchaseSubscriptionRevokeCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Lists the purchases that were canceled, refunded or charged-back.
///
/// A builder for the *voidedpurchases.list* method supported by a *purchase* resource.
/// It is not used directly, but through a [`PurchaseMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.purchases().voidedpurchases_list("packageName")
///              .token("nonumy")
///              .start_time(-22)
///              .start_index(89)
///              .max_results(80)
///              .end_time(-60)
///              .doit().await;
/// # }
/// ```
pub struct PurchaseVoidedpurchaseListCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _token: Option<String>,
    _start_time: Option<i64>,
    _start_index: Option<u32>,
    _max_results: Option<u32>,
    _end_time: Option<i64>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for PurchaseVoidedpurchaseListCall<'a, S> {}

impl<'a, S> PurchaseVoidedpurchaseListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, VoidedPurchasesListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.purchases.voidedpurchases.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "token", "startTime", "startIndex", "maxResults", "endTime"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(8 + self._additional_params.len());
        params.push("packageName", self._package_name);
        if let Some(value) = self._token.as_ref() {
            params.push("token", value);
        }
        if let Some(value) = self._start_time.as_ref() {
            params.push("startTime", value.to_string());
        }
        if let Some(value) = self._start_index.as_ref() {
            params.push("startIndex", value.to_string());
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._end_time.as_ref() {
            params.push("endTime", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/purchases/voidedpurchases";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The package name of the application for which voided purchases need to be returned (for example, 'com.some.thing').
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> PurchaseVoidedpurchaseListCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    ///
    /// Sets the *token* query property to the given value.
    pub fn token(mut self, new_value: &str) -> PurchaseVoidedpurchaseListCall<'a, S> {
        self._token = Some(new_value.to_string());
        self
    }
    /// The time, in milliseconds since the Epoch, of the oldest voided purchase that you want to see in the response. The value of this parameter cannot be older than 30 days and is ignored if a pagination token is set. Default value is current time minus 30 days. Note: This filter is applied on the time at which the record is seen as voided by our systems and not the actual voided time returned in the response.
    ///
    /// Sets the *start time* query property to the given value.
    pub fn start_time(mut self, new_value: i64) -> PurchaseVoidedpurchaseListCall<'a, S> {
        self._start_time = Some(new_value);
        self
    }
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u32) -> PurchaseVoidedpurchaseListCall<'a, S> {
        self._start_index = Some(new_value);
        self
    }
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> PurchaseVoidedpurchaseListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// The time, in milliseconds since the Epoch, of the newest voided purchase that you want to see in the response. The value of this parameter cannot be greater than the current time and is ignored if a pagination token is set. Default value is current time. Note: This filter is applied on the time at which the record is seen as voided by our systems and not the actual voided time returned in the response.
    ///
    /// Sets the *end time* query property to the given value.
    pub fn end_time(mut self, new_value: i64) -> PurchaseVoidedpurchaseListCall<'a, S> {
        self._end_time = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PurchaseVoidedpurchaseListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PurchaseVoidedpurchaseListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PurchaseVoidedpurchaseListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PurchaseVoidedpurchaseListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PurchaseVoidedpurchaseListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns a single review.
///
/// A builder for the *get* method supported by a *review* resource.
/// It is not used directly, but through a [`ReviewMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.reviews().get("packageName", "reviewId")
///              .translation_language("aliquyam")
///              .doit().await;
/// # }
/// ```
pub struct ReviewGetCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _review_id: String,
    _translation_language: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ReviewGetCall<'a, S> {}

impl<'a, S> ReviewGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Review)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.reviews.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "reviewId", "translationLanguage"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("reviewId", self._review_id);
        if let Some(value) = self._translation_language.as_ref() {
            params.push("translationLanguage", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/reviews/{reviewId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{reviewId}", "reviewId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["reviewId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app for which we want reviews; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> ReviewGetCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    ///
    /// Sets the *review id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn review_id(mut self, new_value: &str) -> ReviewGetCall<'a, S> {
        self._review_id = new_value.to_string();
        self
    }
    ///
    /// Sets the *translation language* query property to the given value.
    pub fn translation_language(mut self, new_value: &str) -> ReviewGetCall<'a, S> {
        self._translation_language = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ReviewGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ReviewGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ReviewGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ReviewGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ReviewGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns a list of reviews. Only reviews from last week will be returned.
///
/// A builder for the *list* method supported by a *review* resource.
/// It is not used directly, but through a [`ReviewMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.reviews().list("packageName")
///              .translation_language("At")
///              .token("dolores")
///              .start_index(55)
///              .max_results(39)
///              .doit().await;
/// # }
/// ```
pub struct ReviewListCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _package_name: String,
    _translation_language: Option<String>,
    _token: Option<String>,
    _start_index: Option<u32>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ReviewListCall<'a, S> {}

impl<'a, S> ReviewListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ReviewsListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.reviews.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "packageName", "translationLanguage", "token", "startIndex", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("packageName", self._package_name);
        if let Some(value) = self._translation_language.as_ref() {
            params.push("translationLanguage", value);
        }
        if let Some(value) = self._token.as_ref() {
            params.push("token", value);
        }
        if let Some(value) = self._start_index.as_ref() {
            params.push("startIndex", value.to_string());
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/reviews";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Unique identifier for the Android app for which we want reviews; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> ReviewListCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    ///
    /// Sets the *translation language* query property to the given value.
    pub fn translation_language(mut self, new_value: &str) -> ReviewListCall<'a, S> {
        self._translation_language = Some(new_value.to_string());
        self
    }
    ///
    /// Sets the *token* query property to the given value.
    pub fn token(mut self, new_value: &str) -> ReviewListCall<'a, S> {
        self._token = Some(new_value.to_string());
        self
    }
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u32) -> ReviewListCall<'a, S> {
        self._start_index = Some(new_value);
        self
    }
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> ReviewListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ReviewListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ReviewListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ReviewListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ReviewListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ReviewListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Reply to a single review, or update an existing reply.
///
/// A builder for the *reply* method supported by a *review* resource.
/// It is not used directly, but through a [`ReviewMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_androidpublisher2 as androidpublisher2;
/// use androidpublisher2::api::ReviewsReplyRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ReviewsReplyRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.reviews().reply(req, "packageName", "reviewId")
///              .doit().await;
/// # }
/// ```
pub struct ReviewReplyCall<'a, S>
    where S: 'a {

    hub: &'a AndroidPublisher<S>,
    _request: ReviewsReplyRequest,
    _package_name: String,
    _review_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ReviewReplyCall<'a, S> {}

impl<'a, S> ReviewReplyCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ReviewsReplyResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "androidpublisher.reviews.reply",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "packageName", "reviewId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("packageName", self._package_name);
        params.push("reviewId", self._review_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{packageName}/reviews/{reviewId}:reply";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{packageName}", "packageName"), ("{reviewId}", "reviewId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["reviewId", "packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ReviewsReplyRequest) -> ReviewReplyCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Unique identifier for the Android app for which we want reviews; for example, "com.spiffygame".
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> ReviewReplyCall<'a, S> {
        self._package_name = new_value.to_string();
        self
    }
    ///
    /// Sets the *review id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn review_id(mut self, new_value: &str) -> ReviewReplyCall<'a, S> {
        self._review_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ReviewReplyCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ReviewReplyCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ReviewReplyCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ReviewReplyCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ReviewReplyCall<'a, S> {
        self._scopes.clear();
        self
    }
}


