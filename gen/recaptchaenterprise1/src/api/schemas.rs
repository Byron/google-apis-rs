use super::*;
/// Account defender risk assessment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessment {
    /// Labels for this request.
    
    pub labels: Option<Vec<String>>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessment {}


/// Information about account verification, used for identity verification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1AccountVerificationInfo {
    /// Endpoints that can be used for identity verification.
    
    pub endpoints: Option<Vec<GoogleCloudRecaptchaenterpriseV1EndpointVerificationInfo>>,
    /// Language code preference for the verification message, set as a IETF BCP 47 language code.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Output only. Result of the latest account verification challenge.
    #[serde(rename="latestVerificationResult")]
    
    pub latest_verification_result: Option<String>,
    /// Username of the account that is being verified. Deprecated. Customers should now provide the hashed account ID field in Event.
    
    pub username: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1AccountVerificationInfo {}


/// Settings specific to keys that can be used by Android apps.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1AndroidKeySettings {
    /// If set to true, allowed_package_names are not enforced.
    #[serde(rename="allowAllPackageNames")]
    
    pub allow_all_package_names: Option<bool>,
    /// Android package names of apps allowed to use the key. Example: 'com.companyname.appname'
    #[serde(rename="allowedPackageNames")]
    
    pub allowed_package_names: Option<Vec<String>>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1AndroidKeySettings {}


/// The request message to annotate an Assessment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assessments annotate projects](ProjectAssessmentAnnotateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequest {
    /// Optional. The annotation that will be assigned to the Event. This field can be left empty to provide reasons that apply to an event without concluding whether the event is legitimate or fraudulent.
    
    pub annotation: Option<String>,
    /// Optional. Unique stable hashed user identifier to apply to the assessment. This is an alternative to setting the hashed_account_id in CreateAssessment, for example when the account identifier is not yet known in the initial request. It is recommended that the identifier is hashed using hmac-sha256 with stable secret.
    #[serde(rename="hashedAccountId")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub hashed_account_id: Option<Vec<u8>>,
    /// Optional. Optional reasons for the annotation that will be assigned to the Event.
    
    pub reasons: Option<Vec<String>>,
}

impl client::RequestValue for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequest {}


/// Empty response for AnnotateAssessment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assessments annotate projects](ProjectAssessmentAnnotateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentResponse { _never_set: Option<bool> }

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentResponse {}


/// A recaptcha assessment resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assessments create projects](ProjectAssessmentCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1Assessment {
    /// Assessment returned by account defender when a hashed_account_id is provided.
    #[serde(rename="accountDefenderAssessment")]
    
    pub account_defender_assessment: Option<GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessment>,
    /// Account verification information for identity verification. The assessment event must include a token and site key to use this feature.
    #[serde(rename="accountVerification")]
    
    pub account_verification: Option<GoogleCloudRecaptchaenterpriseV1AccountVerificationInfo>,
    /// The event being assessed.
    
    pub event: Option<GoogleCloudRecaptchaenterpriseV1Event>,
    /// Output only. The resource name for the Assessment in the format "projects/{project}/assessments/{assessment}".
    
    pub name: Option<String>,
    /// The private password leak verification field contains the parameters that are used to to check for leaks privately without sharing user credentials.
    #[serde(rename="privatePasswordLeakVerification")]
    
    pub private_password_leak_verification: Option<GoogleCloudRecaptchaenterpriseV1PrivatePasswordLeakVerification>,
    /// Output only. The risk analysis result for the event being assessed.
    #[serde(rename="riskAnalysis")]
    
    pub risk_analysis: Option<GoogleCloudRecaptchaenterpriseV1RiskAnalysis>,
    /// Output only. Properties of the provided event token.
    #[serde(rename="tokenProperties")]
    
    pub token_properties: Option<GoogleCloudRecaptchaenterpriseV1TokenProperties>,
}

impl client::RequestValue for GoogleCloudRecaptchaenterpriseV1Assessment {}
impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1Assessment {}


/// Metrics related to challenges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1ChallengeMetrics {
    /// Count of submitted challenge solutions that were incorrect or otherwise deemed suspicious such that a subsequent challenge was triggered.
    #[serde(rename="failedCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub failed_count: Option<i64>,
    /// Count of nocaptchas (successful verification without a challenge) issued.
    #[serde(rename="nocaptchaCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub nocaptcha_count: Option<i64>,
    /// Count of reCAPTCHA checkboxes or badges rendered. This is mostly equivalent to a count of pageloads for pages that include reCAPTCHA.
    #[serde(rename="pageloadCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pageload_count: Option<i64>,
    /// Count of nocaptchas (successful verification without a challenge) plus submitted challenge solutions that were correct and resulted in verification.
    #[serde(rename="passedCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub passed_count: Option<i64>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1ChallengeMetrics {}


/// Information about a verification endpoint that can be used for 2FA.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1EndpointVerificationInfo {
    /// Email address for which to trigger a verification request.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// Output only. Timestamp of the last successful verification for the endpoint, if any.
    #[serde(rename="lastVerificationTime")]
    
    pub last_verification_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Phone number for which to trigger a verification request. Should be given in E.164 format.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// Output only. Token to provide to the client to trigger endpoint verification. It must be used within 15 minutes.
    #[serde(rename="requestToken")]
    
    pub request_token: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1EndpointVerificationInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1Event {
    /// Optional. The expected action for this type of event. This should be the same action provided at token generation time on client-side platforms already integrated with recaptcha enterprise.
    #[serde(rename="expectedAction")]
    
    pub expected_action: Option<String>,
    /// Optional. Unique stable hashed user identifier for the request. The identifier must be hashed using hmac-sha256 with stable secret.
    #[serde(rename="hashedAccountId")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub hashed_account_id: Option<Vec<u8>>,
    /// Optional. The site key that was used to invoke reCAPTCHA on your site and generate the token.
    #[serde(rename="siteKey")]
    
    pub site_key: Option<String>,
    /// Optional. The user response token provided by the reCAPTCHA client-side integration on your site.
    
    pub token: Option<String>,
    /// Optional. The user agent present in the request from the user's device related to this event.
    #[serde(rename="userAgent")]
    
    pub user_agent: Option<String>,
    /// Optional. The IP address in the request from the user's device related to this event.
    #[serde(rename="userIpAddress")]
    
    pub user_ip_address: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1Event {}


/// Settings specific to keys that can be used by iOS apps.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1IOSKeySettings {
    /// If set to true, allowed_bundle_ids are not enforced.
    #[serde(rename="allowAllBundleIds")]
    
    pub allow_all_bundle_ids: Option<bool>,
    /// iOS bundle ids of apps allowed to use the key. Example: 'com.companyname.productname.appname'
    #[serde(rename="allowedBundleIds")]
    
    pub allowed_bundle_ids: Option<Vec<String>>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1IOSKeySettings {}


/// A key used to identify and configure applications (web and/or mobile) that use reCAPTCHA Enterprise.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [keys create projects](ProjectKeyCreateCall) (request|response)
/// * [keys get projects](ProjectKeyGetCall) (response)
/// * [keys migrate projects](ProjectKeyMigrateCall) (response)
/// * [keys patch projects](ProjectKeyPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1Key {
    /// Settings for keys that can be used by Android apps.
    #[serde(rename="androidSettings")]
    
    pub android_settings: Option<GoogleCloudRecaptchaenterpriseV1AndroidKeySettings>,
    /// The timestamp corresponding to the creation of this Key.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Human-readable display name of this key. Modifiable by user.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Settings for keys that can be used by iOS apps.
    #[serde(rename="iosSettings")]
    
    pub ios_settings: Option<GoogleCloudRecaptchaenterpriseV1IOSKeySettings>,
    /// See Creating and managing labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// The resource name for the Key in the format "projects/{project}/keys/{key}".
    
    pub name: Option<String>,
    /// Options for user acceptance testing.
    #[serde(rename="testingOptions")]
    
    pub testing_options: Option<GoogleCloudRecaptchaenterpriseV1TestingOptions>,
    /// Settings for WAF
    #[serde(rename="wafSettings")]
    
    pub waf_settings: Option<GoogleCloudRecaptchaenterpriseV1WafSettings>,
    /// Settings for keys that can be used by websites.
    #[serde(rename="webSettings")]
    
    pub web_settings: Option<GoogleCloudRecaptchaenterpriseV1WebKeySettings>,
}

impl client::RequestValue for GoogleCloudRecaptchaenterpriseV1Key {}
impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1Key {}


/// Response to request to list keys in a project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [keys list projects](ProjectKeyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1ListKeysResponse {
    /// Key details.
    
    pub keys: Option<Vec<GoogleCloudRecaptchaenterpriseV1Key>>,
    /// Token to retrieve the next page of results. It is set to empty if no keys remain in results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1ListKeysResponse {}


/// The response to a `ListRelatedAccountGroupMemberships` call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [relatedaccountgroups memberships list projects](ProjectRelatedaccountgroupMembershipListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The memberships listed by the query.
    #[serde(rename="relatedAccountGroupMemberships")]
    
    pub related_account_group_memberships: Option<Vec<GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership>>,
}

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse {}


/// The response to a `ListRelatedAccountGroups` call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [relatedaccountgroups list projects](ProjectRelatedaccountgroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The groups of related accounts listed by the query.
    #[serde(rename="relatedAccountGroups")]
    
    pub related_account_groups: Option<Vec<GoogleCloudRecaptchaenterpriseV1RelatedAccountGroup>>,
}

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse {}


/// Metrics for a single Key.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [keys get metrics projects](ProjectKeyGetMetricCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1Metrics {
    /// Metrics will be continuous and in order by dates, and in the granularity of day. Only challenge-based keys (CHECKBOX, INVISIBLE), will have challenge-based data.
    #[serde(rename="challengeMetrics")]
    
    pub challenge_metrics: Option<Vec<GoogleCloudRecaptchaenterpriseV1ChallengeMetrics>>,
    /// Output only. The name of the metrics, in the format "projects/{project}/keys/{key}/metrics".
    
    pub name: Option<String>,
    /// Metrics will be continuous and in order by dates, and in the granularity of day. All Key types should have score-based data.
    #[serde(rename="scoreMetrics")]
    
    pub score_metrics: Option<Vec<GoogleCloudRecaptchaenterpriseV1ScoreMetrics>>,
    /// Inclusive start time aligned to a day (UTC).
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1Metrics {}


/// The migrate key request message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [keys migrate projects](ProjectKeyMigrateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1MigrateKeyRequest {
    /// Optional. If true, skips the billing check. A reCAPTCHA Enterprise key or migrated key behaves differently than a reCAPTCHA (non-Enterprise version) key when you reach a quota limit (see https://cloud.google.com/recaptcha-enterprise/quotas#quota_limit). To avoid any disruption of your usage, we check that a billing account is present. If your usage of reCAPTCHA is under the free quota, you can safely skip the billing check and proceed with the migration. See https://cloud.google.com/recaptcha-enterprise/docs/billing-information.
    #[serde(rename="skipBillingCheck")]
    
    pub skip_billing_check: Option<bool>,
}

impl client::RequestValue for GoogleCloudRecaptchaenterpriseV1MigrateKeyRequest {}


/// Private password leak verification info.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1PrivatePasswordLeakVerification {
    /// Output only. List of prefixes of the encrypted potential password leaks that matched the given parameters. They must be compared with the client-side decryption prefix of `reencrypted_user_credentials_hash`
    #[serde(rename="encryptedLeakMatchPrefixes")]
    
    #[serde_as(as = "Option<Vec<::client::serde::urlsafe_base64::Wrapper>>")]
    pub encrypted_leak_match_prefixes: Option<Vec<Vec<u8>>>,
    /// Optional. Encrypted Scrypt hash of the canonicalized username+password. It is re-encrypted by the server and returned through `reencrypted_user_credentials_hash`.
    #[serde(rename="encryptedUserCredentialsHash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub encrypted_user_credentials_hash: Option<Vec<u8>>,
    /// Optional. Exactly 26-bit prefix of the SHA-256 hash of the canonicalized username. It is used to look up password leaks associated with that hash prefix.
    #[serde(rename="lookupHashPrefix")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub lookup_hash_prefix: Option<Vec<u8>>,
    /// Output only. Corresponds to the re-encryption of the `encrypted_user_credentials_hash` field. It is used to match potential password leaks within `encrypted_leak_match_prefixes`.
    #[serde(rename="reencryptedUserCredentialsHash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub reencrypted_user_credentials_hash: Option<Vec<u8>>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1PrivatePasswordLeakVerification {}


/// A group of related accounts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1RelatedAccountGroup {
    /// Required. The resource name for the related account group in the format `projects/{project}/relatedaccountgroups/{related_account_group}`.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1RelatedAccountGroup {}


/// A membership in a group of related accounts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership {
    /// The unique stable hashed user identifier of the member. The identifier corresponds to a `hashed_account_id` provided in a previous `CreateAssessment` or `AnnotateAssessment` call.
    #[serde(rename="hashedAccountId")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub hashed_account_id: Option<Vec<u8>>,
    /// Required. The resource name for this membership in the format `projects/{project}/relatedaccountgroups/{relatedaccountgroup}/memberships/{membership}`.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership {}


/// Secret key is used only in legacy reCAPTCHA. It must be used in a 3rd party integration with legacy reCAPTCHA.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [keys retrieve legacy secret key projects](ProjectKeyRetrieveLegacySecretKeyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1RetrieveLegacySecretKeyResponse {
    /// The secret key (also known as shared secret) authorizes communication between your application backend and the reCAPTCHA Enterprise server to create an assessment. The secret key needs to be kept safe for security purposes.
    #[serde(rename="legacySecretKey")]
    
    pub legacy_secret_key: Option<String>,
}

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1RetrieveLegacySecretKeyResponse {}


/// Risk analysis result for an event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1RiskAnalysis {
    /// Reasons contributing to the risk analysis verdict.
    
    pub reasons: Option<Vec<String>>,
    /// Legitimate event score from 0.0 to 1.0. (1.0 means very likely legitimate traffic while 0.0 means very likely non-legitimate traffic).
    
    pub score: Option<f32>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1RiskAnalysis {}


/// Score distribution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1ScoreDistribution {
    /// Map key is score value multiplied by 100. The scores are discrete values between [0, 1]. The maximum number of buckets is on order of a few dozen, but typically much lower (ie. 10).
    #[serde(rename="scoreBuckets")]
    
    #[serde_as(as = "Option<HashMap<_, ::client::serde_with::DisplayFromStr>>")]
    pub score_buckets: Option<HashMap<String, i64>>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1ScoreDistribution {}


/// Metrics related to scoring.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1ScoreMetrics {
    /// Action-based metrics. The map key is the action name which specified by the site owners at time of the "execute" client-side call.
    #[serde(rename="actionMetrics")]
    
    pub action_metrics: Option<HashMap<String, GoogleCloudRecaptchaenterpriseV1ScoreDistribution>>,
    /// Aggregated score metrics for all traffic.
    #[serde(rename="overallMetrics")]
    
    pub overall_metrics: Option<GoogleCloudRecaptchaenterpriseV1ScoreDistribution>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1ScoreMetrics {}


/// The request message to search related account group memberships.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [relatedaccountgroupmemberships search projects](ProjectRelatedaccountgroupmembershipSearchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsRequest {
    /// Optional. The unique stable hashed user identifier we should search connections to. The identifier should correspond to a `hashed_account_id` provided in a previous `CreateAssessment` or `AnnotateAssessment` call.
    #[serde(rename="hashedAccountId")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub hashed_account_id: Option<Vec<u8>>,
    /// Optional. The maximum number of groups to return. The service might return fewer than this value. If unspecified, at most 50 groups are returned. The maximum value is 1000; values above 1000 are coerced to 1000.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional. A page token, received from a previous `SearchRelatedAccountGroupMemberships` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `SearchRelatedAccountGroupMemberships` must match the call that provided the page token.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
}

impl client::RequestValue for GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsRequest {}


/// The response to a `SearchRelatedAccountGroupMemberships` call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [relatedaccountgroupmemberships search projects](ProjectRelatedaccountgroupmembershipSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The queried memberships.
    #[serde(rename="relatedAccountGroupMemberships")]
    
    pub related_account_group_memberships: Option<Vec<GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership>>,
}

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsResponse {}


/// Options for user acceptance testing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1TestingOptions {
    /// For challenge-based keys only (CHECKBOX, INVISIBLE), all challenge requests for this site will return nocaptcha if NOCAPTCHA, or an unsolvable challenge if CHALLENGE.
    #[serde(rename="testingChallenge")]
    
    pub testing_challenge: Option<String>,
    /// All assessments for this Key will return this score. Must be between 0 (likely not legitimate) and 1 (likely legitimate) inclusive.
    #[serde(rename="testingScore")]
    
    pub testing_score: Option<f32>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1TestingOptions {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1TokenProperties {
    /// Action name provided at token generation.
    
    pub action: Option<String>,
    /// The name of the Android package with which the token was generated (Android keys only).
    #[serde(rename="androidPackageName")]
    
    pub android_package_name: Option<String>,
    /// The timestamp corresponding to the generation of the token.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The hostname of the page on which the token was generated (Web keys only).
    
    pub hostname: Option<String>,
    /// Reason associated with the response when valid = false.
    #[serde(rename="invalidReason")]
    
    pub invalid_reason: Option<String>,
    /// The ID of the iOS bundle with which the token was generated (iOS keys only).
    #[serde(rename="iosBundleId")]
    
    pub ios_bundle_id: Option<String>,
    /// Whether the provided user response token is valid. When valid = false, the reason could be specified in invalid_reason or it could also be due to a user failing to solve a challenge or a sitekey mismatch (i.e the sitekey used to generate the token was different than the one specified in the assessment).
    
    pub valid: Option<bool>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1TokenProperties {}


/// Settings specific to keys that can be used for WAF (Web Application Firewall).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1WafSettings {
    /// Required. The WAF feature for which this key is enabled.
    #[serde(rename="wafFeature")]
    
    pub waf_feature: Option<String>,
    /// Required. The WAF service that uses this key.
    #[serde(rename="wafService")]
    
    pub waf_service: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1WafSettings {}


/// Settings specific to keys that can be used by websites.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1WebKeySettings {
    /// If set to true, it means allowed_domains will not be enforced.
    #[serde(rename="allowAllDomains")]
    
    pub allow_all_domains: Option<bool>,
    /// If set to true, the key can be used on AMP (Accelerated Mobile Pages) websites. This is supported only for the SCORE integration type.
    #[serde(rename="allowAmpTraffic")]
    
    pub allow_amp_traffic: Option<bool>,
    /// Domains or subdomains of websites allowed to use the key. All subdomains of an allowed domain are automatically allowed. A valid domain requires a host and must not include any path, port, query or fragment. Examples: 'example.com' or 'subdomain.example.com'
    #[serde(rename="allowedDomains")]
    
    pub allowed_domains: Option<Vec<String>>,
    /// Settings for the frequency and difficulty at which this key triggers captcha challenges. This should only be specified for IntegrationTypes CHECKBOX and INVISIBLE.
    #[serde(rename="challengeSecurityPreference")]
    
    pub challenge_security_preference: Option<String>,
    /// Required. Describes how this key is integrated with the website.
    #[serde(rename="integrationType")]
    
    pub integration_type: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1WebKeySettings {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [keys delete projects](ProjectKeyDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


