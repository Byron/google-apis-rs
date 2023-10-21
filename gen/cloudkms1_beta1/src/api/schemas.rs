use super::*;
/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys test iam permissions projects](ProjectLocationKeyRingCryptoKeyTestIamPermissionCall) (request)
/// * [locations key rings test iam permissions projects](ProjectLocationKeyRingTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with
    /// wildcards (such as '*' or 'storage.*') are not allowed. For more
    /// information see
    /// [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for TestIamPermissionsRequest {}


/// Defines an Identity and Access Management (IAM) policy. It is used to
/// specify access control policies for Cloud Platform resources.
/// 
/// A `Policy` consists of a list of `bindings`. A `Binding` binds a list of
/// `members` to a `role`, where the members can be user accounts, Google groups,
/// Google domains, and service accounts. A `role` is a named list of permissions
/// defined by IAM.
/// 
/// **Example**
/// 
/// ````text
/// {
///   "bindings": [
///     {
///       "role": "roles/owner",
///       "members": [
///         "user:mike@example.com",
///         "group:admins@example.com",
///         "domain:google.com",
///         "serviceAccount:my-other-app@appspot.gserviceaccount.com",
///       ]
///     },
///     {
///       "role": "roles/viewer",
///       "members": ["user:sean@example.com"]
///     }
///   ]
/// }
/// ````
/// 
/// For a description of IAM and its features, see the
/// [IAM developer’s guide](https://cloud.google.com/iam).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys set iam policy projects](ProjectLocationKeyRingCryptoKeySetIamPolicyCall) (response)
/// * [locations key rings crypto keys get iam policy projects](ProjectLocationKeyRingCryptoKeyGetIamPolicyCall) (response)
/// * [locations key rings get iam policy projects](ProjectLocationKeyRingGetIamPolicyCall) (response)
/// * [locations key rings set iam policy projects](ProjectLocationKeyRingSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// no description provided
    #[serde(rename="iamOwned")]
    
    pub iam_owned: Option<bool>,
    /// If more than one rule is specified, the rules are applied in the following
    /// manner:
    /// - All matching LOG rules are always applied.
    /// - If any DENY/DENY_WITH_LOG rule matches, permission is denied.
    ///   Logging will be applied if one or more matching rule requires logging.
    /// - Otherwise, if any ALLOW/ALLOW_WITH_LOG rule matches, permission is
    ///   granted.
    ///   Logging will be applied if one or more matching rule requires logging.
    /// - Otherwise, if no rule applies, permission is denied.
    
    pub rules: Option<Vec<Rule>>,
    /// Version of the `Policy`. The default version is 0.
    
    pub version: Option<i32>,
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<AuditConfig>>,
    /// Associates a list of `members` to a `role`.
    /// Multiple `bindings` must not be specified for the same `role`.
    /// `bindings` with no members will result in an error.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help
    /// prevent simultaneous updates of a policy from overwriting each other.
    /// It is strongly suggested that systems make use of the `etag` in the
    /// read-modify-write cycle to perform policy updates in order to avoid race
    /// conditions: An `etag` is returned in the response to `getIamPolicy`, and
    /// systems are expected to put that etag in the request to `setIamPolicy` to
    /// ensure that their change will be applied to the same version of the policy.
    /// 
    /// If no `etag` is provided in the call to `setIamPolicy`, then the existing
    /// policy is overwritten blindly.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
}

impl client::ResponseResult for Policy {}


/// The response message for Locations.ListLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations list projects](ProjectLocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    
    pub locations: Option<Vec<Location>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLocationsResponse {}


/// A KeyRing is a toplevel logical grouping of CryptoKeys.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings get projects](ProjectLocationKeyRingGetCall) (response)
/// * [locations key rings create projects](ProjectLocationKeyRingCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyRing {
    /// Output only. The time at which this KeyRing was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The resource name for the KeyRing in the format
    /// `projects/*/locations/*/keyRings/*`.
    
    pub name: Option<String>,
}

impl client::RequestValue for KeyRing {}
impl client::ResponseResult for KeyRing {}


/// Response message for KeyManagementService.Encrypt.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys encrypt projects](ProjectLocationKeyRingCryptoKeyEncryptCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EncryptResponse {
    /// The resource name of the CryptoKeyVersion used in encryption.
    
    pub name: Option<String>,
    /// The encrypted data.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub ciphertext: Option<Vec<u8>>,
}

impl client::ResponseResult for EncryptResponse {}


/// Request message for KeyManagementService.UpdateCryptoKeyPrimaryVersion.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys update primary version projects](ProjectLocationKeyRingCryptoKeyUpdatePrimaryVersionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateCryptoKeyPrimaryVersionRequest {
    /// The id of the child CryptoKeyVersion to use as primary.
    #[serde(rename="cryptoKeyVersionId")]
    
    pub crypto_key_version_id: Option<String>,
}

impl client::RequestValue for UpdateCryptoKeyPrimaryVersionRequest {}


/// Request message for KeyManagementService.RestoreCryptoKeyVersion.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys crypto key versions restore projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionRestoreCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestoreCryptoKeyVersionRequest { _never_set: Option<bool> }

impl client::RequestValue for RestoreCryptoKeyVersionRequest {}


/// Response message for KeyManagementService.ListKeyRings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings list projects](ProjectLocationKeyRingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListKeyRingsResponse {
    /// A token to retrieve next page of results. Pass this value in
    /// ListKeyRingsRequest.page_token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of KeyRings that matched the query.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
    /// The list of KeyRings.
    #[serde(rename="keyRings")]
    
    pub key_rings: Option<Vec<KeyRing>>,
}

impl client::ResponseResult for ListKeyRingsResponse {}


/// Write a Data Access (Gin) log
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataAccessOptions { _never_set: Option<bool> }

impl client::Part for DataAccessOptions {}


/// Specifies the audit configuration for a service.
/// The configuration determines which permission types are logged, and what
/// identities, if any, are exempted from logging.
/// An AuditConfig must have one or more AuditLogConfigs.
/// 
/// If there are AuditConfigs for both `allServices` and a specific service,
/// the union of the two AuditConfigs is used for that service: the log_types
/// specified in each AuditConfig are enabled, and the exempted_members in each
/// AuditConfig are exempted.
/// 
/// Example Policy with multiple AuditConfigs:
/// 
/// ````text
/// {
///   "audit_configs": [
///     {
///       "service": "allServices"
///       "audit_log_configs": [
///         {
///           "log_type": "DATA_READ",
///           "exempted_members": [
///             "user:foo@gmail.com"
///           ]
///         },
///         {
///           "log_type": "DATA_WRITE",
///         },
///         {
///           "log_type": "ADMIN_READ",
///         }
///       ]
///     },
///     {
///       "service": "fooservice.googleapis.com"
///       "audit_log_configs": [
///         {
///           "log_type": "DATA_READ",
///         },
///         {
///           "log_type": "DATA_WRITE",
///           "exempted_members": [
///             "user:bar@gmail.com"
///           ]
///         }
///       ]
///     }
///   ]
/// }
/// ````
/// 
/// For fooservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ
/// logging. It also exempts foo@gmail.com from DATA_READ logging, and
/// bar@gmail.com from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Specifies a service that will be enabled for audit logging.
    /// For example, `storage.googleapis.com`, `cloudsql.googleapis.com`.
    /// `allServices` is a special value that covers all services.
    
    pub service: Option<String>,
    /// The configuration for logging of each type of permission.
    /// Next ID: 4
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<AuditLogConfig>>,
    /// no description provided
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
}

impl client::Part for AuditConfig {}


/// A CryptoKeyVersion represents an individual cryptographic key, and the
/// associated key material.
/// 
/// It can be used for cryptographic operations either directly, or via its
/// parent CryptoKey, in which case the server will choose the appropriate
/// version for the operation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys crypto key versions create projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionCreateCall) (request|response)
/// * [locations key rings crypto keys crypto key versions destroy projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionDestroyCall) (response)
/// * [locations key rings crypto keys crypto key versions restore projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionRestoreCall) (response)
/// * [locations key rings crypto keys crypto key versions patch projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionPatchCall) (request|response)
/// * [locations key rings crypto keys crypto key versions get projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CryptoKeyVersion {
    /// Output only. The time at which this CryptoKeyVersion was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The current state of the CryptoKeyVersion.
    
    pub state: Option<String>,
    /// Output only. The resource name for this CryptoKeyVersion in the format
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`.
    
    pub name: Option<String>,
    /// Output only. The time this CryptoKeyVersion's key material was
    /// destroyed. Only present if state is
    /// DESTROYED.
    #[serde(rename="destroyEventTime")]
    
    pub destroy_event_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time this CryptoKeyVersion's key material is scheduled
    /// for destruction. Only present if state is
    /// DESTROY_SCHEDULED.
    #[serde(rename="destroyTime")]
    
    pub destroy_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for CryptoKeyVersion {}
impl client::ResponseResult for CryptoKeyVersion {}


/// Write a Cloud Audit log
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudAuditOptions {
    /// The log_name to populate in the Cloud Audit Record.
    #[serde(rename="logName")]
    
    pub log_name: Option<String>,
}

impl client::Part for CloudAuditOptions {}


/// Associates `members` with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Binding {
    /// Specifies the identities requesting access for a Cloud Platform resource.
    /// `members` can have the following values:
    /// 
    /// * `allUsers`: A special identifier that represents anyone who is
    ///    on the internet; with or without a Google account.
    /// 
    /// * `allAuthenticatedUsers`: A special identifier that represents anyone
    ///    who is authenticated with a Google account or a service account.
    /// 
    /// * `user:{emailid}`: An email address that represents a specific Google
    ///    account. For example, `alice@gmail.com` or `joe@example.com`.
    /// 
    /// 
    /// * `serviceAccount:{emailid}`: An email address that represents a service
    ///    account. For example, `my-other-app@appspot.gserviceaccount.com`.
    /// 
    /// * `group:{emailid}`: An email address that represents a Google group.
    ///    For example, `admins@example.com`.
    /// 
    /// 
    /// * `domain:{domain}`: A Google Apps domain name that represents all the
    ///    users of that domain. For example, `google.com` or `example.com`.
    /// 
    /// 
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to `members`.
    /// For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    /// Required
    
    pub role: Option<String>,
}

impl client::Part for Binding {}


/// Request message for KeyManagementService.Encrypt.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys encrypt projects](ProjectLocationKeyRingCryptoKeyEncryptCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EncryptRequest {
    /// Optional data that, if specified, must also be provided during decryption
    /// through DecryptRequest.additional_authenticated_data.  Must be no
    /// larger than 64KiB.
    #[serde(rename="additionalAuthenticatedData")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub additional_authenticated_data: Option<Vec<u8>>,
    /// Required. The data to encrypt. Must be no larger than 64KiB.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub plaintext: Option<Vec<u8>>,
}

impl client::RequestValue for EncryptRequest {}


/// Response message for KeyManagementService.ListCryptoKeyVersions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys crypto key versions list projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCryptoKeyVersionsResponse {
    /// The list of CryptoKeyVersions.
    #[serde(rename="cryptoKeyVersions")]
    
    pub crypto_key_versions: Option<Vec<CryptoKeyVersion>>,
    /// A token to retrieve next page of results. Pass this value in
    /// ListCryptoKeyVersionsRequest.page_token to retrieve the next page of
    /// results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of CryptoKeyVersions that matched the
    /// query.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListCryptoKeyVersionsResponse {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys test iam permissions projects](ProjectLocationKeyRingCryptoKeyTestIamPermissionCall) (response)
/// * [locations key rings test iam permissions projects](ProjectLocationKeyRingTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is
    /// allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// Request message for KeyManagementService.DestroyCryptoKeyVersion.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys crypto key versions destroy projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionDestroyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DestroyCryptoKeyVersionRequest { _never_set: Option<bool> }

impl client::RequestValue for DestroyCryptoKeyVersionRequest {}


/// A CryptoKey represents a logical key that can be used for cryptographic
/// operations.
/// 
/// A CryptoKey is made up of one or more versions, which
/// represent the actual key material used in cryptographic operations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys patch projects](ProjectLocationKeyRingCryptoKeyPatchCall) (request|response)
/// * [locations key rings crypto keys get projects](ProjectLocationKeyRingCryptoKeyGetCall) (response)
/// * [locations key rings crypto keys create projects](ProjectLocationKeyRingCryptoKeyCreateCall) (request|response)
/// * [locations key rings crypto keys update primary version projects](ProjectLocationKeyRingCryptoKeyUpdatePrimaryVersionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CryptoKey {
    /// Output only. The time at which this CryptoKey was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// next_rotation_time will be advanced by this period when the service
    /// automatically rotates a key. Must be at least one day.
    /// 
    /// If rotation_period is set, next_rotation_time must also be set.
    #[serde(rename="rotationPeriod")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub rotation_period: Option<client::chrono::Duration>,
    /// Output only. A copy of the "primary" CryptoKeyVersion that will be used
    /// by Encrypt when this CryptoKey is given
    /// in EncryptRequest.name.
    /// 
    /// The CryptoKey's primary version can be updated via
    /// UpdateCryptoKeyPrimaryVersion.
    
    pub primary: Option<CryptoKeyVersion>,
    /// Output only. The resource name for this CryptoKey in the format
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    
    pub name: Option<String>,
    /// The immutable purpose of this CryptoKey. Currently, the only acceptable
    /// purpose is ENCRYPT_DECRYPT.
    
    pub purpose: Option<String>,
    /// At next_rotation_time, the Key Management Service will automatically:
    /// 
    /// 1. Create a new version of this CryptoKey.
    /// 2. Mark the new version as primary.
    /// 
    /// Key rotations performed manually via
    /// CreateCryptoKeyVersion and
    /// UpdateCryptoKeyPrimaryVersion
    /// do not affect next_rotation_time.
    #[serde(rename="nextRotationTime")]
    
    pub next_rotation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for CryptoKey {}
impl client::ResponseResult for CryptoKey {}


/// A rule to be applied in a Policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    /// Human-readable description of the rule.
    
    pub description: Option<String>,
    /// Additional restrictions that must be met
    
    pub conditions: Option<Vec<Condition>>,
    /// The config returned to callers of tech.iam.IAM.CheckPolicy for any entries
    /// that match the LOG action.
    #[serde(rename="logConfig")]
    
    pub log_config: Option<Vec<LogConfig>>,
    /// If one or more 'in' clauses are specified, the rule matches if
    /// the PRINCIPAL/AUTHORITY_SELECTOR is in at least one of these entries.
    #[serde(rename="in")]
    
    pub in_: Option<Vec<String>>,
    /// A permission is a string of form '<service>.<resource type>.<verb>'
    /// (e.g., 'storage.buckets.list'). A value of '*' matches all permissions,
    /// and a verb part of '*' (e.g., 'storage.buckets.*') matches all verbs.
    
    pub permissions: Option<Vec<String>>,
    /// Required
    
    pub action: Option<String>,
    /// If one or more 'not_in' clauses are specified, the rule matches
    /// if the PRINCIPAL/AUTHORITY_SELECTOR is in none of the entries.
    /// The format for in and not_in entries is the same as for members in a
    /// Binding (see google/iam/v1/policy.proto).
    #[serde(rename="notIn")]
    
    pub not_in: Option<Vec<String>>,
}

impl client::Part for Rule {}


/// Specifies what kind of log the caller must write
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogConfig {
    /// Counter options.
    
    pub counter: Option<CounterOptions>,
    /// Data access options.
    #[serde(rename="dataAccess")]
    
    pub data_access: Option<DataAccessOptions>,
    /// Cloud audit options.
    #[serde(rename="cloudAudit")]
    
    pub cloud_audit: Option<CloudAuditOptions>,
}

impl client::Part for LogConfig {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys set iam policy projects](ProjectLocationKeyRingCryptoKeySetIamPolicyCall) (request)
/// * [locations key rings set iam policy projects](ProjectLocationKeyRingSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only
    /// the fields in the mask will be modified. If no mask is provided, the
    /// following default mask is used:
    /// paths: "bindings, etag"
    /// This field is only used by Cloud IAM.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of
    /// the policy is limited to a few 10s of KB. An empty policy is a
    /// valid policy but certain Cloud Platform services (such as Projects)
    /// might reject them.
    
    pub policy: Option<Policy>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Request message for KeyManagementService.Decrypt.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys decrypt projects](ProjectLocationKeyRingCryptoKeyDecryptCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DecryptRequest {
    /// Required. The encrypted data originally returned in
    /// EncryptResponse.ciphertext.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub ciphertext: Option<Vec<u8>>,
    /// Optional data that must match the data originally supplied in
    /// EncryptRequest.additional_authenticated_data.
    #[serde(rename="additionalAuthenticatedData")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub additional_authenticated_data: Option<Vec<u8>>,
}

impl client::RequestValue for DecryptRequest {}


/// A resource that represents Google Cloud Platform location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get projects](ProjectLocationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The canonical id for this location. For example: `"us-east1"`.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given
    /// location.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Cross-service attributes for the location. For example
    /// 
    /// ````text
    /// {"cloud.googleapis.com/region": "us-east1"}
    /// ````
    
    pub labels: Option<HashMap<String, String>>,
    /// Resource name for the location, which may vary between implementations.
    /// For example: `"projects/example-project/locations/us-east1"`
    
    pub name: Option<String>,
}

impl client::ResponseResult for Location {}


/// Response message for KeyManagementService.ListCryptoKeys.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys list projects](ProjectLocationKeyRingCryptoKeyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCryptoKeysResponse {
    /// A token to retrieve next page of results. Pass this value in
    /// ListCryptoKeysRequest.page_token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of CryptoKeys.
    #[serde(rename="cryptoKeys")]
    
    pub crypto_keys: Option<Vec<CryptoKey>>,
    /// The total number of CryptoKeys that matched the query.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListCryptoKeysResponse {}


/// A condition to be met.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Condition {
    /// An operator to apply the subject with.
    
    pub op: Option<String>,
    /// Trusted attributes discharged by the service.
    
    pub svc: Option<String>,
    /// Trusted attributes supplied by any service that owns resources and uses
    /// the IAM system for access control.
    
    pub sys: Option<String>,
    /// DEPRECATED. Use 'values' instead.
    
    pub value: Option<String>,
    /// Trusted attributes supplied by the IAM system.
    
    pub iam: Option<String>,
    /// The objects of the condition. This is mutually exclusive with 'value'.
    
    pub values: Option<Vec<String>>,
}

impl client::Part for Condition {}


/// Options for counters
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CounterOptions {
    /// The metric to update.
    
    pub metric: Option<String>,
    /// The field value to attribute.
    
    pub field: Option<String>,
}

impl client::Part for CounterOptions {}


/// Provides the configuration for logging a type of permissions.
/// Example:
/// 
/// ````text
/// {
///   "audit_log_configs": [
///     {
///       "log_type": "DATA_READ",
///       "exempted_members": [
///         "user:foo@gmail.com"
///       ]
///     },
///     {
///       "log_type": "DATA_WRITE",
///     }
///   ]
/// }
/// ````
/// 
/// This enables ‘DATA_READ’ and ‘DATA_WRITE’ logging, while exempting
/// foo@gmail.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of
    /// permission.
    /// Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<String>,
}

impl client::Part for AuditLogConfig {}


/// Response message for KeyManagementService.Decrypt.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys decrypt projects](ProjectLocationKeyRingCryptoKeyDecryptCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DecryptResponse {
    /// The decrypted data originally supplied in EncryptRequest.plaintext.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub plaintext: Option<Vec<u8>>,
}

impl client::ResponseResult for DecryptResponse {}


