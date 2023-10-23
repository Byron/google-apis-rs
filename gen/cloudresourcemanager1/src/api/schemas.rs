use super::*;
/// Identifying information for a single ancestor of a project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Ancestor {
    /// Resource id of the ancestor.
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<ResourceId>,
}

impl client::Part for Ancestor {}


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


/// A `Constraint` that is either enforced or not. For example a constraint `constraints/compute.disableSerialPortAccess`. If it is enforced on a VM instance, serial port connections will not be opened to that instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BooleanConstraint { _never_set: Option<bool> }

impl client::Part for BooleanConstraint {}


/// Used in `policy_type` to specify how `boolean_policy` will behave at this resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BooleanPolicy {
    /// If `true`, then the `Policy` is enforced. If `false`, then any configuration is acceptable. Suppose you have a `Constraint` `constraints/compute.disableSerialPortAccess` with `constraint_default` set to `ALLOW`. A `Policy` for that `Constraint` exhibits the following behavior: - If the `Policy` at this resource has enforced set to `false`, serial port connection attempts will be allowed. - If the `Policy` at this resource has enforced set to `true`, serial port connection attempts will be refused. - If the `Policy` at this resource is `RestoreDefault`, serial port connection attempts will be allowed. - If no `Policy` is set at this resource or anywhere higher in the resource hierarchy, serial port connection attempts will be allowed. - If no `Policy` is set at this resource, but one exists higher in the resource hierarchy, the behavior is as if the`Policy` were set at this resource. The following examples demonstrate the different possible layerings: Example 1 (nearest `Constraint` wins): `organizations/foo` has a `Policy` with: {enforced: false} `projects/bar` has no `Policy` set. The constraint at `projects/bar` and `organizations/foo` will not be enforced. Example 2 (enforcement gets replaced): `organizations/foo` has a `Policy` with: {enforced: false} `projects/bar` has a `Policy` with: {enforced: true} The constraint at `organizations/foo` is not enforced. The constraint at `projects/bar` is enforced. Example 3 (RestoreDefault): `organizations/foo` has a `Policy` with: {enforced: true} `projects/bar` has a `Policy` with: {RestoreDefault: {}} The constraint at `organizations/foo` is enforced. The constraint at `projects/bar` is not enforced, because `constraint_default` for the `Constraint` is `ALLOW`.
    
    pub enforced: Option<bool>,
}

impl client::Part for BooleanPolicy {}


/// The request sent to the ClearOrgPolicy method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clear org policy folders](FolderClearOrgPolicyCall) (request)
/// * [clear org policy organizations](OrganizationClearOrgPolicyCall) (request)
/// * [clear org policy projects](ProjectClearOrgPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClearOrgPolicyRequest {
    /// Name of the `Constraint` of the `Policy` to clear.
    
    pub constraint: Option<String>,
    /// The current version, for concurrency control. Not sending an `etag` will cause the `Policy` to be cleared blindly.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
}

impl client::RequestValue for ClearOrgPolicyRequest {}


/// A `Constraint` describes a way in which a resource’s configuration can be restricted. For example, it controls which cloud services can be activated across an organization, or whether a Compute Engine instance can have serial port connections established. `Constraints` can be configured by the organization’s policy administrator to fit the needs of the organzation by setting Policies for `Constraints` at different locations in the organization’s resource hierarchy. Policies are inherited down the resource hierarchy from higher levels, but can also be overridden. For details about the inheritance rules please read about [Policies](https://cloud.google.com/resource-manager/reference/rest/v1/Policy). `Constraints` have a default behavior determined by the `constraint_default` field, which is the enforcement behavior that is used in the absence of a `Policy` being defined or inherited for the resource in question.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Constraint {
    /// Defines this constraint as being a BooleanConstraint.
    #[serde(rename="booleanConstraint")]
    
    pub boolean_constraint: Option<BooleanConstraint>,
    /// The evaluation behavior of this constraint in the absence of 'Policy'.
    #[serde(rename="constraintDefault")]
    
    pub constraint_default: Option<ConstraintConstraintDefaultEnum>,
    /// Detailed description of what this `Constraint` controls as well as how and where it is enforced. Mutable.
    
    pub description: Option<String>,
    /// The human readable name. Mutable.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Defines this constraint as being a ListConstraint.
    #[serde(rename="listConstraint")]
    
    pub list_constraint: Option<ListConstraint>,
    /// Immutable value, required to globally be unique. For example, `constraints/serviceuser.services`
    
    pub name: Option<String>,
    /// Version of the `Constraint`. Default version is 0;
    
    pub version: Option<i32>,
}

impl client::Part for Constraint {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clear org policy folders](FolderClearOrgPolicyCall) (response)
/// * [delete liens](LienDeleteCall) (response)
/// * [clear org policy organizations](OrganizationClearOrgPolicyCall) (response)
/// * [clear org policy projects](ProjectClearOrgPolicyCall) (response)
/// * [delete projects](ProjectDeleteCall) (response)
/// * [undelete projects](ProjectUndeleteCall) (response)
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


/// The request sent to the GetAncestry method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get ancestry projects](ProjectGetAncestryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetAncestryRequest { _never_set: Option<bool> }

impl client::RequestValue for GetAncestryRequest {}


/// Response from the projects.getAncestry method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get ancestry projects](ProjectGetAncestryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetAncestryResponse {
    /// Ancestors are ordered from bottom to top of the resource hierarchy. The first ancestor is the project itself, followed by the project's parent, etc..
    
    pub ancestor: Option<Vec<Ancestor>>,
}

impl client::ResponseResult for GetAncestryResponse {}


/// The request sent to the GetEffectiveOrgPolicy method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get effective org policy folders](FolderGetEffectiveOrgPolicyCall) (request)
/// * [get effective org policy organizations](OrganizationGetEffectiveOrgPolicyCall) (request)
/// * [get effective org policy projects](ProjectGetEffectiveOrgPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetEffectiveOrgPolicyRequest {
    /// The name of the `Constraint` to compute the effective `Policy`.
    
    pub constraint: Option<String>,
}

impl client::RequestValue for GetEffectiveOrgPolicyRequest {}


/// Request message for `GetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get iam policy organizations](OrganizationGetIamPolicyCall) (request)
/// * [get iam policy projects](ProjectGetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIamPolicyRequest {
    /// OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`.
    
    pub options: Option<GetPolicyOptions>,
}

impl client::RequestValue for GetIamPolicyRequest {}


/// The request sent to the GetOrgPolicy method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get org policy folders](FolderGetOrgPolicyCall) (request)
/// * [get org policy organizations](OrganizationGetOrgPolicyCall) (request)
/// * [get org policy projects](ProjectGetOrgPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetOrgPolicyRequest {
    /// Name of the `Constraint` to get the `Policy`.
    
    pub constraint: Option<String>,
}

impl client::RequestValue for GetOrgPolicyRequest {}


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


/// A Lien represents an encumbrance on the actions that can be performed on a resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create liens](LienCreateCall) (request|response)
/// * [delete liens](LienDeleteCall) (none)
/// * [get liens](LienGetCall) (response)
/// * [list liens](LienListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Lien {
    /// The creation time of this Lien.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A system-generated unique identifier for this Lien. Example: `liens/1234abcd`
    
    pub name: Option<String>,
    /// A stable, user-visible/meaningful string identifying the origin of the Lien, intended to be inspected programmatically. Maximum length of 200 characters. Example: 'compute.googleapis.com'
    
    pub origin: Option<String>,
    /// A reference to the resource this Lien is attached to. The server will validate the parent against those for which Liens are supported. Example: `projects/1234`
    
    pub parent: Option<String>,
    /// Concise user-visible strings indicating why an action cannot be performed on a resource. Maximum length of 200 characters. Example: 'Holds production API key'
    
    pub reason: Option<String>,
    /// The types of operations which should be blocked as a result of this Lien. Each value should correspond to an IAM permission. The server will validate the permissions against those for which Liens are supported. An empty list is meaningless and will be rejected. Example: ['resourcemanager.projects.delete']
    
    pub restrictions: Option<Vec<String>>,
}

impl client::RequestValue for Lien {}
impl client::Resource for Lien {}
impl client::ResponseResult for Lien {}


/// The request sent to the `ListAvailableOrgPolicyConstraints` method on the project, folder, or organization.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list available org policy constraints folders](FolderListAvailableOrgPolicyConstraintCall) (request)
/// * [list available org policy constraints organizations](OrganizationListAvailableOrgPolicyConstraintCall) (request)
/// * [list available org policy constraints projects](ProjectListAvailableOrgPolicyConstraintCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAvailableOrgPolicyConstraintsRequest {
    /// Size of the pages to be returned. This is currently unsupported and will be ignored. The server may at any point start using this field to limit page size.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Page token used to retrieve the next page. This is currently unsupported and will be ignored. The server may at any point start using this field.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
}

impl client::RequestValue for ListAvailableOrgPolicyConstraintsRequest {}


/// The response returned from the `ListAvailableOrgPolicyConstraints` method. Returns all `Constraints` that could be set at this level of the hierarchy (contrast with the response from `ListPolicies`, which returns all policies which are set).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list available org policy constraints folders](FolderListAvailableOrgPolicyConstraintCall) (response)
/// * [list available org policy constraints organizations](OrganizationListAvailableOrgPolicyConstraintCall) (response)
/// * [list available org policy constraints projects](ProjectListAvailableOrgPolicyConstraintCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAvailableOrgPolicyConstraintsResponse {
    /// The collection of constraints that are settable on the request resource.
    
    pub constraints: Option<Vec<Constraint>>,
    /// Page token used to retrieve the next page. This is currently not used.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAvailableOrgPolicyConstraintsResponse {}


/// A `Constraint` that allows or disallows a list of string values, which are configured by an Organization's policy administrator with a `Policy`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConstraint {
    /// Optional. The Google Cloud Console will try to default to a configuration that matches the value specified in this `Constraint`.
    #[serde(rename="suggestedValue")]
    
    pub suggested_value: Option<String>,
    /// Indicates whether subtrees of Cloud Resource Manager resource hierarchy can be used in `Policy.allowed_values` and `Policy.denied_values`. For example, `"under:folders/123"` would match any resource under the 'folders/123' folder.
    #[serde(rename="supportsUnder")]
    
    pub supports_under: Option<bool>,
}

impl client::Part for ListConstraint {}


/// The response message for Liens.ListLiens.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list liens](LienListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLiensResponse {
    /// A list of Liens.
    
    pub liens: Option<Vec<Lien>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLiensResponse {}


/// The request sent to the ListOrgPolicies method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list org policies folders](FolderListOrgPolicyCall) (request)
/// * [list org policies organizations](OrganizationListOrgPolicyCall) (request)
/// * [list org policies projects](ProjectListOrgPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOrgPoliciesRequest {
    /// Size of the pages to be returned. This is currently unsupported and will be ignored. The server may at any point start using this field to limit page size.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Page token used to retrieve the next page. This is currently unsupported and will be ignored. The server may at any point start using this field.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
}

impl client::RequestValue for ListOrgPoliciesRequest {}


/// The response returned from the `ListOrgPolicies` method. It will be empty if no `Policies` are set on the resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list org policies folders](FolderListOrgPolicyCall) (response)
/// * [list org policies organizations](OrganizationListOrgPolicyCall) (response)
/// * [list org policies projects](ProjectListOrgPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOrgPoliciesResponse {
    /// Page token used to retrieve the next page. This is currently not used, but the server may at any point start supplying a valid token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The `Policies` that are set on the resource. It will be empty if no `Policies` are set.
    
    pub policies: Option<Vec<OrgPolicy>>,
}

impl client::ResponseResult for ListOrgPoliciesResponse {}


/// Used in `policy_type` to specify how `list_policy` behaves at this resource. `ListPolicy` can define specific values and subtrees of Cloud Resource Manager resource hierarchy (`Organizations`, `Folders`, `Projects`) that are allowed or denied by setting the `allowed_values` and `denied_values` fields. This is achieved by using the `under:` and optional `is:` prefixes. The `under:` prefix is used to denote resource subtree values. The `is:` prefix is used to denote specific values, and is required only if the value contains a ":". Values prefixed with "is:" are treated the same as values with no prefix. Ancestry subtrees must be in one of the following formats: - "projects/", e.g. "projects/tokyo-rain-123" - "folders/", e.g. "folders/1234" - "organizations/", e.g. "organizations/1234" The `supports_under` field of the associated `Constraint` defines whether ancestry prefixes can be used. You can set `allowed_values` and `denied_values` in the same `Policy` if `all_values` is `ALL_VALUES_UNSPECIFIED`. `ALLOW` or `DENY` are used to allow or deny all values. If `all_values` is set to either `ALLOW` or `DENY`, `allowed_values` and `denied_values` must be unset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPolicy {
    /// The policy all_values state.
    #[serde(rename="allValues")]
    
    pub all_values: Option<ListPolicyAllValuesEnum>,
    /// List of values allowed at this resource. Can only be set if `all_values` is set to `ALL_VALUES_UNSPECIFIED`.
    #[serde(rename="allowedValues")]
    
    pub allowed_values: Option<Vec<String>>,
    /// List of values denied at this resource. Can only be set if `all_values` is set to `ALL_VALUES_UNSPECIFIED`.
    #[serde(rename="deniedValues")]
    
    pub denied_values: Option<Vec<String>>,
    /// Determines the inheritance behavior for this `Policy`. By default, a `ListPolicy` set at a resource supersedes any `Policy` set anywhere up the resource hierarchy. However, if `inherit_from_parent` is set to `true`, then the values from the effective `Policy` of the parent resource are inherited, meaning the values set in this `Policy` are added to the values inherited up the hierarchy. Setting `Policy` hierarchies that inherit both allowed values and denied values isn't recommended in most circumstances to keep the configuration simple and understandable. However, it is possible to set a `Policy` with `allowed_values` set that inherits a `Policy` with `denied_values` set. In this case, the values that are allowed must be in `allowed_values` and not present in `denied_values`. For example, suppose you have a `Constraint` `constraints/serviceuser.services`, which has a `constraint_type` of `list_constraint`, and with `constraint_default` set to `ALLOW`. Suppose that at the Organization level, a `Policy` is applied that restricts the allowed API activations to {`E1`, `E2`}. Then, if a `Policy` is applied to a project below the Organization that has `inherit_from_parent` set to `false` and field all_values set to DENY, then an attempt to activate any API will be denied. The following examples demonstrate different possible layerings for `projects/bar` parented by `organizations/foo`: Example 1 (no inherited values): `organizations/foo` has a `Policy` with values: {allowed_values: "E1" allowed_values:"E2"} `projects/bar` has `inherit_from_parent` `false` and values: {allowed_values: "E3" allowed_values: "E4"} The accepted values at `organizations/foo` are `E1`, `E2`. The accepted values at `projects/bar` are `E3`, and `E4`. Example 2 (inherited values): `organizations/foo` has a `Policy` with values: {allowed_values: "E1" allowed_values:"E2"} `projects/bar` has a `Policy` with values: {value: "E3" value: "E4" inherit_from_parent: true} The accepted values at `organizations/foo` are `E1`, `E2`. The accepted values at `projects/bar` are `E1`, `E2`, `E3`, and `E4`. Example 3 (inheriting both allowed and denied values): `organizations/foo` has a `Policy` with values: {allowed_values: "E1" allowed_values: "E2"} `projects/bar` has a `Policy` with: {denied_values: "E1"} The accepted values at `organizations/foo` are `E1`, `E2`. The value accepted at `projects/bar` is `E2`. Example 4 (RestoreDefault): `organizations/foo` has a `Policy` with values: {allowed_values: "E1" allowed_values:"E2"} `projects/bar` has a `Policy` with values: {RestoreDefault: {}} The accepted values at `organizations/foo` are `E1`, `E2`. The accepted values at `projects/bar` are either all or none depending on the value of `constraint_default` (if `ALLOW`, all; if `DENY`, none). Example 5 (no policy inherits parent policy): `organizations/foo` has no `Policy` set. `projects/bar` has no `Policy` set. The accepted values at both levels are either all or none depending on the value of `constraint_default` (if `ALLOW`, all; if `DENY`, none). Example 6 (ListConstraint allowing all): `organizations/foo` has a `Policy` with values: {allowed_values: "E1" allowed_values: "E2"} `projects/bar` has a `Policy` with: {all: ALLOW} The accepted values at `organizations/foo` are `E1`, E2`. Any value is accepted at `projects/bar`. Example 7 (ListConstraint allowing none): `organizations/foo` has a `Policy` with values: {allowed_values: "E1" allowed_values: "E2"} `projects/bar` has a `Policy` with: {all: DENY} The accepted values at `organizations/foo` are `E1`, E2`. No value is accepted at `projects/bar`. Example 10 (allowed and denied subtrees of Resource Manager hierarchy): Given the following resource hierarchy O1->{F1, F2}; F1->{P1}; F2->{P2, P3}, `organizations/foo` has a `Policy` with values: {allowed_values: "under:organizations/O1"} `projects/bar` has a `Policy` with: {allowed_values: "under:projects/P3"} {denied_values: "under:folders/F2"} The accepted values at `organizations/foo` are `organizations/O1`, `folders/F1`, `folders/F2`, `projects/P1`, `projects/P2`, `projects/P3`. The accepted values at `projects/bar` are `organizations/O1`, `folders/F1`, `projects/P1`.
    #[serde(rename="inheritFromParent")]
    
    pub inherit_from_parent: Option<bool>,
    /// Optional. The Google Cloud Console will try to default to a configuration that matches the value specified in this `Policy`. If `suggested_value` is not set, it will inherit the value specified higher in the hierarchy, unless `inherit_from_parent` is `false`.
    #[serde(rename="suggestedValue")]
    
    pub suggested_value: Option<String>,
}

impl client::Part for ListPolicy {}


/// A page of the response received from the ListProjects method. A paginated response where more pages are available has `next_page_token` set. This token can be used in a subsequent request to retrieve the next request page.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list projects](ProjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListProjectsResponse {
    /// Pagination token. If the result set is too large to fit in a single response, this token is returned. It encodes the position of the current result cursor. Feeding this value into a new list request with the `page_token` parameter gives the next page of the results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set. Pagination tokens have a limited lifetime.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of Projects that matched the list filter. This list can be paginated.
    
    pub projects: Option<Vec<Project>>,
}

impl client::ResponseResult for ListProjectsResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get operations](OperationGetCall) (response)
/// * [create projects](ProjectCreateCall) (response)
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

impl client::Resource for Operation {}
impl client::ResponseResult for Operation {}


/// Defines a Cloud Organization `Policy` which is used to specify `Constraints` for configurations of Cloud Platform resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get effective org policy folders](FolderGetEffectiveOrgPolicyCall) (response)
/// * [get org policy folders](FolderGetOrgPolicyCall) (response)
/// * [set org policy folders](FolderSetOrgPolicyCall) (response)
/// * [get effective org policy organizations](OrganizationGetEffectiveOrgPolicyCall) (response)
/// * [get org policy organizations](OrganizationGetOrgPolicyCall) (response)
/// * [set org policy organizations](OrganizationSetOrgPolicyCall) (response)
/// * [get effective org policy projects](ProjectGetEffectiveOrgPolicyCall) (response)
/// * [get org policy projects](ProjectGetOrgPolicyCall) (response)
/// * [set org policy projects](ProjectSetOrgPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrgPolicy {
    /// For boolean `Constraints`, whether to enforce the `Constraint` or not.
    #[serde(rename="booleanPolicy")]
    
    pub boolean_policy: Option<BooleanPolicy>,
    /// The name of the `Constraint` the `Policy` is configuring, for example, `constraints/serviceuser.services`. A [list of available constraints](https://cloud.google.com/resource-manager/docs/organization-policy/org-policy-constraints) is available. Immutable after creation.
    
    pub constraint: Option<String>,
    /// An opaque tag indicating the current version of the `Policy`, used for concurrency control. When the `Policy` is returned from either a `GetPolicy` or a `ListOrgPolicy` request, this `etag` indicates the version of the current `Policy` to use when executing a read-modify-write loop. When the `Policy` is returned from a `GetEffectivePolicy` request, the `etag` will be unset. When the `Policy` is used in a `SetOrgPolicy` method, use the `etag` value that was returned from a `GetOrgPolicy` request as part of a read-modify-write loop for concurrency control. Not setting the `etag`in a `SetOrgPolicy` request will result in an unconditional write of the `Policy`.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// List of values either allowed or disallowed.
    #[serde(rename="listPolicy")]
    
    pub list_policy: Option<ListPolicy>,
    /// Restores the default behavior of the constraint; independent of `Constraint` type.
    #[serde(rename="restoreDefault")]
    
    pub restore_default: Option<RestoreDefault>,
    /// The time stamp the `Policy` was previously updated. This is set by the server, not specified by the caller, and represents the last time a call to `SetOrgPolicy` was made for that `Policy`. Any value set by the client will be ignored.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Version of the `Policy`. Default version is 0;
    
    pub version: Option<i32>,
}

impl client::ResponseResult for OrgPolicy {}


/// The root node in the resource hierarchy to which a particular entity’s (e.g., company) resources belong.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clear org policy organizations](OrganizationClearOrgPolicyCall) (none)
/// * [get organizations](OrganizationGetCall) (response)
/// * [get effective org policy organizations](OrganizationGetEffectiveOrgPolicyCall) (none)
/// * [get iam policy organizations](OrganizationGetIamPolicyCall) (none)
/// * [get org policy organizations](OrganizationGetOrgPolicyCall) (none)
/// * [list available org policy constraints organizations](OrganizationListAvailableOrgPolicyConstraintCall) (none)
/// * [list org policies organizations](OrganizationListOrgPolicyCall) (none)
/// * [search organizations](OrganizationSearchCall) (none)
/// * [set iam policy organizations](OrganizationSetIamPolicyCall) (none)
/// * [set org policy organizations](OrganizationSetOrgPolicyCall) (none)
/// * [test iam permissions organizations](OrganizationTestIamPermissionCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    /// Timestamp when the Organization was created. Assigned by the server.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A human-readable string that refers to the Organization in the GCP Console UI. This string is set by the server and cannot be changed. The string will be set to the primary domain (for example, "google.com") of the G Suite customer that owns the organization.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The organization's current lifecycle state. Assigned by the server.
    #[serde(rename="lifecycleState")]
    
    pub lifecycle_state: Option<OrganizationLifecycleStateEnum>,
    /// Output only. The resource name of the organization. This is the organization's relative path in the API. Its format is "organizations/[organization_id]". For example, "organizations/1234".
    
    pub name: Option<String>,
    /// The owner of this Organization. The owner should be specified on creation. Once set, it cannot be changed. This field is required.
    
    pub owner: Option<OrganizationOwner>,
}

impl client::Resource for Organization {}
impl client::ResponseResult for Organization {}


/// The entity that owns an Organization. The lifetime of the Organization and all of its descendants are bound to the `OrganizationOwner`. If the `OrganizationOwner` is deleted, the Organization and all its descendants will be deleted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationOwner {
    /// The G Suite customer id used in the Directory API.
    #[serde(rename="directoryCustomerId")]
    
    pub directory_customer_id: Option<String>,
}

impl client::Part for OrganizationOwner {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get iam policy organizations](OrganizationGetIamPolicyCall) (response)
/// * [set iam policy organizations](OrganizationSetIamPolicyCall) (response)
/// * [get iam policy projects](ProjectGetIamPolicyCall) (response)
/// * [set iam policy projects](ProjectSetIamPolicyCall) (response)
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


/// A Project is a high-level Google Cloud Platform entity. It is a container for ACLs, APIs, App Engine Apps, VMs, and other Google Cloud Platform resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clear org policy projects](ProjectClearOrgPolicyCall) (none)
/// * [create projects](ProjectCreateCall) (request)
/// * [delete projects](ProjectDeleteCall) (none)
/// * [get projects](ProjectGetCall) (response)
/// * [get ancestry projects](ProjectGetAncestryCall) (none)
/// * [get effective org policy projects](ProjectGetEffectiveOrgPolicyCall) (none)
/// * [get iam policy projects](ProjectGetIamPolicyCall) (none)
/// * [get org policy projects](ProjectGetOrgPolicyCall) (none)
/// * [list projects](ProjectListCall) (none)
/// * [list available org policy constraints projects](ProjectListAvailableOrgPolicyConstraintCall) (none)
/// * [list org policies projects](ProjectListOrgPolicyCall) (none)
/// * [set iam policy projects](ProjectSetIamPolicyCall) (none)
/// * [set org policy projects](ProjectSetOrgPolicyCall) (none)
/// * [test iam permissions projects](ProjectTestIamPermissionCall) (none)
/// * [undelete projects](ProjectUndeleteCall) (none)
/// * [update projects](ProjectUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    /// Creation time. Read-only.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The labels associated with this Project. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: a-z{0,62}. Label values must be between 0 and 63 characters long and must conform to the regular expression [a-z0-9_-]{0,63}. A label value can be empty. No more than 256 labels can be associated with a given resource. Clients should store labels in a representation such as JSON that does not depend on specific characters being disallowed. Example: "environment" : "dev" Read-write.
    
    pub labels: Option<HashMap<String, String>>,
    /// The Project lifecycle state. Read-only.
    #[serde(rename="lifecycleState")]
    
    pub lifecycle_state: Option<ProjectLifecycleStateEnum>,
    /// The optional user-assigned display name of the Project. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `My Project` Read-write.
    
    pub name: Option<String>,
    /// An optional reference to a parent Resource. Supported parent types include "organization" and "folder". Once set, the parent cannot be cleared. The `parent` can be set on creation or using the `UpdateProject` method; the end user must have the `resourcemanager.projects.create` permission on the parent.
    
    pub parent: Option<ResourceId>,
    /// The unique, user-assigned ID of the Project. It must be 6 to 30 lowercase letters, digits, or hyphens. It must start with a letter. Trailing hyphens are prohibited. Example: `tokyo-rain-123` Read-only after creation.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// The number uniquely identifying the project. Example: `415104041262` Read-only.
    #[serde(rename="projectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub project_number: Option<i64>,
}

impl client::RequestValue for Project {}
impl client::Resource for Project {}
impl client::ResponseResult for Project {}


/// A container to reference an id for any resource type. A `resource` in Google Cloud Platform is a generic term for something you (a developer) may want to interact with through one of our API's. Some examples are an App Engine app, a Compute Engine instance, a Cloud SQL database, and so on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceId {
    /// The type-specific id. This should correspond to the id used in the type-specific API's.
    
    pub id: Option<String>,
    /// The resource type this id is for. At present, the valid types are: "organization", "folder", and "project".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for ResourceId {}


/// Ignores policies set above this resource and restores the `constraint_default` enforcement behavior of the specific `Constraint` at this resource. Suppose that `constraint_default` is set to `ALLOW` for the `Constraint` `constraints/serviceuser.services`. Suppose that organization foo.com sets a `Policy` at their Organization resource node that restricts the allowed service activations to deny all service activations. They could then set a `Policy` with the `policy_type` `restore_default` on several experimental projects, restoring the `constraint_default` enforcement of the `Constraint` for only those projects, allowing those projects to have all services activated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestoreDefault { _never_set: Option<bool> }

impl client::Part for RestoreDefault {}


/// The request sent to the `SearchOrganizations` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search organizations](OrganizationSearchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchOrganizationsRequest {
    /// An optional query string used to filter the Organizations to return in the response. Filter rules are case-insensitive. Organizations may be filtered by `owner.directoryCustomerId` or by `domain`, where the domain is a G Suite domain, for example: * Filter `owner.directorycustomerid:123456789` returns Organization resources with `owner.directory_customer_id` equal to `123456789`. * Filter `domain:google.com` returns Organization resources corresponding to the domain `google.com`. This field is optional.
    
    pub filter: Option<String>,
    /// The maximum number of Organizations to return in the response. The server can return fewer organizations than requested. If unspecified, server picks an appropriate default.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// A pagination token returned from a previous call to `SearchOrganizations` that indicates from where listing should continue. This field is optional.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
}

impl client::RequestValue for SearchOrganizationsRequest {}


/// The response returned from the `SearchOrganizations` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search organizations](OrganizationSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchOrganizationsResponse {
    /// A pagination token to be used to retrieve the next page of results. If the result is too large to fit within the page size specified in the request, this field will be set with a token that can be used to fetch the next page of results. If this field is empty, it indicates that this response contains the last page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of Organizations that matched the search query, possibly paginated.
    
    pub organizations: Option<Vec<Organization>>,
}

impl client::ResponseResult for SearchOrganizationsResponse {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set iam policy organizations](OrganizationSetIamPolicyCall) (request)
/// * [set iam policy projects](ProjectSetIamPolicyCall) (request)
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


/// The request sent to the SetOrgPolicyRequest method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set org policy folders](FolderSetOrgPolicyCall) (request)
/// * [set org policy organizations](OrganizationSetOrgPolicyCall) (request)
/// * [set org policy projects](ProjectSetOrgPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetOrgPolicyRequest {
    /// `Policy` to set on the resource.
    
    pub policy: Option<OrgPolicy>,
}

impl client::RequestValue for SetOrgPolicyRequest {}


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
/// * [test iam permissions organizations](OrganizationTestIamPermissionCall) (request)
/// * [test iam permissions projects](ProjectTestIamPermissionCall) (request)
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
/// * [test iam permissions organizations](OrganizationTestIamPermissionCall) (response)
/// * [test iam permissions projects](ProjectTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// The request sent to the UndeleteProject method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [undelete projects](ProjectUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteProjectRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteProjectRequest {}


