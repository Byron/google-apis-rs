use super::*;
/// Represents an access event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Access {
    /// Caller's IP address, such as "1.1.1.1".
    #[serde(rename="callerIp")]
    
    pub caller_ip: Option<String>,
    /// The caller IP's geolocation, which identifies where the call came from.
    #[serde(rename="callerIpGeo")]
    
    pub caller_ip_geo: Option<Geolocation>,
    /// The method that the service account called, e.g. "SetIamPolicy".
    #[serde(rename="methodName")]
    
    pub method_name: Option<String>,
    /// Associated email, such as "foo@google.com". The email address of the authenticated user (or service account on behalf of third party principal) making the request. For third party identity callers, the `principal_subject` field is populated instead of this field. For privacy reasons, the principal email address is sometimes redacted. For more information, see [Caller identities in audit logs](https://cloud.google.com/logging/docs/audit#user-id).
    #[serde(rename="principalEmail")]
    
    pub principal_email: Option<String>,
    /// A string representing the principal_subject associated with the identity. As compared to `principal_email`, supports principals that aren't associated with email addresses, such as third party principals. For most identities, the format will be `principal://iam.googleapis.com/{identity pool name}/subjects/{subject}` except for some GKE identities (GKE_WORKLOAD, FREEFORM, GKE_HUB_WORKLOAD) that are still in the legacy format `serviceAccount:{identity pool name}[{subject}]`
    #[serde(rename="principalSubject")]
    
    pub principal_subject: Option<String>,
    /// Identity delegation history of an authenticated service account that makes the request. It contains information on the real authorities that try to access GCP resources by delegating on a service account. When multiple authorities are present, they are guaranteed to be sorted based on the original ordering of the identity delegation events.
    #[serde(rename="serviceAccountDelegationInfo")]
    
    pub service_account_delegation_info: Option<Vec<ServiceAccountDelegationInfo>>,
    /// The name of the service account key used to create or exchange credentials for authenticating the service account making the request. This is a scheme-less URI full resource name. For example: "//iam.googleapis.com/projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}" 
    #[serde(rename="serviceAccountKeyName")]
    
    pub service_account_key_name: Option<String>,
    /// This is the API service that the service account made a call to, e.g. "iam.googleapis.com"
    #[serde(rename="serviceName")]
    
    pub service_name: Option<String>,
    /// What kind of user agent is associated, for example operating system shells, embedded or stand-alone applications, etc.
    #[serde(rename="userAgentFamily")]
    
    pub user_agent_family: Option<String>,
    /// A string that represents the username of a user, user account, or other entity involved in the access event. What the entity is and what its role in the access event is depends on the finding that this field appears in. The entity is likely not an IAM principal, but could be a user that is logged into an operating system, if the finding is VM-related, or a user that is logged into some type of application that is involved in the access event.
    #[serde(rename="userName")]
    
    pub user_name: Option<String>,
}

impl client::Part for Access {}


/// Conveys information about a Kubernetes access review (e.g. kubectl auth can-i ...) that was involved in a finding.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccessReview {
    /// Group is the API Group of the Resource. "*" means all.
    
    pub group: Option<String>,
    /// Name is the name of the resource being requested. Empty means all.
    
    pub name: Option<String>,
    /// Namespace of the action being requested. Currently, there is no distinction between no namespace and all namespaces. Both are represented by "" (empty).
    
    pub ns: Option<String>,
    /// Resource is the optional resource type requested. "*" means all.
    
    pub resource: Option<String>,
    /// Subresource is the optional subresource type.
    
    pub subresource: Option<String>,
    /// Verb is a Kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy. "*" means all.
    
    pub verb: Option<String>,
    /// Version is the API Version of the Resource. "*" means all.
    
    pub version: Option<String>,
}

impl client::Part for AccessReview {}


/// Security Command Center representation of a Google Cloud resource. The Asset is a Security Command Center resource that captures information about a single Google Cloud resource. All modifications to an Asset are only within the context of Security Command Center and don't affect the referenced Google Cloud resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    /// The canonical name of the resource. It's either "organizations/{organization_id}/assets/{asset_id}", "folders/{folder_id}/assets/{asset_id}" or "projects/{project_number}/assets/{asset_id}", depending on the closest CRM ancestor of the resource.
    #[serde(rename="canonicalName")]
    
    pub canonical_name: Option<String>,
    /// The time at which the asset was created in Security Command Center.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Cloud IAM Policy information associated with the Google Cloud resource described by the Security Command Center asset. This information is managed and defined by the Google Cloud resource and cannot be modified by the user.
    #[serde(rename="iamPolicy")]
    
    pub iam_policy: Option<IamPolicy>,
    /// The relative resource name of this asset. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/assets/{asset_id}".
    
    pub name: Option<String>,
    /// Resource managed properties. These properties are managed and defined by the Google Cloud resource and cannot be modified by the user.
    #[serde(rename="resourceProperties")]
    
    pub resource_properties: Option<HashMap<String, json::Value>>,
    /// Security Command Center managed properties. These properties are managed by Security Command Center and cannot be modified by the user.
    #[serde(rename="securityCenterProperties")]
    
    pub security_center_properties: Option<SecurityCenterProperties>,
    /// User specified security marks. These marks are entirely managed by the user and come from the SecurityMarks resource that belongs to the asset.
    #[serde(rename="securityMarks")]
    
    pub security_marks: Option<SecurityMarks>,
    /// The time at which the asset was last updated or added in Cloud SCC.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Asset {}


/// The configuration used for Asset Discovery runs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AssetDiscoveryConfig {
    /// The folder ids to use for filtering asset discovery. It consists of only digits, e.g., 756619654966.
    #[serde(rename="folderIds")]
    
    pub folder_ids: Option<Vec<String>>,
    /// The mode to use for filtering asset discovery.
    #[serde(rename="inclusionMode")]
    
    pub inclusion_mode: Option<String>,
    /// The project ids to use for filtering asset discovery.
    #[serde(rename="projectIds")]
    
    pub project_ids: Option<Vec<String>>,
}

impl client::Part for AssetDiscoveryConfig {}


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


/// Request message for bulk findings update. Note: 1. If multiple bulk update requests match the same resource, the order in which they get executed is not defined. 2. Once a bulk operation is started, there is no way to stop it.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [findings bulk mute folders](FolderFindingBulkMuteCall) (request)
/// * [findings bulk mute organizations](OrganizationFindingBulkMuteCall) (request)
/// * [findings bulk mute projects](ProjectFindingBulkMuteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkMuteFindingsRequest {
    /// Expression that identifies findings that should be updated. The expression is a list of zero or more restrictions combined via logical operators `AND` and `OR`. Parentheses are supported, and `OR` has higher precedence than `AND`. Restrictions have the form ` ` and may have a `-` character in front of them to indicate negation. The fields map to those defined in the corresponding resource. The supported operators are: * `=` for all value types. * `>`, `<`, `>=`, `<=` for integer values. * `:`, meaning substring matching, for strings. The supported value types are: * string literals in quotes. * integer literals without quotes. * boolean literals `true` and `false` without quotes.
    
    pub filter: Option<String>,
    /// This can be a mute configuration name or any identifier for mute/unmute of findings based on the filter.
    #[serde(rename="muteAnnotation")]
    
    pub mute_annotation: Option<String>,
}

impl client::RequestValue for BulkMuteFindingsRequest {}


/// Contains compliance information about a security standard indicating unmet recommendations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Compliance {
    /// Policies within the standard/benchmark e.g. A.12.4.1
    
    pub ids: Option<Vec<String>>,
    /// Refers to industry wide standards or benchmarks e.g. "cis", "pci", "owasp", etc.
    
    pub standard: Option<String>,
    /// Version of the standard/benchmark e.g. 1.1
    
    pub version: Option<String>,
}

impl client::Part for Compliance {}


/// Contains information about the IP connection associated with the finding.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Connection {
    /// Destination IP address. Not present for sockets that are listening and not connected.
    #[serde(rename="destinationIp")]
    
    pub destination_ip: Option<String>,
    /// Destination port. Not present for sockets that are listening and not connected.
    #[serde(rename="destinationPort")]
    
    pub destination_port: Option<i32>,
    /// IANA Internet Protocol Number such as TCP(6) and UDP(17).
    
    pub protocol: Option<String>,
    /// Source IP address.
    #[serde(rename="sourceIp")]
    
    pub source_ip: Option<String>,
    /// Source port.
    #[serde(rename="sourcePort")]
    
    pub source_port: Option<i32>,
}

impl client::Part for Connection {}


/// The email address of a contact.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Contact {
    /// An email address. For example, "`person123@company.com`".
    
    pub email: Option<String>,
}

impl client::Part for Contact {}


/// The details pertaining to specific contacts
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactDetails {
    /// A list of contacts
    
    pub contacts: Option<Vec<Contact>>,
}

impl client::Part for ContactDetails {}


/// Container associated with the finding.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Container {
    /// Optional container image id, when provided by the container runtime. Uniquely identifies the container image launched using a container image digest.
    #[serde(rename="imageId")]
    
    pub image_id: Option<String>,
    /// Container labels, as provided by the container runtime.
    
    pub labels: Option<Vec<Label>>,
    /// Container name.
    
    pub name: Option<String>,
    /// Container image URI provided when configuring a pod/container. May identify a container image version using mutable tags.
    
    pub uri: Option<String>,
}

impl client::Part for Container {}


/// CVE stands for Common Vulnerabilities and Exposures. More information: https://cve.mitre.org
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cve {
    /// Describe Common Vulnerability Scoring System specified at https://www.first.org/cvss/v3.1/specification-document
    
    pub cvssv3: Option<Cvssv3>,
    /// The unique identifier for the vulnerability. e.g. CVE-2021-34527
    
    pub id: Option<String>,
    /// Additional information about the CVE. e.g. https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-34527
    
    pub references: Option<Vec<Reference>>,
    /// Whether upstream fix is available for the CVE.
    #[serde(rename="upstreamFixAvailable")]
    
    pub upstream_fix_available: Option<bool>,
}

impl client::Part for Cve {}


/// Common Vulnerability Scoring System version 3.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cvssv3 {
    /// This metric describes the conditions beyond the attacker's control that must exist in order to exploit the vulnerability.
    #[serde(rename="attackComplexity")]
    
    pub attack_complexity: Option<String>,
    /// Base Metrics Represents the intrinsic characteristics of a vulnerability that are constant over time and across user environments. This metric reflects the context by which vulnerability exploitation is possible.
    #[serde(rename="attackVector")]
    
    pub attack_vector: Option<String>,
    /// This metric measures the impact to the availability of the impacted component resulting from a successfully exploited vulnerability.
    #[serde(rename="availabilityImpact")]
    
    pub availability_impact: Option<String>,
    /// The base score is a function of the base metric scores.
    #[serde(rename="baseScore")]
    
    pub base_score: Option<f64>,
    /// This metric measures the impact to the confidentiality of the information resources managed by a software component due to a successfully exploited vulnerability.
    #[serde(rename="confidentialityImpact")]
    
    pub confidentiality_impact: Option<String>,
    /// This metric measures the impact to integrity of a successfully exploited vulnerability.
    #[serde(rename="integrityImpact")]
    
    pub integrity_impact: Option<String>,
    /// This metric describes the level of privileges an attacker must possess before successfully exploiting the vulnerability.
    #[serde(rename="privilegesRequired")]
    
    pub privileges_required: Option<String>,
    /// The Scope metric captures whether a vulnerability in one vulnerable component impacts resources in components beyond its security scope.
    
    pub scope: Option<String>,
    /// This metric captures the requirement for a human user, other than the attacker, to participate in the successful compromise of the vulnerable component.
    #[serde(rename="userInteraction")]
    
    pub user_interaction: Option<String>,
}

impl client::Part for Cvssv3 {}


/// Represents database access information, such as queries. A database may be a sub-resource of an instance (as in the case of CloudSQL instances or Cloud Spanner instances), or the database instance itself. Some database resources may not have the full resource name populated because these resource types are not yet supported by Cloud Asset Inventory (e.g. CloudSQL databases). In these cases only the display name will be provided.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Database {
    /// The human readable name of the database the user connected to.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The target usernames/roles/groups of a SQL privilege grant (not an IAM policy change).
    
    pub grantees: Option<Vec<String>>,
    /// The full resource name of the database the user connected to, if it is supported by CAI. (https://google.aip.dev/122#full-resource-names)
    
    pub name: Option<String>,
    /// The SQL statement associated with the relevant access.
    
    pub query: Option<String>,
    /// The username used to connect to the DB. This may not necessarily be an IAM principal, and has no required format.
    #[serde(rename="userName")]
    
    pub user_name: Option<String>,
}

impl client::Part for Database {}


/// Memory hash detection contributing to the binary family match.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Detection {
    /// The name of the binary associated with the memory hash signature detection.
    
    pub binary: Option<String>,
    /// The percentage of memory page hashes in the signature that were matched.
    #[serde(rename="percentPagesMatched")]
    
    pub percent_pages_matched: Option<f64>,
}

impl client::Part for Detection {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [big query exports delete folders](FolderBigQueryExportDeleteCall) (response)
/// * [mute configs delete folders](FolderMuteConfigDeleteCall) (response)
/// * [notification configs delete folders](FolderNotificationConfigDeleteCall) (response)
/// * [big query exports delete organizations](OrganizationBigQueryExportDeleteCall) (response)
/// * [mute configs delete organizations](OrganizationMuteConfigDeleteCall) (response)
/// * [notification configs delete organizations](OrganizationNotificationConfigDeleteCall) (response)
/// * [operations cancel organizations](OrganizationOperationCancelCall) (response)
/// * [operations delete organizations](OrganizationOperationDeleteCall) (response)
/// * [big query exports delete projects](ProjectBigQueryExportDeleteCall) (response)
/// * [mute configs delete projects](ProjectMuteConfigDeleteCall) (response)
/// * [notification configs delete projects](ProjectNotificationConfigDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// EnvironmentVariable is a name-value pair to store environment variables for Process.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    /// Environment variable name as a JSON encoded string.
    
    pub name: Option<String>,
    /// Environment variable value as a JSON encoded string.
    
    pub val: Option<String>,
}

impl client::Part for EnvironmentVariable {}


/// Resource that has been exfiltrated or exfiltrated_to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExfilResource {
    /// Subcomponents of the asset that is exfiltrated - these could be URIs used during exfiltration, table names, databases, filenames, etc. For example, multiple tables may be exfiltrated from the same CloudSQL instance, or multiple files from the same Cloud Storage bucket.
    
    pub components: Option<Vec<String>>,
    /// Resource's URI (https://google.aip.dev/122#full-resource-names)
    
    pub name: Option<String>,
}

impl client::Part for ExfilResource {}


/// Exfiltration represents a data exfiltration attempt of one or more sources to one or more targets. Sources represent the source of data that is exfiltrated, and Targets represents the destination the data was copied to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Exfiltration {
    /// If there are multiple sources, then the data is considered "joined" between them. For instance, BigQuery can join multiple tables, and each table would be considered a source.
    
    pub sources: Option<Vec<ExfilResource>>,
    /// If there are multiple targets, each target would get a complete copy of the "joined" source data.
    
    pub targets: Option<Vec<ExfilResource>>,
}

impl client::Part for Exfiltration {}


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


/// File information about the related binary/library used by an executable, or the script used by a script interpreter
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct File {
    /// Prefix of the file contents as a JSON encoded string. (Currently only populated for Malicious Script Executed findings.)
    
    pub contents: Option<String>,
    /// The length in bytes of the file prefix that was hashed. If hashed_size == size, any hashes reported represent the entire file.
    #[serde(rename="hashedSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub hashed_size: Option<i64>,
    /// True when the hash covers only a prefix of the file.
    #[serde(rename="partiallyHashed")]
    
    pub partially_hashed: Option<bool>,
    /// Absolute path of the file as a JSON encoded string.
    
    pub path: Option<String>,
    /// SHA256 hash of the first hashed_size bytes of the file encoded as a hex string. If hashed_size == size, sha256 represents the SHA256 hash of the entire file.
    
    pub sha256: Option<String>,
    /// Size of the file in bytes.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size: Option<i64>,
}

impl client::Part for File {}


/// Security Command Center finding. A finding is a record of assessment data like security, risk, health, or privacy, that is ingested into Security Command Center for presentation, notification, analysis, policy testing, and enforcement. For example, a cross-site scripting (XSS) vulnerability in an App Engine application is a finding.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sources findings patch folders](FolderSourceFindingPatchCall) (request|response)
/// * [sources findings set mute folders](FolderSourceFindingSetMuteCall) (response)
/// * [sources findings set state folders](FolderSourceFindingSetStateCall) (response)
/// * [sources findings create organizations](OrganizationSourceFindingCreateCall) (request|response)
/// * [sources findings patch organizations](OrganizationSourceFindingPatchCall) (request|response)
/// * [sources findings set mute organizations](OrganizationSourceFindingSetMuteCall) (response)
/// * [sources findings set state organizations](OrganizationSourceFindingSetStateCall) (response)
/// * [sources findings patch projects](ProjectSourceFindingPatchCall) (request|response)
/// * [sources findings set mute projects](ProjectSourceFindingSetMuteCall) (response)
/// * [sources findings set state projects](ProjectSourceFindingSetStateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Finding {
    /// Access details associated to the Finding, such as more information on the caller, which method was accessed, from where, etc.
    
    pub access: Option<Access>,
    /// The canonical name of the finding. It's either "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}" or "projects/{project_number}/sources/{source_id}/findings/{finding_id}", depending on the closest CRM ancestor of the resource associated with the finding.
    #[serde(rename="canonicalName")]
    
    pub canonical_name: Option<String>,
    /// The additional taxonomy group within findings from a given source. This field is immutable after creation time. Example: "XSS_FLASH_INJECTION"
    
    pub category: Option<String>,
    /// Contains compliance information for security standards associated to the finding.
    
    pub compliances: Option<Vec<Compliance>>,
    /// Contains information about the IP connection associated with the finding.
    
    pub connections: Option<Vec<Connection>>,
    /// Output only. Map containing the points of contact for the given finding. The key represents the type of contact, while the value contains a list of all the contacts that pertain. Please refer to: https://cloud.google.com/resource-manager/docs/managing-notification-contacts#notification-categories { "security": { "contacts": [ { "email": "person1@company.com" }, { "email": "person2@company.com" } ] } }
    
    pub contacts: Option<HashMap<String, ContactDetails>>,
    /// Containers associated with the finding. containers provides information for both Kubernetes and non-Kubernetes containers.
    
    pub containers: Option<Vec<Container>>,
    /// The time at which the finding was created in Security Command Center.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Database associated with the finding.
    
    pub database: Option<Database>,
    /// Contains more detail about the finding.
    
    pub description: Option<String>,
    /// The time the finding was first detected. If an existing finding is updated, then this is the time the update occurred. For example, if the finding represents an open firewall, this property captures the time the detector believes the firewall became open. The accuracy is determined by the detector. If the finding is later resolved, then this time reflects when the finding was resolved. This must not be set to a value greater than the current timestamp.
    #[serde(rename="eventTime")]
    
    pub event_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Represents exfiltration associated with the Finding.
    
    pub exfiltration: Option<Exfiltration>,
    /// Output only. Third party SIEM/SOAR fields within SCC, contains external system information and external system finding fields.
    #[serde(rename="externalSystems")]
    
    pub external_systems: Option<HashMap<String, GoogleCloudSecuritycenterV1ExternalSystem>>,
    /// The URI that, if available, points to a web page outside of Security Command Center where additional information about the finding can be found. This field is guaranteed to be either empty or a well formed URL.
    #[serde(rename="externalUri")]
    
    pub external_uri: Option<String>,
    /// File associated with the finding.
    
    pub files: Option<Vec<File>>,
    /// The class of the finding.
    #[serde(rename="findingClass")]
    
    pub finding_class: Option<String>,
    /// Represents IAM bindings associated with the Finding.
    #[serde(rename="iamBindings")]
    
    pub iam_bindings: Option<Vec<IamBinding>>,
    /// Represents what's commonly known as an Indicator of compromise (IoC) in computer forensics. This is an artifact observed on a network or in an operating system that, with high confidence, indicates a computer intrusion. Reference: https://en.wikipedia.org/wiki/Indicator_of_compromise
    
    pub indicator: Option<Indicator>,
    /// Kernel Rootkit signature.
    #[serde(rename="kernelRootkit")]
    
    pub kernel_rootkit: Option<KernelRootkit>,
    /// Kubernetes resources associated with the finding.
    
    pub kubernetes: Option<Kubernetes>,
    /// MITRE ATT&CK tactics and techniques related to this finding. See: https://attack.mitre.org
    #[serde(rename="mitreAttack")]
    
    pub mitre_attack: Option<MitreAttack>,
    /// Indicates the mute state of a finding (either muted, unmuted or undefined). Unlike other attributes of a finding, a finding provider shouldn't set the value of mute.
    
    pub mute: Option<String>,
    /// First known as mute_annotation. Records additional information about the mute operation e.g. mute config that muted the finding, user who muted the finding, etc. Unlike other attributes of a finding, a finding provider shouldn't set the value of mute.
    #[serde(rename="muteInitiator")]
    
    pub mute_initiator: Option<String>,
    /// Output only. The most recent time this finding was muted or unmuted.
    #[serde(rename="muteUpdateTime")]
    
    pub mute_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The relative resource name of this finding. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}"
    
    pub name: Option<String>,
    /// Next steps associate to the finding.
    #[serde(rename="nextSteps")]
    
    pub next_steps: Option<String>,
    /// The relative resource name of the source the finding belongs to. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name This field is immutable after creation time. For example: "organizations/{organization_id}/sources/{source_id}"
    
    pub parent: Option<String>,
    /// Output only. The human readable display name of the finding source such as "Event Threat Detection" or "Security Health Analytics".
    #[serde(rename="parentDisplayName")]
    
    pub parent_display_name: Option<String>,
    /// Represents operating system processes associated with the Finding.
    
    pub processes: Option<Vec<Process>>,
    /// For findings on Google Cloud resources, the full resource name of the Google Cloud resource this finding is for. See: https://cloud.google.com/apis/design/resource_names#full_resource_name When the finding is for a non-Google Cloud resource, the resourceName can be a customer or partner defined string. This field is immutable after creation time.
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
    /// Output only. User specified security marks. These marks are entirely managed by the user and come from the SecurityMarks resource that belongs to the finding.
    #[serde(rename="securityMarks")]
    
    pub security_marks: Option<SecurityMarks>,
    /// The severity of the finding. This field is managed by the source that writes the finding.
    
    pub severity: Option<String>,
    /// Source specific properties. These properties are managed by the source that writes the finding. The key names in the source_properties map must be between 1 and 255 characters, and must start with a letter and contain alphanumeric characters or underscores only.
    #[serde(rename="sourceProperties")]
    
    pub source_properties: Option<HashMap<String, json::Value>>,
    /// The state of the finding.
    
    pub state: Option<String>,
    /// Represents vulnerability-specific fields like CVE and CVSS scores. CVE stands for Common Vulnerabilities and Exposures (https://cve.mitre.org/about/)
    
    pub vulnerability: Option<Vulnerability>,
}

impl client::RequestValue for Finding {}
impl client::ResponseResult for Finding {}


/// Message that contains the resource name and display name of a folder resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assets group folders](FolderAssetGroupCall) (none)
/// * [assets list folders](FolderAssetListCall) (none)
/// * [assets update security marks folders](FolderAssetUpdateSecurityMarkCall) (none)
/// * [big query exports create folders](FolderBigQueryExportCreateCall) (none)
/// * [big query exports delete folders](FolderBigQueryExportDeleteCall) (none)
/// * [big query exports get folders](FolderBigQueryExportGetCall) (none)
/// * [big query exports list folders](FolderBigQueryExportListCall) (none)
/// * [big query exports patch folders](FolderBigQueryExportPatchCall) (none)
/// * [findings bulk mute folders](FolderFindingBulkMuteCall) (none)
/// * [mute configs create folders](FolderMuteConfigCreateCall) (none)
/// * [mute configs delete folders](FolderMuteConfigDeleteCall) (none)
/// * [mute configs get folders](FolderMuteConfigGetCall) (none)
/// * [mute configs list folders](FolderMuteConfigListCall) (none)
/// * [mute configs patch folders](FolderMuteConfigPatchCall) (none)
/// * [notification configs create folders](FolderNotificationConfigCreateCall) (none)
/// * [notification configs delete folders](FolderNotificationConfigDeleteCall) (none)
/// * [notification configs get folders](FolderNotificationConfigGetCall) (none)
/// * [notification configs list folders](FolderNotificationConfigListCall) (none)
/// * [notification configs patch folders](FolderNotificationConfigPatchCall) (none)
/// * [sources findings external systems patch folders](FolderSourceFindingExternalSystemPatchCall) (none)
/// * [sources findings group folders](FolderSourceFindingGroupCall) (none)
/// * [sources findings list folders](FolderSourceFindingListCall) (none)
/// * [sources findings patch folders](FolderSourceFindingPatchCall) (none)
/// * [sources findings set mute folders](FolderSourceFindingSetMuteCall) (none)
/// * [sources findings set state folders](FolderSourceFindingSetStateCall) (none)
/// * [sources findings update security marks folders](FolderSourceFindingUpdateSecurityMarkCall) (none)
/// * [sources list folders](FolderSourceListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Folder {
    /// Full resource name of this folder. See: https://cloud.google.com/apis/design/resource_names#full_resource_name
    #[serde(rename="resourceFolder")]
    
    pub resource_folder: Option<String>,
    /// The user defined display name for this folder.
    #[serde(rename="resourceFolderDisplayName")]
    
    pub resource_folder_display_name: Option<String>,
}

impl client::Resource for Folder {}


/// Represents a geographical location for a given access.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Geolocation {
    /// A CLDR.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
}

impl client::Part for Geolocation {}


/// Request message for `GetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sources get iam policy organizations](OrganizationSourceGetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIamPolicyRequest {
    /// OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`.
    
    pub options: Option<GetPolicyOptions>,
}

impl client::RequestValue for GetIamPolicyRequest {}


/// Encapsulates settings provided to GetIamPolicy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetPolicyOptions {
    /// Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    #[serde(rename="requestedPolicyVersion")]
    
    pub requested_policy_version: Option<i32>,
}

impl client::Part for GetPolicyOptions {}


/// Configures how to deliver Findings to BigQuery Instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [big query exports create folders](FolderBigQueryExportCreateCall) (request|response)
/// * [big query exports get folders](FolderBigQueryExportGetCall) (response)
/// * [big query exports patch folders](FolderBigQueryExportPatchCall) (request|response)
/// * [big query exports create organizations](OrganizationBigQueryExportCreateCall) (request|response)
/// * [big query exports get organizations](OrganizationBigQueryExportGetCall) (response)
/// * [big query exports patch organizations](OrganizationBigQueryExportPatchCall) (request|response)
/// * [big query exports create projects](ProjectBigQueryExportCreateCall) (request|response)
/// * [big query exports get projects](ProjectBigQueryExportGetCall) (response)
/// * [big query exports patch projects](ProjectBigQueryExportPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudSecuritycenterV1BigQueryExport {
    /// Output only. The time at which the BigQuery export was created. This field is set by the server and will be ignored if provided on export on creation.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The dataset to write findings' updates to. Its format is "projects/[project_id]/datasets/[bigquery_dataset_id]". BigQuery Dataset unique ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_).
    
    pub dataset: Option<String>,
    /// The description of the export (max of 1024 characters).
    
    pub description: Option<String>,
    /// Expression that defines the filter to apply across create/update events of findings. The expression is a list of zero or more restrictions combined via logical operators `AND` and `OR`. Parentheses are supported, and `OR` has higher precedence than `AND`. Restrictions have the form ` ` and may have a `-` character in front of them to indicate negation. The fields map to those defined in the corresponding resource. The supported operators are: * `=` for all value types. * `>`, `<`, `>=`, `<=` for integer values. * `:`, meaning substring matching, for strings. The supported value types are: * string literals in quotes. * integer literals without quotes. * boolean literals `true` and `false` without quotes.
    
    pub filter: Option<String>,
    /// Output only. Email address of the user who last edited the BigQuery export. This field is set by the server and will be ignored if provided on export creation or update.
    #[serde(rename="mostRecentEditor")]
    
    pub most_recent_editor: Option<String>,
    /// The relative resource name of this export. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name. Example format: "organizations/{organization_id}/bigQueryExports/{export_id}" Example format: "folders/{folder_id}/bigQueryExports/{export_id}" Example format: "projects/{project_id}/bigQueryExports/{export_id}" This field is provided in responses, and is ignored when provided in create requests.
    
    pub name: Option<String>,
    /// Output only. The service account that needs permission to create table and upload data to the BigQuery dataset.
    
    pub principal: Option<String>,
    /// Output only. The most recent time at which the BigQuery export was updated. This field is set by the server and will be ignored if provided on export creation or update.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudSecuritycenterV1BigQueryExport {}
impl client::ResponseResult for GoogleCloudSecuritycenterV1BigQueryExport {}


/// Represents a Kubernetes RoleBinding or ClusterRoleBinding.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudSecuritycenterV1Binding {
    /// Name for binding.
    
    pub name: Option<String>,
    /// Namespace for binding.
    
    pub ns: Option<String>,
    /// The Role or ClusterRole referenced by the binding.
    
    pub role: Option<Role>,
    /// Represents one or more subjects that are bound to the role. Not always available for PATCH requests.
    
    pub subjects: Option<Vec<Subject>>,
}

impl client::Part for GoogleCloudSecuritycenterV1Binding {}


/// Representation of third party SIEM/SOAR fields within SCC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sources findings external systems patch folders](FolderSourceFindingExternalSystemPatchCall) (request|response)
/// * [sources findings external systems patch organizations](OrganizationSourceFindingExternalSystemPatchCall) (request|response)
/// * [sources findings external systems patch projects](ProjectSourceFindingExternalSystemPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudSecuritycenterV1ExternalSystem {
    /// References primary/secondary etc assignees in the external system.
    
    pub assignees: Option<Vec<String>>,
    /// The most recent time when the corresponding finding's ticket/tracker was updated in the external system.
    #[serde(rename="externalSystemUpdateTime")]
    
    pub external_system_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Identifier that's used to track the given finding in the external system.
    #[serde(rename="externalUid")]
    
    pub external_uid: Option<String>,
    /// Full resource name of the external system, for example: "organizations/1234/sources/5678/findings/123456/externalSystems/jira", "folders/1234/sources/5678/findings/123456/externalSystems/jira", "projects/1234/sources/5678/findings/123456/externalSystems/jira"
    
    pub name: Option<String>,
    /// Most recent status of the corresponding finding's ticket/tracker in the external system.
    
    pub status: Option<String>,
}

impl client::RequestValue for GoogleCloudSecuritycenterV1ExternalSystem {}
impl client::ResponseResult for GoogleCloudSecuritycenterV1ExternalSystem {}


/// A mute config is a Cloud SCC resource that contains the configuration to mute create/update events of findings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [mute configs create folders](FolderMuteConfigCreateCall) (request|response)
/// * [mute configs get folders](FolderMuteConfigGetCall) (response)
/// * [mute configs patch folders](FolderMuteConfigPatchCall) (request|response)
/// * [mute configs create organizations](OrganizationMuteConfigCreateCall) (request|response)
/// * [mute configs get organizations](OrganizationMuteConfigGetCall) (response)
/// * [mute configs patch organizations](OrganizationMuteConfigPatchCall) (request|response)
/// * [mute configs create projects](ProjectMuteConfigCreateCall) (request|response)
/// * [mute configs get projects](ProjectMuteConfigGetCall) (response)
/// * [mute configs patch projects](ProjectMuteConfigPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudSecuritycenterV1MuteConfig {
    /// Output only. The time at which the mute config was created. This field is set by the server and will be ignored if provided on config creation.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A description of the mute config.
    
    pub description: Option<String>,
    /// The human readable name to be displayed for the mute config.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. An expression that defines the filter to apply across create/update events of findings. While creating a filter string, be mindful of the scope in which the mute configuration is being created. E.g., If a filter contains project = X but is created under the project = Y scope, it might not match any findings. The following field and operator combinations are supported: * severity: `=`, `:` * category: `=`, `:` * resource.name: `=`, `:` * resource.project_name: `=`, `:` * resource.project_display_name: `=`, `:` * resource.folders.resource_folder: `=`, `:` * resource.parent_name: `=`, `:` * resource.parent_display_name: `=`, `:` * resource.type: `=`, `:` * finding_class: `=`, `:` * indicator.ip_addresses: `=`, `:` * indicator.domains: `=`, `:`
    
    pub filter: Option<String>,
    /// Output only. Email address of the user who last edited the mute config. This field is set by the server and will be ignored if provided on config creation or update.
    #[serde(rename="mostRecentEditor")]
    
    pub most_recent_editor: Option<String>,
    /// This field will be ignored if provided on config creation. Format "organizations/{organization}/muteConfigs/{mute_config}" "folders/{folder}/muteConfigs/{mute_config}" "projects/{project}/muteConfigs/{mute_config}"
    
    pub name: Option<String>,
    /// Output only. The most recent time at which the mute config was updated. This field is set by the server and will be ignored if provided on config creation or update.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudSecuritycenterV1MuteConfig {}
impl client::ResponseResult for GoogleCloudSecuritycenterV1MuteConfig {}


/// Request message for grouping by assets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assets group folders](FolderAssetGroupCall) (request)
/// * [assets group organizations](OrganizationAssetGroupCall) (request)
/// * [assets group projects](ProjectAssetGroupCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupAssetsRequest {
    /// When compare_duration is set, the GroupResult's "state_change" property is updated to indicate whether the asset was added, removed, or remained present during the compare_duration period of time that precedes the read_time. This is the time between (read_time - compare_duration) and read_time. The state change value is derived based on the presence of the asset at the two points in time. Intermediate state changes between the two times don't affect the result. For example, the results aren't affected if the asset is removed and re-created again. Possible "state_change" values when compare_duration is specified: * "ADDED": indicates that the asset was not present at the start of compare_duration, but present at reference_time. * "REMOVED": indicates that the asset was present at the start of compare_duration, but not present at reference_time. * "ACTIVE": indicates that the asset was present at both the start and the end of the time period defined by compare_duration and reference_time. If compare_duration is not specified, then the only possible state_change is "UNUSED", which will be the state_change set for all assets present at read_time. If this field is set then `state_change` must be a specified field in `group_by`.
    #[serde(rename="compareDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub compare_duration: Option<client::chrono::Duration>,
    /// Expression that defines the filter to apply across assets. The expression is a list of zero or more restrictions combined via logical operators `AND` and `OR`. Parentheses are supported, and `OR` has higher precedence than `AND`. Restrictions have the form ` ` and may have a `-` character in front of them to indicate negation. The fields map to those defined in the Asset resource. Examples include: * name * security_center_properties.resource_name * resource_properties.a_property * security_marks.marks.marka The supported operators are: * `=` for all value types. * `>`, `<`, `>=`, `<=` for integer values. * `:`, meaning substring matching, for strings. The supported value types are: * string literals in quotes. * integer literals without quotes. * boolean literals `true` and `false` without quotes. The following field and operator combinations are supported: * name: `=` * update_time: `=`, `>`, `<`, `>=`, `<=` Usage: This should be milliseconds since epoch or an RFC3339 string. Examples: `update_time = "2019-06-10T16:07:18-07:00"` `update_time = 1560208038000` * create_time: `=`, `>`, `<`, `>=`, `<=` Usage: This should be milliseconds since epoch or an RFC3339 string. Examples: `create_time = "2019-06-10T16:07:18-07:00"` `create_time = 1560208038000` * iam_policy.policy_blob: `=`, `:` * resource_properties: `=`, `:`, `>`, `<`, `>=`, `<=` * security_marks.marks: `=`, `:` * security_center_properties.resource_name: `=`, `:` * security_center_properties.resource_display_name: `=`, `:` * security_center_properties.resource_type: `=`, `:` * security_center_properties.resource_parent: `=`, `:` * security_center_properties.resource_parent_display_name: `=`, `:` * security_center_properties.resource_project: `=`, `:` * security_center_properties.resource_project_display_name: `=`, `:` * security_center_properties.resource_owners: `=`, `:` For example, `resource_properties.size = 100` is a valid filter string. Use a partial match on the empty string to filter based on a property existing: `resource_properties.my_property : ""` Use a negated partial match on the empty string to filter based on a property not existing: `-resource_properties.my_property : ""`
    
    pub filter: Option<String>,
    /// Required. Expression that defines what assets fields to use for grouping. The string value should follow SQL syntax: comma separated list of fields. For example: "security_center_properties.resource_project,security_center_properties.project". The following fields are supported when compare_duration is not set: * security_center_properties.resource_project * security_center_properties.resource_project_display_name * security_center_properties.resource_type * security_center_properties.resource_parent * security_center_properties.resource_parent_display_name The following fields are supported when compare_duration is set: * security_center_properties.resource_type * security_center_properties.resource_project_display_name * security_center_properties.resource_parent_display_name
    #[serde(rename="groupBy")]
    
    pub group_by: Option<String>,
    /// The maximum number of results to return in a single response. Default is 10, minimum is 1, maximum is 1000.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// The value returned by the last `GroupAssetsResponse`; indicates that this is a continuation of a prior `GroupAssets` call, and that the system should return the next page of data.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Time used as a reference point when filtering assets. The filter is limited to assets existing at the supplied time and their values are those at that specific time. Absence of this field will default to the API's version of NOW.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GroupAssetsRequest {}


/// Response message for grouping by assets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assets group folders](FolderAssetGroupCall) (response)
/// * [assets group organizations](OrganizationAssetGroupCall) (response)
/// * [assets group projects](ProjectAssetGroupCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupAssetsResponse {
    /// Group results. There exists an element for each existing unique combination of property/values. The element contains a count for the number of times those specific property/values appear.
    #[serde(rename="groupByResults")]
    
    pub group_by_results: Option<Vec<GroupResult>>,
    /// Token to retrieve the next page of results, or empty if there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Time used for executing the groupBy request.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The total number of results matching the query.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for GroupAssetsResponse {}


/// Request message for grouping by findings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sources findings group folders](FolderSourceFindingGroupCall) (request)
/// * [sources findings group organizations](OrganizationSourceFindingGroupCall) (request)
/// * [sources findings group projects](ProjectSourceFindingGroupCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupFindingsRequest {
    /// When compare_duration is set, the GroupResult's "state_change" attribute is updated to indicate whether the finding had its state changed, the finding's state remained unchanged, or if the finding was added during the compare_duration period of time that precedes the read_time. This is the time between (read_time - compare_duration) and read_time. The state_change value is derived based on the presence and state of the finding at the two points in time. Intermediate state changes between the two times don't affect the result. For example, the results aren't affected if the finding is made inactive and then active again. Possible "state_change" values when compare_duration is specified: * "CHANGED": indicates that the finding was present and matched the given filter at the start of compare_duration, but changed its state at read_time. * "UNCHANGED": indicates that the finding was present and matched the given filter at the start of compare_duration and did not change state at read_time. * "ADDED": indicates that the finding did not match the given filter or was not present at the start of compare_duration, but was present at read_time. * "REMOVED": indicates that the finding was present and matched the filter at the start of compare_duration, but did not match the filter at read_time. If compare_duration is not specified, then the only possible state_change is "UNUSED", which will be the state_change set for all findings present at read_time. If this field is set then `state_change` must be a specified field in `group_by`.
    #[serde(rename="compareDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub compare_duration: Option<client::chrono::Duration>,
    /// Expression that defines the filter to apply across findings. The expression is a list of one or more restrictions combined via logical operators `AND` and `OR`. Parentheses are supported, and `OR` has higher precedence than `AND`. Restrictions have the form ` ` and may have a `-` character in front of them to indicate negation. Examples include: * name * source_properties.a_property * security_marks.marks.marka The supported operators are: * `=` for all value types. * `>`, `<`, `>=`, `<=` for integer values. * `:`, meaning substring matching, for strings. The supported value types are: * string literals in quotes. * integer literals without quotes. * boolean literals `true` and `false` without quotes. The following field and operator combinations are supported: * name: `=` * parent: `=`, `:` * resource_name: `=`, `:` * state: `=`, `:` * category: `=`, `:` * external_uri: `=`, `:` * event_time: `=`, `>`, `<`, `>=`, `<=` Usage: This should be milliseconds since epoch or an RFC3339 string. Examples: `event_time = "2019-06-10T16:07:18-07:00"` `event_time = 1560208038000` * severity: `=`, `:` * workflow_state: `=`, `:` * security_marks.marks: `=`, `:` * source_properties: `=`, `:`, `>`, `<`, `>=`, `<=` For example, `source_properties.size = 100` is a valid filter string. Use a partial match on the empty string to filter based on a property existing: `source_properties.my_property : ""` Use a negated partial match on the empty string to filter based on a property not existing: `-source_properties.my_property : ""` * resource: * resource.name: `=`, `:` * resource.parent_name: `=`, `:` * resource.parent_display_name: `=`, `:` * resource.project_name: `=`, `:` * resource.project_display_name: `=`, `:` * resource.type: `=`, `:`
    
    pub filter: Option<String>,
    /// Required. Expression that defines what assets fields to use for grouping (including `state_change`). The string value should follow SQL syntax: comma separated list of fields. For example: "parent,resource_name". The following fields are supported: * resource_name * category * state * parent * severity The following fields are supported when compare_duration is set: * state_change
    #[serde(rename="groupBy")]
    
    pub group_by: Option<String>,
    /// The maximum number of results to return in a single response. Default is 10, minimum is 1, maximum is 1000.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// The value returned by the last `GroupFindingsResponse`; indicates that this is a continuation of a prior `GroupFindings` call, and that the system should return the next page of data.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Time used as a reference point when filtering findings. The filter is limited to findings existing at the supplied time and their values are those at that specific time. Absence of this field will default to the API's version of NOW.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GroupFindingsRequest {}


/// Response message for group by findings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sources findings group folders](FolderSourceFindingGroupCall) (response)
/// * [sources findings group organizations](OrganizationSourceFindingGroupCall) (response)
/// * [sources findings group projects](ProjectSourceFindingGroupCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupFindingsResponse {
    /// Group results. There exists an element for each existing unique combination of property/values. The element contains a count for the number of times those specific property/values appear.
    #[serde(rename="groupByResults")]
    
    pub group_by_results: Option<Vec<GroupResult>>,
    /// Token to retrieve the next page of results, or empty if there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Time used for executing the groupBy request.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The total number of results matching the query.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for GroupFindingsResponse {}


/// Result containing the properties and count of a groupBy request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupResult {
    /// Total count of resources for the given properties.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Properties matching the groupBy fields in the request.
    
    pub properties: Option<HashMap<String, json::Value>>,
}

impl client::Part for GroupResult {}


/// Represents a particular IAM binding, which captures a member's role addition, removal, or state.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IamBinding {
    /// The action that was performed on a Binding.
    
    pub action: Option<String>,
    /// A single identity requesting access for a Cloud Platform resource, e.g. "foo@google.com".
    
    pub member: Option<String>,
    /// Role that is assigned to "members". For example, "roles/viewer", "roles/editor", or "roles/owner".
    
    pub role: Option<String>,
}

impl client::Part for IamBinding {}


/// Cloud IAM Policy information associated with the Google Cloud resource described by the Security Command Center asset. This information is managed and defined by the Google Cloud resource and cannot be modified by the user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IamPolicy {
    /// The JSON representation of the Policy associated with the asset. See https://cloud.google.com/iam/reference/rest/v1/Policy for format details.
    #[serde(rename="policyBlob")]
    
    pub policy_blob: Option<String>,
}

impl client::Part for IamPolicy {}


/// Represents what's commonly known as an _indicator of compromise_ (IoC) in computer forensics. This is an artifact observed on a network or in an operating system that, with high confidence, indicates a computer intrusion. For more information, see [Indicator of compromise](https://en.wikipedia.org/wiki/Indicator_of_compromise).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Indicator {
    /// List of domains associated to the Finding.
    
    pub domains: Option<Vec<String>>,
    /// The list of IP addresses that are associated with the finding.
    #[serde(rename="ipAddresses")]
    
    pub ip_addresses: Option<Vec<String>>,
    /// The list of matched signatures indicating that the given process is present in the environment.
    
    pub signatures: Option<Vec<ProcessSignature>>,
    /// The list of URIs associated to the Findings.
    
    pub uris: Option<Vec<String>>,
}

impl client::Part for Indicator {}


/// Kernel mode rootkit signatures.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KernelRootkit {
    /// Rootkit name when available.
    
    pub name: Option<String>,
    /// True when unexpected modifications of kernel code memory are present.
    #[serde(rename="unexpectedCodeModification")]
    
    pub unexpected_code_modification: Option<bool>,
    /// True when `ftrace` points are present with callbacks pointing to regions that are not in the expected kernel or module code range.
    #[serde(rename="unexpectedFtraceHandler")]
    
    pub unexpected_ftrace_handler: Option<bool>,
    /// True when interrupt handlers that are are not in the expected kernel or module code regions are present.
    #[serde(rename="unexpectedInterruptHandler")]
    
    pub unexpected_interrupt_handler: Option<bool>,
    /// True when kernel code pages that are not in the expected kernel or module code regions are present.
    #[serde(rename="unexpectedKernelCodePages")]
    
    pub unexpected_kernel_code_pages: Option<bool>,
    /// True when `kprobe` points are present with callbacks pointing to regions that are not in the expected kernel or module code range.
    #[serde(rename="unexpectedKprobeHandler")]
    
    pub unexpected_kprobe_handler: Option<bool>,
    /// True when unexpected processes in the scheduler run queue are present. Such processes are in the run queue, but not in the process task list.
    #[serde(rename="unexpectedProcessesInRunqueue")]
    
    pub unexpected_processes_in_runqueue: Option<bool>,
    /// True when unexpected modifications of kernel read-only data memory are present.
    #[serde(rename="unexpectedReadOnlyDataModification")]
    
    pub unexpected_read_only_data_modification: Option<bool>,
    /// True when system call handlers that are are not in the expected kernel or module code regions are present.
    #[serde(rename="unexpectedSystemCallHandler")]
    
    pub unexpected_system_call_handler: Option<bool>,
}

impl client::Part for KernelRootkit {}


/// Kubernetes-related attributes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Kubernetes {
    /// Provides information on any Kubernetes access reviews (i.e. privilege checks) relevant to the finding.
    #[serde(rename="accessReviews")]
    
    pub access_reviews: Option<Vec<AccessReview>>,
    /// Provides Kubernetes role binding information for findings that involve RoleBindings or ClusterRoleBindings.
    
    pub bindings: Option<Vec<GoogleCloudSecuritycenterV1Binding>>,
    /// GKE Node Pools associated with the finding. This field will contain NodePool information for each Node, when it is available.
    #[serde(rename="nodePools")]
    
    pub node_pools: Option<Vec<NodePool>>,
    /// Provides Kubernetes Node information.
    
    pub nodes: Option<Vec<Node>>,
    /// Kubernetes Pods associated with the finding. This field will contain Pod records for each container that is owned by a Pod.
    
    pub pods: Option<Vec<Pod>>,
    /// Provides Kubernetes role information for findings that involve Roles or ClusterRoles.
    
    pub roles: Option<Vec<Role>>,
}

impl client::Part for Kubernetes {}


/// Label represents a generic name=value label. Label has separate name and value fields to support filtering with contains().
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Label {
    /// Label name.
    
    pub name: Option<String>,
    /// Label value.
    
    pub value: Option<String>,
}

impl client::Part for Label {}


/// Response message for listing assets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assets list folders](FolderAssetListCall) (response)
/// * [assets list organizations](OrganizationAssetListCall) (response)
/// * [assets list projects](ProjectAssetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAssetsResponse {
    /// Assets matching the list request.
    #[serde(rename="listAssetsResults")]
    
    pub list_assets_results: Option<Vec<ListAssetsResult>>,
    /// Token to retrieve the next page of results, or empty if there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Time used for executing the list request.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The total number of assets matching the query.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListAssetsResponse {}


/// Result containing the Asset and its State.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAssetsResult {
    /// Asset matching the search request.
    
    pub asset: Option<Asset>,
    /// State change of the asset between the points in time.
    #[serde(rename="stateChange")]
    
    pub state_change: Option<String>,
}

impl client::Part for ListAssetsResult {}


/// Response message for listing BigQuery exports.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [big query exports list folders](FolderBigQueryExportListCall) (response)
/// * [big query exports list organizations](OrganizationBigQueryExportListCall) (response)
/// * [big query exports list projects](ProjectBigQueryExportListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBigQueryExportsResponse {
    /// The BigQuery exports from the specified parent.
    #[serde(rename="bigQueryExports")]
    
    pub big_query_exports: Option<Vec<GoogleCloudSecuritycenterV1BigQueryExport>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListBigQueryExportsResponse {}


/// Response message for listing findings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sources findings list folders](FolderSourceFindingListCall) (response)
/// * [sources findings list organizations](OrganizationSourceFindingListCall) (response)
/// * [sources findings list projects](ProjectSourceFindingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFindingsResponse {
    /// Findings matching the list request.
    #[serde(rename="listFindingsResults")]
    
    pub list_findings_results: Option<Vec<ListFindingsResult>>,
    /// Token to retrieve the next page of results, or empty if there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Time used for executing the list request.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The total number of findings matching the query.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListFindingsResponse {}


/// Result containing the Finding and its StateChange.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFindingsResult {
    /// Finding matching the search request.
    
    pub finding: Option<Finding>,
    /// Output only. Resource that is associated with this finding.
    
    pub resource: Option<Resource>,
    /// State change of the finding between the points in time.
    #[serde(rename="stateChange")]
    
    pub state_change: Option<String>,
}

impl client::Part for ListFindingsResult {}


/// Response message for listing mute configs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [mute configs list folders](FolderMuteConfigListCall) (response)
/// * [mute configs list organizations](OrganizationMuteConfigListCall) (response)
/// * [mute configs list projects](ProjectMuteConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMuteConfigsResponse {
    /// The mute configs from the specified parent.
    #[serde(rename="muteConfigs")]
    
    pub mute_configs: Option<Vec<GoogleCloudSecuritycenterV1MuteConfig>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListMuteConfigsResponse {}


/// Response message for listing notification configs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notification configs list folders](FolderNotificationConfigListCall) (response)
/// * [notification configs list organizations](OrganizationNotificationConfigListCall) (response)
/// * [notification configs list projects](ProjectNotificationConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNotificationConfigsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Notification configs belonging to the requested parent.
    #[serde(rename="notificationConfigs")]
    
    pub notification_configs: Option<Vec<NotificationConfig>>,
}

impl client::ResponseResult for ListNotificationConfigsResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations list organizations](OrganizationOperationListCall) (response)
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


/// Response message for listing sources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sources list folders](FolderSourceListCall) (response)
/// * [sources list organizations](OrganizationSourceListCall) (response)
/// * [sources list projects](ProjectSourceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSourcesResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Sources belonging to the requested parent.
    
    pub sources: Option<Vec<Source>>,
}

impl client::ResponseResult for ListSourcesResponse {}


/// A signature corresponding to memory page hashes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MemoryHashSignature {
    /// The binary family.
    #[serde(rename="binaryFamily")]
    
    pub binary_family: Option<String>,
    /// The list of memory hash detections contributing to the binary family match.
    
    pub detections: Option<Vec<Detection>>,
}

impl client::Part for MemoryHashSignature {}


/// MITRE ATT&CK tactics and techniques related to this finding. See: https://attack.mitre.org
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MitreAttack {
    /// Additional MITRE ATT&CK tactics related to this finding, if any.
    #[serde(rename="additionalTactics")]
    
    pub additional_tactics: Option<Vec<String>>,
    /// Additional MITRE ATT&CK techniques related to this finding, if any, along with any of their respective parent techniques.
    #[serde(rename="additionalTechniques")]
    
    pub additional_techniques: Option<Vec<String>>,
    /// The MITRE ATT&CK tactic most closely represented by this finding, if any.
    #[serde(rename="primaryTactic")]
    
    pub primary_tactic: Option<String>,
    /// The MITRE ATT&CK technique most closely represented by this finding, if any. primary_techniques is a repeated field because there are multiple levels of MITRE ATT&CK techniques. If the technique most closely represented by this finding is a sub-technique (e.g. `SCANNING_IP_BLOCKS`), both the sub-technique and its parent technique(s) will be listed (e.g. `SCANNING_IP_BLOCKS`, `ACTIVE_SCANNING`).
    #[serde(rename="primaryTechniques")]
    
    pub primary_techniques: Option<Vec<String>>,
    /// The MITRE ATT&CK version referenced by the above fields. E.g. "8".
    
    pub version: Option<String>,
}

impl client::Part for MitreAttack {}


/// Kubernetes Nodes associated with the finding.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    /// Full Resource name of the Compute Engine VM running the cluster node.
    
    pub name: Option<String>,
}

impl client::Part for Node {}


/// Provides GKE Node Pool information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodePool {
    /// Kubernetes Node pool name.
    
    pub name: Option<String>,
    /// Nodes associated with the finding.
    
    pub nodes: Option<Vec<Node>>,
}

impl client::Part for NodePool {}


/// Cloud Security Command Center (Cloud SCC) notification configs. A notification config is a Cloud SCC resource that contains the configuration to send notifications for create/update events of findings, assets and etc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notification configs create folders](FolderNotificationConfigCreateCall) (request|response)
/// * [notification configs get folders](FolderNotificationConfigGetCall) (response)
/// * [notification configs patch folders](FolderNotificationConfigPatchCall) (request|response)
/// * [notification configs create organizations](OrganizationNotificationConfigCreateCall) (request|response)
/// * [notification configs get organizations](OrganizationNotificationConfigGetCall) (response)
/// * [notification configs patch organizations](OrganizationNotificationConfigPatchCall) (request|response)
/// * [notification configs create projects](ProjectNotificationConfigCreateCall) (request|response)
/// * [notification configs get projects](ProjectNotificationConfigGetCall) (response)
/// * [notification configs patch projects](ProjectNotificationConfigPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// The description of the notification config (max of 1024 characters).
    
    pub description: Option<String>,
    /// The relative resource name of this notification config. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/notificationConfigs/notify_public_bucket", "folders/{folder_id}/notificationConfigs/notify_public_bucket", or "projects/{project_id}/notificationConfigs/notify_public_bucket".
    
    pub name: Option<String>,
    /// The Pub/Sub topic to send notifications to. Its format is "projects/[project_id]/topics/[topic]".
    #[serde(rename="pubsubTopic")]
    
    pub pubsub_topic: Option<String>,
    /// Output only. The service account that needs "pubsub.topics.publish" permission to publish to the Pub/Sub topic.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// The config for triggering streaming-based notifications.
    #[serde(rename="streamingConfig")]
    
    pub streaming_config: Option<StreamingConfig>,
}

impl client::RequestValue for NotificationConfig {}
impl client::ResponseResult for NotificationConfig {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [findings bulk mute folders](FolderFindingBulkMuteCall) (response)
/// * [assets run discovery organizations](OrganizationAssetRunDiscoveryCall) (response)
/// * [findings bulk mute organizations](OrganizationFindingBulkMuteCall) (response)
/// * [operations get organizations](OrganizationOperationGetCall) (response)
/// * [findings bulk mute projects](ProjectFindingBulkMuteCall) (response)
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


/// User specified settings that are attached to the Security Command Center organization.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get organization settings organizations](OrganizationGetOrganizationSettingCall) (response)
/// * [update organization settings organizations](OrganizationUpdateOrganizationSettingCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationSettings {
    /// The configuration used for Asset Discovery runs.
    #[serde(rename="assetDiscoveryConfig")]
    
    pub asset_discovery_config: Option<AssetDiscoveryConfig>,
    /// A flag that indicates if Asset Discovery should be enabled. If the flag is set to `true`, then discovery of assets will occur. If it is set to `false, all historical assets will remain, but discovery of future assets will not occur.
    #[serde(rename="enableAssetDiscovery")]
    
    pub enable_asset_discovery: Option<bool>,
    /// The relative resource name of the settings. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/organizationSettings".
    
    pub name: Option<String>,
}

impl client::RequestValue for OrganizationSettings {}
impl client::ResponseResult for OrganizationSettings {}


/// Kubernetes Pod.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Pod {
    /// Pod containers associated with this finding, if any.
    
    pub containers: Option<Vec<Container>>,
    /// Pod labels. For Kubernetes containers, these are applied to the container.
    
    pub labels: Option<Vec<Label>>,
    /// Kubernetes Pod name.
    
    pub name: Option<String>,
    /// Kubernetes Pod namespace.
    
    pub ns: Option<String>,
}

impl client::Part for Pod {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sources get iam policy organizations](OrganizationSourceGetIamPolicyCall) (response)
/// * [sources set iam policy organizations](OrganizationSourceSetIamPolicyCall) (response)
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


/// Represents an operating system process.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Process {
    /// Process arguments as JSON encoded strings.
    
    pub args: Option<Vec<String>>,
    /// True if `args` is incomplete.
    #[serde(rename="argumentsTruncated")]
    
    pub arguments_truncated: Option<bool>,
    /// File information for the process executable.
    
    pub binary: Option<File>,
    /// Process environment variables.
    #[serde(rename="envVariables")]
    
    pub env_variables: Option<Vec<EnvironmentVariable>>,
    /// True if `env_variables` is incomplete.
    #[serde(rename="envVariablesTruncated")]
    
    pub env_variables_truncated: Option<bool>,
    /// File information for libraries loaded by the process.
    
    pub libraries: Option<Vec<File>>,
    /// The process name visible in utilities like `top` and `ps`; it can be accessed via `/proc/[pid]/comm` and changed with `prctl(PR_SET_NAME)`.
    
    pub name: Option<String>,
    /// The parent process id.
    #[serde(rename="parentPid")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub parent_pid: Option<i64>,
    /// The process id.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pid: Option<i64>,
    /// When the process represents the invocation of a script, `binary` provides information about the interpreter while `script` provides information about the script file provided to the interpreter.
    
    pub script: Option<File>,
}

impl client::Part for Process {}


/// Indicates what signature matched this process.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProcessSignature {
    /// Signature indicating that a binary family was matched.
    #[serde(rename="memoryHashSignature")]
    
    pub memory_hash_signature: Option<MemoryHashSignature>,
    /// Signature indicating that a YARA rule was matched.
    #[serde(rename="yaraRuleSignature")]
    
    pub yara_rule_signature: Option<YaraRuleSignature>,
}

impl client::Part for ProcessSignature {}


/// Additional Links
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Reference {
    /// Source of the reference e.g. NVD
    
    pub source: Option<String>,
    /// Uri for the mentioned source e.g. https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-34527.
    
    pub uri: Option<String>,
}

impl client::Part for Reference {}


/// Information related to the Google Cloud resource that is associated with this finding.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Resource {
    /// The human readable name of the resource.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Contains a Folder message for each folder in the assets ancestry. The first folder is the deepest nested folder, and the last folder is the folder directly under the Organization.
    
    pub folders: Option<Vec<Folder>>,
    /// The full resource name of the resource. See: https://cloud.google.com/apis/design/resource_names#full_resource_name
    
    pub name: Option<String>,
    /// The human readable name of resource's parent.
    #[serde(rename="parentDisplayName")]
    
    pub parent_display_name: Option<String>,
    /// The full resource name of resource's parent.
    #[serde(rename="parentName")]
    
    pub parent_name: Option<String>,
    /// The project ID that the resource belongs to.
    #[serde(rename="projectDisplayName")]
    
    pub project_display_name: Option<String>,
    /// The full resource name of project that the resource belongs to.
    #[serde(rename="projectName")]
    
    pub project_name: Option<String>,
    /// The full resource type of the resource.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Resource {}


/// Kubernetes Role or ClusterRole.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    /// Role type.
    
    pub kind: Option<String>,
    /// Role name.
    
    pub name: Option<String>,
    /// Role namespace.
    
    pub ns: Option<String>,
}

impl client::Part for Role {}


/// Request message for running asset discovery for an organization.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assets run discovery organizations](OrganizationAssetRunDiscoveryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunAssetDiscoveryRequest { _never_set: Option<bool> }

impl client::RequestValue for RunAssetDiscoveryRequest {}


/// Security Command Center managed properties. These properties are managed by Security Command Center and cannot be modified by the user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecurityCenterProperties {
    /// Contains a Folder message for each folder in the assets ancestry. The first folder is the deepest nested folder, and the last folder is the folder directly under the Organization.
    
    pub folders: Option<Vec<Folder>>,
    /// The user defined display name for this resource.
    #[serde(rename="resourceDisplayName")]
    
    pub resource_display_name: Option<String>,
    /// The full resource name of the Google Cloud resource this asset represents. This field is immutable after create time. See: https://cloud.google.com/apis/design/resource_names#full_resource_name
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
    /// Owners of the Google Cloud resource.
    #[serde(rename="resourceOwners")]
    
    pub resource_owners: Option<Vec<String>>,
    /// The full resource name of the immediate parent of the resource. See: https://cloud.google.com/apis/design/resource_names#full_resource_name
    #[serde(rename="resourceParent")]
    
    pub resource_parent: Option<String>,
    /// The user defined display name for the parent of this resource.
    #[serde(rename="resourceParentDisplayName")]
    
    pub resource_parent_display_name: Option<String>,
    /// The full resource name of the project the resource belongs to. See: https://cloud.google.com/apis/design/resource_names#full_resource_name
    #[serde(rename="resourceProject")]
    
    pub resource_project: Option<String>,
    /// The user defined display name for the project of this resource.
    #[serde(rename="resourceProjectDisplayName")]
    
    pub resource_project_display_name: Option<String>,
    /// The type of the Google Cloud resource. Examples include: APPLICATION, PROJECT, and ORGANIZATION. This is a case insensitive field defined by Security Command Center and/or the producer of the resource and is immutable after create time.
    #[serde(rename="resourceType")]
    
    pub resource_type: Option<String>,
}

impl client::Part for SecurityCenterProperties {}


/// User specified security marks that are attached to the parent Security Command Center resource. Security marks are scoped within a Security Command Center organization – they can be modified and viewed by all users who have proper permissions on the organization.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assets update security marks folders](FolderAssetUpdateSecurityMarkCall) (request|response)
/// * [sources findings update security marks folders](FolderSourceFindingUpdateSecurityMarkCall) (request|response)
/// * [assets update security marks organizations](OrganizationAssetUpdateSecurityMarkCall) (request|response)
/// * [sources findings update security marks organizations](OrganizationSourceFindingUpdateSecurityMarkCall) (request|response)
/// * [assets update security marks projects](ProjectAssetUpdateSecurityMarkCall) (request|response)
/// * [sources findings update security marks projects](ProjectSourceFindingUpdateSecurityMarkCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecurityMarks {
    /// The canonical name of the marks. Examples: "organizations/{organization_id}/assets/{asset_id}/securityMarks" "folders/{folder_id}/assets/{asset_id}/securityMarks" "projects/{project_number}/assets/{asset_id}/securityMarks" "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks" "folders/{folder_id}/sources/{source_id}/findings/{finding_id}/securityMarks" "projects/{project_number}/sources/{source_id}/findings/{finding_id}/securityMarks"
    #[serde(rename="canonicalName")]
    
    pub canonical_name: Option<String>,
    /// Mutable user specified security marks belonging to the parent resource. Constraints are as follows: * Keys and values are treated as case insensitive * Keys must be between 1 - 256 characters (inclusive) * Keys must be letters, numbers, underscores, or dashes * Values have leading and trailing whitespace trimmed, remaining characters must be between 1 - 4096 characters (inclusive)
    
    pub marks: Option<HashMap<String, String>>,
    /// The relative resource name of the SecurityMarks. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Examples: "organizations/{organization_id}/assets/{asset_id}/securityMarks" "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    
    pub name: Option<String>,
}

impl client::RequestValue for SecurityMarks {}
impl client::ResponseResult for SecurityMarks {}


/// Identity delegation history of an authenticated service account.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccountDelegationInfo {
    /// The email address of a Google account.
    #[serde(rename="principalEmail")]
    
    pub principal_email: Option<String>,
    /// A string representing the principal_subject associated with the identity. As compared to `principal_email`, supports principals that aren't associated with email addresses, such as third party principals. For most identities, the format will be `principal://iam.googleapis.com/{identity pool name}/subjects/{subject}` except for some GKE identities (GKE_WORKLOAD, FREEFORM, GKE_HUB_WORKLOAD) that are still in the legacy format `serviceAccount:{identity pool name}[{subject}]`
    #[serde(rename="principalSubject")]
    
    pub principal_subject: Option<String>,
}

impl client::Part for ServiceAccountDelegationInfo {}


/// Request message for updating a finding’s state.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sources findings set state folders](FolderSourceFindingSetStateCall) (request)
/// * [sources findings set state organizations](OrganizationSourceFindingSetStateCall) (request)
/// * [sources findings set state projects](ProjectSourceFindingSetStateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetFindingStateRequest {
    /// Required. The time at which the updated state takes effect.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The desired State of the finding.
    
    pub state: Option<String>,
}

impl client::RequestValue for SetFindingStateRequest {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sources set iam policy organizations](OrganizationSourceSetIamPolicyCall) (request)
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


/// Request message for updating a finding’s mute status.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sources findings set mute folders](FolderSourceFindingSetMuteCall) (request)
/// * [sources findings set mute organizations](OrganizationSourceFindingSetMuteCall) (request)
/// * [sources findings set mute projects](ProjectSourceFindingSetMuteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetMuteRequest {
    /// Required. The desired state of the Mute.
    
    pub mute: Option<String>,
}

impl client::RequestValue for SetMuteRequest {}


/// Security Command Center finding source. A finding source is an entity or a mechanism that can produce a finding. A source is like a container of findings that come from the same scanner, logger, monitor, and other tools.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sources create organizations](OrganizationSourceCreateCall) (request|response)
/// * [sources get organizations](OrganizationSourceGetCall) (response)
/// * [sources patch organizations](OrganizationSourcePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    /// The canonical name of the finding. It's either "organizations/{organization_id}/sources/{source_id}", "folders/{folder_id}/sources/{source_id}" or "projects/{project_number}/sources/{source_id}", depending on the closest CRM ancestor of the resource associated with the finding.
    #[serde(rename="canonicalName")]
    
    pub canonical_name: Option<String>,
    /// The description of the source (max of 1024 characters). Example: "Web Security Scanner is a web security scanner for common vulnerabilities in App Engine applications. It can automatically scan and detect four common vulnerabilities, including cross-site-scripting (XSS), Flash injection, mixed content (HTTP in HTTPS), and outdated or insecure libraries."
    
    pub description: Option<String>,
    /// The source's display name. A source's display name must be unique amongst its siblings, for example, two sources with the same parent can't share the same display name. The display name must have a length between 1 and 64 characters (inclusive).
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The relative resource name of this source. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/sources/{source_id}"
    
    pub name: Option<String>,
}

impl client::RequestValue for Source {}
impl client::ResponseResult for Source {}


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


/// The config for streaming-based notifications, which send each event as soon as it is detected.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StreamingConfig {
    /// Expression that defines the filter to apply across create/update events of assets or findings as specified by the event type. The expression is a list of zero or more restrictions combined via logical operators `AND` and `OR`. Parentheses are supported, and `OR` has higher precedence than `AND`. Restrictions have the form ` ` and may have a `-` character in front of them to indicate negation. The fields map to those defined in the corresponding resource. The supported operators are: * `=` for all value types. * `>`, `<`, `>=`, `<=` for integer values. * `:`, meaning substring matching, for strings. The supported value types are: * string literals in quotes. * integer literals without quotes. * boolean literals `true` and `false` without quotes.
    
    pub filter: Option<String>,
}

impl client::Part for StreamingConfig {}


/// Represents a Kubernetes Subject.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Subject {
    /// Authentication type for subject.
    
    pub kind: Option<String>,
    /// Name for subject.
    
    pub name: Option<String>,
    /// Namespace for subject.
    
    pub ns: Option<String>,
}

impl client::Part for Subject {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sources test iam permissions organizations](OrganizationSourceTestIamPermissionCall) (request)
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
/// * [sources test iam permissions organizations](OrganizationSourceTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// Refers to common vulnerability fields e.g. cve, cvss, cwe etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Vulnerability {
    /// CVE stands for Common Vulnerabilities and Exposures (https://cve.mitre.org/about/)
    
    pub cve: Option<Cve>,
}

impl client::Part for Vulnerability {}


/// A signature corresponding to a YARA rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct YaraRuleSignature {
    /// The name of the YARA rule.
    #[serde(rename="yaraRule")]
    
    pub yara_rule: Option<String>,
}

impl client::Part for YaraRuleSignature {}


