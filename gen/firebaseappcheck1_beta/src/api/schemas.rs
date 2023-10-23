use super::*;
/// An app’s App Attest configuration object. This configuration controls certain properties of the `AppCheckToken` returned by ExchangeAppAttestAttestation and ExchangeAppAttestAssertion, such as its ttl. Note that the Team ID registered with your app is used as part of the validation process. Please register it via the Firebase Console or programmatically via the [Firebase Management Service](https://firebase.google.com/docs/projects/api/reference/rest/v1beta1/projects.iosApps/patch).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps app attest config get projects](ProjectAppAppAttestConfigGetCall) (response)
/// * [apps app attest config patch projects](ProjectAppAppAttestConfigPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaAppAttestConfig {
    /// Required. The relative resource name of the App Attest configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/appAttestConfig ```
    
    pub name: Option<String>,
    /// Specifies the duration for which App Check tokens exchanged from App Attest artifacts will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.
    #[serde(rename="tokenTtl")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub token_ttl: Option<client::chrono::Duration>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaAppAttestConfig {}
impl client::ResponseResult for GoogleFirebaseAppcheckV1betaAppAttestConfig {}


/// Encapsulates an *App Check token*, which are used to access Firebase services protected by App Check.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps exchange app attest assertion projects](ProjectAppExchangeAppAttestAssertionCall) (response)
/// * [apps exchange custom token projects](ProjectAppExchangeCustomTokenCall) (response)
/// * [apps exchange debug token projects](ProjectAppExchangeDebugTokenCall) (response)
/// * [apps exchange device check token projects](ProjectAppExchangeDeviceCheckTokenCall) (response)
/// * [apps exchange play integrity token projects](ProjectAppExchangePlayIntegrityTokenCall) (response)
/// * [apps exchange recaptcha enterprise token projects](ProjectAppExchangeRecaptchaEnterpriseTokenCall) (response)
/// * [apps exchange recaptcha token projects](ProjectAppExchangeRecaptchaTokenCall) (response)
/// * [apps exchange recaptcha v3 token projects](ProjectAppExchangeRecaptchaV3TokenCall) (response)
/// * [apps exchange safety net token projects](ProjectAppExchangeSafetyNetTokenCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaAppCheckToken {
    /// An App Check token. App Check tokens are signed [JWTs](https://tools.ietf.org/html/rfc7519) containing claims that identify the attested app and Firebase project. This token is used to access Firebase services protected by App Check.
    #[serde(rename="attestationToken")]
    
    pub attestation_token: Option<String>,
    /// An App Check token. App Check tokens are signed [JWTs](https://tools.ietf.org/html/rfc7519) containing claims that identify the attested app and Firebase project. This token is used to access Firebase services protected by App Check.
    
    pub token: Option<String>,
    /// The duration from the time this token is minted until its expiration. This field is intended to ease client-side token management, since the client may have clock skew, but is still able to accurately measure a duration.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub ttl: Option<client::chrono::Duration>,
}

impl client::ResponseResult for GoogleFirebaseAppcheckV1betaAppCheckToken {}


/// Encapsulates an *App Check token*, which are used to access Firebase services protected by App Check.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaAttestationTokenResponse {
    /// An App Check token. App Check tokens are signed [JWTs](https://tools.ietf.org/html/rfc7519) containing claims that identify the attested app and Firebase project. This token is used to access Firebase services protected by App Check.
    #[serde(rename="attestationToken")]
    
    pub attestation_token: Option<String>,
    /// The duration from the time this token is minted until its expiration. This field is intended to ease client-side token management, since the client may have clock skew, but is still able to accurately measure a duration.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub ttl: Option<client::chrono::Duration>,
}

impl client::Part for GoogleFirebaseAppcheckV1betaAttestationTokenResponse {}


/// Response message for the BatchGetAppAttestConfigs method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps app attest config batch get projects](ProjectAppAppAttestConfigBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaBatchGetAppAttestConfigsResponse {
    /// AppAttestConfigs retrieved.
    
    pub configs: Option<Vec<GoogleFirebaseAppcheckV1betaAppAttestConfig>>,
}

impl client::ResponseResult for GoogleFirebaseAppcheckV1betaBatchGetAppAttestConfigsResponse {}


/// Response message for the BatchGetDeviceCheckConfigs method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps device check config batch get projects](ProjectAppDeviceCheckConfigBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaBatchGetDeviceCheckConfigsResponse {
    /// DeviceCheckConfigs retrieved.
    
    pub configs: Option<Vec<GoogleFirebaseAppcheckV1betaDeviceCheckConfig>>,
}

impl client::ResponseResult for GoogleFirebaseAppcheckV1betaBatchGetDeviceCheckConfigsResponse {}


/// Response message for the BatchGetPlayIntegrityConfigs method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps play integrity config batch get projects](ProjectAppPlayIntegrityConfigBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaBatchGetPlayIntegrityConfigsResponse {
    /// PlayIntegrityConfigs retrieved.
    
    pub configs: Option<Vec<GoogleFirebaseAppcheckV1betaPlayIntegrityConfig>>,
}

impl client::ResponseResult for GoogleFirebaseAppcheckV1betaBatchGetPlayIntegrityConfigsResponse {}


/// Response message for the BatchGetRecaptchaConfigs method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps recaptcha config batch get projects](ProjectAppRecaptchaConfigBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaBatchGetRecaptchaConfigsResponse {
    /// RecaptchaConfigs retrieved.
    
    pub configs: Option<Vec<GoogleFirebaseAppcheckV1betaRecaptchaConfig>>,
}

impl client::ResponseResult for GoogleFirebaseAppcheckV1betaBatchGetRecaptchaConfigsResponse {}


/// Response message for the BatchGetRecaptchaEnterpriseConfigs method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps recaptcha enterprise config batch get projects](ProjectAppRecaptchaEnterpriseConfigBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaBatchGetRecaptchaEnterpriseConfigsResponse {
    /// RecaptchaEnterpriseConfigs retrieved.
    
    pub configs: Option<Vec<GoogleFirebaseAppcheckV1betaRecaptchaEnterpriseConfig>>,
}

impl client::ResponseResult for GoogleFirebaseAppcheckV1betaBatchGetRecaptchaEnterpriseConfigsResponse {}


/// Response message for the BatchGetRecaptchaV3Configs method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps recaptcha v3 config batch get projects](ProjectAppRecaptchaV3ConfigBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaBatchGetRecaptchaV3ConfigsResponse {
    /// RecaptchaV3Configs retrieved.
    
    pub configs: Option<Vec<GoogleFirebaseAppcheckV1betaRecaptchaV3Config>>,
}

impl client::ResponseResult for GoogleFirebaseAppcheckV1betaBatchGetRecaptchaV3ConfigsResponse {}


/// Response message for the BatchGetSafetyNetConfigs method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps safety net config batch get projects](ProjectAppSafetyNetConfigBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaBatchGetSafetyNetConfigsResponse {
    /// SafetyNetConfigs retrieved.
    
    pub configs: Option<Vec<GoogleFirebaseAppcheckV1betaSafetyNetConfig>>,
}

impl client::ResponseResult for GoogleFirebaseAppcheckV1betaBatchGetSafetyNetConfigsResponse {}


/// Request message for the BatchUpdateServices method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [services batch update projects](ProjectServiceBatchUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaBatchUpdateServicesRequest {
    /// Required. The request messages specifying the Services to update. A maximum of 100 objects can be updated in a batch.
    
    pub requests: Option<Vec<GoogleFirebaseAppcheckV1betaUpdateServiceRequest>>,
    /// Optional. A comma-separated list of names of fields in the Services to update. Example: `display_name`. If this field is present, the `update_mask` field in the UpdateServiceRequest messages must all match this field, or the entire batch fails and no updates will be committed.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaBatchUpdateServicesRequest {}


/// Response message for the BatchUpdateServices method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [services batch update projects](ProjectServiceBatchUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaBatchUpdateServicesResponse {
    /// Service objects after the updates have been applied.
    
    pub services: Option<Vec<GoogleFirebaseAppcheckV1betaService>>,
}

impl client::ResponseResult for GoogleFirebaseAppcheckV1betaBatchUpdateServicesResponse {}


/// A *debug token* is a secret used during the development or integration testing of an app. It essentially allows the development or integration testing to bypass app attestation while still allowing App Check to enforce protection on supported production Firebase services.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps debug tokens create projects](ProjectAppDebugTokenCreateCall) (request|response)
/// * [apps debug tokens get projects](ProjectAppDebugTokenGetCall) (response)
/// * [apps debug tokens patch projects](ProjectAppDebugTokenPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaDebugToken {
    /// Required. A human readable display name used to identify this debug token.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. The relative resource name of the debug token, in the format: ``` projects/{project_number}/apps/{app_id}/debugTokens/{debug_token_id} ```
    
    pub name: Option<String>,
    /// Required. Input only. Immutable. The secret token itself. Must be provided during creation, and must be a UUID4, case insensitive. This field is immutable once set, and cannot be provided during an UpdateDebugToken request. You can, however, delete this debug token using DeleteDebugToken to revoke it. For security reasons, this field will never be populated in any response.
    
    pub token: Option<String>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaDebugToken {}
impl client::ResponseResult for GoogleFirebaseAppcheckV1betaDebugToken {}


/// An app’s DeviceCheck configuration object. This configuration is used by ExchangeDeviceCheckToken to validate device tokens issued to apps by DeviceCheck. It also controls certain properties of the returned `AppCheckToken`, such as its ttl. Note that the Team ID registered with your app is used as part of the validation process. Please register it via the Firebase Console or programmatically via the [Firebase Management Service](https://firebase.google.com/docs/projects/api/reference/rest/v1beta1/projects.iosApps/patch).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps device check config get projects](ProjectAppDeviceCheckConfigGetCall) (response)
/// * [apps device check config patch projects](ProjectAppDeviceCheckConfigPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaDeviceCheckConfig {
    /// Required. The key identifier of a private key enabled with DeviceCheck, created in your Apple Developer account.
    #[serde(rename="keyId")]
    
    pub key_id: Option<String>,
    /// Required. The relative resource name of the DeviceCheck configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/deviceCheckConfig ```
    
    pub name: Option<String>,
    /// Required. Input only. The contents of the private key (`.p8`) file associated with the key specified by `key_id`. For security reasons, this field will never be populated in any response.
    #[serde(rename="privateKey")]
    
    pub private_key: Option<String>,
    /// Output only. Whether the `private_key` field was previously set. Since we will never return the `private_key` field, this field is the only way to find out whether it was previously set.
    #[serde(rename="privateKeySet")]
    
    pub private_key_set: Option<bool>,
    /// Specifies the duration for which App Check tokens exchanged from DeviceCheck tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.
    #[serde(rename="tokenTtl")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub token_ttl: Option<client::chrono::Duration>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaDeviceCheckConfig {}
impl client::ResponseResult for GoogleFirebaseAppcheckV1betaDeviceCheckConfig {}


/// Request message for the ExchangeAppAttestAssertion method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps exchange app attest assertion projects](ProjectAppExchangeAppAttestAssertionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaExchangeAppAttestAssertionRequest {
    /// Required. The artifact returned by a previous call to ExchangeAppAttestAttestation.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub artifact: Option<Vec<u8>>,
    /// Required. The CBOR-encoded assertion returned by the client-side App Attest API.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub assertion: Option<Vec<u8>>,
    /// Required. A one-time challenge returned by an immediately prior call to GenerateAppAttestChallenge.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub challenge: Option<Vec<u8>>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaExchangeAppAttestAssertionRequest {}


/// Request message for the ExchangeAppAttestAttestation method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps exchange app attest attestation projects](ProjectAppExchangeAppAttestAttestationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaExchangeAppAttestAttestationRequest {
    /// Required. The App Attest statement returned by the client-side App Attest API. This is a base64url encoded CBOR object in the JSON response.
    #[serde(rename="attestationStatement")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub attestation_statement: Option<Vec<u8>>,
    /// Required. A one-time challenge returned by an immediately prior call to GenerateAppAttestChallenge.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub challenge: Option<Vec<u8>>,
    /// Required. The key ID generated by App Attest for the client app.
    #[serde(rename="keyId")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub key_id: Option<Vec<u8>>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaExchangeAppAttestAttestationRequest {}


/// Response message for the ExchangeAppAttestAttestation method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps exchange app attest attestation projects](ProjectAppExchangeAppAttestAttestationCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaExchangeAppAttestAttestationResponse {
    /// Encapsulates an App Check token.
    #[serde(rename="appCheckToken")]
    
    pub app_check_token: Option<GoogleFirebaseAppcheckV1betaAppCheckToken>,
    /// An artifact that can be used in future calls to ExchangeAppAttestAssertion.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub artifact: Option<Vec<u8>>,
    /// Encapsulates an App Check token.
    #[serde(rename="attestationToken")]
    
    pub attestation_token: Option<GoogleFirebaseAppcheckV1betaAttestationTokenResponse>,
}

impl client::ResponseResult for GoogleFirebaseAppcheckV1betaExchangeAppAttestAttestationResponse {}


/// Request message for the ExchangeCustomToken method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps exchange custom token projects](ProjectAppExchangeCustomTokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaExchangeCustomTokenRequest {
    /// Required. A custom token signed using your project's Admin SDK service account credentials.
    #[serde(rename="customToken")]
    
    pub custom_token: Option<String>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaExchangeCustomTokenRequest {}


/// Request message for the ExchangeDebugToken method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps exchange debug token projects](ProjectAppExchangeDebugTokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaExchangeDebugTokenRequest {
    /// Required. A debug token secret. This string must match a debug token secret previously created using CreateDebugToken.
    #[serde(rename="debugToken")]
    
    pub debug_token: Option<String>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaExchangeDebugTokenRequest {}


/// Request message for the ExchangeDeviceCheckToken method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps exchange device check token projects](ProjectAppExchangeDeviceCheckTokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaExchangeDeviceCheckTokenRequest {
    /// Required. The `device_token` as returned by Apple's client-side [DeviceCheck API](https://developer.apple.com/documentation/devicecheck/dcdevice). This is the base64 encoded `Data` (Swift) or `NSData` (ObjC) object.
    #[serde(rename="deviceToken")]
    
    pub device_token: Option<String>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaExchangeDeviceCheckTokenRequest {}


/// Request message for the ExchangePlayIntegrityToken method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps exchange play integrity token projects](ProjectAppExchangePlayIntegrityTokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaExchangePlayIntegrityTokenRequest {
    /// Required. The [integrity verdict response token from Play Integrity](https://developer.android.com/google/play/integrity/verdict#decrypt-verify) issued to your app.
    #[serde(rename="playIntegrityToken")]
    
    pub play_integrity_token: Option<String>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaExchangePlayIntegrityTokenRequest {}


/// Request message for the ExchangeRecaptchaEnterpriseToken method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps exchange recaptcha enterprise token projects](ProjectAppExchangeRecaptchaEnterpriseTokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaExchangeRecaptchaEnterpriseTokenRequest {
    /// Required. The reCAPTCHA token as returned by the [reCAPTCHA Enterprise JavaScript API](https://cloud.google.com/recaptcha-enterprise/docs/instrument-web-pages).
    #[serde(rename="recaptchaEnterpriseToken")]
    
    pub recaptcha_enterprise_token: Option<String>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaExchangeRecaptchaEnterpriseTokenRequest {}


/// Request message for the ExchangeRecaptchaToken method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps exchange recaptcha token projects](ProjectAppExchangeRecaptchaTokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaExchangeRecaptchaTokenRequest {
    /// Required. The reCAPTCHA token as returned by the [reCAPTCHA v3 JavaScript API](https://developers.google.com/recaptcha/docs/v3).
    #[serde(rename="recaptchaToken")]
    
    pub recaptcha_token: Option<String>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaExchangeRecaptchaTokenRequest {}


/// Request message for the ExchangeRecaptchaV3Token method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps exchange recaptcha v3 token projects](ProjectAppExchangeRecaptchaV3TokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaExchangeRecaptchaV3TokenRequest {
    /// Required. The reCAPTCHA token as returned by the [reCAPTCHA v3 JavaScript API](https://developers.google.com/recaptcha/docs/v3).
    #[serde(rename="recaptchaV3Token")]
    
    pub recaptcha_v3_token: Option<String>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaExchangeRecaptchaV3TokenRequest {}


/// Request message for the ExchangeSafetyNetToken method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps exchange safety net token projects](ProjectAppExchangeSafetyNetTokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaExchangeSafetyNetTokenRequest {
    /// Required. The [SafetyNet attestation response](https://developer.android.com/training/safetynet/attestation#request-attestation-step) issued to your app.
    #[serde(rename="safetyNetToken")]
    
    pub safety_net_token: Option<String>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaExchangeSafetyNetTokenRequest {}


/// Request message for the GenerateAppAttestChallenge method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps generate app attest challenge projects](ProjectAppGenerateAppAttestChallengeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaGenerateAppAttestChallengeRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleFirebaseAppcheckV1betaGenerateAppAttestChallengeRequest {}


/// Response message for the GenerateAppAttestChallenge method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps generate app attest challenge projects](ProjectAppGenerateAppAttestChallengeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaGenerateAppAttestChallengeResponse {
    /// A one-time use challenge for the client to pass to the App Attest API.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub challenge: Option<Vec<u8>>,
    /// The duration from the time this challenge is minted until its expiration. This field is intended to ease client-side token management, since the client may have clock skew, but is still able to accurately measure a duration.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub ttl: Option<client::chrono::Duration>,
}

impl client::ResponseResult for GoogleFirebaseAppcheckV1betaGenerateAppAttestChallengeResponse {}


/// Request message for the GeneratePlayIntegrityChallenge method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps generate play integrity challenge projects](ProjectAppGeneratePlayIntegrityChallengeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaGeneratePlayIntegrityChallengeRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleFirebaseAppcheckV1betaGeneratePlayIntegrityChallengeRequest {}


/// Response message for the GeneratePlayIntegrityChallenge method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps generate play integrity challenge projects](ProjectAppGeneratePlayIntegrityChallengeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaGeneratePlayIntegrityChallengeResponse {
    /// A one-time use [challenge](https://developer.android.com/google/play/integrity/verdict#protect-against-replay-attacks) for the client to pass to the Play Integrity API.
    
    pub challenge: Option<String>,
    /// The duration from the time this challenge is minted until its expiration. This field is intended to ease client-side token management, since the client may have clock skew, but is still able to accurately measure a duration.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub ttl: Option<client::chrono::Duration>,
}

impl client::ResponseResult for GoogleFirebaseAppcheckV1betaGeneratePlayIntegrityChallengeResponse {}


/// Response message for the ListDebugTokens method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps debug tokens list projects](ProjectAppDebugTokenListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaListDebugTokensResponse {
    /// The DebugTokens retrieved.
    #[serde(rename="debugTokens")]
    
    pub debug_tokens: Option<Vec<GoogleFirebaseAppcheckV1betaDebugToken>>,
    /// If the result list is too large to fit in a single response, then a token is returned. If the string is empty or omitted, then this response is the last page of results. This token can be used in a subsequent call to ListDebugTokens to find the next group of DebugTokens. Page tokens are short-lived and should not be persisted.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleFirebaseAppcheckV1betaListDebugTokensResponse {}


/// Response message for the ListServices method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [services list projects](ProjectServiceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaListServicesResponse {
    /// If the result list is too large to fit in a single response, then a token is returned. If the string is empty or omitted, then this response is the last page of results. This token can be used in a subsequent call to ListServices to find the next group of Services. Page tokens are short-lived and should not be persisted.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The Services retrieved.
    
    pub services: Option<Vec<GoogleFirebaseAppcheckV1betaService>>,
}

impl client::ResponseResult for GoogleFirebaseAppcheckV1betaListServicesResponse {}


/// An app’s Play Integrity configuration object. This configuration controls certain properties of the `AppCheckToken` returned by ExchangePlayIntegrityToken, such as its ttl. Note that your registered SHA-256 certificate fingerprints are used to validate tokens issued by the Play Integrity API; please register them via the Firebase Console or programmatically via the [Firebase Management Service](https://firebase.google.com/docs/projects/api/reference/rest/v1beta1/projects.androidApps.sha/create).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps play integrity config get projects](ProjectAppPlayIntegrityConfigGetCall) (response)
/// * [apps play integrity config patch projects](ProjectAppPlayIntegrityConfigPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaPlayIntegrityConfig {
    /// Required. The relative resource name of the Play Integrity configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/playIntegrityConfig ```
    
    pub name: Option<String>,
    /// Specifies the duration for which App Check tokens exchanged from Play Integrity tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.
    #[serde(rename="tokenTtl")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub token_ttl: Option<client::chrono::Duration>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaPlayIntegrityConfig {}
impl client::ResponseResult for GoogleFirebaseAppcheckV1betaPlayIntegrityConfig {}


/// A JWK as specified by [section 4 of RFC 7517](https://tools.ietf.org/html/rfc7517#section-4) and [section 6.3.1 of RFC 7518](https://tools.ietf.org/html/rfc7518#section-6.3.1).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaPublicJwk {
    /// See [section 4.4 of RFC 7517](https://tools.ietf.org/html/rfc7517#section-4.4).
    
    pub alg: Option<String>,
    /// See [section 6.3.1.2 of RFC 7518](https://tools.ietf.org/html/rfc7518#section-6.3.1.2).
    
    pub e: Option<String>,
    /// See [section 4.5 of RFC 7517](https://tools.ietf.org/html/rfc7517#section-4.5).
    
    pub kid: Option<String>,
    /// See [section 4.1 of RFC 7517](https://tools.ietf.org/html/rfc7517#section-4.1).
    
    pub kty: Option<String>,
    /// See [section 6.3.1.1 of RFC 7518](https://tools.ietf.org/html/rfc7518#section-6.3.1.1).
    
    pub n: Option<String>,
    /// See [section 4.2 of RFC 7517](https://tools.ietf.org/html/rfc7517#section-4.2).
    #[serde(rename="use")]
    
    pub use_: Option<String>,
}

impl client::Part for GoogleFirebaseAppcheckV1betaPublicJwk {}


/// The currently active set of public keys that can be used to verify App Check tokens. This object is a JWK set as specified by [section 5 of RFC 7517](https://tools.ietf.org/html/rfc7517#section-5). For security, the response **must not** be cached for longer than six hours.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get jwks](JwkGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaPublicJwkSet {
    /// The set of public keys. See [section 5.1 of RFC 7517](https://tools.ietf.org/html/rfc7517#section-5).
    
    pub keys: Option<Vec<GoogleFirebaseAppcheckV1betaPublicJwk>>,
}

impl client::ResponseResult for GoogleFirebaseAppcheckV1betaPublicJwkSet {}


/// An app’s reCAPTCHA v3 configuration object. This configuration is used by ExchangeRecaptchaToken to validate reCAPTCHA tokens issued to apps by reCAPTCHA v3. It also controls certain properties of the returned `AppCheckToken`, such as its ttl.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps recaptcha config get projects](ProjectAppRecaptchaConfigGetCall) (response)
/// * [apps recaptcha config patch projects](ProjectAppRecaptchaConfigPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaRecaptchaConfig {
    /// Required. The relative resource name of the reCAPTCHA v3 configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaConfig ```
    
    pub name: Option<String>,
    /// Required. Input only. The site secret used to identify your service for reCAPTCHA v3 verification. For security reasons, this field will never be populated in any response.
    #[serde(rename="siteSecret")]
    
    pub site_secret: Option<String>,
    /// Output only. Whether the `site_secret` field was previously set. Since we will never return the `site_secret` field, this field is the only way to find out whether it was previously set.
    #[serde(rename="siteSecretSet")]
    
    pub site_secret_set: Option<bool>,
    /// Specifies the duration for which App Check tokens exchanged from reCAPTCHA tokens will be valid. If unset, a default value of 1 day is assumed. Must be between 30 minutes and 7 days, inclusive.
    #[serde(rename="tokenTtl")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub token_ttl: Option<client::chrono::Duration>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaRecaptchaConfig {}
impl client::ResponseResult for GoogleFirebaseAppcheckV1betaRecaptchaConfig {}


/// An app’s reCAPTCHA Enterprise configuration object. This configuration is used by ExchangeRecaptchaEnterpriseToken to validate reCAPTCHA tokens issued to apps by reCAPTCHA Enterprise. It also controls certain properties of the returned `AppCheckToken`, such as its ttl.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps recaptcha enterprise config get projects](ProjectAppRecaptchaEnterpriseConfigGetCall) (response)
/// * [apps recaptcha enterprise config patch projects](ProjectAppRecaptchaEnterpriseConfigPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaRecaptchaEnterpriseConfig {
    /// Required. The relative resource name of the reCAPTCHA Enterprise configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaEnterpriseConfig ```
    
    pub name: Option<String>,
    /// The score-based site key [created in reCAPTCHA Enterprise](https://cloud.google.com/recaptcha-enterprise/docs/create-key#creating_a_site_key) used to [invoke reCAPTCHA and generate the reCAPTCHA tokens](https://cloud.google.com/recaptcha-enterprise/docs/instrument-web-pages) for your application. Important: This is *not* the `site_secret` (as it is in reCAPTCHA v3), but rather your score-based reCAPTCHA Enterprise site key.
    #[serde(rename="siteKey")]
    
    pub site_key: Option<String>,
    /// Specifies the duration for which App Check tokens exchanged from reCAPTCHA Enterprise tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.
    #[serde(rename="tokenTtl")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub token_ttl: Option<client::chrono::Duration>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaRecaptchaEnterpriseConfig {}
impl client::ResponseResult for GoogleFirebaseAppcheckV1betaRecaptchaEnterpriseConfig {}


/// An app’s reCAPTCHA v3 configuration object. This configuration is used by ExchangeRecaptchaV3Token to validate reCAPTCHA tokens issued to apps by reCAPTCHA v3. It also controls certain properties of the returned `AppCheckToken`, such as its ttl.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps recaptcha v3 config get projects](ProjectAppRecaptchaV3ConfigGetCall) (response)
/// * [apps recaptcha v3 config patch projects](ProjectAppRecaptchaV3ConfigPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaRecaptchaV3Config {
    /// Required. The relative resource name of the reCAPTCHA v3 configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaV3Config ```
    
    pub name: Option<String>,
    /// Required. Input only. The site secret used to identify your service for reCAPTCHA v3 verification. For security reasons, this field will never be populated in any response.
    #[serde(rename="siteSecret")]
    
    pub site_secret: Option<String>,
    /// Output only. Whether the `site_secret` field was previously set. Since we will never return the `site_secret` field, this field is the only way to find out whether it was previously set.
    #[serde(rename="siteSecretSet")]
    
    pub site_secret_set: Option<bool>,
    /// Specifies the duration for which App Check tokens exchanged from reCAPTCHA tokens will be valid. If unset, a default value of 1 day is assumed. Must be between 30 minutes and 7 days, inclusive.
    #[serde(rename="tokenTtl")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub token_ttl: Option<client::chrono::Duration>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaRecaptchaV3Config {}
impl client::ResponseResult for GoogleFirebaseAppcheckV1betaRecaptchaV3Config {}


/// An app’s SafetyNet configuration object. This configuration controls certain properties of the `AppCheckToken` returned by ExchangeSafetyNetToken, such as its ttl. Note that your registered SHA-256 certificate fingerprints are used to validate tokens issued by SafetyNet; please register them via the Firebase Console or programmatically via the [Firebase Management Service](https://firebase.google.com/docs/projects/api/reference/rest/v1beta1/projects.androidApps.sha/create).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps safety net config get projects](ProjectAppSafetyNetConfigGetCall) (response)
/// * [apps safety net config patch projects](ProjectAppSafetyNetConfigPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaSafetyNetConfig {
    /// Required. The relative resource name of the SafetyNet configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/safetyNetConfig ```
    
    pub name: Option<String>,
    /// Specifies the duration for which App Check tokens exchanged from SafetyNet tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.
    #[serde(rename="tokenTtl")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub token_ttl: Option<client::chrono::Duration>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaSafetyNetConfig {}
impl client::ResponseResult for GoogleFirebaseAppcheckV1betaSafetyNetConfig {}


/// The enforcement configuration for a Firebase service supported by App Check.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [services get projects](ProjectServiceGetCall) (response)
/// * [services patch projects](ProjectServicePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaService {
    /// Required. The App Check enforcement mode for this service.
    #[serde(rename="enforcementMode")]
    
    pub enforcement_mode: Option<GoogleFirebaseAppcheckV1betaServiceEnforcementModeEnum>,
    /// Required. The relative resource name of the service configuration object, in the format: ``` projects/{project_number}/services/{service_id} ``` Note that the `service_id` element must be a supported service ID. Currently, the following service IDs are supported: * `firebasestorage.googleapis.com` (Cloud Storage for Firebase) * `firebasedatabase.googleapis.com` (Firebase Realtime Database) * `firestore.googleapis.com` (Cloud Firestore)
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleFirebaseAppcheckV1betaService {}
impl client::ResponseResult for GoogleFirebaseAppcheckV1betaService {}


/// Request message for the UpdateService method as well as an individual update message for the BatchUpdateServices method.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseAppcheckV1betaUpdateServiceRequest {
    /// Required. The Service to update. The Service's `name` field is used to identify the Service to be updated, in the format: ``` projects/{project_number}/services/{service_id} ``` Note that the `service_id` element must be a supported service ID. Currently, the following service IDs are supported: * `firebasestorage.googleapis.com` (Cloud Storage for Firebase) * `firebasedatabase.googleapis.com` (Firebase Realtime Database) * `firestore.googleapis.com` (Cloud Firestore)
    
    pub service: Option<GoogleFirebaseAppcheckV1betaService>,
    /// Required. A comma-separated list of names of fields in the Service to update. Example: `enforcement_mode`.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::Part for GoogleFirebaseAppcheckV1betaUpdateServiceRequest {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps debug tokens delete projects](ProjectAppDebugTokenDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


