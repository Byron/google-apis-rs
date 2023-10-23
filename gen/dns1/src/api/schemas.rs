use super::*;
/// A Change represents a set of ResourceRecordSet additions and deletions applied atomically to a ManagedZone. ResourceRecordSets within a ManagedZone are modified by creating a new Change element in the Changes collection. In turn the Changes collection also records the past modifications to the ResourceRecordSets in a ManagedZone. The current state of the ManagedZone is the sum effect of applying all Change elements in the Changes collection in sequence.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create changes](ChangeCreateCall) (request|response)
/// * [get changes](ChangeGetCall) (response)
/// * [list changes](ChangeListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Change {
    /// Which ResourceRecordSets to add?
    
    pub additions: Option<Vec<ResourceRecordSet>>,
    /// Which ResourceRecordSets to remove? Must match existing data exactly.
    
    pub deletions: Option<Vec<ResourceRecordSet>>,
    /// Unique identifier for the resource; defined by the server (output only).
    
    pub id: Option<String>,
    /// If the DNS queries for the zone will be served.
    #[serde(rename="isServing")]
    
    pub is_serving: Option<bool>,
    /// no description provided
    
    pub kind: Option<String>,
    /// The time that this operation was started by the server (output only). This is in RFC3339 text format.
    #[serde(rename="startTime")]
    
    pub start_time: Option<String>,
    /// Status of the operation (output only). A status of "done" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet.
    
    pub status: Option<ChangeStatusEnum>,
}

impl client::RequestValue for Change {}
impl client::Resource for Change {}
impl client::ResponseResult for Change {}


/// The response to a request to enumerate Changes to a ResourceRecordSets collection.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list changes](ChangeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChangesListResponse {
    /// The requested changes.
    
    pub changes: Option<Vec<Change>>,
    /// no description provided
    
    pub header: Option<ResponseHeader>,
    /// Type of resource.
    
    pub kind: Option<String>,
    /// The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your pagination token. This lets you retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. You cannot retrieve a "snapshot" of collections larger than the maximum page size.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ChangesListResponse {}


/// A DNSSEC key pair.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get dns keys](DnsKeyGetCall) (response)
/// * [list dns keys](DnsKeyListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DnsKey {
    /// String mnemonic specifying the DNSSEC algorithm of this key. Immutable after creation time.
    
    pub algorithm: Option<DnsKeyAlgorithmEnum>,
    /// The time that this resource was created in the control plane. This is in RFC3339 text format. Output only.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<String>,
    /// A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the resource's function.
    
    pub description: Option<String>,
    /// Cryptographic hashes of the DNSKEY resource record associated with this DnsKey. These digests are needed to construct a DS record that points at this DNS key. Output only.
    
    pub digests: Option<Vec<DnsKeyDigest>>,
    /// Unique identifier for the resource; defined by the server (output only).
    
    pub id: Option<String>,
    /// Active keys are used to sign subsequent changes to the ManagedZone. Inactive keys are still present as DNSKEY Resource Records for the use of resolvers validating existing signatures.
    #[serde(rename="isActive")]
    
    pub is_active: Option<bool>,
    /// Length of the key in bits. Specified at creation time, and then immutable.
    #[serde(rename="keyLength")]
    
    pub key_length: Option<u32>,
    /// The key tag is a non-cryptographic hash of the a DNSKEY resource record associated with this DnsKey. The key tag can be used to identify a DNSKEY more quickly (but it is not a unique identifier). In particular, the key tag is used in a parent zone's DS record to point at the DNSKEY in this child ManagedZone. The key tag is a number in the range [0, 65535] and the algorithm to calculate it is specified in RFC4034 Appendix B. Output only.
    #[serde(rename="keyTag")]
    
    pub key_tag: Option<i32>,
    /// no description provided
    
    pub kind: Option<String>,
    /// Base64 encoded public half of this key. Output only.
    #[serde(rename="publicKey")]
    
    pub public_key: Option<String>,
    /// One of "KEY_SIGNING" or "ZONE_SIGNING". Keys of type KEY_SIGNING have the Secure Entry Point flag set and, when active, are used to sign only resource record sets of type DNSKEY. Otherwise, the Secure Entry Point flag is cleared, and this key is used to sign only resource record sets of other types. Immutable after creation time.
    #[serde(rename="type")]
    
    pub type_: Option<DnsKeyTypeEnum>,
}

impl client::Resource for DnsKey {}
impl client::ResponseResult for DnsKey {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DnsKeyDigest {
    /// The base-16 encoded bytes of this digest. Suitable for use in a DS resource record.
    
    pub digest: Option<String>,
    /// Specifies the algorithm used to calculate this digest.
    #[serde(rename="type")]
    
    pub type_: Option<DnsKeyDigestTypeEnum>,
}

impl client::Part for DnsKeyDigest {}


/// Parameters for DnsKey key generation. Used for generating initial keys for a new ManagedZone and as default when adding a new DnsKey.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DnsKeySpec {
    /// String mnemonic specifying the DNSSEC algorithm of this key.
    
    pub algorithm: Option<DnsKeySpecAlgorithmEnum>,
    /// Length of the keys in bits.
    #[serde(rename="keyLength")]
    
    pub key_length: Option<u32>,
    /// Specifies whether this is a key signing key (KSK) or a zone signing key (ZSK). Key signing keys have the Secure Entry Point flag set and, when active, are only used to sign resource record sets of type DNSKEY. Zone signing keys do not have the Secure Entry Point flag set and are used to sign all other types of resource record sets.
    #[serde(rename="keyType")]
    
    pub key_type: Option<DnsKeySpecKeyTypeEnum>,
    /// no description provided
    
    pub kind: Option<String>,
}

impl client::Part for DnsKeySpec {}


/// The response to a request to enumerate DnsKeys in a ManagedZone.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list dns keys](DnsKeyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DnsKeysListResponse {
    /// The requested resources.
    #[serde(rename="dnsKeys")]
    
    pub dns_keys: Option<Vec<DnsKey>>,
    /// no description provided
    
    pub header: Option<ResponseHeader>,
    /// Type of resource.
    
    pub kind: Option<String>,
    /// The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your pagination token. In this way you can retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. There is no way to retrieve a "snapshot" of collections larger than the maximum page size.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for DnsKeysListResponse {}


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


/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts `jose@example.com` from DATA_READ logging, and `aliya@example.com` from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<GoogleIamV1AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
    
    pub service: Option<String>,
}

impl client::Part for GoogleIamV1AuditConfig {}


/// Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<GoogleIamV1AuditLogConfigLogTypeEnum>,
}

impl client::Part for GoogleIamV1AuditLogConfig {}


/// Associates `members`, or principals, with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1Binding {
    /// The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. 
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    
    pub role: Option<String>,
}

impl client::Part for GoogleIamV1Binding {}


/// Request message for `GetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get iam policy managed zones](ManagedZoneGetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1GetIamPolicyRequest {
    /// OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`.
    
    pub options: Option<GoogleIamV1GetPolicyOptions>,
}

impl client::RequestValue for GoogleIamV1GetIamPolicyRequest {}


/// Encapsulates settings provided to GetIamPolicy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1GetPolicyOptions {
    /// Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    #[serde(rename="requestedPolicyVersion")]
    
    pub requested_policy_version: Option<i32>,
}

impl client::Part for GoogleIamV1GetPolicyOptions {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get iam policy managed zones](ManagedZoneGetIamPolicyCall) (response)
/// * [set iam policy managed zones](ManagedZoneSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<GoogleIamV1AuditConfig>>,
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<GoogleIamV1Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for GoogleIamV1Policy {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set iam policy managed zones](ManagedZoneSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<GoogleIamV1Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"`
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GoogleIamV1SetIamPolicyRequest {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions managed zones](ManagedZoneTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for GoogleIamV1TestIamPermissionsRequest {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions managed zones](ManagedZoneTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleIamV1TestIamPermissionsResponse {}


/// A zone is a subtree of the DNS namespace under one administrative responsibility. A ManagedZone is a resource that represents a DNS zone hosted by the Cloud DNS service.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create managed zones](ManagedZoneCreateCall) (request|response)
/// * [delete managed zones](ManagedZoneDeleteCall) (none)
/// * [get managed zones](ManagedZoneGetCall) (response)
/// * [get iam policy managed zones](ManagedZoneGetIamPolicyCall) (none)
/// * [list managed zones](ManagedZoneListCall) (none)
/// * [patch managed zones](ManagedZonePatchCall) (request)
/// * [set iam policy managed zones](ManagedZoneSetIamPolicyCall) (none)
/// * [test iam permissions managed zones](ManagedZoneTestIamPermissionCall) (none)
/// * [update managed zones](ManagedZoneUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedZone {
    /// no description provided
    #[serde(rename="cloudLoggingConfig")]
    
    pub cloud_logging_config: Option<ManagedZoneCloudLoggingConfig>,
    /// The time that this resource was created on the server. This is in RFC3339 text format. Output only.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<String>,
    /// A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the managed zone's function.
    
    pub description: Option<String>,
    /// The DNS name of this managed zone, for instance "example.com.".
    #[serde(rename="dnsName")]
    
    pub dns_name: Option<String>,
    /// DNSSEC configuration.
    #[serde(rename="dnssecConfig")]
    
    pub dnssec_config: Option<ManagedZoneDnsSecConfig>,
    /// The presence for this field indicates that outbound forwarding is enabled for this zone. The value of this field contains the set of destinations to forward to.
    #[serde(rename="forwardingConfig")]
    
    pub forwarding_config: Option<ManagedZoneForwardingConfig>,
    /// Unique identifier for the resource; defined by the server (output only)
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// no description provided
    
    pub kind: Option<String>,
    /// User labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// User assigned name for this resource. Must be unique within the project. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes.
    
    pub name: Option<String>,
    /// Optionally specifies the NameServerSet for this ManagedZone. A NameServerSet is a set of DNS name servers that all host the same ManagedZones. Most users leave this field unset. If you need to use this field, contact your account team.
    #[serde(rename="nameServerSet")]
    
    pub name_server_set: Option<String>,
    /// Delegate your managed_zone to these virtual name servers; defined by the server (output only)
    #[serde(rename="nameServers")]
    
    pub name_servers: Option<Vec<String>>,
    /// The presence of this field indicates that DNS Peering is enabled for this zone. The value of this field contains the network to peer with.
    #[serde(rename="peeringConfig")]
    
    pub peering_config: Option<ManagedZonePeeringConfig>,
    /// For privately visible zones, the set of Virtual Private Cloud resources that the zone is visible from.
    #[serde(rename="privateVisibilityConfig")]
    
    pub private_visibility_config: Option<ManagedZonePrivateVisibilityConfig>,
    /// The presence of this field indicates that this is a managed reverse lookup zone and Cloud DNS resolves reverse lookup queries using automatically configured records for VPC resources. This only applies to networks listed under private_visibility_config.
    #[serde(rename="reverseLookupConfig")]
    
    pub reverse_lookup_config: Option<ManagedZoneReverseLookupConfig>,
    /// This field links to the associated service directory namespace. Do not set this field for public zones or forwarding zones.
    #[serde(rename="serviceDirectoryConfig")]
    
    pub service_directory_config: Option<ManagedZoneServiceDirectoryConfig>,
    /// The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources.
    
    pub visibility: Option<ManagedZoneVisibilityEnum>,
}

impl client::RequestValue for ManagedZone {}
impl client::Resource for ManagedZone {}
impl client::ResponseResult for ManagedZone {}


/// Cloud Logging configurations for publicly visible zones.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedZoneCloudLoggingConfig {
    /// If set, enable query logging for this ManagedZone. False by default, making logging opt-in.
    #[serde(rename="enableLogging")]
    
    pub enable_logging: Option<bool>,
    /// no description provided
    
    pub kind: Option<String>,
}

impl client::Part for ManagedZoneCloudLoggingConfig {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedZoneDnsSecConfig {
    /// Specifies parameters for generating initial DnsKeys for this ManagedZone. Can only be changed while the state is OFF.
    #[serde(rename="defaultKeySpecs")]
    
    pub default_key_specs: Option<Vec<DnsKeySpec>>,
    /// no description provided
    
    pub kind: Option<String>,
    /// Specifies the mechanism for authenticated denial-of-existence responses. Can only be changed while the state is OFF.
    #[serde(rename="nonExistence")]
    
    pub non_existence: Option<ManagedZoneDnsSecConfigNonExistenceEnum>,
    /// Specifies whether DNSSEC is enabled, and what mode it is in.
    
    pub state: Option<ManagedZoneDnsSecConfigStateEnum>,
}

impl client::Part for ManagedZoneDnsSecConfig {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedZoneForwardingConfig {
    /// no description provided
    
    pub kind: Option<String>,
    /// List of target name servers to forward to. Cloud DNS selects the best available name server if more than one target is given.
    #[serde(rename="targetNameServers")]
    
    pub target_name_servers: Option<Vec<ManagedZoneForwardingConfigNameServerTarget>>,
}

impl client::Part for ManagedZoneForwardingConfig {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedZoneForwardingConfigNameServerTarget {
    /// Forwarding path for this NameServerTarget. If unset or set to DEFAULT, Cloud DNS makes forwarding decisions based on IP address ranges; that is, RFC1918 addresses go to the VPC network, non-RFC1918 addresses go to the internet. When set to PRIVATE, Cloud DNS always sends queries through the VPC network for this target.
    #[serde(rename="forwardingPath")]
    
    pub forwarding_path: Option<ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum>,
    /// IPv4 address of a target name server.
    #[serde(rename="ipv4Address")]
    
    pub ipv4_address: Option<String>,
    /// IPv6 address of a target name server. Does not accept both fields (ipv4 & ipv6) being populated. Public preview as of November 2022.
    #[serde(rename="ipv6Address")]
    
    pub ipv6_address: Option<String>,
    /// no description provided
    
    pub kind: Option<String>,
}

impl client::Part for ManagedZoneForwardingConfigNameServerTarget {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list managed zone operations](ManagedZoneOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedZoneOperationsListResponse {
    /// no description provided
    
    pub header: Option<ResponseHeader>,
    /// Type of resource.
    
    pub kind: Option<String>,
    /// The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your page token. This lets you retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. You cannot retrieve a consistent snapshot of a collection larger than the maximum page size.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The operation resources.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ManagedZoneOperationsListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedZonePeeringConfig {
    /// no description provided
    
    pub kind: Option<String>,
    /// The network with which to peer.
    #[serde(rename="targetNetwork")]
    
    pub target_network: Option<ManagedZonePeeringConfigTargetNetwork>,
}

impl client::Part for ManagedZonePeeringConfig {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedZonePeeringConfigTargetNetwork {
    /// The time at which the zone was deactivated, in RFC 3339 date-time format. An empty string indicates that the peering connection is active. The producer network can deactivate a zone. The zone is automatically deactivated if the producer network that the zone targeted is deleted. Output only.
    #[serde(rename="deactivateTime")]
    
    pub deactivate_time: Option<String>,
    /// no description provided
    
    pub kind: Option<String>,
    /// The fully qualified URL of the VPC network to forward queries to. This should be formatted like https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}
    #[serde(rename="networkUrl")]
    
    pub network_url: Option<String>,
}

impl client::Part for ManagedZonePeeringConfigTargetNetwork {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedZonePrivateVisibilityConfig {
    /// The list of Google Kubernetes Engine clusters that can see this zone.
    #[serde(rename="gkeClusters")]
    
    pub gke_clusters: Option<Vec<ManagedZonePrivateVisibilityConfigGKECluster>>,
    /// no description provided
    
    pub kind: Option<String>,
    /// The list of VPC networks that can see this zone.
    
    pub networks: Option<Vec<ManagedZonePrivateVisibilityConfigNetwork>>,
}

impl client::Part for ManagedZonePrivateVisibilityConfig {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedZonePrivateVisibilityConfigGKECluster {
    /// The resource name of the cluster to bind this ManagedZone to. This should be specified in the format like: projects/*/locations/*/clusters/*. This is referenced from GKE projects.locations.clusters.get API: https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters/get
    #[serde(rename="gkeClusterName")]
    
    pub gke_cluster_name: Option<String>,
    /// no description provided
    
    pub kind: Option<String>,
}

impl client::Part for ManagedZonePrivateVisibilityConfigGKECluster {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedZonePrivateVisibilityConfigNetwork {
    /// no description provided
    
    pub kind: Option<String>,
    /// The fully qualified URL of the VPC network to bind to. Format this URL like https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}
    #[serde(rename="networkUrl")]
    
    pub network_url: Option<String>,
}

impl client::Part for ManagedZonePrivateVisibilityConfigNetwork {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedZoneReverseLookupConfig {
    /// no description provided
    
    pub kind: Option<String>,
}

impl client::Part for ManagedZoneReverseLookupConfig {}


/// Contains information about Service Directory-backed zones.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedZoneServiceDirectoryConfig {
    /// no description provided
    
    pub kind: Option<String>,
    /// Contains information about the namespace associated with the zone.
    
    pub namespace: Option<ManagedZoneServiceDirectoryConfigNamespace>,
}

impl client::Part for ManagedZoneServiceDirectoryConfig {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedZoneServiceDirectoryConfigNamespace {
    /// The time that the namespace backing this zone was deleted; an empty string if it still exists. This is in RFC3339 text format. Output only.
    #[serde(rename="deletionTime")]
    
    pub deletion_time: Option<String>,
    /// no description provided
    
    pub kind: Option<String>,
    /// The fully qualified URL of the namespace associated with the zone. Format must be https://servicedirectory.googleapis.com/v1/projects/{project}/locations/{location}/namespaces/{namespace}
    #[serde(rename="namespaceUrl")]
    
    pub namespace_url: Option<String>,
}

impl client::Part for ManagedZoneServiceDirectoryConfigNamespace {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list managed zones](ManagedZoneListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedZonesListResponse {
    /// no description provided
    
    pub header: Option<ResponseHeader>,
    /// Type of resource.
    
    pub kind: Option<String>,
    /// The managed zone resources.
    #[serde(rename="managedZones")]
    
    pub managed_zones: Option<Vec<ManagedZone>>,
    /// The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your page token. This lets you the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. You cannot retrieve a consistent snapshot of a collection larger than the maximum page size.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ManagedZonesListResponse {}


/// An operation represents a successful mutation performed on a Cloud DNS resource. Operations provide: - An audit log of server resource mutations. - A way to recover/retry API calls in the case where the response is never received by the caller. Use the caller specified client_operation_id.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get managed zone operations](ManagedZoneOperationGetCall) (response)
/// * [patch managed zones](ManagedZonePatchCall) (response)
/// * [update managed zones](ManagedZoneUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// Only populated if the operation targeted a DnsKey (output only).
    #[serde(rename="dnsKeyContext")]
    
    pub dns_key_context: Option<OperationDnsKeyContext>,
    /// Unique identifier for the resource. This is the client_operation_id if the client specified it when the mutation was initiated, otherwise, it is generated by the server. The name must be 1-63 characters long and match the regular expression [-a-z0-9]? (output only)
    
    pub id: Option<String>,
    /// no description provided
    
    pub kind: Option<String>,
    /// The time that this operation was started by the server. This is in RFC3339 text format (output only).
    #[serde(rename="startTime")]
    
    pub start_time: Option<String>,
    /// Status of the operation. Can be one of the following: "PENDING" or "DONE" (output only). A status of "DONE" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet.
    
    pub status: Option<OperationStatusEnum>,
    /// Type of the operation. Operations include insert, update, and delete (output only).
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// User who requested the operation, for example: user@example.com. cloud-dns-system for operations automatically done by the system. (output only)
    
    pub user: Option<String>,
    /// Only populated if the operation targeted a ManagedZone (output only).
    #[serde(rename="zoneContext")]
    
    pub zone_context: Option<OperationManagedZoneContext>,
}

impl client::ResponseResult for Operation {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationDnsKeyContext {
    /// The post-operation DnsKey resource.
    #[serde(rename="newValue")]
    
    pub new_value: Option<DnsKey>,
    /// The pre-operation DnsKey resource.
    #[serde(rename="oldValue")]
    
    pub old_value: Option<DnsKey>,
}

impl client::Part for OperationDnsKeyContext {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationManagedZoneContext {
    /// The post-operation ManagedZone resource.
    #[serde(rename="newValue")]
    
    pub new_value: Option<ManagedZone>,
    /// The pre-operation ManagedZone resource.
    #[serde(rename="oldValue")]
    
    pub old_value: Option<ManagedZone>,
}

impl client::Part for OperationManagedZoneContext {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list policies](PolicyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PoliciesListResponse {
    /// no description provided
    
    pub header: Option<ResponseHeader>,
    /// Type of resource.
    
    pub kind: Option<String>,
    /// The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your page token. This lets you the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. You cannot retrieve a consistent snapshot of a collection larger than the maximum page size.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The policy resources.
    
    pub policies: Option<Vec<Policy>>,
}

impl client::ResponseResult for PoliciesListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [patch policies](PolicyPatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PoliciesPatchResponse {
    /// no description provided
    
    pub header: Option<ResponseHeader>,
    /// no description provided
    
    pub policy: Option<Policy>,
}

impl client::ResponseResult for PoliciesPatchResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update policies](PolicyUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PoliciesUpdateResponse {
    /// no description provided
    
    pub header: Option<ResponseHeader>,
    /// no description provided
    
    pub policy: Option<Policy>,
}

impl client::ResponseResult for PoliciesUpdateResponse {}


/// A policy is a collection of DNS rules applied to one or more Virtual Private Cloud resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create policies](PolicyCreateCall) (request|response)
/// * [get policies](PolicyGetCall) (response)
/// * [patch policies](PolicyPatchCall) (request)
/// * [update policies](PolicyUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Sets an alternative name server for the associated networks. When specified, all DNS queries are forwarded to a name server that you choose. Names such as .internal are not available when an alternative name server is specified.
    #[serde(rename="alternativeNameServerConfig")]
    
    pub alternative_name_server_config: Option<PolicyAlternativeNameServerConfig>,
    /// A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the policy's function.
    
    pub description: Option<String>,
    /// Allows networks bound to this policy to receive DNS queries sent by VMs or applications over VPN connections. When enabled, a virtual IP address is allocated from each of the subnetworks that are bound to this policy.
    #[serde(rename="enableInboundForwarding")]
    
    pub enable_inbound_forwarding: Option<bool>,
    /// Controls whether logging is enabled for the networks bound to this policy. Defaults to no logging if not set.
    #[serde(rename="enableLogging")]
    
    pub enable_logging: Option<bool>,
    /// Unique identifier for the resource; defined by the server (output only).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// no description provided
    
    pub kind: Option<String>,
    /// User-assigned name for this policy.
    
    pub name: Option<String>,
    /// List of network names specifying networks to which this policy is applied.
    
    pub networks: Option<Vec<PolicyNetwork>>,
}

impl client::RequestValue for Policy {}
impl client::ResponseResult for Policy {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PolicyAlternativeNameServerConfig {
    /// no description provided
    
    pub kind: Option<String>,
    /// Sets an alternative name server for the associated networks. When specified, all DNS queries are forwarded to a name server that you choose. Names such as .internal are not available when an alternative name server is specified.
    #[serde(rename="targetNameServers")]
    
    pub target_name_servers: Option<Vec<PolicyAlternativeNameServerConfigTargetNameServer>>,
}

impl client::Part for PolicyAlternativeNameServerConfig {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PolicyAlternativeNameServerConfigTargetNameServer {
    /// Forwarding path for this TargetNameServer. If unset or set to DEFAULT, Cloud DNS makes forwarding decisions based on address ranges; that is, RFC1918 addresses go to the VPC network, non-RFC1918 addresses go to the internet. When set to PRIVATE, Cloud DNS always sends queries through the VPC network for this target.
    #[serde(rename="forwardingPath")]
    
    pub forwarding_path: Option<PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum>,
    /// IPv4 address to forward queries to.
    #[serde(rename="ipv4Address")]
    
    pub ipv4_address: Option<String>,
    /// IPv6 address to forward to. Does not accept both fields (ipv4 & ipv6) being populated. Public preview as of November 2022.
    #[serde(rename="ipv6Address")]
    
    pub ipv6_address: Option<String>,
    /// no description provided
    
    pub kind: Option<String>,
}

impl client::Part for PolicyAlternativeNameServerConfigTargetNameServer {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PolicyNetwork {
    /// no description provided
    
    pub kind: Option<String>,
    /// The fully qualified URL of the VPC network to bind to. This should be formatted like https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}
    #[serde(rename="networkUrl")]
    
    pub network_url: Option<String>,
}

impl client::Part for PolicyNetwork {}


/// A project resource. The project is a top level container for resources including Cloud DNS ManagedZones. Projects can be created only in the APIs console. Next tag: 7.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get projects](ProjectGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    /// User assigned unique identifier for the resource (output only).
    
    pub id: Option<String>,
    /// no description provided
    
    pub kind: Option<String>,
    /// Unique numeric identifier for the resource; defined by the server (output only).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub number: Option<u64>,
    /// Quotas assigned to this project (output only).
    
    pub quota: Option<Quota>,
}

impl client::Resource for Project {}
impl client::ResponseResult for Project {}


/// Limits associated with a Project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Quota {
    /// Maximum allowed number of DnsKeys per ManagedZone.
    #[serde(rename="dnsKeysPerManagedZone")]
    
    pub dns_keys_per_managed_zone: Option<i32>,
    /// Maximum allowed number of GKE clusters to which a privately scoped zone can be attached.
    #[serde(rename="gkeClustersPerManagedZone")]
    
    pub gke_clusters_per_managed_zone: Option<i32>,
    /// Maximum allowed number of GKE clusters per policy.
    #[serde(rename="gkeClustersPerPolicy")]
    
    pub gke_clusters_per_policy: Option<i32>,
    /// Maximum allowed number of GKE clusters per response policy.
    #[serde(rename="gkeClustersPerResponsePolicy")]
    
    pub gke_clusters_per_response_policy: Option<i32>,
    /// Maximum allowed number of items per routing policy.
    #[serde(rename="itemsPerRoutingPolicy")]
    
    pub items_per_routing_policy: Option<i32>,
    /// no description provided
    
    pub kind: Option<String>,
    /// Maximum allowed number of managed zones in the project.
    #[serde(rename="managedZones")]
    
    pub managed_zones: Option<i32>,
    /// Maximum allowed number of managed zones which can be attached to a GKE cluster.
    #[serde(rename="managedZonesPerGkeCluster")]
    
    pub managed_zones_per_gke_cluster: Option<i32>,
    /// Maximum allowed number of managed zones which can be attached to a network.
    #[serde(rename="managedZonesPerNetwork")]
    
    pub managed_zones_per_network: Option<i32>,
    /// Maximum allowed number of networks to which a privately scoped zone can be attached.
    #[serde(rename="networksPerManagedZone")]
    
    pub networks_per_managed_zone: Option<i32>,
    /// Maximum allowed number of networks per policy.
    #[serde(rename="networksPerPolicy")]
    
    pub networks_per_policy: Option<i32>,
    /// Maximum allowed number of networks per response policy.
    #[serde(rename="networksPerResponsePolicy")]
    
    pub networks_per_response_policy: Option<i32>,
    /// Maximum allowed number of consumer peering zones per target network owned by this producer project
    #[serde(rename="peeringZonesPerTargetNetwork")]
    
    pub peering_zones_per_target_network: Option<i32>,
    /// Maximum allowed number of policies per project.
    
    pub policies: Option<i32>,
    /// Maximum allowed number of ResourceRecords per ResourceRecordSet.
    #[serde(rename="resourceRecordsPerRrset")]
    
    pub resource_records_per_rrset: Option<i32>,
    /// Maximum allowed number of response policies per project.
    #[serde(rename="responsePolicies")]
    
    pub response_policies: Option<i32>,
    /// Maximum allowed number of rules per response policy.
    #[serde(rename="responsePolicyRulesPerResponsePolicy")]
    
    pub response_policy_rules_per_response_policy: Option<i32>,
    /// Maximum allowed number of ResourceRecordSets to add per ChangesCreateRequest.
    #[serde(rename="rrsetAdditionsPerChange")]
    
    pub rrset_additions_per_change: Option<i32>,
    /// Maximum allowed number of ResourceRecordSets to delete per ChangesCreateRequest.
    #[serde(rename="rrsetDeletionsPerChange")]
    
    pub rrset_deletions_per_change: Option<i32>,
    /// Maximum allowed number of ResourceRecordSets per zone in the project.
    #[serde(rename="rrsetsPerManagedZone")]
    
    pub rrsets_per_managed_zone: Option<i32>,
    /// Maximum allowed number of target name servers per managed forwarding zone.
    #[serde(rename="targetNameServersPerManagedZone")]
    
    pub target_name_servers_per_managed_zone: Option<i32>,
    /// Maximum allowed number of alternative target name servers per policy.
    #[serde(rename="targetNameServersPerPolicy")]
    
    pub target_name_servers_per_policy: Option<i32>,
    /// Maximum allowed size for total rrdata in one ChangesCreateRequest in bytes.
    #[serde(rename="totalRrdataSizePerChange")]
    
    pub total_rrdata_size_per_change: Option<i32>,
    /// DNSSEC algorithm and key length types that can be used for DnsKeys.
    #[serde(rename="whitelistedKeySpecs")]
    
    pub whitelisted_key_specs: Option<Vec<DnsKeySpec>>,
}

impl client::Part for Quota {}


/// A RRSetRoutingPolicy represents ResourceRecordSet data that is returned dynamically with the response varying based on configured properties such as geolocation or by weighted random selection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RRSetRoutingPolicy {
    /// no description provided
    
    pub geo: Option<RRSetRoutingPolicyGeoPolicy>,
    /// no description provided
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="primaryBackup")]
    
    pub primary_backup: Option<RRSetRoutingPolicyPrimaryBackupPolicy>,
    /// no description provided
    
    pub wrr: Option<RRSetRoutingPolicyWrrPolicy>,
}

impl client::Part for RRSetRoutingPolicy {}


/// Configures a RRSetRoutingPolicy that routes based on the geo location of the querying user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RRSetRoutingPolicyGeoPolicy {
    /// Without fencing, if health check fails for all configured items in the current geo bucket, we'll failover to the next nearest geo bucket. With fencing, if health check is enabled, as long as some targets in the current geo bucket are healthy, we'll return only the healthy targets. However, if they're all unhealthy, we won't failover to the next nearest bucket, we'll simply return all the items in the current bucket even though they're unhealthy.
    #[serde(rename="enableFencing")]
    
    pub enable_fencing: Option<bool>,
    /// The primary geo routing configuration. If there are multiple items with the same location, an error is returned instead.
    
    pub items: Option<Vec<RRSetRoutingPolicyGeoPolicyGeoPolicyItem>>,
    /// no description provided
    
    pub kind: Option<String>,
}

impl client::Part for RRSetRoutingPolicyGeoPolicy {}


/// ResourceRecordSet data for one geo location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RRSetRoutingPolicyGeoPolicyGeoPolicyItem {
    /// For A and AAAA types only. Endpoints to return in the query result only if they are healthy. These can be specified along with rrdata within this item.
    #[serde(rename="healthCheckedTargets")]
    
    pub health_checked_targets: Option<RRSetRoutingPolicyHealthCheckTargets>,
    /// no description provided
    
    pub kind: Option<String>,
    /// The geo-location granularity is a GCP region. This location string should correspond to a GCP region. e.g. "us-east1", "southamerica-east1", "asia-east1", etc.
    
    pub location: Option<String>,
    /// no description provided
    
    pub rrdatas: Option<Vec<String>>,
    /// DNSSEC generated signatures for all the rrdata within this item. Note that if health checked targets are provided for DNSSEC enabled zones, there's a restriction of 1 ip per item. .
    #[serde(rename="signatureRrdatas")]
    
    pub signature_rrdatas: Option<Vec<String>>,
}

impl client::Part for RRSetRoutingPolicyGeoPolicyGeoPolicyItem {}


/// HealthCheckTargets describes endpoints to health-check when responding to Routing Policy queries. Only the healthy endpoints will be included in the response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RRSetRoutingPolicyHealthCheckTargets {
    /// no description provided
    #[serde(rename="internalLoadBalancers")]
    
    pub internal_load_balancers: Option<Vec<RRSetRoutingPolicyLoadBalancerTarget>>,
}

impl client::Part for RRSetRoutingPolicyHealthCheckTargets {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RRSetRoutingPolicyLoadBalancerTarget {
    /// The frontend IP address of the
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// no description provided
    #[serde(rename="ipProtocol")]
    
    pub ip_protocol: Option<RRSetRoutingPolicyLoadBalancerTargetIpProtocolEnum>,
    /// no description provided
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="loadBalancerType")]
    
    pub load_balancer_type: Option<RRSetRoutingPolicyLoadBalancerTargetLoadBalancerTypeEnum>,
    /// The fully qualified url of the network on which the ILB is
    #[serde(rename="networkUrl")]
    
    pub network_url: Option<String>,
    /// Load Balancer to health check. The configured port of the Load Balancer.
    
    pub port: Option<String>,
    /// present. This should be formatted like https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network} The project ID in which the ILB exists.
    
    pub project: Option<String>,
    /// The region for regional ILBs.
    
    pub region: Option<String>,
}

impl client::Part for RRSetRoutingPolicyLoadBalancerTarget {}


/// Configures a RRSetRoutingPolicy such that all queries are responded with the primary_targets if they are healthy. And if all of them are unhealthy, then we fallback to a geo localized policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RRSetRoutingPolicyPrimaryBackupPolicy {
    /// Backup targets provide a regional failover policy for the otherwise global primary targets. If serving state is set to BACKUP, this policy essentially becomes a geo routing policy.
    #[serde(rename="backupGeoTargets")]
    
    pub backup_geo_targets: Option<RRSetRoutingPolicyGeoPolicy>,
    /// no description provided
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="primaryTargets")]
    
    pub primary_targets: Option<RRSetRoutingPolicyHealthCheckTargets>,
    /// When serving state is PRIMARY, this field provides the option of sending a small percentage of the traffic to the backup targets.
    #[serde(rename="trickleTraffic")]
    
    pub trickle_traffic: Option<f64>,
}

impl client::Part for RRSetRoutingPolicyPrimaryBackupPolicy {}


/// Configures a RRSetRoutingPolicy that routes in a weighted round robin fashion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RRSetRoutingPolicyWrrPolicy {
    /// no description provided
    
    pub items: Option<Vec<RRSetRoutingPolicyWrrPolicyWrrPolicyItem>>,
    /// no description provided
    
    pub kind: Option<String>,
}

impl client::Part for RRSetRoutingPolicyWrrPolicy {}


/// A routing block which contains the routing information for one WRR item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RRSetRoutingPolicyWrrPolicyWrrPolicyItem {
    /// endpoints that need to be health checked before making the routing decision. The unhealthy endpoints will be omitted from the result. If all endpoints within a buckete are unhealthy, we'll choose a different bucket (sampled w.r.t. its weight) for responding. Note that if DNSSEC is enabled for this zone, only one of rrdata or health_checked_targets can be set.
    #[serde(rename="healthCheckedTargets")]
    
    pub health_checked_targets: Option<RRSetRoutingPolicyHealthCheckTargets>,
    /// no description provided
    
    pub kind: Option<String>,
    /// no description provided
    
    pub rrdatas: Option<Vec<String>>,
    /// DNSSEC generated signatures for all the rrdata within this item. Note that if health checked targets are provided for DNSSEC enabled zones, there's a restriction of 1 ip per item. .
    #[serde(rename="signatureRrdatas")]
    
    pub signature_rrdatas: Option<Vec<String>>,
    /// The weight corresponding to this subset of rrdata. When multiple WeightedRoundRobinPolicyItems are configured, the probability of returning an rrset is proportional to its weight relative to the sum of weights configured for all items. This weight should be non-negative.
    
    pub weight: Option<f64>,
}

impl client::Part for RRSetRoutingPolicyWrrPolicyWrrPolicyItem {}


/// A unit of data that is returned by the DNS servers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create resource record sets](ResourceRecordSetCreateCall) (request|response)
/// * [delete resource record sets](ResourceRecordSetDeleteCall) (none)
/// * [get resource record sets](ResourceRecordSetGetCall) (response)
/// * [list resource record sets](ResourceRecordSetListCall) (none)
/// * [patch resource record sets](ResourceRecordSetPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceRecordSet {
    /// no description provided
    
    pub kind: Option<String>,
    /// For example, www.example.com.
    
    pub name: Option<String>,
    /// Configures dynamic query responses based on geo location of querying user or a weighted round robin based routing policy. A ResourceRecordSet should only have either rrdata (static) or routing_policy (dynamic). An error is returned otherwise.
    #[serde(rename="routingPolicy")]
    
    pub routing_policy: Option<RRSetRoutingPolicy>,
    /// As defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1) -- see examples.
    
    pub rrdatas: Option<Vec<String>>,
    /// As defined in RFC 4034 (section 3.2).
    #[serde(rename="signatureRrdatas")]
    
    pub signature_rrdatas: Option<Vec<String>>,
    /// Number of seconds that this ResourceRecordSet can be cached by resolvers.
    
    pub ttl: Option<i32>,
    /// The identifier of a supported record type. See the list of Supported DNS record types.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for ResourceRecordSet {}
impl client::Resource for ResourceRecordSet {}
impl client::ResponseResult for ResourceRecordSet {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete resource record sets](ResourceRecordSetDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceRecordSetsDeleteResponse { _never_set: Option<bool> }

impl client::ResponseResult for ResourceRecordSetsDeleteResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list resource record sets](ResourceRecordSetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceRecordSetsListResponse {
    /// no description provided
    
    pub header: Option<ResponseHeader>,
    /// Type of resource.
    
    pub kind: Option<String>,
    /// The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your pagination token. This lets you retrieve complete contents of even larger collections, one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of elements returned are an inconsistent view of the collection. You cannot retrieve a consistent snapshot of a collection larger than the maximum page size.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The resource record set resources.
    
    pub rrsets: Option<Vec<ResourceRecordSet>>,
}

impl client::ResponseResult for ResourceRecordSetsListResponse {}


/// Elements common to every response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponseHeader {
    /// For mutating operation requests that completed successfully. This is the client_operation_id if the client specified it, otherwise it is generated by the server (output only).
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
}

impl client::Part for ResponseHeader {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list response policies](ResponsePolicyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponsePoliciesListResponse {
    /// no description provided
    
    pub header: Option<ResponseHeader>,
    /// The presence of this field indicates that more results exist following your last page of results in pagination order. To fetch them, make another list request by using this value as your page token. This lets you view the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. You cannot retrieve a consistent snapshot of a collection larger than the maximum page size.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The Response Policy resources.
    #[serde(rename="responsePolicies")]
    
    pub response_policies: Option<Vec<ResponsePolicy>>,
}

impl client::ResponseResult for ResponsePoliciesListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [patch response policies](ResponsePolicyPatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponsePoliciesPatchResponse {
    /// no description provided
    
    pub header: Option<ResponseHeader>,
    /// no description provided
    #[serde(rename="responsePolicy")]
    
    pub response_policy: Option<ResponsePolicy>,
}

impl client::ResponseResult for ResponsePoliciesPatchResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update response policies](ResponsePolicyUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponsePoliciesUpdateResponse {
    /// no description provided
    
    pub header: Option<ResponseHeader>,
    /// no description provided
    #[serde(rename="responsePolicy")]
    
    pub response_policy: Option<ResponsePolicy>,
}

impl client::ResponseResult for ResponsePoliciesUpdateResponse {}


/// A Response Policy is a collection of selectors that apply to queries made against one or more Virtual Private Cloud networks.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create response policies](ResponsePolicyCreateCall) (request|response)
/// * [get response policies](ResponsePolicyGetCall) (response)
/// * [patch response policies](ResponsePolicyPatchCall) (request)
/// * [update response policies](ResponsePolicyUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponsePolicy {
    /// User-provided description for this Response Policy.
    
    pub description: Option<String>,
    /// The list of Google Kubernetes Engine clusters to which this response policy is applied.
    #[serde(rename="gkeClusters")]
    
    pub gke_clusters: Option<Vec<ResponsePolicyGKECluster>>,
    /// Unique identifier for the resource; defined by the server (output only).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// no description provided
    
    pub kind: Option<String>,
    /// User labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// List of network names specifying networks to which this policy is applied.
    
    pub networks: Option<Vec<ResponsePolicyNetwork>>,
    /// User assigned name for this Response Policy.
    #[serde(rename="responsePolicyName")]
    
    pub response_policy_name: Option<String>,
}

impl client::RequestValue for ResponsePolicy {}
impl client::ResponseResult for ResponsePolicy {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponsePolicyGKECluster {
    /// The resource name of the cluster to bind this response policy to. This should be specified in the format like: projects/*/locations/*/clusters/*. This is referenced from GKE projects.locations.clusters.get API: https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters/get
    #[serde(rename="gkeClusterName")]
    
    pub gke_cluster_name: Option<String>,
    /// no description provided
    
    pub kind: Option<String>,
}

impl client::Part for ResponsePolicyGKECluster {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponsePolicyNetwork {
    /// no description provided
    
    pub kind: Option<String>,
    /// The fully qualified URL of the VPC network to bind to. This should be formatted like https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}
    #[serde(rename="networkUrl")]
    
    pub network_url: Option<String>,
}

impl client::Part for ResponsePolicyNetwork {}


/// A Response Policy Rule is a selector that applies its behavior to queries that match the selector. Selectors are DNS names, which may be wildcards or exact matches. Each DNS query subject to a Response Policy matches at most one ResponsePolicyRule, as identified by the dns_name field with the longest matching suffix.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create response policy rules](ResponsePolicyRuleCreateCall) (request|response)
/// * [delete response policy rules](ResponsePolicyRuleDeleteCall) (none)
/// * [get response policy rules](ResponsePolicyRuleGetCall) (response)
/// * [list response policy rules](ResponsePolicyRuleListCall) (none)
/// * [patch response policy rules](ResponsePolicyRulePatchCall) (request)
/// * [update response policy rules](ResponsePolicyRuleUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponsePolicyRule {
    /// Answer this query with a behavior rather than DNS data.
    
    pub behavior: Option<ResponsePolicyRuleBehaviorEnum>,
    /// The DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule.
    #[serde(rename="dnsName")]
    
    pub dns_name: Option<String>,
    /// no description provided
    
    pub kind: Option<String>,
    /// Answer this query directly with DNS data. These ResourceRecordSets override any other DNS behavior for the matched name; in particular they override private zones, the public internet, and GCP internal DNS. No SOA nor NS types are allowed.
    #[serde(rename="localData")]
    
    pub local_data: Option<ResponsePolicyRuleLocalData>,
    /// An identifier for this rule. Must be unique with the ResponsePolicy.
    #[serde(rename="ruleName")]
    
    pub rule_name: Option<String>,
}

impl client::RequestValue for ResponsePolicyRule {}
impl client::Resource for ResponsePolicyRule {}
impl client::ResponseResult for ResponsePolicyRule {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponsePolicyRuleLocalData {
    /// All resource record sets for this selector, one per resource record type. The name must match the dns_name.
    #[serde(rename="localDatas")]
    
    pub local_datas: Option<Vec<ResourceRecordSet>>,
}

impl client::Part for ResponsePolicyRuleLocalData {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list response policy rules](ResponsePolicyRuleListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponsePolicyRulesListResponse {
    /// no description provided
    
    pub header: Option<ResponseHeader>,
    /// The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your page token. This lets you the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. You cannot retrieve a consistent snapshot of a collection larger than the maximum page size.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The Response Policy Rule resources.
    #[serde(rename="responsePolicyRules")]
    
    pub response_policy_rules: Option<Vec<ResponsePolicyRule>>,
}

impl client::ResponseResult for ResponsePolicyRulesListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [patch response policy rules](ResponsePolicyRulePatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponsePolicyRulesPatchResponse {
    /// no description provided
    
    pub header: Option<ResponseHeader>,
    /// no description provided
    #[serde(rename="responsePolicyRule")]
    
    pub response_policy_rule: Option<ResponsePolicyRule>,
}

impl client::ResponseResult for ResponsePolicyRulesPatchResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update response policy rules](ResponsePolicyRuleUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponsePolicyRulesUpdateResponse {
    /// no description provided
    
    pub header: Option<ResponseHeader>,
    /// no description provided
    #[serde(rename="responsePolicyRule")]
    
    pub response_policy_rule: Option<ResponsePolicyRule>,
}

impl client::ResponseResult for ResponsePolicyRulesUpdateResponse {}


