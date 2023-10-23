use super::*;
/// Request message for KeyManagementService.AsymmetricDecrypt.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys crypto key versions asymmetric decrypt projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionAsymmetricDecryptCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AsymmetricDecryptRequest {
    /// Required. The data encrypted with the named CryptoKeyVersion's public key using OAEP.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub ciphertext: Option<Vec<u8>>,
    /// Optional. An optional CRC32C checksum of the AsymmetricDecryptRequest.ciphertext. If specified, KeyManagementService will verify the integrity of the received AsymmetricDecryptRequest.ciphertext using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(AsymmetricDecryptRequest.ciphertext) is equal to AsymmetricDecryptRequest.ciphertext_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="ciphertextCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ciphertext_crc32c: Option<i64>,
}

impl client::RequestValue for AsymmetricDecryptRequest {}


/// Response message for KeyManagementService.AsymmetricDecrypt.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys crypto key versions asymmetric decrypt projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionAsymmetricDecryptCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AsymmetricDecryptResponse {
    /// The decrypted data originally encrypted with the matching public key.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub plaintext: Option<Vec<u8>>,
    /// Integrity verification field. A CRC32C checksum of the returned AsymmetricDecryptResponse.plaintext. An integrity check of AsymmetricDecryptResponse.plaintext can be performed by computing the CRC32C checksum of AsymmetricDecryptResponse.plaintext and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="plaintextCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub plaintext_crc32c: Option<i64>,
    /// The ProtectionLevel of the CryptoKeyVersion used in decryption.
    #[serde(rename="protectionLevel")]
    
    pub protection_level: Option<AsymmetricDecryptResponseProtectionLevelEnum>,
    /// Integrity verification field. A flag indicating whether AsymmetricDecryptRequest.ciphertext_crc32c was received by KeyManagementService and used for the integrity verification of the ciphertext. A false value of this field indicates either that AsymmetricDecryptRequest.ciphertext_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set AsymmetricDecryptRequest.ciphertext_crc32c but this field is still false, discard the response and perform a limited number of retries.
    #[serde(rename="verifiedCiphertextCrc32c")]
    
    pub verified_ciphertext_crc32c: Option<bool>,
}

impl client::ResponseResult for AsymmetricDecryptResponse {}


/// Request message for KeyManagementService.AsymmetricSign.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys crypto key versions asymmetric sign projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionAsymmetricSignCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AsymmetricSignRequest {
    /// Optional. The data to sign. It can't be supplied if AsymmetricSignRequest.digest is supplied.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Optional. An optional CRC32C checksum of the AsymmetricSignRequest.data. If specified, KeyManagementService will verify the integrity of the received AsymmetricSignRequest.data using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(AsymmetricSignRequest.data) is equal to AsymmetricSignRequest.data_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="dataCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub data_crc32c: Option<i64>,
    /// Optional. The digest of the data to sign. The digest must be produced with the same digest algorithm as specified by the key version's algorithm. This field may not be supplied if AsymmetricSignRequest.data is supplied.
    
    pub digest: Option<Digest>,
    /// Optional. An optional CRC32C checksum of the AsymmetricSignRequest.digest. If specified, KeyManagementService will verify the integrity of the received AsymmetricSignRequest.digest using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(AsymmetricSignRequest.digest) is equal to AsymmetricSignRequest.digest_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="digestCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub digest_crc32c: Option<i64>,
}

impl client::RequestValue for AsymmetricSignRequest {}


/// Response message for KeyManagementService.AsymmetricSign.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys crypto key versions asymmetric sign projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionAsymmetricSignCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AsymmetricSignResponse {
    /// The resource name of the CryptoKeyVersion used for signing. Check this field to verify that the intended resource was used for signing.
    
    pub name: Option<String>,
    /// The ProtectionLevel of the CryptoKeyVersion used for signing.
    #[serde(rename="protectionLevel")]
    
    pub protection_level: Option<AsymmetricSignResponseProtectionLevelEnum>,
    /// The created signature.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub signature: Option<Vec<u8>>,
    /// Integrity verification field. A CRC32C checksum of the returned AsymmetricSignResponse.signature. An integrity check of AsymmetricSignResponse.signature can be performed by computing the CRC32C checksum of AsymmetricSignResponse.signature and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="signatureCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub signature_crc32c: Option<i64>,
    /// Integrity verification field. A flag indicating whether AsymmetricSignRequest.data_crc32c was received by KeyManagementService and used for the integrity verification of the data. A false value of this field indicates either that AsymmetricSignRequest.data_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set AsymmetricSignRequest.data_crc32c but this field is still false, discard the response and perform a limited number of retries.
    #[serde(rename="verifiedDataCrc32c")]
    
    pub verified_data_crc32c: Option<bool>,
    /// Integrity verification field. A flag indicating whether AsymmetricSignRequest.digest_crc32c was received by KeyManagementService and used for the integrity verification of the digest. A false value of this field indicates either that AsymmetricSignRequest.digest_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set AsymmetricSignRequest.digest_crc32c but this field is still false, discard the response and perform a limited number of retries.
    #[serde(rename="verifiedDigestCrc32c")]
    
    pub verified_digest_crc32c: Option<bool>,
}

impl client::ResponseResult for AsymmetricSignResponse {}


/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts `jose@example.com` from DATA_READ logging, and `aliya@example.com` from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
    
    pub service: Option<String>,
}

impl client::Part for AuditConfig {}


/// Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<AuditLogConfigLogTypeEnum>,
}

impl client::Part for AuditLogConfig {}


/// Associates `members`, or principals, with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Binding {
    /// The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. 
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    
    pub role: Option<String>,
}

impl client::Part for Binding {}


/// A Certificate represents an X.509 certificate used to authenticate HTTPS connections to EKM replicas.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Certificate {
    /// Output only. The issuer distinguished name in RFC 2253 format. Only present if parsed is true.
    
    pub issuer: Option<String>,
    /// Output only. The certificate is not valid after this time. Only present if parsed is true.
    #[serde(rename="notAfterTime")]
    
    pub not_after_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The certificate is not valid before this time. Only present if parsed is true.
    #[serde(rename="notBeforeTime")]
    
    pub not_before_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. True if the certificate was parsed successfully.
    
    pub parsed: Option<bool>,
    /// Required. The raw certificate bytes in DER format.
    #[serde(rename="rawDer")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub raw_der: Option<Vec<u8>>,
    /// Output only. The certificate serial number as a hex string. Only present if parsed is true.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
    /// Output only. The SHA-256 certificate fingerprint as a hex string. Only present if parsed is true.
    #[serde(rename="sha256Fingerprint")]
    
    pub sha256_fingerprint: Option<String>,
    /// Output only. The subject distinguished name in RFC 2253 format. Only present if parsed is true.
    
    pub subject: Option<String>,
    /// Output only. The subject Alternative DNS names. Only present if parsed is true.
    #[serde(rename="subjectAlternativeDnsNames")]
    
    pub subject_alternative_dns_names: Option<Vec<String>>,
}

impl client::Part for Certificate {}


/// Certificate chains needed to verify the attestation. Certificates in chains are PEM-encoded and are ordered based on https://tools.ietf.org/html/rfc5246#section-7.4.2.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateChains {
    /// Cavium certificate chain corresponding to the attestation.
    #[serde(rename="caviumCerts")]
    
    pub cavium_certs: Option<Vec<String>>,
    /// Google card certificate chain corresponding to the attestation.
    #[serde(rename="googleCardCerts")]
    
    pub google_card_certs: Option<Vec<String>>,
    /// Google partition certificate chain corresponding to the attestation.
    #[serde(rename="googlePartitionCerts")]
    
    pub google_partition_certs: Option<Vec<String>>,
}

impl client::Part for CertificateChains {}


/// A CryptoKey represents a logical key that can be used for cryptographic operations. A CryptoKey is made up of zero or more versions, which represent the actual key material used in cryptographic operations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys create projects](ProjectLocationKeyRingCryptoKeyCreateCall) (request|response)
/// * [locations key rings crypto keys get projects](ProjectLocationKeyRingCryptoKeyGetCall) (response)
/// * [locations key rings crypto keys patch projects](ProjectLocationKeyRingCryptoKeyPatchCall) (request|response)
/// * [locations key rings crypto keys update primary version projects](ProjectLocationKeyRingCryptoKeyUpdatePrimaryVersionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CryptoKey {
    /// Output only. The time at which this CryptoKey was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. The resource name of the backend environment where the key material for all CryptoKeyVersions associated with this CryptoKey reside and where all related cryptographic operations are performed. Only applicable if CryptoKeyVersions have a ProtectionLevel of EXTERNAL_VPC, with the resource name in the format `projects/*/locations/*/ekmConnections/*`. Note, this list is non-exhaustive and may apply to additional ProtectionLevels in the future.
    #[serde(rename="cryptoKeyBackend")]
    
    pub crypto_key_backend: Option<String>,
    /// Immutable. The period of time that versions of this key spend in the DESTROY_SCHEDULED state before transitioning to DESTROYED. If not specified at creation time, the default duration is 24 hours.
    #[serde(rename="destroyScheduledDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub destroy_scheduled_duration: Option<client::chrono::Duration>,
    /// Immutable. Whether this key may contain imported versions only.
    #[serde(rename="importOnly")]
    
    pub import_only: Option<bool>,
    /// Labels with user-defined metadata. For more information, see [Labeling Keys](https://cloud.google.com/kms/docs/labeling-keys).
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The resource name for this CryptoKey in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    
    pub name: Option<String>,
    /// At next_rotation_time, the Key Management Service will automatically: 1. Create a new version of this CryptoKey. 2. Mark the new version as primary. Key rotations performed manually via CreateCryptoKeyVersion and UpdateCryptoKeyPrimaryVersion do not affect next_rotation_time. Keys with purpose ENCRYPT_DECRYPT support automatic rotation. For other keys, this field must be omitted.
    #[serde(rename="nextRotationTime")]
    
    pub next_rotation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. A copy of the "primary" CryptoKeyVersion that will be used by Encrypt when this CryptoKey is given in EncryptRequest.name. The CryptoKey's primary version can be updated via UpdateCryptoKeyPrimaryVersion. Keys with purpose ENCRYPT_DECRYPT may have a primary. For other keys, this field will be omitted.
    
    pub primary: Option<CryptoKeyVersion>,
    /// Immutable. The immutable purpose of this CryptoKey.
    
    pub purpose: Option<CryptoKeyPurposeEnum>,
    /// next_rotation_time will be advanced by this period when the service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours. If rotation_period is set, next_rotation_time must also be set. Keys with purpose ENCRYPT_DECRYPT support automatic rotation. For other keys, this field must be omitted.
    #[serde(rename="rotationPeriod")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub rotation_period: Option<client::chrono::Duration>,
    /// A template describing settings for new CryptoKeyVersion instances. The properties of new CryptoKeyVersion instances created by either CreateCryptoKeyVersion or auto-rotation are controlled by this template.
    #[serde(rename="versionTemplate")]
    
    pub version_template: Option<CryptoKeyVersionTemplate>,
}

impl client::RequestValue for CryptoKey {}
impl client::ResponseResult for CryptoKey {}


/// A CryptoKeyVersion represents an individual cryptographic key, and the associated key material. An ENABLED version can be used for cryptographic operations. For security reasons, the raw cryptographic key material represented by a CryptoKeyVersion can never be viewed or exported. It can only be used to encrypt, decrypt, or sign data when an authorized user or application invokes Cloud KMS.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys crypto key versions create projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionCreateCall) (request|response)
/// * [locations key rings crypto keys crypto key versions destroy projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionDestroyCall) (response)
/// * [locations key rings crypto keys crypto key versions get projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionGetCall) (response)
/// * [locations key rings crypto keys crypto key versions import projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionImportCall) (response)
/// * [locations key rings crypto keys crypto key versions patch projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionPatchCall) (request|response)
/// * [locations key rings crypto keys crypto key versions restore projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionRestoreCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CryptoKeyVersion {
    /// Output only. The CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports.
    
    pub algorithm: Option<CryptoKeyVersionAlgorithmEnum>,
    /// Output only. Statement that was generated and signed by the HSM at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google. Only provided for key versions with protection_level HSM.
    
    pub attestation: Option<KeyOperationAttestation>,
    /// Output only. The time at which this CryptoKeyVersion was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time this CryptoKeyVersion's key material was destroyed. Only present if state is DESTROYED.
    #[serde(rename="destroyEventTime")]
    
    pub destroy_event_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time this CryptoKeyVersion's key material is scheduled for destruction. Only present if state is DESTROY_SCHEDULED.
    #[serde(rename="destroyTime")]
    
    pub destroy_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// ExternalProtectionLevelOptions stores a group of additional fields for configuring a CryptoKeyVersion that are specific to the EXTERNAL protection level and EXTERNAL_VPC protection levels.
    #[serde(rename="externalProtectionLevelOptions")]
    
    pub external_protection_level_options: Option<ExternalProtectionLevelOptions>,
    /// Output only. The time this CryptoKeyVersion's key material was generated.
    #[serde(rename="generateTime")]
    
    pub generate_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The root cause of the most recent import failure. Only present if state is IMPORT_FAILED.
    #[serde(rename="importFailureReason")]
    
    pub import_failure_reason: Option<String>,
    /// Output only. The name of the ImportJob used in the most recent import of this CryptoKeyVersion. Only present if the underlying key material was imported.
    #[serde(rename="importJob")]
    
    pub import_job: Option<String>,
    /// Output only. The time at which this CryptoKeyVersion's key material was most recently imported.
    #[serde(rename="importTime")]
    
    pub import_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The resource name for this CryptoKeyVersion in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`.
    
    pub name: Option<String>,
    /// Output only. The ProtectionLevel describing how crypto operations are performed with this CryptoKeyVersion.
    #[serde(rename="protectionLevel")]
    
    pub protection_level: Option<CryptoKeyVersionProtectionLevelEnum>,
    /// Output only. Whether or not this key version is eligible for reimport, by being specified as a target in ImportCryptoKeyVersionRequest.crypto_key_version.
    #[serde(rename="reimportEligible")]
    
    pub reimport_eligible: Option<bool>,
    /// The current state of the CryptoKeyVersion.
    
    pub state: Option<CryptoKeyVersionStateEnum>,
}

impl client::RequestValue for CryptoKeyVersion {}
impl client::ResponseResult for CryptoKeyVersion {}


/// A CryptoKeyVersionTemplate specifies the properties to use when creating a new CryptoKeyVersion, either manually with CreateCryptoKeyVersion or automatically as a result of auto-rotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CryptoKeyVersionTemplate {
    /// Required. Algorithm to use when creating a CryptoKeyVersion based on this template. For backwards compatibility, GOOGLE_SYMMETRIC_ENCRYPTION is implied if both this field is omitted and CryptoKey.purpose is ENCRYPT_DECRYPT.
    
    pub algorithm: Option<CryptoKeyVersionTemplateAlgorithmEnum>,
    /// ProtectionLevel to use when creating a CryptoKeyVersion based on this template. Immutable. Defaults to SOFTWARE.
    #[serde(rename="protectionLevel")]
    
    pub protection_level: Option<CryptoKeyVersionTemplateProtectionLevelEnum>,
}

impl client::Part for CryptoKeyVersionTemplate {}


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
    /// Optional. Optional data that must match the data originally supplied in EncryptRequest.additional_authenticated_data.
    #[serde(rename="additionalAuthenticatedData")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub additional_authenticated_data: Option<Vec<u8>>,
    /// Optional. An optional CRC32C checksum of the DecryptRequest.additional_authenticated_data. If specified, KeyManagementService will verify the integrity of the received DecryptRequest.additional_authenticated_data using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(DecryptRequest.additional_authenticated_data) is equal to DecryptRequest.additional_authenticated_data_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="additionalAuthenticatedDataCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub additional_authenticated_data_crc32c: Option<i64>,
    /// Required. The encrypted data originally returned in EncryptResponse.ciphertext.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub ciphertext: Option<Vec<u8>>,
    /// Optional. An optional CRC32C checksum of the DecryptRequest.ciphertext. If specified, KeyManagementService will verify the integrity of the received DecryptRequest.ciphertext using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(DecryptRequest.ciphertext) is equal to DecryptRequest.ciphertext_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="ciphertextCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ciphertext_crc32c: Option<i64>,
}

impl client::RequestValue for DecryptRequest {}


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
    /// Integrity verification field. A CRC32C checksum of the returned DecryptResponse.plaintext. An integrity check of DecryptResponse.plaintext can be performed by computing the CRC32C checksum of DecryptResponse.plaintext and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: receiving this response message indicates that KeyManagementService is able to successfully decrypt the ciphertext. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="plaintextCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub plaintext_crc32c: Option<i64>,
    /// The ProtectionLevel of the CryptoKeyVersion used in decryption.
    #[serde(rename="protectionLevel")]
    
    pub protection_level: Option<DecryptResponseProtectionLevelEnum>,
    /// Whether the Decryption was performed using the primary key version.
    #[serde(rename="usedPrimary")]
    
    pub used_primary: Option<bool>,
}

impl client::ResponseResult for DecryptResponse {}


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


/// A Digest holds a cryptographic message digest.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Digest {
    /// A message digest produced with the SHA-256 algorithm.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sha256: Option<Vec<u8>>,
    /// A message digest produced with the SHA-384 algorithm.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sha384: Option<Vec<u8>>,
    /// A message digest produced with the SHA-512 algorithm.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sha512: Option<Vec<u8>>,
}

impl client::Part for Digest {}


/// An EkmConnection represents an individual EKM connection. It can be used for creating CryptoKeys and CryptoKeyVersions with a ProtectionLevel of EXTERNAL_VPC, as well as performing cryptographic operations using keys created within the EkmConnection.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations ekm connections create projects](ProjectLocationEkmConnectionCreateCall) (request|response)
/// * [locations ekm connections get projects](ProjectLocationEkmConnectionGetCall) (response)
/// * [locations ekm connections patch projects](ProjectLocationEkmConnectionPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EkmConnection {
    /// Output only. The time at which the EkmConnection was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Etag of the currently stored EkmConnection.
    
    pub etag: Option<String>,
    /// Output only. The resource name for the EkmConnection in the format `projects/*/locations/*/ekmConnections/*`.
    
    pub name: Option<String>,
    /// A list of ServiceResolvers where the EKM can be reached. There should be one ServiceResolver per EKM replica. Currently, only a single ServiceResolver is supported.
    #[serde(rename="serviceResolvers")]
    
    pub service_resolvers: Option<Vec<ServiceResolver>>,
}

impl client::RequestValue for EkmConnection {}
impl client::ResponseResult for EkmConnection {}


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
    /// Optional. Optional data that, if specified, must also be provided during decryption through DecryptRequest.additional_authenticated_data. The maximum size depends on the key version's protection_level. For SOFTWARE, EXTERNAL, and EXTERNAL_VPC keys the AAD must be no larger than 64KiB. For HSM keys, the combined length of the plaintext and additional_authenticated_data fields must be no larger than 8KiB.
    #[serde(rename="additionalAuthenticatedData")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub additional_authenticated_data: Option<Vec<u8>>,
    /// Optional. An optional CRC32C checksum of the EncryptRequest.additional_authenticated_data. If specified, KeyManagementService will verify the integrity of the received EncryptRequest.additional_authenticated_data using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(EncryptRequest.additional_authenticated_data) is equal to EncryptRequest.additional_authenticated_data_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="additionalAuthenticatedDataCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub additional_authenticated_data_crc32c: Option<i64>,
    /// Required. The data to encrypt. Must be no larger than 64KiB. The maximum size depends on the key version's protection_level. For SOFTWARE, EXTERNAL, and EXTERNAL_VPC keys, the plaintext must be no larger than 64KiB. For HSM keys, the combined length of the plaintext and additional_authenticated_data fields must be no larger than 8KiB.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub plaintext: Option<Vec<u8>>,
    /// Optional. An optional CRC32C checksum of the EncryptRequest.plaintext. If specified, KeyManagementService will verify the integrity of the received EncryptRequest.plaintext using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(EncryptRequest.plaintext) is equal to EncryptRequest.plaintext_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="plaintextCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub plaintext_crc32c: Option<i64>,
}

impl client::RequestValue for EncryptRequest {}


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
    /// The encrypted data.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub ciphertext: Option<Vec<u8>>,
    /// Integrity verification field. A CRC32C checksum of the returned EncryptResponse.ciphertext. An integrity check of EncryptResponse.ciphertext can be performed by computing the CRC32C checksum of EncryptResponse.ciphertext and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="ciphertextCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ciphertext_crc32c: Option<i64>,
    /// The resource name of the CryptoKeyVersion used in encryption. Check this field to verify that the intended resource was used for encryption.
    
    pub name: Option<String>,
    /// The ProtectionLevel of the CryptoKeyVersion used in encryption.
    #[serde(rename="protectionLevel")]
    
    pub protection_level: Option<EncryptResponseProtectionLevelEnum>,
    /// Integrity verification field. A flag indicating whether EncryptRequest.additional_authenticated_data_crc32c was received by KeyManagementService and used for the integrity verification of the AAD. A false value of this field indicates either that EncryptRequest.additional_authenticated_data_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set EncryptRequest.additional_authenticated_data_crc32c but this field is still false, discard the response and perform a limited number of retries.
    #[serde(rename="verifiedAdditionalAuthenticatedDataCrc32c")]
    
    pub verified_additional_authenticated_data_crc32c: Option<bool>,
    /// Integrity verification field. A flag indicating whether EncryptRequest.plaintext_crc32c was received by KeyManagementService and used for the integrity verification of the plaintext. A false value of this field indicates either that EncryptRequest.plaintext_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set EncryptRequest.plaintext_crc32c but this field is still false, discard the response and perform a limited number of retries.
    #[serde(rename="verifiedPlaintextCrc32c")]
    
    pub verified_plaintext_crc32c: Option<bool>,
}

impl client::ResponseResult for EncryptResponse {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Expr {
    /// Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax.
    
    pub expression: Option<String>,
    /// Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for Expr {}


/// ExternalProtectionLevelOptions stores a group of additional fields for configuring a CryptoKeyVersion that are specific to the EXTERNAL protection level and EXTERNAL_VPC protection levels.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExternalProtectionLevelOptions {
    /// The path to the external key material on the EKM when using EkmConnection e.g., "v0/my/key". Set this field instead of external_key_uri when using an EkmConnection.
    #[serde(rename="ekmConnectionKeyPath")]
    
    pub ekm_connection_key_path: Option<String>,
    /// The URI for an external resource that this CryptoKeyVersion represents.
    #[serde(rename="externalKeyUri")]
    
    pub external_key_uri: Option<String>,
}

impl client::Part for ExternalProtectionLevelOptions {}


/// Request message for KeyManagementService.GenerateRandomBytes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations generate random bytes projects](ProjectLocationGenerateRandomByteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateRandomBytesRequest {
    /// The length in bytes of the amount of randomness to retrieve. Minimum 8 bytes, maximum 1024 bytes.
    #[serde(rename="lengthBytes")]
    
    pub length_bytes: Option<i32>,
    /// The ProtectionLevel to use when generating the random data. Currently, only HSM protection level is supported.
    #[serde(rename="protectionLevel")]
    
    pub protection_level: Option<GenerateRandomBytesRequestProtectionLevelEnum>,
}

impl client::RequestValue for GenerateRandomBytesRequest {}


/// Response message for KeyManagementService.GenerateRandomBytes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations generate random bytes projects](ProjectLocationGenerateRandomByteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateRandomBytesResponse {
    /// The generated data.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Integrity verification field. A CRC32C checksum of the returned GenerateRandomBytesResponse.data. An integrity check of GenerateRandomBytesResponse.data can be performed by computing the CRC32C checksum of GenerateRandomBytesResponse.data and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="dataCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub data_crc32c: Option<i64>,
}

impl client::ResponseResult for GenerateRandomBytesResponse {}


/// Request message for KeyManagementService.ImportCryptoKeyVersion.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys crypto key versions import projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportCryptoKeyVersionRequest {
    /// Required. The algorithm of the key being imported. This does not need to match the version_template of the CryptoKey this version imports into.
    
    pub algorithm: Option<ImportCryptoKeyVersionRequestAlgorithmEnum>,
    /// Optional. The optional name of an existing CryptoKeyVersion to target for an import operation. If this field is not present, a new CryptoKeyVersion containing the supplied key material is created. If this field is present, the supplied key material is imported into the existing CryptoKeyVersion. To import into an existing CryptoKeyVersion, the CryptoKeyVersion must be a child of ImportCryptoKeyVersionRequest.parent, have been previously created via ImportCryptoKeyVersion, and be in DESTROYED or IMPORT_FAILED state. The key material and algorithm must match the previous CryptoKeyVersion exactly if the CryptoKeyVersion has ever contained key material.
    #[serde(rename="cryptoKeyVersion")]
    
    pub crypto_key_version: Option<String>,
    /// Required. The name of the ImportJob that was used to wrap this key material.
    #[serde(rename="importJob")]
    
    pub import_job: Option<String>,
    /// Optional. This field has the same meaning as wrapped_key. Prefer to use that field in new work. Either that field or this field (but not both) must be specified.
    #[serde(rename="rsaAesWrappedKey")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub rsa_aes_wrapped_key: Option<Vec<u8>>,
    /// Optional. The wrapped key material to import. Before wrapping, key material must be formatted. If importing symmetric key material, the expected key material format is plain bytes. If importing asymmetric key material, the expected key material format is PKCS#8-encoded DER (the PrivateKeyInfo structure from RFC 5208). When wrapping with import methods (RSA_OAEP_3072_SHA1_AES_256 or RSA_OAEP_4096_SHA1_AES_256 or RSA_OAEP_3072_SHA256_AES_256 or RSA_OAEP_4096_SHA256_AES_256), this field must contain the concatenation of: 1. An ephemeral AES-256 wrapping key wrapped with the public_key using RSAES-OAEP with SHA-1/SHA-256, MGF1 with SHA-1/SHA-256, and an empty label. 2. The formatted key to be imported, wrapped with the ephemeral AES-256 key using AES-KWP (RFC 5649). This format is the same as the format produced by PKCS#11 mechanism CKM_RSA_AES_KEY_WRAP. When wrapping with import methods (RSA_OAEP_3072_SHA256 or RSA_OAEP_4096_SHA256), this field must contain the formatted key to be imported, wrapped with the public_key using RSAES-OAEP with SHA-256, MGF1 with SHA-256, and an empty label.
    #[serde(rename="wrappedKey")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub wrapped_key: Option<Vec<u8>>,
}

impl client::RequestValue for ImportCryptoKeyVersionRequest {}


/// An ImportJob can be used to create CryptoKeys and CryptoKeyVersions using pre-existing key material, generated outside of Cloud KMS. When an ImportJob is created, Cloud KMS will generate a “wrapping key”, which is a public/private key pair. You use the wrapping key to encrypt (also known as wrap) the pre-existing key material to protect it during the import process. The nature of the wrapping key depends on the choice of import_method. When the wrapping key generation is complete, the state will be set to ACTIVE and the public_key can be fetched. The fetched public key can then be used to wrap your pre-existing key material. Once the key material is wrapped, it can be imported into a new CryptoKeyVersion in an existing CryptoKey by calling ImportCryptoKeyVersion. Multiple CryptoKeyVersions can be imported with a single ImportJob. Cloud KMS uses the private key portion of the wrapping key to unwrap the key material. Only Cloud KMS has access to the private key. An ImportJob expires 3 days after it is created. Once expired, Cloud KMS will no longer be able to import or unwrap any key material that was wrapped with the ImportJob’s public key. For more information, see [Importing a key](https://cloud.google.com/kms/docs/importing-a-key).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings import jobs create projects](ProjectLocationKeyRingImportJobCreateCall) (request|response)
/// * [locations key rings import jobs get projects](ProjectLocationKeyRingImportJobGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportJob {
    /// Output only. Statement that was generated and signed by the key creator (for example, an HSM) at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google. Only present if the chosen ImportMethod is one with a protection level of HSM.
    
    pub attestation: Option<KeyOperationAttestation>,
    /// Output only. The time at which this ImportJob was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time this ImportJob expired. Only present if state is EXPIRED.
    #[serde(rename="expireEventTime")]
    
    pub expire_event_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time at which this ImportJob is scheduled for expiration and can no longer be used to import key material.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time this ImportJob's key material was generated.
    #[serde(rename="generateTime")]
    
    pub generate_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Immutable. The wrapping method to be used for incoming key material.
    #[serde(rename="importMethod")]
    
    pub import_method: Option<ImportJobImportMethodEnum>,
    /// Output only. The resource name for this ImportJob in the format `projects/*/locations/*/keyRings/*/importJobs/*`.
    
    pub name: Option<String>,
    /// Required. Immutable. The protection level of the ImportJob. This must match the protection_level of the version_template on the CryptoKey you attempt to import into.
    #[serde(rename="protectionLevel")]
    
    pub protection_level: Option<ImportJobProtectionLevelEnum>,
    /// Output only. The public key with which to wrap key material prior to import. Only returned if state is ACTIVE.
    #[serde(rename="publicKey")]
    
    pub public_key: Option<WrappingPublicKey>,
    /// Output only. The current state of the ImportJob, indicating if it can be used.
    
    pub state: Option<ImportJobStateEnum>,
}

impl client::RequestValue for ImportJob {}
impl client::ResponseResult for ImportJob {}


/// Contains an HSM-generated attestation about a key operation. For more information, see [Verifying attestations] (https://cloud.google.com/kms/docs/attest-key).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyOperationAttestation {
    /// Output only. The certificate chains needed to validate the attestation
    #[serde(rename="certChains")]
    
    pub cert_chains: Option<CertificateChains>,
    /// Output only. The attestation data provided by the HSM when the key operation was performed.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub content: Option<Vec<u8>>,
    /// Output only. The format of the attestation data.
    
    pub format: Option<KeyOperationAttestationFormatEnum>,
}

impl client::Part for KeyOperationAttestation {}


/// A KeyRing is a toplevel logical grouping of CryptoKeys.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings create projects](ProjectLocationKeyRingCreateCall) (request|response)
/// * [locations key rings get projects](ProjectLocationKeyRingGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyRing {
    /// Output only. The time at which this KeyRing was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The resource name for the KeyRing in the format `projects/*/locations/*/keyRings/*`.
    
    pub name: Option<String>,
}

impl client::RequestValue for KeyRing {}
impl client::ResponseResult for KeyRing {}


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
    /// A token to retrieve next page of results. Pass this value in ListCryptoKeyVersionsRequest.page_token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of CryptoKeyVersions that matched the query.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListCryptoKeyVersionsResponse {}


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
    /// The list of CryptoKeys.
    #[serde(rename="cryptoKeys")]
    
    pub crypto_keys: Option<Vec<CryptoKey>>,
    /// A token to retrieve next page of results. Pass this value in ListCryptoKeysRequest.page_token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of CryptoKeys that matched the query.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListCryptoKeysResponse {}


/// Response message for EkmService.ListEkmConnections.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations ekm connections list projects](ProjectLocationEkmConnectionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListEkmConnectionsResponse {
    /// The list of EkmConnections.
    #[serde(rename="ekmConnections")]
    
    pub ekm_connections: Option<Vec<EkmConnection>>,
    /// A token to retrieve next page of results. Pass this value in ListEkmConnectionsRequest.page_token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of EkmConnections that matched the query.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListEkmConnectionsResponse {}


/// Response message for KeyManagementService.ListImportJobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings import jobs list projects](ProjectLocationKeyRingImportJobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListImportJobsResponse {
    /// The list of ImportJobs.
    #[serde(rename="importJobs")]
    
    pub import_jobs: Option<Vec<ImportJob>>,
    /// A token to retrieve next page of results. Pass this value in ListImportJobsRequest.page_token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of ImportJobs that matched the query.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListImportJobsResponse {}


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
    /// The list of KeyRings.
    #[serde(rename="keyRings")]
    
    pub key_rings: Option<Vec<KeyRing>>,
    /// A token to retrieve next page of results. Pass this value in ListKeyRingsRequest.page_token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of KeyRings that matched the query.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListKeyRingsResponse {}


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
    /// The friendly name for this location, typically a nearby city name. For example, "Tokyo".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}
    
    pub labels: Option<HashMap<String, String>>,
    /// The canonical id for this location. For example: `"us-east1"`.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given location.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"`
    
    pub name: Option<String>,
}

impl client::ResponseResult for Location {}


/// Request message for KeyManagementService.MacSign.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys crypto key versions mac sign projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionMacSignCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MacSignRequest {
    /// Required. The data to sign. The MAC tag is computed over this data field based on the specific algorithm.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Optional. An optional CRC32C checksum of the MacSignRequest.data. If specified, KeyManagementService will verify the integrity of the received MacSignRequest.data using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(MacSignRequest.data) is equal to MacSignRequest.data_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="dataCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub data_crc32c: Option<i64>,
}

impl client::RequestValue for MacSignRequest {}


/// Response message for KeyManagementService.MacSign.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys crypto key versions mac sign projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionMacSignCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MacSignResponse {
    /// The created signature.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub mac: Option<Vec<u8>>,
    /// Integrity verification field. A CRC32C checksum of the returned MacSignResponse.mac. An integrity check of MacSignResponse.mac can be performed by computing the CRC32C checksum of MacSignResponse.mac and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="macCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub mac_crc32c: Option<i64>,
    /// The resource name of the CryptoKeyVersion used for signing. Check this field to verify that the intended resource was used for signing.
    
    pub name: Option<String>,
    /// The ProtectionLevel of the CryptoKeyVersion used for signing.
    #[serde(rename="protectionLevel")]
    
    pub protection_level: Option<MacSignResponseProtectionLevelEnum>,
    /// Integrity verification field. A flag indicating whether MacSignRequest.data_crc32c was received by KeyManagementService and used for the integrity verification of the data. A false value of this field indicates either that MacSignRequest.data_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set MacSignRequest.data_crc32c but this field is still false, discard the response and perform a limited number of retries.
    #[serde(rename="verifiedDataCrc32c")]
    
    pub verified_data_crc32c: Option<bool>,
}

impl client::ResponseResult for MacSignResponse {}


/// Request message for KeyManagementService.MacVerify.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys crypto key versions mac verify projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionMacVerifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MacVerifyRequest {
    /// Required. The data used previously as a MacSignRequest.data to generate the MAC tag.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Optional. An optional CRC32C checksum of the MacVerifyRequest.data. If specified, KeyManagementService will verify the integrity of the received MacVerifyRequest.data using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(MacVerifyRequest.data) is equal to MacVerifyRequest.data_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="dataCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub data_crc32c: Option<i64>,
    /// Required. The signature to verify.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub mac: Option<Vec<u8>>,
    /// Optional. An optional CRC32C checksum of the MacVerifyRequest.mac. If specified, KeyManagementService will verify the integrity of the received MacVerifyRequest.mac using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(MacVerifyRequest.tag) is equal to MacVerifyRequest.mac_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type.
    #[serde(rename="macCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub mac_crc32c: Option<i64>,
}

impl client::RequestValue for MacVerifyRequest {}


/// Response message for KeyManagementService.MacVerify.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys crypto key versions mac verify projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionMacVerifyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MacVerifyResponse {
    /// The resource name of the CryptoKeyVersion used for verification. Check this field to verify that the intended resource was used for verification.
    
    pub name: Option<String>,
    /// The ProtectionLevel of the CryptoKeyVersion used for verification.
    #[serde(rename="protectionLevel")]
    
    pub protection_level: Option<MacVerifyResponseProtectionLevelEnum>,
    /// This field indicates whether or not the verification operation for MacVerifyRequest.mac over MacVerifyRequest.data was successful.
    
    pub success: Option<bool>,
    /// Integrity verification field. A flag indicating whether MacVerifyRequest.data_crc32c was received by KeyManagementService and used for the integrity verification of the data. A false value of this field indicates either that MacVerifyRequest.data_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set MacVerifyRequest.data_crc32c but this field is still false, discard the response and perform a limited number of retries.
    #[serde(rename="verifiedDataCrc32c")]
    
    pub verified_data_crc32c: Option<bool>,
    /// Integrity verification field. A flag indicating whether MacVerifyRequest.mac_crc32c was received by KeyManagementService and used for the integrity verification of the data. A false value of this field indicates either that MacVerifyRequest.mac_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set MacVerifyRequest.mac_crc32c but this field is still false, discard the response and perform a limited number of retries.
    #[serde(rename="verifiedMacCrc32c")]
    
    pub verified_mac_crc32c: Option<bool>,
    /// Integrity verification field. This value is used for the integrity verification of [MacVerifyResponse.success]. If the value of this field contradicts the value of [MacVerifyResponse.success], discard the response and perform a limited number of retries.
    #[serde(rename="verifiedSuccessIntegrity")]
    
    pub verified_success_integrity: Option<bool>,
}

impl client::ResponseResult for MacVerifyResponse {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations ekm config get iam policy projects](ProjectLocationEkmConfigGetIamPolicyCall) (response)
/// * [locations ekm config set iam policy projects](ProjectLocationEkmConfigSetIamPolicyCall) (response)
/// * [locations ekm connections get iam policy projects](ProjectLocationEkmConnectionGetIamPolicyCall) (response)
/// * [locations ekm connections set iam policy projects](ProjectLocationEkmConnectionSetIamPolicyCall) (response)
/// * [locations key rings crypto keys get iam policy projects](ProjectLocationKeyRingCryptoKeyGetIamPolicyCall) (response)
/// * [locations key rings crypto keys set iam policy projects](ProjectLocationKeyRingCryptoKeySetIamPolicyCall) (response)
/// * [locations key rings import jobs get iam policy projects](ProjectLocationKeyRingImportJobGetIamPolicyCall) (response)
/// * [locations key rings import jobs set iam policy projects](ProjectLocationKeyRingImportJobSetIamPolicyCall) (response)
/// * [locations key rings get iam policy projects](ProjectLocationKeyRingGetIamPolicyCall) (response)
/// * [locations key rings set iam policy projects](ProjectLocationKeyRingSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<AuditConfig>>,
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// The public key for a given CryptoKeyVersion. Obtained via GetPublicKey.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations key rings crypto keys crypto key versions get public key projects](ProjectLocationKeyRingCryptoKeyCryptoKeyVersionGetPublicKeyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublicKey {
    /// The Algorithm associated with this key.
    
    pub algorithm: Option<PublicKeyAlgorithmEnum>,
    /// The name of the CryptoKeyVersion public key. Provided here for verification. NOTE: This field is in Beta.
    
    pub name: Option<String>,
    /// The public key, encoded in PEM format. For more information, see the [RFC 7468](https://tools.ietf.org/html/rfc7468) sections for [General Considerations](https://tools.ietf.org/html/rfc7468#section-2) and [Textual Encoding of Subject Public Key Info] (https://tools.ietf.org/html/rfc7468#section-13).
    
    pub pem: Option<String>,
    /// Integrity verification field. A CRC32C checksum of the returned PublicKey.pem. An integrity check of PublicKey.pem can be performed by computing the CRC32C checksum of PublicKey.pem and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type. NOTE: This field is in Beta.
    #[serde(rename="pemCrc32c")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pem_crc32c: Option<i64>,
    /// The ProtectionLevel of the CryptoKeyVersion public key.
    #[serde(rename="protectionLevel")]
    
    pub protection_level: Option<PublicKeyProtectionLevelEnum>,
}

impl client::ResponseResult for PublicKey {}


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


/// A ServiceResolver represents an EKM replica that can be reached within an EkmConnection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceResolver {
    /// Optional. The filter applied to the endpoints of the resolved service. If no filter is specified, all endpoints will be considered. An endpoint will be chosen arbitrarily from the filtered list for each request. For endpoint filter syntax and examples, see https://cloud.google.com/service-directory/docs/reference/rpc/google.cloud.servicedirectory.v1#resolveservicerequest.
    #[serde(rename="endpointFilter")]
    
    pub endpoint_filter: Option<String>,
    /// Required. The hostname of the EKM replica used at TLS and HTTP layers.
    
    pub hostname: Option<String>,
    /// Required. A list of leaf server certificates used to authenticate HTTPS connections to the EKM replica. Currently, a maximum of 10 Certificate is supported.
    #[serde(rename="serverCertificates")]
    
    pub server_certificates: Option<Vec<Certificate>>,
    /// Required. The resource name of the Service Directory service pointing to an EKM replica, in the format `projects/*/locations/*/namespaces/*/services/*`.
    #[serde(rename="serviceDirectoryService")]
    
    pub service_directory_service: Option<String>,
}

impl client::Part for ServiceResolver {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations ekm config set iam policy projects](ProjectLocationEkmConfigSetIamPolicyCall) (request)
/// * [locations ekm connections set iam policy projects](ProjectLocationEkmConnectionSetIamPolicyCall) (request)
/// * [locations key rings crypto keys set iam policy projects](ProjectLocationKeyRingCryptoKeySetIamPolicyCall) (request)
/// * [locations key rings import jobs set iam policy projects](ProjectLocationKeyRingImportJobSetIamPolicyCall) (request)
/// * [locations key rings set iam policy projects](ProjectLocationKeyRingSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"`
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations ekm config test iam permissions projects](ProjectLocationEkmConfigTestIamPermissionCall) (request)
/// * [locations ekm connections test iam permissions projects](ProjectLocationEkmConnectionTestIamPermissionCall) (request)
/// * [locations key rings crypto keys test iam permissions projects](ProjectLocationKeyRingCryptoKeyTestIamPermissionCall) (request)
/// * [locations key rings import jobs test iam permissions projects](ProjectLocationKeyRingImportJobTestIamPermissionCall) (request)
/// * [locations key rings test iam permissions projects](ProjectLocationKeyRingTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for TestIamPermissionsRequest {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations ekm config test iam permissions projects](ProjectLocationEkmConfigTestIamPermissionCall) (response)
/// * [locations ekm connections test iam permissions projects](ProjectLocationEkmConnectionTestIamPermissionCall) (response)
/// * [locations key rings crypto keys test iam permissions projects](ProjectLocationKeyRingCryptoKeyTestIamPermissionCall) (response)
/// * [locations key rings import jobs test iam permissions projects](ProjectLocationKeyRingImportJobTestIamPermissionCall) (response)
/// * [locations key rings test iam permissions projects](ProjectLocationKeyRingTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


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
    /// Required. The id of the child CryptoKeyVersion to use as primary.
    #[serde(rename="cryptoKeyVersionId")]
    
    pub crypto_key_version_id: Option<String>,
}

impl client::RequestValue for UpdateCryptoKeyPrimaryVersionRequest {}


/// The public key component of the wrapping key. For details of the type of key this public key corresponds to, see the ImportMethod.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WrappingPublicKey {
    /// The public key, encoded in PEM format. For more information, see the [RFC 7468](https://tools.ietf.org/html/rfc7468) sections for [General Considerations](https://tools.ietf.org/html/rfc7468#section-2) and [Textual Encoding of Subject Public Key Info] (https://tools.ietf.org/html/rfc7468#section-13).
    
    pub pem: Option<String>,
}

impl client::Part for WrappingPublicKey {}


