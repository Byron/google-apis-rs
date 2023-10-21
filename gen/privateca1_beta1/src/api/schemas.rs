use super::*;
/// URLs where a CertificateAuthority will publish content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccessUrls {
    /// The URL where this CertificateAuthority's CA certificate is published. This will only be set for CAs that have been activated.
    #[serde(rename="caCertificateAccessUrl")]
    
    pub ca_certificate_access_url: Option<String>,
    /// The URL where this CertificateAuthority's CRLs are published. This will only be set for CAs that have been activated.
    #[serde(rename="crlAccessUrl")]
    
    pub crl_access_url: Option<String>,
}

impl client::Part for AccessUrls {}


/// Request message for CertificateAuthorityService.ActivateCertificateAuthority.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities activate projects](ProjectLocationCertificateAuthorityActivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivateCertificateAuthorityRequest {
    /// Required. The signed CA certificate issued from FetchCertificateAuthorityCsrResponse.pem_csr.
    #[serde(rename="pemCaCertificate")]
    
    pub pem_ca_certificate: Option<String>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes since the first request. For example, consider a situation where you make an initial request and t he request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// Required. Must include information about the issuer of 'pem_ca_certificate', and any further issuers until the self-signed CA.
    #[serde(rename="subordinateConfig")]
    
    pub subordinate_config: Option<SubordinateConfig>,
}

impl client::RequestValue for ActivateCertificateAuthorityRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AllowedConfigList {
    /// Required. All Certificates issued by the CertificateAuthority must match at least one listed ReusableConfigWrapper. If a ReusableConfigWrapper has an empty field, any value will be allowed for that field.
    #[serde(rename="allowedConfigValues")]
    
    pub allowed_config_values: Option<Vec<ReusableConfigWrapper>>,
}

impl client::Part for AllowedConfigList {}


/// AllowedSubjectAltNames specifies the allowed values for SubjectAltNames by the CertificateAuthority when issuing Certificates.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AllowedSubjectAltNames {
    /// Optional. Specifies if to allow custom X509Extension values.
    #[serde(rename="allowCustomSans")]
    
    pub allow_custom_sans: Option<bool>,
    /// Optional. Specifies if glob patterns used for allowed_dns_names allow wildcard certificates. If this is set, certificate requests with wildcard domains will be permitted to match a glob pattern specified in allowed_dns_names. Otherwise, certificate requests with wildcard domains will be permitted only if allowed_dns_names contains a literal wildcard.
    #[serde(rename="allowGlobbingDnsWildcards")]
    
    pub allow_globbing_dns_wildcards: Option<bool>,
    /// Optional. Contains valid, fully-qualified host names. Glob patterns are also supported. To allow an explicit wildcard certificate, escape with backlash (i.e. `\*`). E.g. for globbed entries: `*bar.com` will allow `foo.bar.com`, but not `*.bar.com`, unless the allow_globbing_dns_wildcards field is set. E.g. for wildcard entries: `\*.bar.com` will allow `*.bar.com`, but not `foo.bar.com`.
    #[serde(rename="allowedDnsNames")]
    
    pub allowed_dns_names: Option<Vec<String>>,
    /// Optional. Contains valid RFC 2822 E-mail addresses. Glob patterns are also supported.
    #[serde(rename="allowedEmailAddresses")]
    
    pub allowed_email_addresses: Option<Vec<String>>,
    /// Optional. Contains valid 32-bit IPv4 addresses and subnet ranges or RFC 4291 IPv6 addresses and subnet ranges. Subnet ranges are specified using the '/' notation (e.g. 10.0.0.0/8, 2001:700:300:1800::/64). Glob patterns are supported only for ip address entries (i.e. not for subnet ranges).
    #[serde(rename="allowedIps")]
    
    pub allowed_ips: Option<Vec<String>>,
    /// Optional. Contains valid RFC 3986 URIs. Glob patterns are also supported. To match across path seperators (i.e. '/') use the double star glob pattern (i.e. '**').
    #[serde(rename="allowedUris")]
    
    pub allowed_uris: Option<Vec<String>>,
}

impl client::Part for AllowedSubjectAltNames {}


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
    
    pub log_type: Option<String>,
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


/// Describes values that are relevant in a CA certificate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CaOptions {
    /// Optional. Refers to the "CA" X.509 extension, which is a boolean value. When this value is missing, the extension will be omitted from the CA certificate.
    #[serde(rename="isCa")]
    
    pub is_ca: Option<bool>,
    /// Optional. Refers to the path length restriction X.509 extension. For a CA certificate, this value describes the depth of subordinate CA certificates that are allowed. If this value is less than 0, the request will fail. If this value is missing, the max path length will be omitted from the CA certificate.
    #[serde(rename="maxIssuerPathLength")]
    
    pub max_issuer_path_length: Option<i32>,
}

impl client::Part for CaOptions {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// A Certificate corresponds to a signed X.509 certificate issued by a CertificateAuthority.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities certificates create projects](ProjectLocationCertificateAuthorityCertificateCreateCall) (request|response)
/// * [locations certificate authorities certificates get projects](ProjectLocationCertificateAuthorityCertificateGetCall) (response)
/// * [locations certificate authorities certificates patch projects](ProjectLocationCertificateAuthorityCertificatePatchCall) (request|response)
/// * [locations certificate authorities certificates revoke projects](ProjectLocationCertificateAuthorityCertificateRevokeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Certificate {
    /// Output only. A structured description of the issued X.509 certificate.
    #[serde(rename="certificateDescription")]
    
    pub certificate_description: Option<CertificateDescription>,
    /// Immutable. A description of the certificate and key that does not require X.509 or ASN.1.
    
    pub config: Option<CertificateConfig>,
    /// Output only. The time at which this Certificate was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Labels with user-defined metadata.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. Immutable. The desired lifetime of a certificate. Used to create the "not_before_time" and "not_after_time" fields inside an X.509 certificate. Note that the lifetime may be truncated if it would extend past the life of any certificate authority in the issuing chain.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub lifetime: Option<client::chrono::Duration>,
    /// Output only. The resource path for this Certificate in the format `projects/*/locations/*/certificateAuthorities/*/certificates/*`.
    
    pub name: Option<String>,
    /// Output only. The pem-encoded, signed X.509 certificate.
    #[serde(rename="pemCertificate")]
    
    pub pem_certificate: Option<String>,
    /// Output only. The chain that may be used to verify the X.509 certificate. Expected to be in issuer-to-root order according to RFC 5246.
    #[serde(rename="pemCertificateChain")]
    
    pub pem_certificate_chain: Option<Vec<String>>,
    /// Immutable. A pem-encoded X.509 certificate signing request (CSR).
    #[serde(rename="pemCsr")]
    
    pub pem_csr: Option<String>,
    /// Output only. Details regarding the revocation of this Certificate. This Certificate is considered revoked if and only if this field is present.
    #[serde(rename="revocationDetails")]
    
    pub revocation_details: Option<RevocationDetails>,
    /// Output only. The time at which this Certificate was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Certificate {}
impl client::ResponseResult for Certificate {}


/// A CertificateAuthority represents an individual Certificate Authority. A CertificateAuthority can be used to create Certificates.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities create projects](ProjectLocationCertificateAuthorityCreateCall) (request)
/// * [locations certificate authorities get projects](ProjectLocationCertificateAuthorityGetCall) (response)
/// * [locations certificate authorities patch projects](ProjectLocationCertificateAuthorityPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateAuthority {
    /// Output only. URLs for accessing content published by this CA, such as the CA certificate and CRLs.
    #[serde(rename="accessUrls")]
    
    pub access_urls: Option<AccessUrls>,
    /// Output only. A structured description of this CertificateAuthority's CA certificate and its issuers. Ordered as self-to-root.
    #[serde(rename="caCertificateDescriptions")]
    
    pub ca_certificate_descriptions: Option<Vec<CertificateDescription>>,
    /// Optional. The CertificateAuthorityPolicy to enforce when issuing Certificates from this CertificateAuthority.
    #[serde(rename="certificatePolicy")]
    
    pub certificate_policy: Option<CertificateAuthorityPolicy>,
    /// Required. Immutable. The config used to create a self-signed X.509 certificate or CSR.
    
    pub config: Option<CertificateConfig>,
    /// Output only. The time at which this CertificateAuthority was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time at which this CertificateAuthority will be deleted, if scheduled for deletion.
    #[serde(rename="deleteTime")]
    
    pub delete_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. The name of a Cloud Storage bucket where this CertificateAuthority will publish content, such as the CA certificate and CRLs. This must be a bucket name, without any prefixes (such as `gs://`) or suffixes (such as `.googleapis.com`). For example, to use a bucket named `my-bucket`, you would simply specify `my-bucket`. If not specified, a managed bucket will be created.
    #[serde(rename="gcsBucket")]
    
    pub gcs_bucket: Option<String>,
    /// Optional. The IssuingOptions to follow when issuing Certificates from this CertificateAuthority.
    #[serde(rename="issuingOptions")]
    
    pub issuing_options: Option<IssuingOptions>,
    /// Required. Immutable. Used when issuing certificates for this CertificateAuthority. If this CertificateAuthority is a self-signed CertificateAuthority, this key is also used to sign the self-signed CA certificate. Otherwise, it is used to sign a CSR.
    #[serde(rename="keySpec")]
    
    pub key_spec: Option<KeyVersionSpec>,
    /// Optional. Labels with user-defined metadata.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. The desired lifetime of the CA certificate. Used to create the "not_before_time" and "not_after_time" fields inside an X.509 certificate.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub lifetime: Option<client::chrono::Duration>,
    /// Output only. The resource name for this CertificateAuthority in the format `projects/*/locations/*/certificateAuthorities/*`.
    
    pub name: Option<String>,
    /// Output only. This CertificateAuthority's certificate chain, including the current CertificateAuthority's certificate. Ordered such that the root issuer is the final element (consistent with RFC 5246). For a self-signed CA, this will only list the current CertificateAuthority's certificate.
    #[serde(rename="pemCaCertificates")]
    
    pub pem_ca_certificates: Option<Vec<String>>,
    /// Output only. The State for this CertificateAuthority.
    
    pub state: Option<String>,
    /// Optional. If this is a subordinate CertificateAuthority, this field will be set with the subordinate configuration, which describes its issuers. This may be updated, but this CertificateAuthority must continue to validate.
    #[serde(rename="subordinateConfig")]
    
    pub subordinate_config: Option<SubordinateConfig>,
    /// Required. Immutable. The Tier of this CertificateAuthority.
    
    pub tier: Option<String>,
    /// Required. Immutable. The Type of this CertificateAuthority.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. The time at which this CertificateAuthority was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for CertificateAuthority {}
impl client::ResponseResult for CertificateAuthority {}


/// The issuing policy for a CertificateAuthority. Certificates will not be successfully issued from this CertificateAuthority if they violate the policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateAuthorityPolicy {
    /// Optional. If any value is specified here, then all Certificates issued by the CertificateAuthority must match at least one listed value. If no value is specified, all values will be allowed for this fied. Glob patterns are also supported.
    #[serde(rename="allowedCommonNames")]
    
    pub allowed_common_names: Option<Vec<String>>,
    /// Optional. All Certificates issued by the CertificateAuthority must match at least one listed ReusableConfigWrapper in the list.
    #[serde(rename="allowedConfigList")]
    
    pub allowed_config_list: Option<AllowedConfigList>,
    /// Optional. If specified, then only methods allowed in the IssuanceModes may be used to issue Certificates.
    #[serde(rename="allowedIssuanceModes")]
    
    pub allowed_issuance_modes: Option<IssuanceModes>,
    /// Optional. If any Subject is specified here, then all Certificates issued by the CertificateAuthority must match at least one listed Subject. If a Subject has an empty field, any value will be allowed for that field.
    #[serde(rename="allowedLocationsAndOrganizations")]
    
    pub allowed_locations_and_organizations: Option<Vec<Subject>>,
    /// Optional. If a AllowedSubjectAltNames is specified here, then all Certificates issued by the CertificateAuthority must match AllowedSubjectAltNames. If no value or an empty value is specified, any value will be allowed for the SubjectAltNames field.
    #[serde(rename="allowedSans")]
    
    pub allowed_sans: Option<AllowedSubjectAltNames>,
    /// Optional. The maximum lifetime allowed by the CertificateAuthority. Note that if the any part if the issuing chain expires before a Certificate's requested maximum_lifetime, the effective lifetime will be explicitly truncated.
    #[serde(rename="maximumLifetime")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub maximum_lifetime: Option<client::chrono::Duration>,
    /// Optional. All Certificates issued by the CertificateAuthority will use the provided configuration values, overwriting any requested configuration values.
    #[serde(rename="overwriteConfigValues")]
    
    pub overwrite_config_values: Option<ReusableConfigWrapper>,
}

impl client::Part for CertificateAuthorityPolicy {}


/// A CertificateConfig describes an X.509 certificate or CSR that is to be created, as an alternative to using ASN.1.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateConfig {
    /// Optional. The public key that corresponds to this config. This is, for example, used when issuing Certificates, but not when creating a self-signed CertificateAuthority or CertificateAuthority CSR.
    #[serde(rename="publicKey")]
    
    pub public_key: Option<PublicKey>,
    /// Required. Describes how some of the technical fields in a certificate should be populated.
    #[serde(rename="reusableConfig")]
    
    pub reusable_config: Option<ReusableConfigWrapper>,
    /// Required. Specifies some of the values in a certificate that are related to the subject.
    #[serde(rename="subjectConfig")]
    
    pub subject_config: Option<SubjectConfig>,
}

impl client::Part for CertificateConfig {}


/// A CertificateDescription describes an X.509 certificate or CSR that has been issued, as an alternative to using ASN.1 / X.509.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateDescription {
    /// Describes lists of issuer CA certificate URLs that appear in the "Authority Information Access" extension in the certificate.
    #[serde(rename="aiaIssuingCertificateUrls")]
    
    pub aia_issuing_certificate_urls: Option<Vec<String>>,
    /// Identifies the subject_key_id of the parent certificate, per https://tools.ietf.org/html/rfc5280#section-4.2.1.1
    #[serde(rename="authorityKeyId")]
    
    pub authority_key_id: Option<KeyId>,
    /// The hash of the x.509 certificate.
    #[serde(rename="certFingerprint")]
    
    pub cert_fingerprint: Option<CertificateFingerprint>,
    /// Describes some of the technical fields in a certificate.
    #[serde(rename="configValues")]
    
    pub config_values: Option<ReusableConfigValues>,
    /// Describes a list of locations to obtain CRL information, i.e. the DistributionPoint.fullName described by https://tools.ietf.org/html/rfc5280#section-4.2.1.13
    #[serde(rename="crlDistributionPoints")]
    
    pub crl_distribution_points: Option<Vec<String>>,
    /// The public key that corresponds to an issued certificate.
    #[serde(rename="publicKey")]
    
    pub public_key: Option<PublicKey>,
    /// Describes some of the values in a certificate that are related to the subject and lifetime.
    #[serde(rename="subjectDescription")]
    
    pub subject_description: Option<SubjectDescription>,
    /// Provides a means of identifiying certificates that contain a particular public key, per https://tools.ietf.org/html/rfc5280#section-4.2.1.2.
    #[serde(rename="subjectKeyId")]
    
    pub subject_key_id: Option<KeyId>,
}

impl client::Part for CertificateDescription {}


/// A group of fingerprints for the x509 certificate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateFingerprint {
    /// The SHA 256 hash, encoded in hexadecimal, of the DER x509 certificate.
    #[serde(rename="sha256Hash")]
    
    pub sha256_hash: Option<String>,
}

impl client::Part for CertificateFingerprint {}


/// A CertificateRevocationList corresponds to a signed X.509 certificate Revocation List (CRL). A CRL contains the serial numbers of certificates that should no longer be trusted.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities certificate revocation lists get projects](ProjectLocationCertificateAuthorityCertificateRevocationListGetCall) (response)
/// * [locations certificate authorities certificate revocation lists patch projects](ProjectLocationCertificateAuthorityCertificateRevocationListPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateRevocationList {
    /// Output only. The location where 'pem_crl' can be accessed.
    #[serde(rename="accessUrl")]
    
    pub access_url: Option<String>,
    /// Output only. The time at which this CertificateRevocationList was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Labels with user-defined metadata.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The resource path for this CertificateRevocationList in the format `projects/*/locations/*/certificateAuthorities/*/ certificateRevocationLists/*`.
    
    pub name: Option<String>,
    /// Output only. The PEM-encoded X.509 CRL.
    #[serde(rename="pemCrl")]
    
    pub pem_crl: Option<String>,
    /// Output only. The revoked serial numbers that appear in pem_crl.
    #[serde(rename="revokedCertificates")]
    
    pub revoked_certificates: Option<Vec<RevokedCertificate>>,
    /// Output only. The CRL sequence number that appears in pem_crl.
    #[serde(rename="sequenceNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub sequence_number: Option<i64>,
    /// Output only. The State for this CertificateRevocationList.
    
    pub state: Option<String>,
    /// Output only. The time at which this CertificateRevocationList was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for CertificateRevocationList {}
impl client::ResponseResult for CertificateRevocationList {}


/// Request message for CertificateAuthorityService.DisableCertificateAuthority.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities disable projects](ProjectLocationCertificateAuthorityDisableCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DisableCertificateAuthorityRequest {
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes since the first request. For example, consider a situation where you make an initial request and t he request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for DisableCertificateAuthorityRequest {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Request message for CertificateAuthorityService.EnableCertificateAuthority.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities enable projects](ProjectLocationCertificateAuthorityEnableCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnableCertificateAuthorityRequest {
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes since the first request. For example, consider a situation where you make an initial request and t he request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for EnableCertificateAuthorityRequest {}


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


/// KeyUsage.ExtendedKeyUsageOptions has fields that correspond to certain common OIDs that could be specified as an extended key usage value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExtendedKeyUsageOptions {
    /// Corresponds to OID 1.3.6.1.5.5.7.3.2. Officially described as "TLS WWW client authentication", though regularly used for non-WWW TLS.
    #[serde(rename="clientAuth")]
    
    pub client_auth: Option<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.3. Officially described as "Signing of downloadable executable code client authentication".
    #[serde(rename="codeSigning")]
    
    pub code_signing: Option<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.4. Officially described as "Email protection".
    #[serde(rename="emailProtection")]
    
    pub email_protection: Option<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.9. Officially described as "Signing OCSP responses".
    #[serde(rename="ocspSigning")]
    
    pub ocsp_signing: Option<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.1. Officially described as "TLS WWW server authentication", though regularly used for non-WWW TLS.
    #[serde(rename="serverAuth")]
    
    pub server_auth: Option<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.8. Officially described as "Binding the hash of an object to a time".
    #[serde(rename="timeStamping")]
    
    pub time_stamping: Option<bool>,
}

impl client::Part for ExtendedKeyUsageOptions {}


/// Response message for CertificateAuthorityService.FetchCertificateAuthorityCsr.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities fetch projects](ProjectLocationCertificateAuthorityFetchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FetchCertificateAuthorityCsrResponse {
    /// Output only. The PEM-encoded signed certificate signing request (CSR).
    #[serde(rename="pemCsr")]
    
    pub pem_csr: Option<String>,
}

impl client::ResponseResult for FetchCertificateAuthorityCsrResponse {}


/// IssuanceModes specifies the allowed ways in which Certificates may be requested from this CertificateAuthority.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IssuanceModes {
    /// Required. When true, allows callers to create Certificates by specifying a CertificateConfig.
    #[serde(rename="allowConfigBasedIssuance")]
    
    pub allow_config_based_issuance: Option<bool>,
    /// Required. When true, allows callers to create Certificates by specifying a CSR.
    #[serde(rename="allowCsrBasedIssuance")]
    
    pub allow_csr_based_issuance: Option<bool>,
}

impl client::Part for IssuanceModes {}


/// Options that affect all certificates issued by a CertificateAuthority.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IssuingOptions {
    /// Required. When true, includes a URL to the issuing CA certificate in the "authority information access" X.509 extension.
    #[serde(rename="includeCaCertUrl")]
    
    pub include_ca_cert_url: Option<bool>,
    /// Required. When true, includes a URL to the CRL corresponding to certificates issued from a CertificateAuthority. CRLs will expire 7 days from their creation. However, we will rebuild daily. CRLs are also rebuilt shortly after a certificate is revoked.
    #[serde(rename="includeCrlAccessUrl")]
    
    pub include_crl_access_url: Option<bool>,
}

impl client::Part for IssuingOptions {}


/// A KeyId identifies a specific public key, usually by hashing the public key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyId {
    /// Optional. The value of this KeyId encoded in lowercase hexadecimal. This is most likely the 160 bit SHA-1 hash of the public key.
    #[serde(rename="keyId")]
    
    pub key_id: Option<String>,
}

impl client::Part for KeyId {}


/// A KeyUsage describes key usage values that may appear in an X.509 certificate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyUsage {
    /// Describes high-level ways in which a key may be used.
    #[serde(rename="baseKeyUsage")]
    
    pub base_key_usage: Option<KeyUsageOptions>,
    /// Detailed scenarios in which a key may be used.
    #[serde(rename="extendedKeyUsage")]
    
    pub extended_key_usage: Option<ExtendedKeyUsageOptions>,
    /// Used to describe extended key usages that are not listed in the KeyUsage.ExtendedKeyUsageOptions message.
    #[serde(rename="unknownExtendedKeyUsages")]
    
    pub unknown_extended_key_usages: Option<Vec<ObjectId>>,
}

impl client::Part for KeyUsage {}


/// KeyUsage.KeyUsageOptions corresponds to the key usage values described in https://tools.ietf.org/html/rfc5280#section-4.2.1.3.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyUsageOptions {
    /// The key may be used to sign certificates.
    #[serde(rename="certSign")]
    
    pub cert_sign: Option<bool>,
    /// The key may be used for cryptographic commitments. Note that this may also be referred to as "non-repudiation".
    #[serde(rename="contentCommitment")]
    
    pub content_commitment: Option<bool>,
    /// The key may be used sign certificate revocation lists.
    #[serde(rename="crlSign")]
    
    pub crl_sign: Option<bool>,
    /// The key may be used to encipher data.
    #[serde(rename="dataEncipherment")]
    
    pub data_encipherment: Option<bool>,
    /// The key may be used to decipher only.
    #[serde(rename="decipherOnly")]
    
    pub decipher_only: Option<bool>,
    /// The key may be used for digital signatures.
    #[serde(rename="digitalSignature")]
    
    pub digital_signature: Option<bool>,
    /// The key may be used to encipher only.
    #[serde(rename="encipherOnly")]
    
    pub encipher_only: Option<bool>,
    /// The key may be used in a key agreement protocol.
    #[serde(rename="keyAgreement")]
    
    pub key_agreement: Option<bool>,
    /// The key may be used to encipher other keys.
    #[serde(rename="keyEncipherment")]
    
    pub key_encipherment: Option<bool>,
}

impl client::Part for KeyUsageOptions {}


/// A Cloud KMS key configuration that a CertificateAuthority will use.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyVersionSpec {
    /// Required. The algorithm to use for creating a managed Cloud KMS key for a for a simplified experience. All managed keys will be have their ProtectionLevel as `HSM`.
    
    pub algorithm: Option<String>,
    /// Required. The resource name for an existing Cloud KMS CryptoKeyVersion in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`. This option enables full flexibility in the key's capabilities and properties.
    #[serde(rename="cloudKmsKeyVersion")]
    
    pub cloud_kms_key_version: Option<String>,
}

impl client::Part for KeyVersionSpec {}


/// Response message for CertificateAuthorityService.ListCertificateAuthorities.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities list projects](ProjectLocationCertificateAuthorityListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCertificateAuthoritiesResponse {
    /// The list of CertificateAuthorities.
    #[serde(rename="certificateAuthorities")]
    
    pub certificate_authorities: Option<Vec<CertificateAuthority>>,
    /// A token to retrieve next page of results. Pass this value in ListCertificateAuthoritiesRequest.next_page_token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of locations (e.g. "us-west1") that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListCertificateAuthoritiesResponse {}


/// Response message for CertificateAuthorityService.ListCertificateRevocationLists.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities certificate revocation lists list projects](ProjectLocationCertificateAuthorityCertificateRevocationListListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCertificateRevocationListsResponse {
    /// The list of CertificateRevocationLists.
    #[serde(rename="certificateRevocationLists")]
    
    pub certificate_revocation_lists: Option<Vec<CertificateRevocationList>>,
    /// A token to retrieve next page of results. Pass this value in ListCertificateRevocationListsRequest.next_page_token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of locations (e.g. "us-west1") that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListCertificateRevocationListsResponse {}


/// Response message for CertificateAuthorityService.ListCertificates.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities certificates list projects](ProjectLocationCertificateAuthorityCertificateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCertificatesResponse {
    /// The list of Certificates.
    
    pub certificates: Option<Vec<Certificate>>,
    /// A token to retrieve next page of results. Pass this value in ListCertificatesRequest.next_page_token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of locations (e.g. "us-west1") that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListCertificatesResponse {}


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


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations list projects](ProjectLocationOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListOperationsResponse {}


/// Response message for CertificateAuthorityService.ListReusableConfigs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations reusable configs list projects](ProjectLocationReusableConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListReusableConfigsResponse {
    /// A token to retrieve next page of results. Pass this value in ListReusableConfigsRequest.next_page_token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of ReusableConfigs.
    #[serde(rename="reusableConfigs")]
    
    pub reusable_configs: Option<Vec<ReusableConfig>>,
    /// A list of locations (e.g. "us-west1") that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListReusableConfigsResponse {}


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


/// An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectId {
    /// Required. The parts of an OID path. The most significant parts of the path come first.
    #[serde(rename="objectIdPath")]
    
    pub object_id_path: Option<Vec<i32>>,
}

impl client::Part for ObjectId {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities certificate revocation lists patch projects](ProjectLocationCertificateAuthorityCertificateRevocationListPatchCall) (response)
/// * [locations certificate authorities activate projects](ProjectLocationCertificateAuthorityActivateCall) (response)
/// * [locations certificate authorities create projects](ProjectLocationCertificateAuthorityCreateCall) (response)
/// * [locations certificate authorities disable projects](ProjectLocationCertificateAuthorityDisableCall) (response)
/// * [locations certificate authorities enable projects](ProjectLocationCertificateAuthorityEnableCall) (response)
/// * [locations certificate authorities patch projects](ProjectLocationCertificateAuthorityPatchCall) (response)
/// * [locations certificate authorities restore projects](ProjectLocationCertificateAuthorityRestoreCall) (response)
/// * [locations certificate authorities schedule delete projects](ProjectLocationCertificateAuthorityScheduleDeleteCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities certificate revocation lists get iam policy projects](ProjectLocationCertificateAuthorityCertificateRevocationListGetIamPolicyCall) (response)
/// * [locations certificate authorities certificate revocation lists set iam policy projects](ProjectLocationCertificateAuthorityCertificateRevocationListSetIamPolicyCall) (response)
/// * [locations certificate authorities get iam policy projects](ProjectLocationCertificateAuthorityGetIamPolicyCall) (response)
/// * [locations certificate authorities set iam policy projects](ProjectLocationCertificateAuthoritySetIamPolicyCall) (response)
/// * [locations reusable configs get iam policy projects](ProjectLocationReusableConfigGetIamPolicyCall) (response)
/// * [locations reusable configs set iam policy projects](ProjectLocationReusableConfigSetIamPolicyCall) (response)
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


/// A PublicKey describes a public key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublicKey {
    /// Required. A public key. When this is specified in a request, the padding and encoding can be any of the options described by the respective 'KeyType' value. When this is generated by the service, it will always be an RFC 5280 [SubjectPublicKeyInfo](https://tools.ietf.org/html/rfc5280#section-4.1) structure containing an algorithm identifier and a key.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub key: Option<Vec<u8>>,
    /// Optional. The type of public key. If specified, it must match the public key used for the`key` field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for PublicKey {}


/// Request message for CertificateAuthorityService.RestoreCertificateAuthority.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities restore projects](ProjectLocationCertificateAuthorityRestoreCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestoreCertificateAuthorityRequest {
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes since the first request. For example, consider a situation where you make an initial request and t he request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for RestoreCertificateAuthorityRequest {}


/// A ReusableConfig refers to a managed ReusableConfigValues. Those, in turn, are used to describe certain fields of an X.509 certificate, such as the key usage fields, fields specific to CA certificates, certificate policy extensions and custom extensions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations reusable configs get projects](ProjectLocationReusableConfigGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReusableConfig {
    /// Output only. The time at which this ReusableConfig was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A human-readable description of scenarios these ReusableConfigValues may be compatible with.
    
    pub description: Option<String>,
    /// Optional. Labels with user-defined metadata.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The resource path for this ReusableConfig in the format `projects/*/locations/*/reusableConfigs/*`.
    
    pub name: Option<String>,
    /// Output only. The time at which this ReusableConfig was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The config values.
    
    pub values: Option<ReusableConfigValues>,
}

impl client::ResponseResult for ReusableConfig {}


/// A ReusableConfigValues is used to describe certain fields of an X.509 certificate, such as the key usage fields, fields specific to CA certificates, certificate policy extensions and custom extensions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReusableConfigValues {
    /// Optional. Describes custom X.509 extensions.
    #[serde(rename="additionalExtensions")]
    
    pub additional_extensions: Option<Vec<X509Extension>>,
    /// Optional. Describes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the "Authority Information Access" extension in the certificate.
    #[serde(rename="aiaOcspServers")]
    
    pub aia_ocsp_servers: Option<Vec<String>>,
    /// Optional. Describes options in this ReusableConfigValues that are relevant in a CA certificate.
    #[serde(rename="caOptions")]
    
    pub ca_options: Option<CaOptions>,
    /// Optional. Indicates the intended use for keys that correspond to a certificate.
    #[serde(rename="keyUsage")]
    
    pub key_usage: Option<KeyUsage>,
    /// Optional. Describes the X.509 certificate policy object identifiers, per https://tools.ietf.org/html/rfc5280#section-4.2.1.4.
    #[serde(rename="policyIds")]
    
    pub policy_ids: Option<Vec<ObjectId>>,
}

impl client::Part for ReusableConfigValues {}


/// A ReusableConfigWrapper describes values that may assist in creating an X.509 certificate, or a reference to a pre-defined set of values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReusableConfigWrapper {
    /// Required. A resource path to a ReusableConfig in the format `projects/*/locations/*/reusableConfigs/*`.
    #[serde(rename="reusableConfig")]
    
    pub reusable_config: Option<String>,
    /// Required. A user-specified inline ReusableConfigValues.
    #[serde(rename="reusableConfigValues")]
    
    pub reusable_config_values: Option<ReusableConfigValues>,
}

impl client::Part for ReusableConfigWrapper {}


/// Describes fields that are relavent to the revocation of a Certificate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevocationDetails {
    /// Indicates why a Certificate was revoked.
    #[serde(rename="revocationState")]
    
    pub revocation_state: Option<String>,
    /// The time at which this Certificate was revoked.
    #[serde(rename="revocationTime")]
    
    pub revocation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for RevocationDetails {}


/// Request message for CertificateAuthorityService.RevokeCertificate.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities certificates revoke projects](ProjectLocationCertificateAuthorityCertificateRevokeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevokeCertificateRequest {
    /// Required. The RevocationReason for revoking this certificate.
    
    pub reason: Option<String>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes since the first request. For example, consider a situation where you make an initial request and t he request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for RevokeCertificateRequest {}


/// Describes a revoked Certificate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevokedCertificate {
    /// The resource path for the Certificate in the format `projects/*/locations/*/certificateAuthorities/*/certificates/*`.
    
    pub certificate: Option<String>,
    /// The serial number of the Certificate.
    #[serde(rename="hexSerialNumber")]
    
    pub hex_serial_number: Option<String>,
    /// The reason the Certificate was revoked.
    #[serde(rename="revocationReason")]
    
    pub revocation_reason: Option<String>,
}

impl client::Part for RevokedCertificate {}


/// Request message for CertificateAuthorityService.ScheduleDeleteCertificateAuthority.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities schedule delete projects](ProjectLocationCertificateAuthorityScheduleDeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScheduleDeleteCertificateAuthorityRequest {
    /// Optional. This field allows the CA to be scheduled for deletion even if the CA has active certs. Active certs include both unrevoked and unexpired certs.
    #[serde(rename="ignoreActiveCertificates")]
    
    pub ignore_active_certificates: Option<bool>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes since the first request. For example, consider a situation where you make an initial request and t he request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for ScheduleDeleteCertificateAuthorityRequest {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities certificate revocation lists set iam policy projects](ProjectLocationCertificateAuthorityCertificateRevocationListSetIamPolicyCall) (request)
/// * [locations certificate authorities set iam policy projects](ProjectLocationCertificateAuthoritySetIamPolicyCall) (request)
/// * [locations reusable configs set iam policy projects](ProjectLocationReusableConfigSetIamPolicyCall) (request)
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


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for Status {}


/// Subject describes parts of a distinguished name that, in turn, describes the subject of the certificate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Subject {
    /// The country code of the subject.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// The locality or city of the subject.
    
    pub locality: Option<String>,
    /// The organization of the subject.
    
    pub organization: Option<String>,
    /// The organizational_unit of the subject.
    #[serde(rename="organizationalUnit")]
    
    pub organizational_unit: Option<String>,
    /// The postal code of the subject.
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// The province, territory, or regional state of the subject.
    
    pub province: Option<String>,
    /// The street address of the subject.
    #[serde(rename="streetAddress")]
    
    pub street_address: Option<String>,
}

impl client::Part for Subject {}


/// SubjectAltNames corresponds to a more modern way of listing what the asserted identity is in a certificate (i.e., compared to the "common name" in the distinguished name).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubjectAltNames {
    /// Contains additional subject alternative name values.
    #[serde(rename="customSans")]
    
    pub custom_sans: Option<Vec<X509Extension>>,
    /// Contains only valid, fully-qualified host names.
    #[serde(rename="dnsNames")]
    
    pub dns_names: Option<Vec<String>>,
    /// Contains only valid RFC 2822 E-mail addresses.
    #[serde(rename="emailAddresses")]
    
    pub email_addresses: Option<Vec<String>>,
    /// Contains only valid 32-bit IPv4 addresses or RFC 4291 IPv6 addresses.
    #[serde(rename="ipAddresses")]
    
    pub ip_addresses: Option<Vec<String>>,
    /// Contains only valid RFC 3986 URIs.
    
    pub uris: Option<Vec<String>>,
}

impl client::Part for SubjectAltNames {}


/// These values are used to create the distinguished name and subject alternative name fields in an X.509 certificate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubjectConfig {
    /// Optional. The "common name" of the distinguished name.
    #[serde(rename="commonName")]
    
    pub common_name: Option<String>,
    /// Required. Contains distinguished name fields such as the location and organization.
    
    pub subject: Option<Subject>,
    /// Optional. The subject alternative name fields.
    #[serde(rename="subjectAltName")]
    
    pub subject_alt_name: Option<SubjectAltNames>,
}

impl client::Part for SubjectConfig {}


/// These values describe fields in an issued X.509 certificate such as the distinguished name, subject alternative names, serial number, and lifetime.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubjectDescription {
    /// The "common name" of the distinguished name.
    #[serde(rename="commonName")]
    
    pub common_name: Option<String>,
    /// The serial number encoded in lowercase hexadecimal.
    #[serde(rename="hexSerialNumber")]
    
    pub hex_serial_number: Option<String>,
    /// For convenience, the actual lifetime of an issued certificate. Corresponds to 'not_after_time' - 'not_before_time'.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub lifetime: Option<client::chrono::Duration>,
    /// The time at which the certificate expires.
    #[serde(rename="notAfterTime")]
    
    pub not_after_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The time at which the certificate becomes valid.
    #[serde(rename="notBeforeTime")]
    
    pub not_before_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Contains distinguished name fields such as the location and organization.
    
    pub subject: Option<Subject>,
    /// The subject alternative name fields.
    #[serde(rename="subjectAltName")]
    
    pub subject_alt_name: Option<SubjectAltNames>,
}

impl client::Part for SubjectDescription {}


/// Describes a subordinate CA's issuers. This is either a resource path to a known issuing CertificateAuthority, or a PEM issuer certificate chain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubordinateConfig {
    /// Required. This can refer to a CertificateAuthority that was used to create a subordinate CertificateAuthority. This field is used for information and usability purposes only. The resource name is in the format `projects/*/locations/*/certificateAuthorities/*`.
    #[serde(rename="certificateAuthority")]
    
    pub certificate_authority: Option<String>,
    /// Required. Contains the PEM certificate chain for the issuers of this CertificateAuthority, but not pem certificate for this CA itself.
    #[serde(rename="pemIssuerChain")]
    
    pub pem_issuer_chain: Option<SubordinateConfigChain>,
}

impl client::Part for SubordinateConfig {}


/// This message describes a subordinate CA's issuer certificate chain. This wrapper exists for compatibility reasons.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubordinateConfigChain {
    /// Required. Expected to be in leaf-to-root order according to RFC 5246.
    #[serde(rename="pemCertificates")]
    
    pub pem_certificates: Option<Vec<String>>,
}

impl client::Part for SubordinateConfigChain {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate authorities certificate revocation lists test iam permissions projects](ProjectLocationCertificateAuthorityCertificateRevocationListTestIamPermissionCall) (request)
/// * [locations certificate authorities test iam permissions projects](ProjectLocationCertificateAuthorityTestIamPermissionCall) (request)
/// * [locations reusable configs test iam permissions projects](ProjectLocationReusableConfigTestIamPermissionCall) (request)
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
/// * [locations certificate authorities certificate revocation lists test iam permissions projects](ProjectLocationCertificateAuthorityCertificateRevocationListTestIamPermissionCall) (response)
/// * [locations certificate authorities test iam permissions projects](ProjectLocationCertificateAuthorityTestIamPermissionCall) (response)
/// * [locations reusable configs test iam permissions projects](ProjectLocationReusableConfigTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// An X509Extension specifies an X.509 extension, which may be used in different parts of X.509 objects like certificates, CSRs, and CRLs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct X509Extension {
    /// Required. Indicates whether or not this extension is critical (i.e., if the client does not know how to handle this extension, the client should consider this to be an error).
    
    pub critical: Option<bool>,
    /// Required. The OID for this X.509 extension.
    #[serde(rename="objectId")]
    
    pub object_id: Option<ObjectId>,
    /// Required. The value of this X.509 extension.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub value: Option<Vec<u8>>,
}

impl client::Part for X509Extension {}


