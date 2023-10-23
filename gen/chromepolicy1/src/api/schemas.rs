use super::*;
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChromeCrosDpanelAutosettingsProtoPolicyApiLifecycle {
    /// Description about current life cycle.
    
    pub description: Option<String>,
    /// End supporting date for current policy.
    #[serde(rename="endSupport")]
    
    pub end_support: Option<GoogleTypeDate>,
    /// Indicate current life cycle stage of the policy API.
    #[serde(rename="policyApiLifecycleStage")]
    
    pub policy_api_lifecycle_stage: Option<ChromeCrosDpanelAutosettingsProtoPolicyApiLifecyclePolicyApiLifecycleStageEnum>,
}

impl client::Part for ChromeCrosDpanelAutosettingsProtoPolicyApiLifecycle {}


/// Additional key names that will be used to identify the target of the policy value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1AdditionalTargetKeyName {
    /// Key name.
    
    pub key: Option<String>,
    /// Key description.
    #[serde(rename="keyDescription")]
    
    pub key_description: Option<String>,
}

impl client::Part for GoogleChromePolicyVersionsV1AdditionalTargetKeyName {}


/// Request message for specifying that multiple policy values will be deleted.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies groups batch delete customers](CustomerPolicyGroupBatchDeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1BatchDeleteGroupPoliciesRequest {
    /// List of policies that will be deleted as defined by the `requests`. All requests in the list must follow these restrictions: 1. All schemas in the list must have the same root namespace. 2. All `policyTargetKey.targetResource` values must point to a group resource. 3. All `policyTargetKey` values must have the same `app_id` key name in the `additionalTargetKeys`. 4. No two modification requests can reference the same `policySchema` + ` policyTargetKey` pair. 
    
    pub requests: Option<Vec<GoogleChromePolicyVersionsV1DeleteGroupPolicyRequest>>,
}

impl client::RequestValue for GoogleChromePolicyVersionsV1BatchDeleteGroupPoliciesRequest {}


/// Request message for specifying that multiple policy values inherit their value from their parents.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies orgunits batch inherit customers](CustomerPolicyOrgunitBatchInheritCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1BatchInheritOrgUnitPoliciesRequest {
    /// List of policies that have to inherit their values as defined by the `requests`. All requests in the list must follow these restrictions: 1. All schemas in the list must have the same root namespace. 2. All `policyTargetKey.targetResource` values must point to an org unit resource. 3. All `policyTargetKey` values must have the same key names in the ` additionalTargetKeys`. This also means if one of the targets has an empty `additionalTargetKeys` map, all of the targets must have an empty `additionalTargetKeys` map. 4. No two modification requests can reference the same `policySchema` + ` policyTargetKey` pair. 
    
    pub requests: Option<Vec<GoogleChromePolicyVersionsV1InheritOrgUnitPolicyRequest>>,
}

impl client::RequestValue for GoogleChromePolicyVersionsV1BatchInheritOrgUnitPoliciesRequest {}


/// Request message for modifying multiple policy values for a specific group-based target.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies groups batch modify customers](CustomerPolicyGroupBatchModifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1BatchModifyGroupPoliciesRequest {
    /// List of policies to modify as defined by the `requests`. All requests in the list must follow these restrictions: 1. All schemas in the list must have the same root namespace. 2. All `policyTargetKey.targetResource` values must point to a group resource. 3. All `policyTargetKey` values must have the same `app_id` key name in the `additionalTargetKeys`. 4. No two modification requests can reference the same `policySchema` + ` policyTargetKey` pair. 
    
    pub requests: Option<Vec<GoogleChromePolicyVersionsV1ModifyGroupPolicyRequest>>,
}

impl client::RequestValue for GoogleChromePolicyVersionsV1BatchModifyGroupPoliciesRequest {}


/// Request message for modifying multiple policy values for a specific target.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies orgunits batch modify customers](CustomerPolicyOrgunitBatchModifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1BatchModifyOrgUnitPoliciesRequest {
    /// List of policies to modify as defined by the `requests`. All requests in the list must follow these restrictions: 1. All schemas in the list must have the same root namespace. 2. All `policyTargetKey.targetResource` values must point to an org unit resource. 3. All `policyTargetKey` values must have the same key names in the ` additionalTargetKeys`. This also means if one of the targets has an empty `additionalTargetKeys` map, all of the targets must have an empty `additionalTargetKeys` map. 4. No two modification requests can reference the same `policySchema` + ` policyTargetKey` pair. 
    
    pub requests: Option<Vec<GoogleChromePolicyVersionsV1ModifyOrgUnitPolicyRequest>>,
}

impl client::RequestValue for GoogleChromePolicyVersionsV1BatchModifyOrgUnitPoliciesRequest {}


/// Request object for creating a certificate.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies networks define certificate customers](CustomerPolicyNetworkDefineCertificateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1DefineCertificateRequest {
    /// Optional. The optional name of the certificate. If not specified, the certificate issuer will be used as the name.
    #[serde(rename="ceritificateName")]
    
    pub ceritificate_name: Option<String>,
    /// Required. The raw contents of the .PEM, .CRT, or .CER file.
    
    pub certificate: Option<String>,
    /// Optional. Certificate settings within the chrome.networks.certificates namespace.
    
    pub settings: Option<Vec<GoogleChromePolicyVersionsV1NetworkSetting>>,
    /// Required. The target resource on which this certificate is applied. The following resources are supported: * Organizational Unit ("orgunits/{orgunit_id}")
    #[serde(rename="targetResource")]
    
    pub target_resource: Option<String>,
}

impl client::RequestValue for GoogleChromePolicyVersionsV1DefineCertificateRequest {}


/// Response object for creating a certificate.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies networks define certificate customers](CustomerPolicyNetworkDefineCertificateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1DefineCertificateResponse {
    /// The guid of the certificate created by the action.
    #[serde(rename="networkId")]
    
    pub network_id: Option<String>,
    /// the affiliated settings of the certificate (NOT IMPLEMENTED)
    
    pub settings: Option<Vec<GoogleChromePolicyVersionsV1NetworkSetting>>,
    /// the resource at which the certificate is defined.
    #[serde(rename="targetResource")]
    
    pub target_resource: Option<String>,
}

impl client::ResponseResult for GoogleChromePolicyVersionsV1DefineCertificateResponse {}


/// Request object for creating a new network.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies networks define network customers](CustomerPolicyNetworkDefineNetworkCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1DefineNetworkRequest {
    /// Required. Name of the new created network.
    
    pub name: Option<String>,
    /// Required. Detailed network settings.
    
    pub settings: Option<Vec<GoogleChromePolicyVersionsV1NetworkSetting>>,
    /// Required. The target resource on which this new network will be defined. The following resources are supported: * Organizational Unit ("orgunits/{orgunit_id}")
    #[serde(rename="targetResource")]
    
    pub target_resource: Option<String>,
}

impl client::RequestValue for GoogleChromePolicyVersionsV1DefineNetworkRequest {}


/// Response object for creating a network.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies networks define network customers](CustomerPolicyNetworkDefineNetworkCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1DefineNetworkResponse {
    /// Network ID of the new created network.
    #[serde(rename="networkId")]
    
    pub network_id: Option<String>,
    /// Detailed network settings of the new created network
    
    pub settings: Option<Vec<GoogleChromePolicyVersionsV1NetworkSetting>>,
    /// The target resource on which this new network will be defined. The following resources are supported: * Organizational Unit ("orgunits/{orgunit_id}")
    #[serde(rename="targetResource")]
    
    pub target_resource: Option<String>,
}

impl client::ResponseResult for GoogleChromePolicyVersionsV1DefineNetworkResponse {}


/// Request parameters for deleting the policy value of a specific group target.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1DeleteGroupPolicyRequest {
    /// The fully qualified name of the policy schema that is being inherited.
    #[serde(rename="policySchema")]
    
    pub policy_schema: Option<String>,
    /// Required. The key of the target for which we want to modify a policy. The target resource must point to a Group.
    #[serde(rename="policyTargetKey")]
    
    pub policy_target_key: Option<GoogleChromePolicyVersionsV1PolicyTargetKey>,
}

impl client::Part for GoogleChromePolicyVersionsV1DeleteGroupPolicyRequest {}


/// Information about any range constraints.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1FieldConstraints {
    /// The allowed range for numeric fields.
    #[serde(rename="numericRangeConstraint")]
    
    pub numeric_range_constraint: Option<GoogleChromePolicyVersionsV1NumericRangeConstraint>,
}

impl client::Part for GoogleChromePolicyVersionsV1FieldConstraints {}


/// Request parameters for inheriting policy value of a specific org unit target from the policy value of its parent org unit.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1InheritOrgUnitPolicyRequest {
    /// The fully qualified name of the policy schema that is being inherited.
    #[serde(rename="policySchema")]
    
    pub policy_schema: Option<String>,
    /// Required. The key of the target for which we want to modify a policy. The target resource must point to an Org Unit.
    #[serde(rename="policyTargetKey")]
    
    pub policy_target_key: Option<GoogleChromePolicyVersionsV1PolicyTargetKey>,
}

impl client::Part for GoogleChromePolicyVersionsV1InheritOrgUnitPolicyRequest {}


/// Request message for listing the group priority ordering of an app.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies groups list group priority ordering customers](CustomerPolicyGroupListGroupPriorityOrderingCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1ListGroupPriorityOrderingRequest {
    /// Required. The namespace of the policy type for the request.
    #[serde(rename="policyNamespace")]
    
    pub policy_namespace: Option<String>,
    /// Required. The key of the target for which we want to retrieve the group priority ordering. The target resource must point to an app.
    #[serde(rename="policyTargetKey")]
    
    pub policy_target_key: Option<GoogleChromePolicyVersionsV1PolicyTargetKey>,
}

impl client::RequestValue for GoogleChromePolicyVersionsV1ListGroupPriorityOrderingRequest {}


/// Response message for listing the group priority ordering of an app.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies groups list group priority ordering customers](CustomerPolicyGroupListGroupPriorityOrderingCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1ListGroupPriorityOrderingResponse {
    /// Output only. The group IDs, in priority ordering.
    #[serde(rename="groupIds")]
    
    pub group_ids: Option<Vec<String>>,
    /// Output only. The namespace of the policy type of the group IDs.
    #[serde(rename="policyNamespace")]
    
    pub policy_namespace: Option<String>,
    /// Output only. The target resource for which the group priority ordering has been retrieved.
    #[serde(rename="policyTargetKey")]
    
    pub policy_target_key: Option<GoogleChromePolicyVersionsV1PolicyTargetKey>,
}

impl client::ResponseResult for GoogleChromePolicyVersionsV1ListGroupPriorityOrderingResponse {}


/// Response message for listing policy schemas that match a filter.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policy schemas list customers](CustomerPolicySchemaListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1ListPolicySchemasResponse {
    /// The page token used to get the next page of policy schemas.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of policy schemas that match the query.
    #[serde(rename="policySchemas")]
    
    pub policy_schemas: Option<Vec<GoogleChromePolicyVersionsV1PolicySchema>>,
}

impl client::ResponseResult for GoogleChromePolicyVersionsV1ListPolicySchemasResponse {}


/// Request parameters for modifying a policy value for a specific group target.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1ModifyGroupPolicyRequest {
    /// Required. The key of the target for which we want to modify a policy. The target resource must point to a Group.
    #[serde(rename="policyTargetKey")]
    
    pub policy_target_key: Option<GoogleChromePolicyVersionsV1PolicyTargetKey>,
    /// The new value for the policy.
    #[serde(rename="policyValue")]
    
    pub policy_value: Option<GoogleChromePolicyVersionsV1PolicyValue>,
    /// Required. Policy fields to update. Only fields in this mask will be updated; other fields in `policy_value` will be ignored (even if they have values). If a field is in this list it must have a value in 'policy_value'.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::Part for GoogleChromePolicyVersionsV1ModifyGroupPolicyRequest {}


/// Request parameters for modifying a policy value for a specific org unit target.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1ModifyOrgUnitPolicyRequest {
    /// Required. The key of the target for which we want to modify a policy. The target resource must point to an Org Unit.
    #[serde(rename="policyTargetKey")]
    
    pub policy_target_key: Option<GoogleChromePolicyVersionsV1PolicyTargetKey>,
    /// The new value for the policy.
    #[serde(rename="policyValue")]
    
    pub policy_value: Option<GoogleChromePolicyVersionsV1PolicyValue>,
    /// Required. Policy fields to update. Only fields in this mask will be updated; other fields in `policy_value` will be ignored (even if they have values). If a field is in this list it must have a value in 'policy_value'.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::Part for GoogleChromePolicyVersionsV1ModifyOrgUnitPolicyRequest {}


/// A network setting contains network configurations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1NetworkSetting {
    /// The fully qualified name of the network setting.
    #[serde(rename="policySchema")]
    
    pub policy_schema: Option<String>,
    /// The value of the network setting.
    
    pub value: Option<HashMap<String, json::Value>>,
}

impl client::Part for GoogleChromePolicyVersionsV1NetworkSetting {}


/// A constraint on upper and/or lower bounds, with at least one being set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1NumericRangeConstraint {
    /// Maximum value.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub maximum: Option<i64>,
    /// Minimum value.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub minimum: Option<i64>,
}

impl client::Part for GoogleChromePolicyVersionsV1NumericRangeConstraint {}


/// Resource representing a policy schema.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policy schemas get customers](CustomerPolicySchemaGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1PolicySchema {
    /// Output only. Specific access restrictions related to this policy.
    #[serde(rename="accessRestrictions")]
    
    pub access_restrictions: Option<Vec<String>>,
    /// Output only. Additional key names that will be used to identify the target of the policy value. When specifying a `policyTargetKey`, each of the additional keys specified here will have to be included in the `additionalTargetKeys` map.
    #[serde(rename="additionalTargetKeyNames")]
    
    pub additional_target_key_names: Option<Vec<GoogleChromePolicyVersionsV1AdditionalTargetKeyName>>,
    /// Title of the category in which a setting belongs.
    #[serde(rename="categoryTitle")]
    
    pub category_title: Option<String>,
    /// Schema definition using proto descriptor.
    
    pub definition: Option<Proto2FileDescriptorProto>,
    /// Output only. Detailed description of each field that is part of the schema.
    #[serde(rename="fieldDescriptions")]
    
    pub field_descriptions: Option<Vec<GoogleChromePolicyVersionsV1PolicySchemaFieldDescription>>,
    /// Format: name=customers/{customer}/policySchemas/{schema_namespace}
    
    pub name: Option<String>,
    /// Output only. Special notice messages related to setting certain values in certain fields in the schema.
    
    pub notices: Option<Vec<GoogleChromePolicyVersionsV1PolicySchemaNoticeDescription>>,
    /// Output only. Current lifecycle information.
    #[serde(rename="policyApiLifecycle")]
    
    pub policy_api_lifecycle: Option<ChromeCrosDpanelAutosettingsProtoPolicyApiLifecycle>,
    /// Deprecated field because of typo.
    #[serde(rename="policyApiLifeycle")]
    
    pub policy_api_lifeycle: Option<ChromeCrosDpanelAutosettingsProtoPolicyApiLifecycle>,
    /// Output only. Description about the policy schema for user consumption.
    #[serde(rename="policyDescription")]
    
    pub policy_description: Option<String>,
    /// Output only. The fully qualified name of the policy schema. This value is used to fill the field `policy_schema` in PolicyValue when calling BatchInheritOrgUnitPolicies BatchModifyOrgUnitPolicies BatchModifyGroupPolicies or BatchDeleteGroupPolicies.
    #[serde(rename="schemaName")]
    
    pub schema_name: Option<String>,
    /// Output only. URI to related support article for this schema.
    #[serde(rename="supportUri")]
    
    pub support_uri: Option<String>,
    /// Output only. Information about applicable target resources for the policy.
    #[serde(rename="validTargetResources")]
    
    pub valid_target_resources: Option<Vec<GoogleChromePolicyVersionsV1PolicySchemaValidTargetResourcesEnum>>,
}

impl client::ResponseResult for GoogleChromePolicyVersionsV1PolicySchema {}


/// The field and the value it must have for another field to be allowed to be set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1PolicySchemaFieldDependencies {
    /// The source field which this field depends on.
    #[serde(rename="sourceField")]
    
    pub source_field: Option<String>,
    /// The value which the source field must have for this field to be allowed to be set.
    #[serde(rename="sourceFieldValue")]
    
    pub source_field_value: Option<String>,
}

impl client::Part for GoogleChromePolicyVersionsV1PolicySchemaFieldDependencies {}


/// Provides detailed information for a particular field that is part of a PolicySchema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1PolicySchemaFieldDescription {
    /// Output only. Client default if the policy is unset.
    #[serde(rename="defaultValue")]
    
    pub default_value: Option<json::Value>,
    /// Deprecated. Use name and field_description instead. The description for the field.
    
    pub description: Option<String>,
    /// Output only. The name of the field for associated with this description.
    
    pub field: Option<String>,
    /// Output only. Information on any input constraints associated on the values for the field.
    #[serde(rename="fieldConstraints")]
    
    pub field_constraints: Option<GoogleChromePolicyVersionsV1FieldConstraints>,
    /// Output only. Provides a list of fields and values. At least one of the fields must have the corresponding value in order for this field to be allowed to be set.
    #[serde(rename="fieldDependencies")]
    
    pub field_dependencies: Option<Vec<GoogleChromePolicyVersionsV1PolicySchemaFieldDependencies>>,
    /// Output only. The description of the field.
    #[serde(rename="fieldDescription")]
    
    pub field_description: Option<String>,
    /// Output only. Any input constraints associated on the values for the field.
    #[serde(rename="inputConstraint")]
    
    pub input_constraint: Option<String>,
    /// Output only. If the field has a set of known values, this field will provide a description for these values.
    #[serde(rename="knownValueDescriptions")]
    
    pub known_value_descriptions: Option<Vec<GoogleChromePolicyVersionsV1PolicySchemaFieldKnownValueDescription>>,
    /// Output only. The name of the field.
    
    pub name: Option<String>,
    /// Output only. Provides the description of the fields nested in this field, if the field is a message type that defines multiple fields.
    #[serde(rename="nestedFieldDescriptions")]
    
    pub nested_field_descriptions: Option<Vec<GoogleChromePolicyVersionsV1PolicySchemaFieldDescription>>,
    /// Output only. Provides a list of fields that are required to be set if this field has a certain value.
    #[serde(rename="requiredItems")]
    
    pub required_items: Option<Vec<GoogleChromePolicyVersionsV1PolicySchemaRequiredItems>>,
}

impl client::Part for GoogleChromePolicyVersionsV1PolicySchemaFieldDescription {}


/// Provides detailed information about a known value that is allowed for a particular field in a PolicySchema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1PolicySchemaFieldKnownValueDescription {
    /// Output only. Additional description for this value.
    
    pub description: Option<String>,
    /// Output only. The string represenstation of the value that can be set for the field.
    
    pub value: Option<String>,
}

impl client::Part for GoogleChromePolicyVersionsV1PolicySchemaFieldKnownValueDescription {}


/// Provides special notice messages related to a particular value in a field that is part of a PolicySchema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1PolicySchemaNoticeDescription {
    /// Output only. Whether the user needs to acknowledge the notice message before the value can be set.
    #[serde(rename="acknowledgementRequired")]
    
    pub acknowledgement_required: Option<bool>,
    /// Output only. The field name associated with the notice.
    
    pub field: Option<String>,
    /// Output only. The notice message associate with the value of the field.
    #[serde(rename="noticeMessage")]
    
    pub notice_message: Option<String>,
    /// Output only. The value of the field that has a notice. When setting the field to this value, the user may be required to acknowledge the notice message in order for the value to be set.
    #[serde(rename="noticeValue")]
    
    pub notice_value: Option<String>,
}

impl client::Part for GoogleChromePolicyVersionsV1PolicySchemaNoticeDescription {}


/// The fields that will become required based on the value of this field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1PolicySchemaRequiredItems {
    /// The value(s) of the field that provoke required field enforcement. An empty field_conditions implies that any value assigned to this field will provoke required field enforcement.
    #[serde(rename="fieldConditions")]
    
    pub field_conditions: Option<Vec<String>>,
    /// The fields that are required as a consequence of the field conditions.
    #[serde(rename="requiredFields")]
    
    pub required_fields: Option<Vec<String>>,
}

impl client::Part for GoogleChromePolicyVersionsV1PolicySchemaRequiredItems {}


/// The key used to identify the target on which the policy will be applied.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1PolicyTargetKey {
    /// Map containing the additional target key name and value pairs used to further identify the target of the policy.
    #[serde(rename="additionalTargetKeys")]
    
    pub additional_target_keys: Option<HashMap<String, String>>,
    /// The target resource on which this policy is applied. The following resources are supported: * Organizational Unit ("orgunits/{orgunit_id}") * Group ("groups/{group_id}")
    #[serde(rename="targetResource")]
    
    pub target_resource: Option<String>,
}

impl client::Part for GoogleChromePolicyVersionsV1PolicyTargetKey {}


/// A particular value for a policy managed by the service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1PolicyValue {
    /// The fully qualified name of the policy schema associated with this policy.
    #[serde(rename="policySchema")]
    
    pub policy_schema: Option<String>,
    /// The value of the policy that is compatible with the schema that it is associated with.
    
    pub value: Option<HashMap<String, json::Value>>,
}

impl client::Part for GoogleChromePolicyVersionsV1PolicyValue {}


/// Request object for removing a certificate.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies networks remove certificate customers](CustomerPolicyNetworkRemoveCertificateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1RemoveCertificateRequest {
    /// Required. The GUID of the certificate to remove.
    #[serde(rename="networkId")]
    
    pub network_id: Option<String>,
    /// Required. The target resource on which this certificate will be removed. The following resources are supported: * Organizational Unit ("orgunits/{orgunit_id}")
    #[serde(rename="targetResource")]
    
    pub target_resource: Option<String>,
}

impl client::RequestValue for GoogleChromePolicyVersionsV1RemoveCertificateRequest {}


/// Response object for removing a certificate.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies networks remove certificate customers](CustomerPolicyNetworkRemoveCertificateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1RemoveCertificateResponse { _never_set: Option<bool> }

impl client::ResponseResult for GoogleChromePolicyVersionsV1RemoveCertificateResponse {}


/// Request object for removing a network
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies networks remove network customers](CustomerPolicyNetworkRemoveNetworkCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1RemoveNetworkRequest {
    /// Required. The GUID of the network to remove.
    #[serde(rename="networkId")]
    
    pub network_id: Option<String>,
    /// Required. The target resource on which this network will be removed. The following resources are supported: * Organizational Unit ("orgunits/{orgunit_id}")
    #[serde(rename="targetResource")]
    
    pub target_resource: Option<String>,
}

impl client::RequestValue for GoogleChromePolicyVersionsV1RemoveNetworkRequest {}


/// Response object for removing a network.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies networks remove network customers](CustomerPolicyNetworkRemoveNetworkCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1RemoveNetworkResponse { _never_set: Option<bool> }

impl client::ResponseResult for GoogleChromePolicyVersionsV1RemoveNetworkResponse {}


/// Request message for getting the resolved policy value for a specific target.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies resolve customers](CustomerPolicyResolveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1ResolveRequest {
    /// The maximum number of policies to return, defaults to 100 and has a maximum of 1000.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// The page token used to retrieve a specific page of the request.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// The schema filter to apply to the resolve request. Specify a schema name to view a particular schema, for example: chrome.users.ShowLogoutButton Wildcards are supported, but only in the leaf portion of the schema name. Wildcards cannot be used in namespace directly. Please read https://developers.google.com/chrome/policy/guides/policy-schemas for details on schema namespaces. For example: Valid: "chrome.users.*", "chrome.users.apps.*", "chrome.printers.*" Invalid: "*", "*.users", "chrome.*", "chrome.*.apps.*"
    #[serde(rename="policySchemaFilter")]
    
    pub policy_schema_filter: Option<String>,
    /// Required. The key of the target resource on which the policies should be resolved.
    #[serde(rename="policyTargetKey")]
    
    pub policy_target_key: Option<GoogleChromePolicyVersionsV1PolicyTargetKey>,
}

impl client::RequestValue for GoogleChromePolicyVersionsV1ResolveRequest {}


/// Response message for getting the resolved policy value for a specific target.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies resolve customers](CustomerPolicyResolveCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1ResolveResponse {
    /// The page token used to get the next set of resolved policies found by the request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of resolved policies found by the resolve request.
    #[serde(rename="resolvedPolicies")]
    
    pub resolved_policies: Option<Vec<GoogleChromePolicyVersionsV1ResolvedPolicy>>,
}

impl client::ResponseResult for GoogleChromePolicyVersionsV1ResolveResponse {}


/// The resolved value of a policy for a given target.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1ResolvedPolicy {
    /// Output only. The added source key establishes at which level an entity was explicitly added for management. This is useful for certain type of policies that are only applied if they are explicitly added for management. For example: apps and networks. An entity can only be deleted from management in an Organizational Unit that it was explicitly added to. If this is not present it means that the policy is managed without the need to explicitly add an entity, for example: standard user or device policies.
    #[serde(rename="addedSourceKey")]
    
    pub added_source_key: Option<GoogleChromePolicyVersionsV1PolicyTargetKey>,
    /// Output only. The source resource from which this policy value is obtained. May be the same as `targetKey` if the policy is directly modified on the target, otherwise it would be another resource from which the policy gets its value (if applicable). If not present, the source is the default value for the customer.
    #[serde(rename="sourceKey")]
    
    pub source_key: Option<GoogleChromePolicyVersionsV1PolicyTargetKey>,
    /// Output only. The target resource for which the resolved policy value applies.
    #[serde(rename="targetKey")]
    
    pub target_key: Option<GoogleChromePolicyVersionsV1PolicyTargetKey>,
    /// Output only. The resolved value of the policy.
    
    pub value: Option<GoogleChromePolicyVersionsV1PolicyValue>,
}

impl client::Part for GoogleChromePolicyVersionsV1ResolvedPolicy {}


/// Request message for updating the group priority ordering of an app.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies groups update group priority ordering customers](CustomerPolicyGroupUpdateGroupPriorityOrderingCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1UpdateGroupPriorityOrderingRequest {
    /// Required. The group IDs, in desired priority ordering.
    #[serde(rename="groupIds")]
    
    pub group_ids: Option<Vec<String>>,
    /// Required. The namespace of the policy type for the request.
    #[serde(rename="policyNamespace")]
    
    pub policy_namespace: Option<String>,
    /// Required. The key of the target for which we want to update the group priority ordering. The target resource must point to an app.
    #[serde(rename="policyTargetKey")]
    
    pub policy_target_key: Option<GoogleChromePolicyVersionsV1PolicyTargetKey>,
}

impl client::RequestValue for GoogleChromePolicyVersionsV1UpdateGroupPriorityOrderingRequest {}


/// Request message for uploading a file for a policy.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [upload media](MediaUploadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1UploadPolicyFileRequest {
    /// Required. The fully qualified policy schema and field name this file is uploaded for. This information will be used to validate the content type of the file.
    #[serde(rename="policyField")]
    
    pub policy_field: Option<String>,
}

impl client::RequestValue for GoogleChromePolicyVersionsV1UploadPolicyFileRequest {}


/// Response message for downloading an uploaded file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [upload media](MediaUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromePolicyVersionsV1UploadPolicyFileResponse {
    /// The uri for end user to download the file.
    #[serde(rename="downloadUri")]
    
    pub download_uri: Option<String>,
}

impl client::ResponseResult for GoogleChromePolicyVersionsV1UploadPolicyFileResponse {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies groups batch delete customers](CustomerPolicyGroupBatchDeleteCall) (response)
/// * [policies groups batch modify customers](CustomerPolicyGroupBatchModifyCall) (response)
/// * [policies groups update group priority ordering customers](CustomerPolicyGroupUpdateGroupPriorityOrderingCall) (response)
/// * [policies orgunits batch inherit customers](CustomerPolicyOrgunitBatchInheritCall) (response)
/// * [policies orgunits batch modify customers](CustomerPolicyOrgunitBatchModifyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeDate {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for GoogleTypeDate {}


/// Describes a message type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Proto2DescriptorProto {
    /// no description provided
    #[serde(rename="enumType")]
    
    pub enum_type: Option<Vec<Proto2EnumDescriptorProto>>,
    /// no description provided
    
    pub field: Option<Vec<Proto2FieldDescriptorProto>>,
    /// no description provided
    
    pub name: Option<String>,
    /// no description provided
    #[serde(rename="nestedType")]
    
    pub nested_type: Option<Vec<Proto2DescriptorProto>>,
    /// no description provided
    #[serde(rename="oneofDecl")]
    
    pub oneof_decl: Option<Vec<Proto2OneofDescriptorProto>>,
}

impl client::Part for Proto2DescriptorProto {}


/// Describes an enum type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Proto2EnumDescriptorProto {
    /// no description provided
    
    pub name: Option<String>,
    /// no description provided
    
    pub value: Option<Vec<Proto2EnumValueDescriptorProto>>,
}

impl client::Part for Proto2EnumDescriptorProto {}


/// Describes a value within an enum.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Proto2EnumValueDescriptorProto {
    /// no description provided
    
    pub name: Option<String>,
    /// no description provided
    
    pub number: Option<i32>,
}

impl client::Part for Proto2EnumValueDescriptorProto {}


/// Describes a field within a message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Proto2FieldDescriptorProto {
    /// For numeric types, contains the original text representation of the value. For booleans, "true" or "false". For strings, contains the default text contents (not escaped in any way). For bytes, contains the C escaped value. All bytes >= 128 are escaped.
    #[serde(rename="defaultValue")]
    
    pub default_value: Option<String>,
    /// JSON name of this field. The value is set by protocol compiler. If the user has set a "json_name" option on this field, that option's value will be used. Otherwise, it's deduced from the field's name by converting it to camelCase.
    #[serde(rename="jsonName")]
    
    pub json_name: Option<String>,
    /// no description provided
    
    pub label: Option<Proto2FieldDescriptorProtoLabelEnum>,
    /// no description provided
    
    pub name: Option<String>,
    /// no description provided
    
    pub number: Option<i32>,
    /// If set, gives the index of a oneof in the containing type's oneof_decl list. This field is a member of that oneof.
    #[serde(rename="oneofIndex")]
    
    pub oneof_index: Option<i32>,
    /// If true, this is a proto3 "optional". When a proto3 field is optional, it tracks presence regardless of field type. When proto3_optional is true, this field must be belong to a oneof to signal to old proto3 clients that presence is tracked for this field. This oneof is known as a "synthetic" oneof, and this field must be its sole member (each proto3 optional field gets its own synthetic oneof). Synthetic oneofs exist in the descriptor only, and do not generate any API. Synthetic oneofs must be ordered after all "real" oneofs. For message fields, proto3_optional doesn't create any semantic change, since non-repeated message fields always track presence. However it still indicates the semantic detail of whether the user wrote "optional" or not. This can be useful for round-tripping the .proto file. For consistency we give message fields a synthetic oneof also, even though it is not required to track presence. This is especially important because the parser can't tell if a field is a message or an enum, so it must always create a synthetic oneof. Proto2 optional fields do not set this flag, because they already indicate optional with `LABEL_OPTIONAL`.
    #[serde(rename="proto3Optional")]
    
    pub proto3_optional: Option<bool>,
    /// If type_name is set, this need not be set. If both this and type_name are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.
    #[serde(rename="type")]
    
    pub type_: Option<Proto2FieldDescriptorProtoTypeEnum>,
    /// For message and enum types, this is the name of the type. If the name starts with a '.', it is fully-qualified. Otherwise, C++-like scoping rules are used to find the type (i.e. first the nested types within this message are searched, then within the parent, on up to the root namespace).
    #[serde(rename="typeName")]
    
    pub type_name: Option<String>,
}

impl client::Part for Proto2FieldDescriptorProto {}


/// Describes a complete .proto file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Proto2FileDescriptorProto {
    /// no description provided
    #[serde(rename="enumType")]
    
    pub enum_type: Option<Vec<Proto2EnumDescriptorProto>>,
    /// All top-level definitions in this file.
    #[serde(rename="messageType")]
    
    pub message_type: Option<Vec<Proto2DescriptorProto>>,
    /// file name, relative to root of source tree
    
    pub name: Option<String>,
    /// e.g. "foo", "foo.bar", etc.
    
    pub package: Option<String>,
    /// The syntax of the proto file. The supported values are "proto2", "proto3", and "editions". If `edition` is present, this value must be "editions".
    
    pub syntax: Option<String>,
}

impl client::Part for Proto2FileDescriptorProto {}


/// Describes a oneof.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Proto2OneofDescriptorProto {
    /// no description provided
    
    pub name: Option<String>,
}

impl client::Part for Proto2OneofDescriptorProto {}


