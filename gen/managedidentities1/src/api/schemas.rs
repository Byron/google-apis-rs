use super::*;
/// Request message for AttachTrust
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains attach trust projects](ProjectLocationGlobalDomainAttachTrustCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AttachTrustRequest {
    /// Required. The domain trust resource.
    
    pub trust: Option<Trust>,
}

impl client::RequestValue for AttachTrustRequest {}


/// Represents a Managed Microsoft Identities backup.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains backups create projects](ProjectLocationGlobalDomainBackupCreateCall) (request)
/// * [locations global domains backups get projects](ProjectLocationGlobalDomainBackupGetCall) (response)
/// * [locations global domains backups patch projects](ProjectLocationGlobalDomainBackupPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Backup {
    /// Output only. The time the backups was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Resource labels to represent user provided metadata.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The unique name of the Backup in the form of `projects/{project_id}/locations/global/domains/{domain_name}/backups/{name}`
    
    pub name: Option<String>,
    /// Output only. The current state of the backup.
    
    pub state: Option<String>,
    /// Output only. Additional information about the current status of this backup, if available.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// Output only. Indicates whether it’s an on-demand backup or scheduled.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. Last update time.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Backup {}
impl client::ResponseResult for Backup {}


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


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global operations cancel projects](ProjectLocationGlobalOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// Certificate used to configure LDAPS.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Certificate {
    /// The certificate expire time.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The issuer of this certificate.
    #[serde(rename="issuingCertificate")]
    
    pub issuing_certificate: Option<Option<Box<Certificate>>>,
    /// The certificate subject.
    
    pub subject: Option<String>,
    /// The additional hostnames for the domain.
    #[serde(rename="subjectAlternativeName")]
    
    pub subject_alternative_name: Option<Vec<String>>,
    /// The certificate thumbprint which uniquely identifies the certificate.
    
    pub thumbprint: Option<String>,
}

impl client::Part for Certificate {}


/// Request message for DetachTrust
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains detach trust projects](ProjectLocationGlobalDomainDetachTrustCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetachTrustRequest {
    /// Required. The domain trust resource to removed.
    
    pub trust: Option<Trust>,
}

impl client::RequestValue for DetachTrustRequest {}


/// Represents a managed Microsoft Active Directory domain. If the domain is being changed, it will be placed into the UPDATING state, which indicates that the resource is being reconciled. At this point, Get will reflect an intermediate state.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains create projects](ProjectLocationGlobalDomainCreateCall) (request)
/// * [locations global domains get projects](ProjectLocationGlobalDomainGetCall) (response)
/// * [locations global domains patch projects](ProjectLocationGlobalDomainPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Domain {
    /// Optional. The name of delegated administrator account used to perform Active Directory operations. If not specified, `setupadmin` will be used.
    
    pub admin: Option<String>,
    /// Optional. Configuration for audit logs. True if audit logs are enabled, else false. Default is audit logs disabled.
    #[serde(rename="auditLogsEnabled")]
    
    pub audit_logs_enabled: Option<bool>,
    /// Optional. The full names of the Google Compute Engine [networks](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) the domain instance is connected to. Networks can be added using UpdateDomain. The domain is only available on networks listed in `authorized_networks`. If CIDR subnets overlap between networks, domain creation will fail.
    #[serde(rename="authorizedNetworks")]
    
    pub authorized_networks: Option<Vec<String>>,
    /// Output only. The time the instance was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The fully-qualified domain name of the exposed domain used by clients to connect to the service. Similar to what would be chosen for an Active Directory set up on an internal network.
    
    pub fqdn: Option<String>,
    /// Optional. Resource labels that can contain user-provided metadata.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. Locations where domain needs to be provisioned. regions e.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block.
    
    pub locations: Option<Vec<String>>,
    /// Required. The unique name of the domain using the form: `projects/{project_id}/locations/global/domains/{domain_name}`.
    
    pub name: Option<String>,
    /// Required. The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger. Ranges must be unique and non-overlapping with existing subnets in [Domain].[authorized_networks].
    #[serde(rename="reservedIpRange")]
    
    pub reserved_ip_range: Option<String>,
    /// Output only. The current state of this domain.
    
    pub state: Option<String>,
    /// Output only. Additional information about the current status of this domain, if available.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// Output only. The current trusts associated with the domain.
    
    pub trusts: Option<Vec<Trust>>,
    /// Output only. The last update time.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Domain {}
impl client::ResponseResult for Domain {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global operations cancel projects](ProjectLocationGlobalOperationCancelCall) (response)
/// * [locations global operations delete projects](ProjectLocationGlobalOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


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


/// ExtendSchemaRequest is the request message for ExtendSchema method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains extend schema projects](ProjectLocationGlobalDomainExtendSchemaCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExtendSchemaRequest {
    /// Required. Description for Schema Change.
    
    pub description: Option<String>,
    /// File uploaded as a byte stream input.
    #[serde(rename="fileContents")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub file_contents: Option<Vec<u8>>,
    /// File stored in Cloud Storage bucket and represented in the form projects/{project_id}/buckets/{bucket_name}/objects/{object_name} File should be in the same project as the domain.
    #[serde(rename="gcsPath")]
    
    pub gcs_path: Option<String>,
}

impl client::RequestValue for ExtendSchemaRequest {}


/// LDAPSSettings represents the ldaps settings for domain resource. LDAP is the Lightweight Directory Access Protocol, defined in https://tools.ietf.org/html/rfc4511. The settings object configures LDAP over SSL/TLS, whether it is over port 636 or the StartTLS operation. If LDAPSSettings is being changed, it will be placed into the UPDATING state, which indicates that the resource is being reconciled. At this point, Get will reflect an intermediate state.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains get ldapssettings projects](ProjectLocationGlobalDomainGetLdapssettingCall) (response)
/// * [locations global domains update ldapssettings projects](ProjectLocationGlobalDomainUpdateLdapssettingCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LDAPSSettings {
    /// Output only. The certificate used to configure LDAPS. Certificates can be chained with a maximum length of 15.
    
    pub certificate: Option<Certificate>,
    /// Input only. The password used to encrypt the uploaded PFX certificate.
    #[serde(rename="certificatePassword")]
    
    pub certificate_password: Option<String>,
    /// Input only. The uploaded PKCS12-formatted certificate to configure LDAPS with. It will enable the domain controllers in this domain to accept LDAPS connections (either LDAP over SSL/TLS or the StartTLS operation). A valid certificate chain must form a valid x.509 certificate chain (or be comprised of a single self-signed certificate. It must be encrypted with either: 1) PBES2 + PBKDF2 + AES256 encryption and SHA256 PRF; or 2) pbeWithSHA1And3-KeyTripleDES-CBC Private key must be included for the leaf / single self-signed certificate. Note: For a fqdn your-example-domain.com, the wildcard fqdn is *.your-example-domain.com. Specifically the leaf certificate must have: - Either a blank subject or a subject with CN matching the wildcard fqdn. - Exactly two SANs - the fqdn and wildcard fqdn. - Encipherment and digital key signature key usages. - Server authentication extended key usage (OID=1.3.6.1.5.5.7.3.1) - Private key must be in one of the following formats: RSA, ECDSA, ED25519. - Private key must have appropriate key length: 2048 for RSA, 256 for ECDSA - Signature algorithm of the leaf certificate cannot be MD2, MD5 or SHA1.
    #[serde(rename="certificatePfx")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub certificate_pfx: Option<Vec<u8>>,
    /// The resource name of the LDAPS settings. Uses the form: `projects/{project}/locations/{location}/domains/{domain}`.
    
    pub name: Option<String>,
    /// Output only. The current state of this LDAPS settings.
    
    pub state: Option<String>,
    /// Output only. Last update time.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for LDAPSSettings {}
impl client::ResponseResult for LDAPSSettings {}


/// ListBackupsResponse is the response message for ListBackups method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains backups list projects](ProjectLocationGlobalDomainBackupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBackupsResponse {
    /// A list of Cloud AD backups in the domain.
    
    pub backups: Option<Vec<Backup>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListBackupsResponse {}


/// Response message for ListDomains
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains list projects](ProjectLocationGlobalDomainListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDomainsResponse {
    /// A list of Managed Identities Service domains in the project.
    
    pub domains: Option<Vec<Domain>>,
    /// A token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListDomainsResponse {}


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
/// * [locations global operations list projects](ProjectLocationGlobalOperationListCall) (response)
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


/// ListPeeringsResponse is the response message for ListPeerings method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global peerings list projects](ProjectLocationGlobalPeeringListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPeeringsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of Managed Identities Service Peerings in the project.
    
    pub peerings: Option<Vec<Peering>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListPeeringsResponse {}


/// ListSqlIntegrationsResponse is the response message for ListSqlIntegrations method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains sql integrations list projects](ProjectLocationGlobalDomainSqlIntegrationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSqlIntegrationsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of SQLIntegrations of a domain.
    #[serde(rename="sqlIntegrations")]
    
    pub sql_integrations: Option<Vec<SqlIntegration>>,
    /// A list of locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListSqlIntegrationsResponse {}


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


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains backups create projects](ProjectLocationGlobalDomainBackupCreateCall) (response)
/// * [locations global domains backups delete projects](ProjectLocationGlobalDomainBackupDeleteCall) (response)
/// * [locations global domains backups patch projects](ProjectLocationGlobalDomainBackupPatchCall) (response)
/// * [locations global domains attach trust projects](ProjectLocationGlobalDomainAttachTrustCall) (response)
/// * [locations global domains create projects](ProjectLocationGlobalDomainCreateCall) (response)
/// * [locations global domains delete projects](ProjectLocationGlobalDomainDeleteCall) (response)
/// * [locations global domains detach trust projects](ProjectLocationGlobalDomainDetachTrustCall) (response)
/// * [locations global domains extend schema projects](ProjectLocationGlobalDomainExtendSchemaCall) (response)
/// * [locations global domains patch projects](ProjectLocationGlobalDomainPatchCall) (response)
/// * [locations global domains reconfigure trust projects](ProjectLocationGlobalDomainReconfigureTrustCall) (response)
/// * [locations global domains restore projects](ProjectLocationGlobalDomainRestoreCall) (response)
/// * [locations global domains update ldapssettings projects](ProjectLocationGlobalDomainUpdateLdapssettingCall) (response)
/// * [locations global domains validate trust projects](ProjectLocationGlobalDomainValidateTrustCall) (response)
/// * [locations global operations get projects](ProjectLocationGlobalOperationGetCall) (response)
/// * [locations global peerings create projects](ProjectLocationGlobalPeeringCreateCall) (response)
/// * [locations global peerings delete projects](ProjectLocationGlobalPeeringDeleteCall) (response)
/// * [locations global peerings patch projects](ProjectLocationGlobalPeeringPatchCall) (response)
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


/// Represents a Managed Service for Microsoft Active Directory Peering.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global peerings create projects](ProjectLocationGlobalPeeringCreateCall) (request)
/// * [locations global peerings get projects](ProjectLocationGlobalPeeringGetCall) (response)
/// * [locations global peerings patch projects](ProjectLocationGlobalPeeringPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Peering {
    /// Required. The full names of the Google Compute Engine [networks](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail.
    #[serde(rename="authorizedNetwork")]
    
    pub authorized_network: Option<String>,
    /// Output only. The time the instance was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form: `projects/{project_id}/locations/global/domains/{domain_name}`
    #[serde(rename="domainResource")]
    
    pub domain_resource: Option<String>,
    /// Optional. Resource labels to represent user-provided metadata.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. Unique name of the peering in this scope including projects and location using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`.
    
    pub name: Option<String>,
    /// Output only. The current state of this Peering.
    
    pub state: Option<String>,
    /// Output only. Additional information about the current status of this peering, if available.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// Output only. Last update time.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Peering {}
impl client::ResponseResult for Peering {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains backups get iam policy projects](ProjectLocationGlobalDomainBackupGetIamPolicyCall) (response)
/// * [locations global domains backups set iam policy projects](ProjectLocationGlobalDomainBackupSetIamPolicyCall) (response)
/// * [locations global domains get iam policy projects](ProjectLocationGlobalDomainGetIamPolicyCall) (response)
/// * [locations global domains set iam policy projects](ProjectLocationGlobalDomainSetIamPolicyCall) (response)
/// * [locations global peerings get iam policy projects](ProjectLocationGlobalPeeringGetIamPolicyCall) (response)
/// * [locations global peerings set iam policy projects](ProjectLocationGlobalPeeringSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// Request message for ReconfigureTrust
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains reconfigure trust projects](ProjectLocationGlobalDomainReconfigureTrustCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReconfigureTrustRequest {
    /// Required. The target DNS server IP addresses to resolve the remote domain involved in the trust.
    #[serde(rename="targetDnsIpAddresses")]
    
    pub target_dns_ip_addresses: Option<Vec<String>>,
    /// Required. The fully-qualified target domain name which will be in trust with current domain.
    #[serde(rename="targetDomainName")]
    
    pub target_domain_name: Option<String>,
}

impl client::RequestValue for ReconfigureTrustRequest {}


/// Request message for ResetAdminPassword
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains reset admin password projects](ProjectLocationGlobalDomainResetAdminPasswordCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResetAdminPasswordRequest { _never_set: Option<bool> }

impl client::RequestValue for ResetAdminPasswordRequest {}


/// Response message for ResetAdminPassword
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains reset admin password projects](ProjectLocationGlobalDomainResetAdminPasswordCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResetAdminPasswordResponse {
    /// A random password. See admin for more information.
    
    pub password: Option<String>,
}

impl client::ResponseResult for ResetAdminPasswordResponse {}


/// RestoreDomainRequest is the request received by RestoreDomain rpc
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains restore projects](ProjectLocationGlobalDomainRestoreCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestoreDomainRequest {
    /// Required. ID of the backup to be restored
    #[serde(rename="backupId")]
    
    pub backup_id: Option<String>,
}

impl client::RequestValue for RestoreDomainRequest {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains backups set iam policy projects](ProjectLocationGlobalDomainBackupSetIamPolicyCall) (request)
/// * [locations global domains set iam policy projects](ProjectLocationGlobalDomainSetIamPolicyCall) (request)
/// * [locations global peerings set iam policy projects](ProjectLocationGlobalPeeringSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Represents the SQL instance integrated with Managed AD.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains sql integrations get projects](ProjectLocationGlobalDomainSqlIntegrationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlIntegration {
    /// Output only. The time the SQL integration was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The unique name of the SQL integration in the form of `projects/{project_id}/locations/global/domains/{domain_name}/sqlIntegrations/{sql_integration}`
    
    pub name: Option<String>,
    /// The full resource name of an integrated SQL instance
    #[serde(rename="sqlInstance")]
    
    pub sql_instance: Option<String>,
    /// Output only. The current state of the SQL integration.
    
    pub state: Option<String>,
    /// Output only. The time the SQL integration was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for SqlIntegration {}


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


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains backups test iam permissions projects](ProjectLocationGlobalDomainBackupTestIamPermissionCall) (request)
/// * [locations global domains test iam permissions projects](ProjectLocationGlobalDomainTestIamPermissionCall) (request)
/// * [locations global peerings test iam permissions projects](ProjectLocationGlobalPeeringTestIamPermissionCall) (request)
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
/// * [locations global domains backups test iam permissions projects](ProjectLocationGlobalDomainBackupTestIamPermissionCall) (response)
/// * [locations global domains test iam permissions projects](ProjectLocationGlobalDomainTestIamPermissionCall) (response)
/// * [locations global peerings test iam permissions projects](ProjectLocationGlobalPeeringTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// Represents a relationship between two domains. This allows a controller in one domain to authenticate a user in another domain. If the trust is being changed, it will be placed into the UPDATING state, which indicates that the resource is being reconciled. At this point, Get will reflect an intermediate state.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Trust {
    /// Output only. The time the instance was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The last heartbeat time when the trust was known to be connected.
    #[serde(rename="lastTrustHeartbeatTime")]
    
    pub last_trust_heartbeat_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The trust authentication type, which decides whether the trusted side has forest/domain wide access or selective access to an approved set of resources.
    #[serde(rename="selectiveAuthentication")]
    
    pub selective_authentication: Option<bool>,
    /// Output only. The current state of the trust.
    
    pub state: Option<String>,
    /// Output only. Additional information about the current state of the trust, if available.
    #[serde(rename="stateDescription")]
    
    pub state_description: Option<String>,
    /// Required. The target DNS server IP addresses which can resolve the remote domain involved in the trust.
    #[serde(rename="targetDnsIpAddresses")]
    
    pub target_dns_ip_addresses: Option<Vec<String>>,
    /// Required. The fully qualified target domain name which will be in trust with the current domain.
    #[serde(rename="targetDomainName")]
    
    pub target_domain_name: Option<String>,
    /// Required. The trust direction, which decides if the current domain is trusted, trusting, or both.
    #[serde(rename="trustDirection")]
    
    pub trust_direction: Option<String>,
    /// Required. The trust secret used for the handshake with the target domain. This will not be stored.
    #[serde(rename="trustHandshakeSecret")]
    
    pub trust_handshake_secret: Option<String>,
    /// Required. The type of trust represented by the trust resource.
    #[serde(rename="trustType")]
    
    pub trust_type: Option<String>,
    /// Output only. The last update time.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Trust {}


/// Request message for ValidateTrust
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global domains validate trust projects](ProjectLocationGlobalDomainValidateTrustCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ValidateTrustRequest {
    /// Required. The domain trust to validate trust state for.
    
    pub trust: Option<Trust>,
}

impl client::RequestValue for ValidateTrustRequest {}


