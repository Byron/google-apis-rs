use super::*;
/// State of the latest attempt to authorize a domain for certificate issuance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizationAttemptInfo {
    /// Output only. Human readable explanation for reaching the state. Provided to help address the configuration issues. Not guaranteed to be stable. For programmatic access use FailureReason enum.
    
    pub details: Option<String>,
    /// Domain name of the authorization attempt.
    
    pub domain: Option<String>,
    /// Output only. Reason for failure of the authorization attempt for the domain.
    #[serde(rename="failureReason")]
    
    pub failure_reason: Option<AuthorizationAttemptInfoFailureReasonEnum>,
    /// Output only. State of the domain for managed certificate issuance.
    
    pub state: Option<AuthorizationAttemptInfoStateEnum>,
}

impl client::Part for AuthorizationAttemptInfo {}


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


/// Defines TLS certificate.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificates create projects](ProjectLocationCertificateCreateCall) (request)
/// * [locations certificates get projects](ProjectLocationCertificateGetCall) (response)
/// * [locations certificates patch projects](ProjectLocationCertificatePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Certificate {
    /// Output only. The creation timestamp of a Certificate.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// One or more paragraphs of text description of a certificate.
    
    pub description: Option<String>,
    /// Output only. The expiry timestamp of a Certificate.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Set of labels associated with a Certificate.
    
    pub labels: Option<HashMap<String, String>>,
    /// If set, contains configuration and state of a managed certificate.
    
    pub managed: Option<ManagedCertificate>,
    /// A user-defined name of the certificate. Certificate names must be unique globally and match pattern `projects/*/locations/*/certificates/*`.
    
    pub name: Option<String>,
    /// Output only. The PEM-encoded certificate chain.
    #[serde(rename="pemCertificate")]
    
    pub pem_certificate: Option<String>,
    /// Output only. The list of Subject Alternative Names of dnsName type defined in the certificate (see RFC 5280 4.2.1.6). Managed certificates that haven't been provisioned yet have this field populated with a value of the managed.domains field.
    #[serde(rename="sanDnsnames")]
    
    pub san_dnsnames: Option<Vec<String>>,
    /// Immutable. The scope of the certificate.
    
    pub scope: Option<CertificateScopeEnum>,
    /// If set, defines data of a self-managed certificate.
    #[serde(rename="selfManaged")]
    
    pub self_managed: Option<SelfManagedCertificate>,
    /// Output only. The last update timestamp of a Certificate.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Certificate {}
impl client::ResponseResult for Certificate {}


/// The CA that issues the workload certificate. It includes CA address, type, authentication to CA service, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateAuthorityConfig {
    /// Defines a CertificateAuthorityServiceConfig.
    #[serde(rename="certificateAuthorityServiceConfig")]
    
    pub certificate_authority_service_config: Option<CertificateAuthorityServiceConfig>,
}

impl client::Part for CertificateAuthorityConfig {}


/// Contains information required to contact CA service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateAuthorityServiceConfig {
    /// Required. A CA pool resource used to issue a certificate. The CA pool string has a relative resource path following the form "projects/{project}/locations/{location}/caPools/{ca_pool}".
    #[serde(rename="caPool")]
    
    pub ca_pool: Option<String>,
}

impl client::Part for CertificateAuthorityServiceConfig {}


/// CertificateIssuanceConfig specifies how to issue and manage a certificate.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate issuance configs create projects](ProjectLocationCertificateIssuanceConfigCreateCall) (request)
/// * [locations certificate issuance configs get projects](ProjectLocationCertificateIssuanceConfigGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateIssuanceConfig {
    /// Required. The CA that issues the workload certificate. It includes the CA address, type, authentication to CA service, etc.
    #[serde(rename="certificateAuthorityConfig")]
    
    pub certificate_authority_config: Option<CertificateAuthorityConfig>,
    /// Output only. The creation timestamp of a CertificateIssuanceConfig.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// One or more paragraphs of text description of a CertificateIssuanceConfig.
    
    pub description: Option<String>,
    /// Required. The key algorithm to use when generating the private key.
    #[serde(rename="keyAlgorithm")]
    
    pub key_algorithm: Option<CertificateIssuanceConfigKeyAlgorithmEnum>,
    /// Set of labels associated with a CertificateIssuanceConfig.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. Workload certificate lifetime requested.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub lifetime: Option<client::chrono::Duration>,
    /// A user-defined name of the certificate issuance config. CertificateIssuanceConfig names must be unique globally and match pattern `projects/*/locations/*/certificateIssuanceConfigs/*`.
    
    pub name: Option<String>,
    /// Required. Specifies the percentage of elapsed time of the certificate lifetime to wait before renewing the certificate. Must be a number between 1-99, inclusive.
    #[serde(rename="rotationWindowPercentage")]
    
    pub rotation_window_percentage: Option<i32>,
    /// Output only. The last update timestamp of a CertificateIssuanceConfig.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for CertificateIssuanceConfig {}
impl client::ResponseResult for CertificateIssuanceConfig {}


/// Defines a collection of certificate configurations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate maps create projects](ProjectLocationCertificateMapCreateCall) (request)
/// * [locations certificate maps get projects](ProjectLocationCertificateMapGetCall) (response)
/// * [locations certificate maps patch projects](ProjectLocationCertificateMapPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateMap {
    /// Output only. The creation timestamp of a Certificate Map.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// One or more paragraphs of text description of a certificate map.
    
    pub description: Option<String>,
    /// Output only. A list of GCLB targets that use this Certificate Map. A Target Proxy is only present on this list if it's attached to a Forwarding Rule.
    #[serde(rename="gclbTargets")]
    
    pub gclb_targets: Option<Vec<GclbTarget>>,
    /// Set of labels associated with a Certificate Map.
    
    pub labels: Option<HashMap<String, String>>,
    /// A user-defined name of the Certificate Map. Certificate Map names must be unique globally and match pattern `projects/*/locations/*/certificateMaps/*`.
    
    pub name: Option<String>,
    /// Output only. The update timestamp of a Certificate Map.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for CertificateMap {}
impl client::ResponseResult for CertificateMap {}


/// Defines a certificate map entry.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate maps certificate map entries create projects](ProjectLocationCertificateMapCertificateMapEntryCreateCall) (request)
/// * [locations certificate maps certificate map entries get projects](ProjectLocationCertificateMapCertificateMapEntryGetCall) (response)
/// * [locations certificate maps certificate map entries patch projects](ProjectLocationCertificateMapCertificateMapEntryPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateMapEntry {
    /// A set of Certificates defines for the given `hostname`. There can be defined up to fifteen certificates in each Certificate Map Entry. Each certificate must match pattern `projects/*/locations/*/certificates/*`.
    
    pub certificates: Option<Vec<String>>,
    /// Output only. The creation timestamp of a Certificate Map Entry.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// One or more paragraphs of text description of a certificate map entry.
    
    pub description: Option<String>,
    /// A Hostname (FQDN, e.g. `example.com`) or a wildcard hostname expression (`*.example.com`) for a set of hostnames with common suffix. Used as Server Name Indication (SNI) for selecting a proper certificate.
    
    pub hostname: Option<String>,
    /// Set of labels associated with a Certificate Map Entry.
    
    pub labels: Option<HashMap<String, String>>,
    /// A predefined matcher for particular cases, other than SNI selection.
    
    pub matcher: Option<CertificateMapEntryMatcherEnum>,
    /// A user-defined name of the Certificate Map Entry. Certificate Map Entry names must be unique globally and match pattern `projects/*/locations/*/certificateMaps/*/certificateMapEntries/*`.
    
    pub name: Option<String>,
    /// Output only. A serving state of this Certificate Map Entry.
    
    pub state: Option<CertificateMapEntryStateEnum>,
    /// Output only. The update timestamp of a Certificate Map Entry.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for CertificateMapEntry {}
impl client::ResponseResult for CertificateMapEntry {}


/// A DnsAuthorization resource describes a way to perform domain authorization for certificate issuance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations dns authorizations create projects](ProjectLocationDnsAuthorizationCreateCall) (request)
/// * [locations dns authorizations get projects](ProjectLocationDnsAuthorizationGetCall) (response)
/// * [locations dns authorizations patch projects](ProjectLocationDnsAuthorizationPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DnsAuthorization {
    /// Output only. The creation timestamp of a DnsAuthorization.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// One or more paragraphs of text description of a DnsAuthorization.
    
    pub description: Option<String>,
    /// Output only. DNS Resource Record that needs to be added to DNS configuration.
    #[serde(rename="dnsResourceRecord")]
    
    pub dns_resource_record: Option<DnsResourceRecord>,
    /// Required. Immutable. A domain that is being authorized. A DnsAuthorization resource covers a single domain and its wildcard, e.g. authorization for `example.com` can be used to issue certificates for `example.com` and `*.example.com`.
    
    pub domain: Option<String>,
    /// Set of labels associated with a DnsAuthorization.
    
    pub labels: Option<HashMap<String, String>>,
    /// A user-defined name of the dns authorization. DnsAuthorization names must be unique globally and match pattern `projects/*/locations/*/dnsAuthorizations/*`.
    
    pub name: Option<String>,
    /// Output only. The last update timestamp of a DnsAuthorization.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for DnsAuthorization {}
impl client::ResponseResult for DnsAuthorization {}


/// The structure describing the DNS Resource Record that needs to be added to DNS configuration for the authorization to be usable by certificate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DnsResourceRecord {
    /// Output only. Data of the DNS Resource Record.
    
    pub data: Option<String>,
    /// Output only. Fully qualified name of the DNS Resource Record. e.g. `_acme-challenge.example.com`
    
    pub name: Option<String>,
    /// Output only. Type of the DNS Resource Record. Currently always set to "CNAME".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for DnsResourceRecord {}


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


/// Describes a Target Proxy that uses this Certificate Map.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GclbTarget {
    /// Output only. IP configurations for this Target Proxy where the Certificate Map is serving.
    #[serde(rename="ipConfigs")]
    
    pub ip_configs: Option<Vec<IpConfig>>,
    /// Output only. This field returns the resource name in the following format: `//compute.googleapis.com/projects/*/global/targetHttpsProxies/*`.
    #[serde(rename="targetHttpsProxy")]
    
    pub target_https_proxy: Option<String>,
    /// Output only. This field returns the resource name in the following format: `//compute.googleapis.com/projects/*/global/targetSslProxies/*`.
    #[serde(rename="targetSslProxy")]
    
    pub target_ssl_proxy: Option<String>,
}

impl client::Part for GclbTarget {}


/// Defines IP configuration where this Certificate Map is serving.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IpConfig {
    /// Output only. An external IP address.
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// Output only. Ports.
    
    pub ports: Option<Vec<u32>>,
}

impl client::Part for IpConfig {}


/// Response for the `ListCertificateIssuanceConfigs` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate issuance configs list projects](ProjectLocationCertificateIssuanceConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCertificateIssuanceConfigsResponse {
    /// A list of certificate configs for the parent resource.
    #[serde(rename="certificateIssuanceConfigs")]
    
    pub certificate_issuance_configs: Option<Vec<CertificateIssuanceConfig>>,
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListCertificateIssuanceConfigsResponse {}


/// Response for the `ListCertificateMapEntries` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate maps certificate map entries list projects](ProjectLocationCertificateMapCertificateMapEntryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCertificateMapEntriesResponse {
    /// A list of certificate map entries for the parent resource.
    #[serde(rename="certificateMapEntries")]
    
    pub certificate_map_entries: Option<Vec<CertificateMapEntry>>,
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListCertificateMapEntriesResponse {}


/// Response for the `ListCertificateMaps` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate maps list projects](ProjectLocationCertificateMapListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCertificateMapsResponse {
    /// A list of certificate maps for the parent resource.
    #[serde(rename="certificateMaps")]
    
    pub certificate_maps: Option<Vec<CertificateMap>>,
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListCertificateMapsResponse {}


/// Response for the `ListCertificates` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificates list projects](ProjectLocationCertificateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCertificatesResponse {
    /// A list of certificates for the parent resource.
    
    pub certificates: Option<Vec<Certificate>>,
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListCertificatesResponse {}


/// Response for the `ListDnsAuthorizations` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations dns authorizations list projects](ProjectLocationDnsAuthorizationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDnsAuthorizationsResponse {
    /// A list of dns authorizations for the parent resource.
    #[serde(rename="dnsAuthorizations")]
    
    pub dns_authorizations: Option<Vec<DnsAuthorization>>,
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListDnsAuthorizationsResponse {}


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


/// Configuration and state of a Managed Certificate. Certificate Manager provisions and renews Managed Certificates automatically, for as long as it's authorized to do so.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedCertificate {
    /// Output only. Detailed state of the latest authorization attempt for each domain specified for managed certificate resource.
    #[serde(rename="authorizationAttemptInfo")]
    
    pub authorization_attempt_info: Option<Vec<AuthorizationAttemptInfo>>,
    /// Immutable. Authorizations that will be used for performing domain authorization.
    #[serde(rename="dnsAuthorizations")]
    
    pub dns_authorizations: Option<Vec<String>>,
    /// Immutable. The domains for which a managed SSL certificate will be generated. Wildcard domains are only supported with DNS challenge resolution.
    
    pub domains: Option<Vec<String>>,
    /// Immutable. The resource name for a CertificateIssuanceConfig used to configure private PKI certificates in the format `projects/*/locations/*/certificateIssuanceConfigs/*`. If this field is not set, the certificates will instead be publicly signed as documented at https://cloud.google.com/load-balancing/docs/ssl-certificates/google-managed-certs#caa.
    #[serde(rename="issuanceConfig")]
    
    pub issuance_config: Option<String>,
    /// Output only. Information about issues with provisioning a Managed Certificate.
    #[serde(rename="provisioningIssue")]
    
    pub provisioning_issue: Option<ProvisioningIssue>,
    /// Output only. State of the managed certificate resource.
    
    pub state: Option<ManagedCertificateStateEnum>,
}

impl client::Part for ManagedCertificate {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations certificate issuance configs create projects](ProjectLocationCertificateIssuanceConfigCreateCall) (response)
/// * [locations certificate issuance configs delete projects](ProjectLocationCertificateIssuanceConfigDeleteCall) (response)
/// * [locations certificate maps certificate map entries create projects](ProjectLocationCertificateMapCertificateMapEntryCreateCall) (response)
/// * [locations certificate maps certificate map entries delete projects](ProjectLocationCertificateMapCertificateMapEntryDeleteCall) (response)
/// * [locations certificate maps certificate map entries patch projects](ProjectLocationCertificateMapCertificateMapEntryPatchCall) (response)
/// * [locations certificate maps create projects](ProjectLocationCertificateMapCreateCall) (response)
/// * [locations certificate maps delete projects](ProjectLocationCertificateMapDeleteCall) (response)
/// * [locations certificate maps patch projects](ProjectLocationCertificateMapPatchCall) (response)
/// * [locations certificates create projects](ProjectLocationCertificateCreateCall) (response)
/// * [locations certificates delete projects](ProjectLocationCertificateDeleteCall) (response)
/// * [locations certificates patch projects](ProjectLocationCertificatePatchCall) (response)
/// * [locations dns authorizations create projects](ProjectLocationDnsAuthorizationCreateCall) (response)
/// * [locations dns authorizations delete projects](ProjectLocationDnsAuthorizationDeleteCall) (response)
/// * [locations dns authorizations patch projects](ProjectLocationDnsAuthorizationPatchCall) (response)
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


/// Information about issues with provisioning a Managed Certificate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProvisioningIssue {
    /// Output only. Human readable explanation about the issue. Provided to help address the configuration issues. Not guaranteed to be stable. For programmatic access use Reason enum.
    
    pub details: Option<String>,
    /// Output only. Reason for provisioning failures.
    
    pub reason: Option<ProvisioningIssueReasonEnum>,
}

impl client::Part for ProvisioningIssue {}


/// Certificate data for a SelfManaged Certificate. SelfManaged Certificates are uploaded by the user. Updating such certificates before they expire remains the user's responsibility.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SelfManagedCertificate {
    /// Input only. The PEM-encoded certificate chain. Leaf certificate comes first, followed by intermediate ones if any.
    #[serde(rename="pemCertificate")]
    
    pub pem_certificate: Option<String>,
    /// Input only. The PEM-encoded private key of the leaf certificate.
    #[serde(rename="pemPrivateKey")]
    
    pub pem_private_key: Option<String>,
}

impl client::Part for SelfManagedCertificate {}


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


