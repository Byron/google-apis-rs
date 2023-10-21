use super::*;
/// Response of creating the IDP authentication URL.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create auth uri relyingparty](RelyingpartyCreateAuthUriCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateAuthUriResponse {
    /// all providers the user has once used to do federated login
    #[serde(rename="allProviders")]
    
    pub all_providers: Option<Vec<String>>,
    /// The URI used by the IDP to authenticate the user.
    #[serde(rename="authUri")]
    
    pub auth_uri: Option<String>,
    /// True if captcha is required.
    #[serde(rename="captchaRequired")]
    
    pub captcha_required: Option<bool>,
    /// True if the authUri is for user's existing provider.
    #[serde(rename="forExistingProvider")]
    
    pub for_existing_provider: Option<bool>,
    /// The fixed string identitytoolkit#CreateAuthUriResponse".
    
    pub kind: Option<String>,
    /// The provider ID of the auth URI.
    #[serde(rename="providerId")]
    
    pub provider_id: Option<String>,
    /// Whether the user is registered if the identifier is an email.
    
    pub registered: Option<bool>,
    /// Session ID which should be passed in the following verifyAssertion request.
    #[serde(rename="sessionId")]
    
    pub session_id: Option<String>,
    /// All sign-in methods this user has used.
    #[serde(rename="signinMethods")]
    
    pub signin_methods: Option<Vec<String>>,
}

impl client::ResponseResult for CreateAuthUriResponse {}


/// Respone of deleting account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete account relyingparty](RelyingpartyDeleteAccountCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteAccountResponse {
    /// The fixed string "identitytoolkit#DeleteAccountResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for DeleteAccountResponse {}


/// Response of downloading accounts in batch.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [download account relyingparty](RelyingpartyDownloadAccountCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DownloadAccountResponse {
    /// The fixed string "identitytoolkit#DownloadAccountResponse".
    
    pub kind: Option<String>,
    /// The next page token. To be used in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The user accounts data.
    
    pub users: Option<Vec<UserInfo>>,
}

impl client::ResponseResult for DownloadAccountResponse {}


/// Response of email signIn.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [email link signin relyingparty](RelyingpartyEmailLinkSigninCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmailLinkSigninResponse {
    /// The user's email.
    
    pub email: Option<String>,
    /// Expiration time of STS id token in seconds.
    #[serde(rename="expiresIn")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expires_in: Option<i64>,
    /// The STS id token to login the newly signed in user.
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// Whether the user is new.
    #[serde(rename="isNewUser")]
    
    pub is_new_user: Option<bool>,
    /// The fixed string "identitytoolkit#EmailLinkSigninResponse".
    
    pub kind: Option<String>,
    /// The RP local ID of the user.
    #[serde(rename="localId")]
    
    pub local_id: Option<String>,
    /// The refresh token for the signed in user.
    #[serde(rename="refreshToken")]
    
    pub refresh_token: Option<String>,
}

impl client::ResponseResult for EmailLinkSigninResponse {}


/// Template for an email template.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmailTemplate {
    /// Email body.
    
    pub body: Option<String>,
    /// Email body format.
    
    pub format: Option<String>,
    /// From address of the email.
    
    pub from: Option<String>,
    /// From display name.
    #[serde(rename="fromDisplayName")]
    
    pub from_display_name: Option<String>,
    /// Reply-to address.
    #[serde(rename="replyTo")]
    
    pub reply_to: Option<String>,
    /// Subject of the email.
    
    pub subject: Option<String>,
}

impl client::Part for EmailTemplate {}


/// Response of getting account information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get account info relyingparty](RelyingpartyGetAccountInfoCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetAccountInfoResponse {
    /// The fixed string "identitytoolkit#GetAccountInfoResponse".
    
    pub kind: Option<String>,
    /// The info of the users.
    
    pub users: Option<Vec<UserInfo>>,
}

impl client::ResponseResult for GetAccountInfoResponse {}


/// Response of getting a code for user confirmation (reset password, change email etc.).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get oob confirmation code relyingparty](RelyingpartyGetOobConfirmationCodeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetOobConfirmationCodeResponse {
    /// The email address that the email is sent to.
    
    pub email: Option<String>,
    /// The fixed string "identitytoolkit#GetOobConfirmationCodeResponse".
    
    pub kind: Option<String>,
    /// The code to be send to the user.
    #[serde(rename="oobCode")]
    
    pub oob_code: Option<String>,
}

impl client::ResponseResult for GetOobConfirmationCodeResponse {}


/// Response of getting recaptcha param.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get recaptcha param relyingparty](RelyingpartyGetRecaptchaParamCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetRecaptchaParamResponse {
    /// The fixed string "identitytoolkit#GetRecaptchaParamResponse".
    
    pub kind: Option<String>,
    /// Site key registered at recaptcha.
    #[serde(rename="recaptchaSiteKey")]
    
    pub recaptcha_site_key: Option<String>,
    /// The stoken field for the recaptcha widget, used to request captcha challenge.
    #[serde(rename="recaptchaStoken")]
    
    pub recaptcha_stoken: Option<String>,
}

impl client::ResponseResult for GetRecaptchaParamResponse {}


/// Request to get the IDP authentication URL.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create auth uri relyingparty](RelyingpartyCreateAuthUriCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyCreateAuthUriRequest {
    /// The app ID of the mobile app, base64(CERT_SHA1):PACKAGE_NAME for Android, BUNDLE_ID for iOS.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// Explicitly specify the auth flow type. Currently only support "CODE_FLOW" type. The field is only used for Google provider.
    #[serde(rename="authFlowType")]
    
    pub auth_flow_type: Option<String>,
    /// The relying party OAuth client ID.
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// The opaque value used by the client to maintain context info between the authentication request and the IDP callback.
    
    pub context: Option<String>,
    /// The URI to which the IDP redirects the user after the federated login flow.
    #[serde(rename="continueUri")]
    
    pub continue_uri: Option<String>,
    /// The query parameter that client can customize by themselves in auth url. The following parameters are reserved for server so that they cannot be customized by clients: client_id, response_type, scope, redirect_uri, state, oauth_token.
    #[serde(rename="customParameter")]
    
    pub custom_parameter: Option<HashMap<String, String>>,
    /// The hosted domain to restrict sign-in to accounts at that domain for Google Apps hosted accounts.
    #[serde(rename="hostedDomain")]
    
    pub hosted_domain: Option<String>,
    /// The email or federated ID of the user.
    
    pub identifier: Option<String>,
    /// The developer's consumer key for OpenId OAuth Extension
    #[serde(rename="oauthConsumerKey")]
    
    pub oauth_consumer_key: Option<String>,
    /// Additional oauth scopes, beyond the basid user profile, that the user would be prompted to grant
    #[serde(rename="oauthScope")]
    
    pub oauth_scope: Option<String>,
    /// Optional realm for OpenID protocol. The sub string "scheme://domain:port" of the param "continueUri" is used if this is not set.
    #[serde(rename="openidRealm")]
    
    pub openid_realm: Option<String>,
    /// The native app package for OTA installation.
    #[serde(rename="otaApp")]
    
    pub ota_app: Option<String>,
    /// The IdP ID. For white listed IdPs it's a short domain name e.g. google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it's the OP identifier.
    #[serde(rename="providerId")]
    
    pub provider_id: Option<String>,
    /// The session_id passed by client.
    #[serde(rename="sessionId")]
    
    pub session_id: Option<String>,
    /// For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from.
    #[serde(rename="tenantId")]
    
    pub tenant_id: Option<String>,
    /// Tenant project number to be used for idp discovery.
    #[serde(rename="tenantProjectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub tenant_project_number: Option<u64>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartyCreateAuthUriRequest {}


/// Request to delete account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete account relyingparty](RelyingpartyDeleteAccountCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyDeleteAccountRequest {
    /// GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration.
    #[serde(rename="delegatedProjectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub delegated_project_number: Option<i64>,
    /// The GITKit token or STS id token of the authenticated user.
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// The local ID of the user.
    #[serde(rename="localId")]
    
    pub local_id: Option<String>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartyDeleteAccountRequest {}


/// Request to download user account in batch.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [download account relyingparty](RelyingpartyDownloadAccountCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyDownloadAccountRequest {
    /// GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration.
    #[serde(rename="delegatedProjectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub delegated_project_number: Option<i64>,
    /// The max number of results to return in the response.
    #[serde(rename="maxResults")]
    
    pub max_results: Option<u32>,
    /// The token for the next page. This should be taken from the previous response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Specify which project (field value is actually project id) to operate. Only used when provided credential.
    #[serde(rename="targetProjectId")]
    
    pub target_project_id: Option<String>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartyDownloadAccountRequest {}


/// Request to sign in with email.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [email link signin relyingparty](RelyingpartyEmailLinkSigninCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyEmailLinkSigninRequest {
    /// The email address of the user.
    
    pub email: Option<String>,
    /// Token for linking flow.
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// The confirmation code.
    #[serde(rename="oobCode")]
    
    pub oob_code: Option<String>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartyEmailLinkSigninRequest {}


/// Request to get the account information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get account info relyingparty](RelyingpartyGetAccountInfoCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyGetAccountInfoRequest {
    /// GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration.
    #[serde(rename="delegatedProjectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub delegated_project_number: Option<i64>,
    /// The list of emails of the users to inquiry.
    
    pub email: Option<Vec<String>>,
    /// The GITKit token of the authenticated user.
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// The list of local ID's of the users to inquiry.
    #[serde(rename="localId")]
    
    pub local_id: Option<Vec<String>>,
    /// Privileged caller can query users by specified phone number.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<Vec<String>>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartyGetAccountInfoRequest {}


/// Response of getting the project configuration.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get project config relyingparty](RelyingpartyGetProjectConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyGetProjectConfigResponse {
    /// Whether to allow password user sign in or sign up.
    #[serde(rename="allowPasswordUser")]
    
    pub allow_password_user: Option<bool>,
    /// Browser API key, needed when making http request to Apiary.
    #[serde(rename="apiKey")]
    
    pub api_key: Option<String>,
    /// Authorized domains.
    #[serde(rename="authorizedDomains")]
    
    pub authorized_domains: Option<Vec<String>>,
    /// Change email template.
    #[serde(rename="changeEmailTemplate")]
    
    pub change_email_template: Option<EmailTemplate>,
    /// no description provided
    #[serde(rename="dynamicLinksDomain")]
    
    pub dynamic_links_domain: Option<String>,
    /// Whether anonymous user is enabled.
    #[serde(rename="enableAnonymousUser")]
    
    pub enable_anonymous_user: Option<bool>,
    /// OAuth2 provider configuration.
    #[serde(rename="idpConfig")]
    
    pub idp_config: Option<Vec<IdpConfig>>,
    /// Legacy reset password email template.
    #[serde(rename="legacyResetPasswordTemplate")]
    
    pub legacy_reset_password_template: Option<EmailTemplate>,
    /// Project ID of the relying party.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Reset password email template.
    #[serde(rename="resetPasswordTemplate")]
    
    pub reset_password_template: Option<EmailTemplate>,
    /// Whether to use email sending provided by Firebear.
    #[serde(rename="useEmailSending")]
    
    pub use_email_sending: Option<bool>,
    /// Verify email template.
    #[serde(rename="verifyEmailTemplate")]
    
    pub verify_email_template: Option<EmailTemplate>,
}

impl client::ResponseResult for IdentitytoolkitRelyingpartyGetProjectConfigResponse {}


/// Respone of getting public keys.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get public keys relyingparty](RelyingpartyGetPublicKeyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyGetPublicKeysResponse(pub Option<HashMap<String, String>>);

impl client::ResponseResult for IdentitytoolkitRelyingpartyGetPublicKeysResponse {}


/// Request to reset the password.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reset password relyingparty](RelyingpartyResetPasswordCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyResetPasswordRequest {
    /// The email address of the user.
    
    pub email: Option<String>,
    /// The new password inputted by the user.
    #[serde(rename="newPassword")]
    
    pub new_password: Option<String>,
    /// The old password inputted by the user.
    #[serde(rename="oldPassword")]
    
    pub old_password: Option<String>,
    /// The confirmation code.
    #[serde(rename="oobCode")]
    
    pub oob_code: Option<String>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartyResetPasswordRequest {}


/// Request for Identitytoolkit-SendVerificationCode
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [send verification code relyingparty](RelyingpartySendVerificationCodeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartySendVerificationCodeRequest {
    /// Receipt of successful app token validation with APNS.
    #[serde(rename="iosReceipt")]
    
    pub ios_receipt: Option<String>,
    /// Secret delivered to iOS app via APNS.
    #[serde(rename="iosSecret")]
    
    pub ios_secret: Option<String>,
    /// The phone number to send the verification code to in E.164 format.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// Recaptcha solution.
    #[serde(rename="recaptchaToken")]
    
    pub recaptcha_token: Option<String>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartySendVerificationCodeRequest {}


/// Response for Identitytoolkit-SendVerificationCode
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [send verification code relyingparty](RelyingpartySendVerificationCodeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartySendVerificationCodeResponse {
    /// Encrypted session information
    #[serde(rename="sessionInfo")]
    
    pub session_info: Option<String>,
}

impl client::ResponseResult for IdentitytoolkitRelyingpartySendVerificationCodeResponse {}


/// Request to set the account information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set account info relyingparty](RelyingpartySetAccountInfoCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartySetAccountInfoRequest {
    /// The captcha challenge.
    #[serde(rename="captchaChallenge")]
    
    pub captcha_challenge: Option<String>,
    /// Response to the captcha.
    #[serde(rename="captchaResponse")]
    
    pub captcha_response: Option<String>,
    /// The timestamp when the account is created.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// The custom attributes to be set in the user's id token.
    #[serde(rename="customAttributes")]
    
    pub custom_attributes: Option<String>,
    /// GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration.
    #[serde(rename="delegatedProjectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub delegated_project_number: Option<i64>,
    /// The attributes users request to delete.
    #[serde(rename="deleteAttribute")]
    
    pub delete_attribute: Option<Vec<String>>,
    /// The IDPs the user request to delete.
    #[serde(rename="deleteProvider")]
    
    pub delete_provider: Option<Vec<String>>,
    /// Whether to disable the user.
    #[serde(rename="disableUser")]
    
    pub disable_user: Option<bool>,
    /// The name of the user.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The email of the user.
    
    pub email: Option<String>,
    /// Mark the email as verified or not.
    #[serde(rename="emailVerified")]
    
    pub email_verified: Option<bool>,
    /// The GITKit token of the authenticated user.
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// Instance id token of the app.
    #[serde(rename="instanceId")]
    
    pub instance_id: Option<String>,
    /// Last login timestamp.
    #[serde(rename="lastLoginAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_login_at: Option<i64>,
    /// The local ID of the user.
    #[serde(rename="localId")]
    
    pub local_id: Option<String>,
    /// The out-of-band code of the change email request.
    #[serde(rename="oobCode")]
    
    pub oob_code: Option<String>,
    /// The new password of the user.
    
    pub password: Option<String>,
    /// Privileged caller can update user with specified phone number.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// The photo url of the user.
    #[serde(rename="photoUrl")]
    
    pub photo_url: Option<String>,
    /// The associated IDPs of the user.
    
    pub provider: Option<Vec<String>>,
    /// Whether return sts id token and refresh token instead of gitkit token.
    #[serde(rename="returnSecureToken")]
    
    pub return_secure_token: Option<bool>,
    /// Mark the user to upgrade to federated login.
    #[serde(rename="upgradeToFederatedLogin")]
    
    pub upgrade_to_federated_login: Option<bool>,
    /// Timestamp in seconds for valid login token.
    #[serde(rename="validSince")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub valid_since: Option<i64>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartySetAccountInfoRequest {}


/// Request to set the project configuration.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set project config relyingparty](RelyingpartySetProjectConfigCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartySetProjectConfigRequest {
    /// Whether to allow password user sign in or sign up.
    #[serde(rename="allowPasswordUser")]
    
    pub allow_password_user: Option<bool>,
    /// Browser API key, needed when making http request to Apiary.
    #[serde(rename="apiKey")]
    
    pub api_key: Option<String>,
    /// Authorized domains for widget redirect.
    #[serde(rename="authorizedDomains")]
    
    pub authorized_domains: Option<Vec<String>>,
    /// Change email template.
    #[serde(rename="changeEmailTemplate")]
    
    pub change_email_template: Option<EmailTemplate>,
    /// GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration.
    #[serde(rename="delegatedProjectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub delegated_project_number: Option<i64>,
    /// Whether to enable anonymous user.
    #[serde(rename="enableAnonymousUser")]
    
    pub enable_anonymous_user: Option<bool>,
    /// Oauth2 provider configuration.
    #[serde(rename="idpConfig")]
    
    pub idp_config: Option<Vec<IdpConfig>>,
    /// Legacy reset password email template.
    #[serde(rename="legacyResetPasswordTemplate")]
    
    pub legacy_reset_password_template: Option<EmailTemplate>,
    /// Reset password email template.
    #[serde(rename="resetPasswordTemplate")]
    
    pub reset_password_template: Option<EmailTemplate>,
    /// Whether to use email sending provided by Firebear.
    #[serde(rename="useEmailSending")]
    
    pub use_email_sending: Option<bool>,
    /// Verify email template.
    #[serde(rename="verifyEmailTemplate")]
    
    pub verify_email_template: Option<EmailTemplate>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartySetProjectConfigRequest {}


/// Response of setting the project configuration.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set project config relyingparty](RelyingpartySetProjectConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartySetProjectConfigResponse {
    /// Project ID of the relying party.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::ResponseResult for IdentitytoolkitRelyingpartySetProjectConfigResponse {}


/// Request to sign out user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sign out user relyingparty](RelyingpartySignOutUserCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartySignOutUserRequest {
    /// Instance id token of the app.
    #[serde(rename="instanceId")]
    
    pub instance_id: Option<String>,
    /// The local ID of the user.
    #[serde(rename="localId")]
    
    pub local_id: Option<String>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartySignOutUserRequest {}


/// Response of signing out user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sign out user relyingparty](RelyingpartySignOutUserCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartySignOutUserResponse {
    /// The local ID of the user.
    #[serde(rename="localId")]
    
    pub local_id: Option<String>,
}

impl client::ResponseResult for IdentitytoolkitRelyingpartySignOutUserResponse {}


/// Request to signup new user, create anonymous user or anonymous user reauth.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [signup new user relyingparty](RelyingpartySignupNewUserCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartySignupNewUserRequest {
    /// The captcha challenge.
    #[serde(rename="captchaChallenge")]
    
    pub captcha_challenge: Option<String>,
    /// Response to the captcha.
    #[serde(rename="captchaResponse")]
    
    pub captcha_response: Option<String>,
    /// Whether to disable the user. Only can be used by service account.
    
    pub disabled: Option<bool>,
    /// The name of the user.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The email of the user.
    
    pub email: Option<String>,
    /// Mark the email as verified or not. Only can be used by service account.
    #[serde(rename="emailVerified")]
    
    pub email_verified: Option<bool>,
    /// The GITKit token of the authenticated user.
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// Instance id token of the app.
    #[serde(rename="instanceId")]
    
    pub instance_id: Option<String>,
    /// Privileged caller can create user with specified user id.
    #[serde(rename="localId")]
    
    pub local_id: Option<String>,
    /// The new password of the user.
    
    pub password: Option<String>,
    /// Privileged caller can create user with specified phone number.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// The photo url of the user.
    #[serde(rename="photoUrl")]
    
    pub photo_url: Option<String>,
    /// For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from.
    #[serde(rename="tenantId")]
    
    pub tenant_id: Option<String>,
    /// Tenant project number to be used for idp discovery.
    #[serde(rename="tenantProjectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub tenant_project_number: Option<u64>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartySignupNewUserRequest {}


/// Request to upload user account in batch.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [upload account relyingparty](RelyingpartyUploadAccountCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyUploadAccountRequest {
    /// Whether allow overwrite existing account when user local_id exists.
    #[serde(rename="allowOverwrite")]
    
    pub allow_overwrite: Option<bool>,
    /// no description provided
    #[serde(rename="blockSize")]
    
    pub block_size: Option<i32>,
    /// The following 4 fields are for standard scrypt algorithm.
    #[serde(rename="cpuMemCost")]
    
    pub cpu_mem_cost: Option<i32>,
    /// GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration.
    #[serde(rename="delegatedProjectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub delegated_project_number: Option<i64>,
    /// no description provided
    #[serde(rename="dkLen")]
    
    pub dk_len: Option<i32>,
    /// The password hash algorithm.
    #[serde(rename="hashAlgorithm")]
    
    pub hash_algorithm: Option<String>,
    /// Memory cost for hash calculation. Used by scrypt similar algorithms.
    #[serde(rename="memoryCost")]
    
    pub memory_cost: Option<i32>,
    /// no description provided
    
    pub parallelization: Option<i32>,
    /// Rounds for hash calculation. Used by scrypt and similar algorithms.
    
    pub rounds: Option<i32>,
    /// The salt separator.
    #[serde(rename="saltSeparator")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub salt_separator: Option<Vec<u8>>,
    /// If true, backend will do sanity check(including duplicate email and federated id) when uploading account.
    #[serde(rename="sanityCheck")]
    
    pub sanity_check: Option<bool>,
    /// The key for to hash the password.
    #[serde(rename="signerKey")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub signer_key: Option<Vec<u8>>,
    /// Specify which project (field value is actually project id) to operate. Only used when provided credential.
    #[serde(rename="targetProjectId")]
    
    pub target_project_id: Option<String>,
    /// The account info to be stored.
    
    pub users: Option<Vec<UserInfo>>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartyUploadAccountRequest {}


/// Request to verify the IDP assertion.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify assertion relyingparty](RelyingpartyVerifyAssertionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyVerifyAssertionRequest {
    /// When it's true, automatically creates a new account if the user doesn't exist. When it's false, allows existing user to sign in normally and throws exception if the user doesn't exist.
    #[serde(rename="autoCreate")]
    
    pub auto_create: Option<bool>,
    /// GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration.
    #[serde(rename="delegatedProjectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub delegated_project_number: Option<i64>,
    /// The GITKit token of the authenticated user.
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// Instance id token of the app.
    #[serde(rename="instanceId")]
    
    pub instance_id: Option<String>,
    /// The GITKit token for the non-trusted IDP pending to be confirmed by the user.
    #[serde(rename="pendingIdToken")]
    
    pub pending_id_token: Option<String>,
    /// The post body if the request is a HTTP POST.
    #[serde(rename="postBody")]
    
    pub post_body: Option<String>,
    /// The URI to which the IDP redirects the user back. It may contain federated login result params added by the IDP.
    #[serde(rename="requestUri")]
    
    pub request_uri: Option<String>,
    /// Whether return 200 and IDP credential rather than throw exception when federated id is already linked.
    #[serde(rename="returnIdpCredential")]
    
    pub return_idp_credential: Option<bool>,
    /// Whether to return refresh tokens.
    #[serde(rename="returnRefreshToken")]
    
    pub return_refresh_token: Option<bool>,
    /// Whether return sts id token and refresh token instead of gitkit token.
    #[serde(rename="returnSecureToken")]
    
    pub return_secure_token: Option<bool>,
    /// Session ID, which should match the one in previous createAuthUri request.
    #[serde(rename="sessionId")]
    
    pub session_id: Option<String>,
    /// For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from.
    #[serde(rename="tenantId")]
    
    pub tenant_id: Option<String>,
    /// Tenant project number to be used for idp discovery.
    #[serde(rename="tenantProjectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub tenant_project_number: Option<u64>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartyVerifyAssertionRequest {}


/// Request to verify a custom token
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify custom token relyingparty](RelyingpartyVerifyCustomTokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyVerifyCustomTokenRequest {
    /// GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration.
    #[serde(rename="delegatedProjectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub delegated_project_number: Option<i64>,
    /// Instance id token of the app.
    #[serde(rename="instanceId")]
    
    pub instance_id: Option<String>,
    /// Whether return sts id token and refresh token instead of gitkit token.
    #[serde(rename="returnSecureToken")]
    
    pub return_secure_token: Option<bool>,
    /// The custom token to verify
    
    pub token: Option<String>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartyVerifyCustomTokenRequest {}


/// Request to verify the password.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify password relyingparty](RelyingpartyVerifyPasswordCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyVerifyPasswordRequest {
    /// The captcha challenge.
    #[serde(rename="captchaChallenge")]
    
    pub captcha_challenge: Option<String>,
    /// Response to the captcha.
    #[serde(rename="captchaResponse")]
    
    pub captcha_response: Option<String>,
    /// GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration.
    #[serde(rename="delegatedProjectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub delegated_project_number: Option<i64>,
    /// The email of the user.
    
    pub email: Option<String>,
    /// The GITKit token of the authenticated user.
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// Instance id token of the app.
    #[serde(rename="instanceId")]
    
    pub instance_id: Option<String>,
    /// The password inputed by the user.
    
    pub password: Option<String>,
    /// The GITKit token for the non-trusted IDP, which is to be confirmed by the user.
    #[serde(rename="pendingIdToken")]
    
    pub pending_id_token: Option<String>,
    /// Whether return sts id token and refresh token instead of gitkit token.
    #[serde(rename="returnSecureToken")]
    
    pub return_secure_token: Option<bool>,
    /// For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from.
    #[serde(rename="tenantId")]
    
    pub tenant_id: Option<String>,
    /// Tenant project number to be used for idp discovery.
    #[serde(rename="tenantProjectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub tenant_project_number: Option<u64>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartyVerifyPasswordRequest {}


/// Request for Identitytoolkit-VerifyPhoneNumber
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify phone number relyingparty](RelyingpartyVerifyPhoneNumberCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyVerifyPhoneNumberRequest {
    /// no description provided
    
    pub code: Option<String>,
    /// no description provided
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// no description provided
    
    pub operation: Option<String>,
    /// no description provided
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// The session info previously returned by IdentityToolkit-SendVerificationCode.
    #[serde(rename="sessionInfo")]
    
    pub session_info: Option<String>,
    /// no description provided
    #[serde(rename="temporaryProof")]
    
    pub temporary_proof: Option<String>,
    /// no description provided
    #[serde(rename="verificationProof")]
    
    pub verification_proof: Option<String>,
}

impl client::RequestValue for IdentitytoolkitRelyingpartyVerifyPhoneNumberRequest {}


/// Response for Identitytoolkit-VerifyPhoneNumber
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify phone number relyingparty](RelyingpartyVerifyPhoneNumberCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyVerifyPhoneNumberResponse {
    /// no description provided
    #[serde(rename="expiresIn")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expires_in: Option<i64>,
    /// no description provided
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// no description provided
    #[serde(rename="isNewUser")]
    
    pub is_new_user: Option<bool>,
    /// no description provided
    #[serde(rename="localId")]
    
    pub local_id: Option<String>,
    /// no description provided
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// no description provided
    #[serde(rename="refreshToken")]
    
    pub refresh_token: Option<String>,
    /// no description provided
    #[serde(rename="temporaryProof")]
    
    pub temporary_proof: Option<String>,
    /// no description provided
    #[serde(rename="temporaryProofExpiresIn")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub temporary_proof_expires_in: Option<i64>,
    /// no description provided
    #[serde(rename="verificationProof")]
    
    pub verification_proof: Option<String>,
    /// no description provided
    #[serde(rename="verificationProofExpiresIn")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub verification_proof_expires_in: Option<i64>,
}

impl client::ResponseResult for IdentitytoolkitRelyingpartyVerifyPhoneNumberResponse {}


/// Template for a single idp configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdpConfig {
    /// OAuth2 client ID.
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// Whether this IDP is enabled.
    
    pub enabled: Option<bool>,
    /// Percent of users who will be prompted/redirected federated login for this IDP.
    #[serde(rename="experimentPercent")]
    
    pub experiment_percent: Option<i32>,
    /// OAuth2 provider.
    
    pub provider: Option<String>,
    /// OAuth2 client secret.
    
    pub secret: Option<String>,
    /// Whitelisted client IDs for audience check.
    #[serde(rename="whitelistedAudiences")]
    
    pub whitelisted_audiences: Option<Vec<String>>,
}

impl client::Part for IdpConfig {}


/// Request of getting a code for user confirmation (reset password, change email etc.)
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get oob confirmation code relyingparty](RelyingpartyGetOobConfirmationCodeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Relyingparty {
    /// whether or not to install the android app on the device where the link is opened
    #[serde(rename="androidInstallApp")]
    
    pub android_install_app: Option<bool>,
    /// minimum version of the app. if the version on the device is lower than this version then the user is taken to the play store to upgrade the app
    #[serde(rename="androidMinimumVersion")]
    
    pub android_minimum_version: Option<String>,
    /// android package name of the android app to handle the action code
    #[serde(rename="androidPackageName")]
    
    pub android_package_name: Option<String>,
    /// whether or not the app can handle the oob code without first going to web
    #[serde(rename="canHandleCodeInApp")]
    
    pub can_handle_code_in_app: Option<bool>,
    /// The recaptcha response from the user.
    #[serde(rename="captchaResp")]
    
    pub captcha_resp: Option<String>,
    /// The recaptcha challenge presented to the user.
    
    pub challenge: Option<String>,
    /// The url to continue to the Gitkit app
    #[serde(rename="continueUrl")]
    
    pub continue_url: Option<String>,
    /// The email of the user.
    
    pub email: Option<String>,
    /// iOS app store id to download the app if it's not already installed
    #[serde(rename="iOSAppStoreId")]
    
    pub i_os_app_store_id: Option<String>,
    /// the iOS bundle id of iOS app to handle the action code
    #[serde(rename="iOSBundleId")]
    
    pub i_os_bundle_id: Option<String>,
    /// The user's Gitkit login token for email change.
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// The fixed string "identitytoolkit#relyingparty".
    
    pub kind: Option<String>,
    /// The new email if the code is for email change.
    #[serde(rename="newEmail")]
    
    pub new_email: Option<String>,
    /// The request type.
    #[serde(rename="requestType")]
    
    pub request_type: Option<String>,
    /// The IP address of the user.
    #[serde(rename="userIp")]
    
    pub user_ip: Option<String>,
}

impl client::RequestValue for Relyingparty {}


/// Response of resetting the password.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reset password relyingparty](RelyingpartyResetPasswordCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResetPasswordResponse {
    /// The user's email. If the out-of-band code is for email recovery, the user's original email.
    
    pub email: Option<String>,
    /// The fixed string "identitytoolkit#ResetPasswordResponse".
    
    pub kind: Option<String>,
    /// If the out-of-band code is for email recovery, the user's new email.
    #[serde(rename="newEmail")]
    
    pub new_email: Option<String>,
    /// The request type.
    #[serde(rename="requestType")]
    
    pub request_type: Option<String>,
}

impl client::ResponseResult for ResetPasswordResponse {}


/// Respone of setting the account information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set account info relyingparty](RelyingpartySetAccountInfoCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetAccountInfoResponse {
    /// The name of the user.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The email of the user.
    
    pub email: Option<String>,
    /// If email has been verified.
    #[serde(rename="emailVerified")]
    
    pub email_verified: Option<bool>,
    /// If idToken is STS id token, then this field will be expiration time of STS id token in seconds.
    #[serde(rename="expiresIn")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expires_in: Option<i64>,
    /// The Gitkit id token to login the newly sign up user.
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// The fixed string "identitytoolkit#SetAccountInfoResponse".
    
    pub kind: Option<String>,
    /// The local ID of the user.
    #[serde(rename="localId")]
    
    pub local_id: Option<String>,
    /// The new email the user attempts to change to.
    #[serde(rename="newEmail")]
    
    pub new_email: Option<String>,
    /// The user's hashed password.
    #[serde(rename="passwordHash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub password_hash: Option<Vec<u8>>,
    /// The photo url of the user.
    #[serde(rename="photoUrl")]
    
    pub photo_url: Option<String>,
    /// The user's profiles at the associated IdPs.
    #[serde(rename="providerUserInfo")]
    
    pub provider_user_info: Option<Vec<SetAccountInfoResponseProviderUserInfo>>,
    /// If idToken is STS id token, then this field will be refresh token.
    #[serde(rename="refreshToken")]
    
    pub refresh_token: Option<String>,
}

impl client::ResponseResult for SetAccountInfoResponse {}


/// Response of signing up new user, creating anonymous user or anonymous user reauth.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [signup new user relyingparty](RelyingpartySignupNewUserCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SignupNewUserResponse {
    /// The name of the user.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The email of the user.
    
    pub email: Option<String>,
    /// If idToken is STS id token, then this field will be expiration time of STS id token in seconds.
    #[serde(rename="expiresIn")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expires_in: Option<i64>,
    /// The Gitkit id token to login the newly sign up user.
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// The fixed string "identitytoolkit#SignupNewUserResponse".
    
    pub kind: Option<String>,
    /// The RP local ID of the user.
    #[serde(rename="localId")]
    
    pub local_id: Option<String>,
    /// If idToken is STS id token, then this field will be refresh token.
    #[serde(rename="refreshToken")]
    
    pub refresh_token: Option<String>,
}

impl client::ResponseResult for SignupNewUserResponse {}


/// Respone of uploading accounts in batch.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [upload account relyingparty](RelyingpartyUploadAccountCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadAccountResponse {
    /// The error encountered while processing the account info.
    
    pub error: Option<Vec<UploadAccountResponseError>>,
    /// The fixed string "identitytoolkit#UploadAccountResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for UploadAccountResponse {}


/// Template for an individual account info.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    /// User creation timestamp.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// The custom attributes to be set in the user's id token.
    #[serde(rename="customAttributes")]
    
    pub custom_attributes: Option<String>,
    /// Whether the user is authenticated by the developer.
    #[serde(rename="customAuth")]
    
    pub custom_auth: Option<bool>,
    /// Whether the user is disabled.
    
    pub disabled: Option<bool>,
    /// The name of the user.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The email of the user.
    
    pub email: Option<String>,
    /// Whether the email has been verified.
    #[serde(rename="emailVerified")]
    
    pub email_verified: Option<bool>,
    /// last login timestamp.
    #[serde(rename="lastLoginAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_login_at: Option<i64>,
    /// The local ID of the user.
    #[serde(rename="localId")]
    
    pub local_id: Option<String>,
    /// The user's hashed password.
    #[serde(rename="passwordHash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub password_hash: Option<Vec<u8>>,
    /// The timestamp when the password was last updated.
    #[serde(rename="passwordUpdatedAt")]
    
    pub password_updated_at: Option<f64>,
    /// User's phone number.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// The URL of the user profile photo.
    #[serde(rename="photoUrl")]
    
    pub photo_url: Option<String>,
    /// The IDP of the user.
    #[serde(rename="providerUserInfo")]
    
    pub provider_user_info: Option<Vec<UserInfoProviderUserInfo>>,
    /// The user's plain text password.
    #[serde(rename="rawPassword")]
    
    pub raw_password: Option<String>,
    /// The user's password salt.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub salt: Option<Vec<u8>>,
    /// User's screen name at Twitter or login name at Github.
    #[serde(rename="screenName")]
    
    pub screen_name: Option<String>,
    /// Timestamp in seconds for valid login token.
    #[serde(rename="validSince")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub valid_since: Option<i64>,
    /// Version of the user's password.
    
    pub version: Option<i32>,
}

impl client::Part for UserInfo {}


/// Response of verifying the IDP assertion.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify assertion relyingparty](RelyingpartyVerifyAssertionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerifyAssertionResponse {
    /// The action code.
    
    pub action: Option<String>,
    /// URL for OTA app installation.
    #[serde(rename="appInstallationUrl")]
    
    pub app_installation_url: Option<String>,
    /// The custom scheme used by mobile app.
    #[serde(rename="appScheme")]
    
    pub app_scheme: Option<String>,
    /// The opaque value used by the client to maintain context info between the authentication request and the IDP callback.
    
    pub context: Option<String>,
    /// The birth date of the IdP account.
    #[serde(rename="dateOfBirth")]
    
    pub date_of_birth: Option<String>,
    /// The display name of the user.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The email returned by the IdP. NOTE: The federated login user may not own the email.
    
    pub email: Option<String>,
    /// It's true if the email is recycled.
    #[serde(rename="emailRecycled")]
    
    pub email_recycled: Option<bool>,
    /// The value is true if the IDP is also the email provider. It means the user owns the email.
    #[serde(rename="emailVerified")]
    
    pub email_verified: Option<bool>,
    /// Client error code.
    #[serde(rename="errorMessage")]
    
    pub error_message: Option<String>,
    /// If idToken is STS id token, then this field will be expiration time of STS id token in seconds.
    #[serde(rename="expiresIn")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expires_in: Option<i64>,
    /// The unique ID identifies the IdP account.
    #[serde(rename="federatedId")]
    
    pub federated_id: Option<String>,
    /// The first name of the user.
    #[serde(rename="firstName")]
    
    pub first_name: Option<String>,
    /// The full name of the user.
    #[serde(rename="fullName")]
    
    pub full_name: Option<String>,
    /// The ID token.
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// It's the identifier param in the createAuthUri request if the identifier is an email. It can be used to check whether the user input email is different from the asserted email.
    #[serde(rename="inputEmail")]
    
    pub input_email: Option<String>,
    /// True if it's a new user sign-in, false if it's a returning user.
    #[serde(rename="isNewUser")]
    
    pub is_new_user: Option<bool>,
    /// The fixed string "identitytoolkit#VerifyAssertionResponse".
    
    pub kind: Option<String>,
    /// The language preference of the user.
    
    pub language: Option<String>,
    /// The last name of the user.
    #[serde(rename="lastName")]
    
    pub last_name: Option<String>,
    /// The RP local ID if it's already been mapped to the IdP account identified by the federated ID.
    #[serde(rename="localId")]
    
    pub local_id: Option<String>,
    /// Whether the assertion is from a non-trusted IDP and need account linking confirmation.
    #[serde(rename="needConfirmation")]
    
    pub need_confirmation: Option<bool>,
    /// Whether need client to supply email to complete the federated login flow.
    #[serde(rename="needEmail")]
    
    pub need_email: Option<bool>,
    /// The nick name of the user.
    #[serde(rename="nickName")]
    
    pub nick_name: Option<String>,
    /// The OAuth2 access token.
    #[serde(rename="oauthAccessToken")]
    
    pub oauth_access_token: Option<String>,
    /// The OAuth2 authorization code.
    #[serde(rename="oauthAuthorizationCode")]
    
    pub oauth_authorization_code: Option<String>,
    /// The lifetime in seconds of the OAuth2 access token.
    #[serde(rename="oauthExpireIn")]
    
    pub oauth_expire_in: Option<i32>,
    /// The OIDC id token.
    #[serde(rename="oauthIdToken")]
    
    pub oauth_id_token: Option<String>,
    /// The user approved request token for the OpenID OAuth extension.
    #[serde(rename="oauthRequestToken")]
    
    pub oauth_request_token: Option<String>,
    /// The scope for the OpenID OAuth extension.
    #[serde(rename="oauthScope")]
    
    pub oauth_scope: Option<String>,
    /// The OAuth1 access token secret.
    #[serde(rename="oauthTokenSecret")]
    
    pub oauth_token_secret: Option<String>,
    /// The original email stored in the mapping storage. It's returned when the federated ID is associated to a different email.
    #[serde(rename="originalEmail")]
    
    pub original_email: Option<String>,
    /// The URI of the public accessible profiel picture.
    #[serde(rename="photoUrl")]
    
    pub photo_url: Option<String>,
    /// The IdP ID. For white listed IdPs it's a short domain name e.g. google.com, aol.com, live.net and yahoo.com. If the "providerId" param is set to OpenID OP identifer other than the whilte listed IdPs the OP identifier is returned. If the "identifier" param is federated ID in the createAuthUri request. The domain part of the federated ID is returned.
    #[serde(rename="providerId")]
    
    pub provider_id: Option<String>,
    /// Raw IDP-returned user info.
    #[serde(rename="rawUserInfo")]
    
    pub raw_user_info: Option<String>,
    /// If idToken is STS id token, then this field will be refresh token.
    #[serde(rename="refreshToken")]
    
    pub refresh_token: Option<String>,
    /// The screen_name of a Twitter user or the login name at Github.
    #[serde(rename="screenName")]
    
    pub screen_name: Option<String>,
    /// The timezone of the user.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// When action is 'map', contains the idps which can be used for confirmation.
    #[serde(rename="verifiedProvider")]
    
    pub verified_provider: Option<Vec<String>>,
}

impl client::ResponseResult for VerifyAssertionResponse {}


/// Response from verifying a custom token
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify custom token relyingparty](RelyingpartyVerifyCustomTokenCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerifyCustomTokenResponse {
    /// If idToken is STS id token, then this field will be expiration time of STS id token in seconds.
    #[serde(rename="expiresIn")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expires_in: Option<i64>,
    /// The GITKit token for authenticated user.
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// True if it's a new user sign-in, false if it's a returning user.
    #[serde(rename="isNewUser")]
    
    pub is_new_user: Option<bool>,
    /// The fixed string "identitytoolkit#VerifyCustomTokenResponse".
    
    pub kind: Option<String>,
    /// If idToken is STS id token, then this field will be refresh token.
    #[serde(rename="refreshToken")]
    
    pub refresh_token: Option<String>,
}

impl client::ResponseResult for VerifyCustomTokenResponse {}


/// Request of verifying the password.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify password relyingparty](RelyingpartyVerifyPasswordCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerifyPasswordResponse {
    /// The name of the user.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The email returned by the IdP. NOTE: The federated login user may not own the email.
    
    pub email: Option<String>,
    /// If idToken is STS id token, then this field will be expiration time of STS id token in seconds.
    #[serde(rename="expiresIn")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expires_in: Option<i64>,
    /// The GITKit token for authenticated user.
    #[serde(rename="idToken")]
    
    pub id_token: Option<String>,
    /// The fixed string "identitytoolkit#VerifyPasswordResponse".
    
    pub kind: Option<String>,
    /// The RP local ID if it's already been mapped to the IdP account identified by the federated ID.
    #[serde(rename="localId")]
    
    pub local_id: Option<String>,
    /// The OAuth2 access token.
    #[serde(rename="oauthAccessToken")]
    
    pub oauth_access_token: Option<String>,
    /// The OAuth2 authorization code.
    #[serde(rename="oauthAuthorizationCode")]
    
    pub oauth_authorization_code: Option<String>,
    /// The lifetime in seconds of the OAuth2 access token.
    #[serde(rename="oauthExpireIn")]
    
    pub oauth_expire_in: Option<i32>,
    /// The URI of the user's photo at IdP
    #[serde(rename="photoUrl")]
    
    pub photo_url: Option<String>,
    /// If idToken is STS id token, then this field will be refresh token.
    #[serde(rename="refreshToken")]
    
    pub refresh_token: Option<String>,
    /// Whether the email is registered.
    
    pub registered: Option<bool>,
}

impl client::ResponseResult for VerifyPasswordResponse {}


/// The user's profiles at the associated IdPs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetAccountInfoResponseProviderUserInfo {
    /// The user's display name at the IDP.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// User's identifier at IDP.
    #[serde(rename="federatedId")]
    
    pub federated_id: Option<String>,
    /// The user's photo url at the IDP.
    #[serde(rename="photoUrl")]
    
    pub photo_url: Option<String>,
    /// The IdP ID. For whitelisted IdPs it's a short domain name, e.g., google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it's the OP identifier.
    #[serde(rename="providerId")]
    
    pub provider_id: Option<String>,
}

impl client::NestedType for SetAccountInfoResponseProviderUserInfo {}
impl client::Part for SetAccountInfoResponseProviderUserInfo {}


/// The error encountered while processing the account info.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadAccountResponseError {
    /// The index of the malformed account, starting from 0.
    
    pub index: Option<i32>,
    /// Detailed error message for the account info.
    
    pub message: Option<String>,
}

impl client::NestedType for UploadAccountResponseError {}
impl client::Part for UploadAccountResponseError {}


/// The IDP of the user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserInfoProviderUserInfo {
    /// The user's display name at the IDP.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// User's email at IDP.
    
    pub email: Option<String>,
    /// User's identifier at IDP.
    #[serde(rename="federatedId")]
    
    pub federated_id: Option<String>,
    /// User's phone number.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// The user's photo url at the IDP.
    #[serde(rename="photoUrl")]
    
    pub photo_url: Option<String>,
    /// The IdP ID. For white listed IdPs it's a short domain name, e.g., google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it's the OP identifier.
    #[serde(rename="providerId")]
    
    pub provider_id: Option<String>,
    /// User's raw identifier directly returned from IDP.
    #[serde(rename="rawId")]
    
    pub raw_id: Option<String>,
    /// User's screen name at Twitter or login name at Github.
    #[serde(rename="screenName")]
    
    pub screen_name: Option<String>,
}

impl client::NestedType for UserInfoProviderUserInfo {}
impl client::Part for UserInfoProviderUserInfo {}


