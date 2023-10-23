use super::*;
/// Similar to PolicySpec but with an extra 'launch' field for launch reference. The PolicySpec here is specific for dry-run/darklaunch.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2AlternatePolicySpec {
    /// Reference to the launch that will be used while audit logging and to control the launch. Should be set only in the alternate policy.
    
    pub launch: Option<String>,
    /// Specify `Constraint` for configurations of Cloud Platform resources.
    
    pub spec: Option<GoogleCloudOrgpolicyV2PolicySpec>,
}

impl client::Part for GoogleCloudOrgpolicyV2AlternatePolicySpec {}


/// A `constraint` describes a way to restrict resource's configuration. For example, you could enforce a constraint that controls which cloud services can be activated across an organization, or whether a Compute Engine instance can have serial port connections established. `Constraints` can be configured by the organization's policy administrator to fit the needs of the organization by setting a `policy` that includes `constraints` at different locations in the organization's resource hierarchy. Policies are inherited down the resource hierarchy from higher levels, but can also be overridden. For details about the inheritance rules please read about `policies`. `Constraints` have a default behavior determined by the `constraint_default` field, which is the enforcement behavior that is used in the absence of a `policy` being defined or inherited for the resource in question.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2Constraint {
    /// Defines this constraint as being a BooleanConstraint.
    #[serde(rename="booleanConstraint")]
    
    pub boolean_constraint: Option<GoogleCloudOrgpolicyV2ConstraintBooleanConstraint>,
    /// The evaluation behavior of this constraint in the absence of 'Policy'.
    #[serde(rename="constraintDefault")]
    
    pub constraint_default: Option<GoogleCloudOrgpolicyV2ConstraintConstraintDefaultEnum>,
    /// Detailed description of what this `Constraint` controls as well as how and where it is enforced. Mutable.
    
    pub description: Option<String>,
    /// The human readable name. Mutable.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Defines this constraint as being a ListConstraint.
    #[serde(rename="listConstraint")]
    
    pub list_constraint: Option<GoogleCloudOrgpolicyV2ConstraintListConstraint>,
    /// Immutable. The resource name of the Constraint. Must be in one of the following forms: * `projects/{project_number}/constraints/{constraint_name}` * `folders/{folder_id}/constraints/{constraint_name}` * `organizations/{organization_id}/constraints/{constraint_name}` For example, "/projects/123/constraints/compute.disableSerialPortAccess".
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudOrgpolicyV2Constraint {}


/// A `Constraint` that is either enforced or not. For example a constraint `constraints/compute.disableSerialPortAccess`. If it is enforced on a VM instance, serial port connections will not be opened to that instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2ConstraintBooleanConstraint { _never_set: Option<bool> }

impl client::Part for GoogleCloudOrgpolicyV2ConstraintBooleanConstraint {}


/// A `Constraint` that allows or disallows a list of string values, which are configured by an Organization's policy administrator with a `Policy`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2ConstraintListConstraint {
    /// Indicates whether values grouped into categories can be used in `Policy.allowed_values` and `Policy.denied_values`. For example, `"in:Python"` would match any value in the 'Python' group.
    #[serde(rename="supportsIn")]
    
    pub supports_in: Option<bool>,
    /// Indicates whether subtrees of Cloud Resource Manager resource hierarchy can be used in `Policy.allowed_values` and `Policy.denied_values`. For example, `"under:folders/123"` would match any resource under the 'folders/123' folder.
    #[serde(rename="supportsUnder")]
    
    pub supports_under: Option<bool>,
}

impl client::Part for GoogleCloudOrgpolicyV2ConstraintListConstraint {}


/// A custom constraint defined by customers which can *only* be applied to the given resource types and organization. By creating a custom constraint, customers can applied policies of this custom constraint. *Creating a custom constraint itself does NOT apply any policy enforcement*.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custom constraints create organizations](OrganizationCustomConstraintCreateCall) (request|response)
/// * [custom constraints get organizations](OrganizationCustomConstraintGetCall) (response)
/// * [custom constraints patch organizations](OrganizationCustomConstraintPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2CustomConstraint {
    /// Allow or deny type.
    #[serde(rename="actionType")]
    
    pub action_type: Option<GoogleCloudOrgpolicyV2CustomConstraintActionTypeEnum>,
    /// Org policy condition/expression. For example: `resource.instanceName.matches("[production|test]_.*_(\d)+")'` or, `resource.management.auto_upgrade == true` The max length of the condition is 1000 characters.
    
    pub condition: Option<String>,
    /// Detailed information about this custom policy constraint. The max length of the description is 2000 characters.
    
    pub description: Option<String>,
    /// One line display name for the UI. The max length of the display_name is 200 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// All the operations being applied for this constraint.
    #[serde(rename="methodTypes")]
    
    pub method_types: Option<Vec<GoogleCloudOrgpolicyV2CustomConstraintMethodTypesEnum>>,
    /// Immutable. Name of the constraint. This is unique within the organization. Format of the name should be * `organizations/{organization_id}/customConstraints/{custom_constraint_id}` Example : "organizations/123/customConstraints/custom.createOnlyE2TypeVms" The max length is 70 characters and the min length is 1. Note that the prefix "organizations/{organization_id}/customConstraints/" is not counted.
    
    pub name: Option<String>,
    /// Immutable. The Resource Instance type on which this policy applies to. Format will be of the form : "/" Example: * `compute.googleapis.com/Instance`.
    #[serde(rename="resourceTypes")]
    
    pub resource_types: Option<Vec<String>>,
    /// Output only. The last time this custom constraint was updated. This represents the last time that the `CreateCustomConstraint` or `UpdateCustomConstraint` RPC was called
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudOrgpolicyV2CustomConstraint {}
impl client::ResponseResult for GoogleCloudOrgpolicyV2CustomConstraint {}


/// The response returned from the ListConstraints method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [constraints list folders](FolderConstraintListCall) (response)
/// * [constraints list organizations](OrganizationConstraintListCall) (response)
/// * [constraints list projects](ProjectConstraintListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2ListConstraintsResponse {
    /// The collection of constraints that are available on the targeted resource.
    
    pub constraints: Option<Vec<GoogleCloudOrgpolicyV2Constraint>>,
    /// Page token used to retrieve the next page. This is currently not used.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudOrgpolicyV2ListConstraintsResponse {}


/// The response returned from the ListCustomConstraints method. It will be empty if no `CustomConstraints` are set on the organization resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custom constraints list organizations](OrganizationCustomConstraintListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2ListCustomConstraintsResponse {
    /// All `CustomConstraints` that exist on the organization resource. It will be empty if no `CustomConstraints` are set.
    #[serde(rename="customConstraints")]
    
    pub custom_constraints: Option<Vec<GoogleCloudOrgpolicyV2CustomConstraint>>,
    /// Page token used to retrieve the next page. This is currently not used, but the server may at any point start supplying a valid token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudOrgpolicyV2ListCustomConstraintsResponse {}


/// The response returned from the ListPolicies method. It will be empty if no `Policies` are set on the resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies list folders](FolderPolicyListCall) (response)
/// * [policies list organizations](OrganizationPolicyListCall) (response)
/// * [policies list projects](ProjectPolicyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2ListPoliciesResponse {
    /// Page token used to retrieve the next page. This is currently not used, but the server may at any point start supplying a valid token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// All `Policies` that exist on the resource. It will be empty if no `Policies` are set.
    
    pub policies: Option<Vec<GoogleCloudOrgpolicyV2Policy>>,
}

impl client::ResponseResult for GoogleCloudOrgpolicyV2ListPoliciesResponse {}


/// Defines a Cloud Organization `Policy` which is used to specify `Constraints` for configurations of Cloud Platform resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies create folders](FolderPolicyCreateCall) (request|response)
/// * [policies get folders](FolderPolicyGetCall) (response)
/// * [policies get effective policy folders](FolderPolicyGetEffectivePolicyCall) (response)
/// * [policies patch folders](FolderPolicyPatchCall) (request|response)
/// * [policies create organizations](OrganizationPolicyCreateCall) (request|response)
/// * [policies get organizations](OrganizationPolicyGetCall) (response)
/// * [policies get effective policy organizations](OrganizationPolicyGetEffectivePolicyCall) (response)
/// * [policies patch organizations](OrganizationPolicyPatchCall) (request|response)
/// * [policies create projects](ProjectPolicyCreateCall) (request|response)
/// * [policies get projects](ProjectPolicyGetCall) (response)
/// * [policies get effective policy projects](ProjectPolicyGetEffectivePolicyCall) (response)
/// * [policies patch projects](ProjectPolicyPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2Policy {
    /// Deprecated.
    
    pub alternate: Option<GoogleCloudOrgpolicyV2AlternatePolicySpec>,
    /// dry-run policy. Audit-only policy, can be used to monitor how the policy would have impacted the existing and future resources if it's enforced.
    #[serde(rename="dryRunSpec")]
    
    pub dry_run_spec: Option<GoogleCloudOrgpolicyV2PolicySpec>,
    /// Immutable. The resource name of the Policy. Must be one of the following forms, where constraint_name is the name of the constraint which this Policy configures: * `projects/{project_number}/policies/{constraint_name}` * `folders/{folder_id}/policies/{constraint_name}` * `organizations/{organization_id}/policies/{constraint_name}` For example, "projects/123/policies/compute.disableSerialPortAccess". Note: `projects/{project_id}/policies/{constraint_name}` is also an acceptable name for API requests, but responses will return the name using the equivalent project number.
    
    pub name: Option<String>,
    /// Basic information about the Organization Policy.
    
    pub spec: Option<GoogleCloudOrgpolicyV2PolicySpec>,
}

impl client::RequestValue for GoogleCloudOrgpolicyV2Policy {}
impl client::ResponseResult for GoogleCloudOrgpolicyV2Policy {}


/// Defines a Cloud Organization `PolicySpec` which is used to specify `Constraints` for configurations of Cloud Platform resources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2PolicySpec {
    /// An opaque tag indicating the current version of the `Policy`, used for concurrency control. This field is ignored if used in a `CreatePolicy` request. When the `Policy` is returned from either a `GetPolicy` or a `ListPolicies` request, this `etag` indicates the version of the current `Policy` to use when executing a read-modify-write loop. When the `Policy` is returned from a `GetEffectivePolicy` request, the `etag` will be unset.
    
    pub etag: Option<String>,
    /// Determines the inheritance behavior for this `Policy`. If `inherit_from_parent` is true, PolicyRules set higher up in the hierarchy (up to the closest root) are inherited and present in the effective policy. If it is false, then no rules are inherited, and this Policy becomes the new root for evaluation. This field can be set only for Policies which configure list constraints.
    #[serde(rename="inheritFromParent")]
    
    pub inherit_from_parent: Option<bool>,
    /// Ignores policies set above this resource and restores the `constraint_default` enforcement behavior of the specific `Constraint` at this resource. This field can be set in policies for either list or boolean constraints. If set, `rules` must be empty and `inherit_from_parent` must be set to false.
    
    pub reset: Option<bool>,
    /// Up to 10 PolicyRules are allowed. In Policies for boolean constraints, the following requirements apply: - There must be one and only one PolicyRule where condition is unset. - BooleanPolicyRules with conditions must set `enforced` to the opposite of the PolicyRule without a condition. - During policy evaluation, PolicyRules with conditions that are true for a target resource take precedence.
    
    pub rules: Option<Vec<GoogleCloudOrgpolicyV2PolicySpecPolicyRule>>,
    /// Output only. The time stamp this was previously updated. This represents the last time a call to `CreatePolicy` or `UpdatePolicy` was made for that `Policy`.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudOrgpolicyV2PolicySpec {}


/// A rule used to express this policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2PolicySpecPolicyRule {
    /// Setting this to true means that all values are allowed. This field can be set only in Policies for list constraints.
    #[serde(rename="allowAll")]
    
    pub allow_all: Option<bool>,
    /// A condition which determines whether this rule is used in the evaluation of the policy. When set, the `expression` field in the `Expr' must include from 1 to 10 subexpressions, joined by the "||" or "&&" operators. Each subexpression must be of the form "resource.matchTag('/tag_key_short_name, 'tag_value_short_name')". or "resource.matchTagId('tagKeys/key_id', 'tagValues/value_id')". where key_name and value_name are the resource names for Label Keys and Values. These names are available from the Tag Manager Service. An example expression is: "resource.matchTag('123456789/environment, 'prod')". or "resource.matchTagId('tagKeys/123', 'tagValues/456')".
    
    pub condition: Option<GoogleTypeExpr>,
    /// Setting this to true means that all values are denied. This field can be set only in Policies for list constraints.
    #[serde(rename="denyAll")]
    
    pub deny_all: Option<bool>,
    /// If `true`, then the `Policy` is enforced. If `false`, then any configuration is acceptable. This field can be set only in Policies for boolean constraints.
    
    pub enforce: Option<bool>,
    /// List of values to be used for this PolicyRule. This field can be set only in Policies for list constraints.
    
    pub values: Option<GoogleCloudOrgpolicyV2PolicySpecPolicyRuleStringValues>,
}

impl client::Part for GoogleCloudOrgpolicyV2PolicySpecPolicyRule {}


/// A message that holds specific allowed and denied values. This message can define specific values and subtrees of Cloud Resource Manager resource hierarchy (`Organizations`, `Folders`, `Projects`) that are allowed or denied. This is achieved by using the `under:` and optional `is:` prefixes. The `under:` prefix is used to denote resource subtree values. The `is:` prefix is used to denote specific values, and is required only if the value contains a ":". Values prefixed with "is:" are treated the same as values with no prefix. Ancestry subtrees must be in one of the following formats: - "projects/", e.g. "projects/tokyo-rain-123" - "folders/", e.g. "folders/1234" - "organizations/", e.g. "organizations/1234" The `supports_under` field of the associated `Constraint` defines whether ancestry prefixes can be used.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2PolicySpecPolicyRuleStringValues {
    /// List of values allowed at this resource.
    #[serde(rename="allowedValues")]
    
    pub allowed_values: Option<Vec<String>>,
    /// List of values denied at this resource.
    #[serde(rename="deniedValues")]
    
    pub denied_values: Option<Vec<String>>,
}

impl client::Part for GoogleCloudOrgpolicyV2PolicySpecPolicyRuleStringValues {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies delete folders](FolderPolicyDeleteCall) (response)
/// * [custom constraints delete organizations](OrganizationCustomConstraintDeleteCall) (response)
/// * [policies delete organizations](OrganizationPolicyDeleteCall) (response)
/// * [policies delete projects](ProjectPolicyDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeExpr {
    /// Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax.
    
    pub expression: Option<String>,
    /// Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for GoogleTypeExpr {}


