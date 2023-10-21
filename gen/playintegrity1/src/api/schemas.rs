use super::*;
/// Contains a signal helping apps differentiating between likely genuine users and likely non-genuine traffic (such as accounts being used for fraud, accounts used by automated traffic, or accounts used in device farms) based on the presence and volume of Play store activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountActivity {
    /// Required. Indicates the activity level of the account.
    #[serde(rename="activityLevel")]
    
    pub activity_level: Option<String>,
}

impl client::Part for AccountActivity {}


/// Contains the account information such as the licensing status for the user in the scope.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountDetails {
    /// Details about the account activity for the user in the scope.
    #[serde(rename="accountActivity")]
    
    pub account_activity: Option<AccountActivity>,
    /// Required. Details about the licensing status of the user for the app in the scope.
    #[serde(rename="appLicensingVerdict")]
    
    pub app_licensing_verdict: Option<String>,
}

impl client::Part for AccountDetails {}


/// Contains the application integrity information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppIntegrity {
    /// Required. Details about the app recognition verdict
    #[serde(rename="appRecognitionVerdict")]
    
    pub app_recognition_verdict: Option<String>,
    /// The SHA256 hash of the requesting app's signing certificates (base64 web-safe encoded). Set iff app_recognition_verdict != UNEVALUATED.
    #[serde(rename="certificateSha256Digest")]
    
    pub certificate_sha256_digest: Option<Vec<String>>,
    /// Package name of the application under attestation. Set iff app_recognition_verdict != UNEVALUATED.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// Version code of the application. Set iff app_recognition_verdict != UNEVALUATED.
    #[serde(rename="versionCode")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version_code: Option<i64>,
}

impl client::Part for AppIntegrity {}


/// Request to decode the integrity token.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [decode integrity token](MethodDecodeIntegrityTokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DecodeIntegrityTokenRequest {
    /// Encoded integrity token.
    #[serde(rename="integrityToken")]
    
    pub integrity_token: Option<String>,
}

impl client::RequestValue for DecodeIntegrityTokenRequest {}


/// Response containing the decoded integrity payload.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [decode integrity token](MethodDecodeIntegrityTokenCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DecodeIntegrityTokenResponse {
    /// Plain token payload generated from the decoded integrity token.
    #[serde(rename="tokenPayloadExternal")]
    
    pub token_payload_external: Option<TokenPayloadExternal>,
}

impl client::ResponseResult for DecodeIntegrityTokenResponse {}


/// Contains the device attestation information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceIntegrity {
    /// Details about the integrity of the device the app is running on
    #[serde(rename="deviceRecognitionVerdict")]
    
    pub device_recognition_verdict: Option<Vec<String>>,
}

impl client::Part for DeviceIntegrity {}


/// Contains the integrity request information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RequestDetails {
    /// Nonce that was provided in the request (which is base64 web-safe no-wrap).
    
    pub nonce: Option<String>,
    /// Request hash that was provided in the request.
    #[serde(rename="requestHash")]
    
    pub request_hash: Option<String>,
    /// Required. Application package name this attestation was requested for. Note: This field makes no guarantees or promises on the caller integrity. For details on application integrity, check application_integrity.
    #[serde(rename="requestPackageName")]
    
    pub request_package_name: Option<String>,
    /// Required. Timestamp, in milliseconds, of the integrity application request.
    #[serde(rename="timestampMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub timestamp_millis: Option<i64>,
}

impl client::Part for RequestDetails {}


/// Contains additional information generated for testing responses.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestingDetails {
    /// Required. Indicates that the information contained in this payload is a testing response that is statically overridden for a tester.
    #[serde(rename="isTestingResponse")]
    
    pub is_testing_response: Option<bool>,
}

impl client::Part for TestingDetails {}


/// Contains basic app information and integrity signals like device attestation and licensing details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TokenPayloadExternal {
    /// Required. Details about the Play Store account.
    #[serde(rename="accountDetails")]
    
    pub account_details: Option<AccountDetails>,
    /// Required. Details about the application integrity.
    #[serde(rename="appIntegrity")]
    
    pub app_integrity: Option<AppIntegrity>,
    /// Required. Details about the device integrity.
    #[serde(rename="deviceIntegrity")]
    
    pub device_integrity: Option<DeviceIntegrity>,
    /// Required. Details about the integrity request.
    #[serde(rename="requestDetails")]
    
    pub request_details: Option<RequestDetails>,
    /// Indicates that this payload is generated for testing purposes and contains any additional data that is linked with testing status.
    #[serde(rename="testingDetails")]
    
    pub testing_details: Option<TestingDetails>,
}

impl client::Part for TokenPayloadExternal {}


