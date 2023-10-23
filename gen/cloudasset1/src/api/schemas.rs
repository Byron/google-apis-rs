use super::*;
/// Specifies roles and/or permissions to analyze, to determine both the identities possessing them and the resources they control. If multiple values are specified, results will include roles or permissions matching any of them. The total number of roles and permissions should be equal or less than 10.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccessSelector {
    /// Optional. The permissions to appear in result.
    
    pub permissions: Option<Vec<String>>,
    /// Optional. The roles to appear in result.
    
    pub roles: Option<Vec<String>>,
}

impl client::Part for AccessSelector {}


/// A request message for AssetService.AnalyzeIamPolicyLongrunning.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze iam policy longrunning](MethodAnalyzeIamPolicyLongrunningCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeIamPolicyLongrunningRequest {
    /// Required. The request query.
    #[serde(rename="analysisQuery")]
    
    pub analysis_query: Option<IamPolicyAnalysisQuery>,
    /// Required. Output configuration indicating where the results will be output to.
    #[serde(rename="outputConfig")]
    
    pub output_config: Option<IamPolicyAnalysisOutputConfig>,
    /// Optional. The name of a saved query, which must be in the format of: * projects/project_number/savedQueries/saved_query_id * folders/folder_number/savedQueries/saved_query_id * organizations/organization_number/savedQueries/saved_query_id If both `analysis_query` and `saved_analysis_query` are provided, they will be merged together with the `saved_analysis_query` as base and the `analysis_query` as overrides. For more details of the merge behavior, please refer to the [MergeFrom](https://developers.google.com/protocol-buffers/docs/reference/cpp/google.protobuf.message#Message.MergeFrom.details) doc. Note that you cannot override primitive fields with default value, such as 0 or empty string, etc., because we use proto3, which doesn't support field presence yet.
    #[serde(rename="savedAnalysisQuery")]
    
    pub saved_analysis_query: Option<String>,
}

impl client::RequestValue for AnalyzeIamPolicyLongrunningRequest {}


/// A response message for AssetService.AnalyzeIamPolicy.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze iam policy](MethodAnalyzeIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeIamPolicyResponse {
    /// Represents whether all entries in the main_analysis and service_account_impersonation_analysis have been fully explored to answer the query in the request.
    #[serde(rename="fullyExplored")]
    
    pub fully_explored: Option<bool>,
    /// The main analysis that matches the original request.
    #[serde(rename="mainAnalysis")]
    
    pub main_analysis: Option<IamPolicyAnalysis>,
    /// The service account impersonation analysis if AnalyzeIamPolicyRequest.analyze_service_account_impersonation is enabled.
    #[serde(rename="serviceAccountImpersonationAnalysis")]
    
    pub service_account_impersonation_analysis: Option<Vec<IamPolicyAnalysis>>,
}

impl client::ResponseResult for AnalyzeIamPolicyResponse {}


/// The response message for resource move analysis.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze move](MethodAnalyzeMoveCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeMoveResponse {
    /// The list of analyses returned from performing the intended resource move analysis. The analysis is grouped by different Google Cloud services.
    #[serde(rename="moveAnalysis")]
    
    pub move_analysis: Option<Vec<MoveAnalysis>>,
}

impl client::ResponseResult for AnalyzeMoveResponse {}


/// The response message for AssetService.AnalyzeOrgPolicies.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze org policies](MethodAnalyzeOrgPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeOrgPoliciesResponse {
    /// The definition of the constraint in the request.
    
    pub constraint: Option<AnalyzerOrgPolicyConstraint>,
    /// The page token to fetch the next page for AnalyzeOrgPoliciesResponse.org_policy_results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The organization policies under the AnalyzeOrgPoliciesRequest.scope with the AnalyzeOrgPoliciesRequest.constraint.
    #[serde(rename="orgPolicyResults")]
    
    pub org_policy_results: Option<Vec<OrgPolicyResult>>,
}

impl client::ResponseResult for AnalyzeOrgPoliciesResponse {}


/// The response message for AssetService.AnalyzeOrgPolicyGovernedAssets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze org policy governed assets](MethodAnalyzeOrgPolicyGovernedAssetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeOrgPolicyGovernedAssetsResponse {
    /// The definition of the constraint in the request.
    
    pub constraint: Option<AnalyzerOrgPolicyConstraint>,
    /// The list of the analyzed governed assets.
    #[serde(rename="governedAssets")]
    
    pub governed_assets: Option<Vec<GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedAsset>>,
    /// The page token to fetch the next page for AnalyzeOrgPolicyGovernedAssetsResponse.governed_assets.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for AnalyzeOrgPolicyGovernedAssetsResponse {}


/// The response message for AssetService.AnalyzeOrgPolicyGovernedContainers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze org policy governed containers](MethodAnalyzeOrgPolicyGovernedContainerCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeOrgPolicyGovernedContainersResponse {
    /// The definition of the constraint in the request.
    
    pub constraint: Option<AnalyzerOrgPolicyConstraint>,
    /// The list of the analyzed governed containers.
    #[serde(rename="governedContainers")]
    
    pub governed_containers: Option<Vec<GoogleCloudAssetV1GovernedContainer>>,
    /// The page token to fetch the next page for AnalyzeOrgPolicyGovernedContainersResponse.governed_containers.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for AnalyzeOrgPolicyGovernedContainersResponse {}


/// This organization policy message is a modified version of the one defined in the Organization Policy system. This message contains several fields defined in the original organization policy with some new fields for analysis purpose.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzerOrgPolicy {
    /// The [full resource name] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of an organization/folder/project resource where this organization policy applies to. For any user defined org policies, this field has the same value as the [attached_resource] field. Only for default policy, this field has the different value.
    #[serde(rename="appliedResource")]
    
    pub applied_resource: Option<String>,
    /// The [full resource name] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of an organization/folder/project resource where this organization policy is set. Notice that some type of constraints are defined with default policy. This field will be empty for them.
    #[serde(rename="attachedResource")]
    
    pub attached_resource: Option<String>,
    /// If `inherit_from_parent` is true, Rules set higher up in the hierarchy (up to the closest root) are inherited and present in the effective policy. If it is false, then no rules are inherited, and this policy becomes the effective root for evaluation.
    #[serde(rename="inheritFromParent")]
    
    pub inherit_from_parent: Option<bool>,
    /// Ignores policies set above this resource and restores the default behavior of the constraint at this resource. This field can be set in policies for either list or boolean constraints. If set, `rules` must be empty and `inherit_from_parent` must be set to false.
    
    pub reset: Option<bool>,
    /// List of rules for this organization policy.
    
    pub rules: Option<Vec<GoogleCloudAssetV1Rule>>,
}

impl client::Part for AnalyzerOrgPolicy {}


/// The organization policy constraint definition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzerOrgPolicyConstraint {
    /// The definition of the custom constraint.
    #[serde(rename="customConstraint")]
    
    pub custom_constraint: Option<GoogleCloudAssetV1CustomConstraint>,
    /// The definition of the canned constraint defined by Google.
    #[serde(rename="googleDefinedConstraint")]
    
    pub google_defined_constraint: Option<GoogleCloudAssetV1Constraint>,
}

impl client::Part for AnalyzerOrgPolicyConstraint {}


/// An asset in Google Cloud. An asset can be any resource in the Google Cloud [resource hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy), a resource outside the Google Cloud resource hierarchy (such as Google Kubernetes Engine clusters and objects), or a policy (e.g. IAM policy), or a relationship (e.g. an INSTANCE_TO_INSTANCEGROUP relationship). See [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list assets](AssetListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    /// Please also refer to the [access level user guide](https://cloud.google.com/access-context-manager/docs/overview#access-levels).
    #[serde(rename="accessLevel")]
    
    pub access_level: Option<GoogleIdentityAccesscontextmanagerV1AccessLevel>,
    /// Please also refer to the [access policy user guide](https://cloud.google.com/access-context-manager/docs/overview#access-policies).
    #[serde(rename="accessPolicy")]
    
    pub access_policy: Option<GoogleIdentityAccesscontextmanagerV1AccessPolicy>,
    /// The ancestry path of an asset in Google Cloud [resource hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy), represented as a list of relative resource names. An ancestry path starts with the closest ancestor in the hierarchy and ends at root. If the asset is a project, folder, or organization, the ancestry path starts from the asset itself. Example: `["projects/123456789", "folders/5432", "organizations/1234"]`
    
    pub ancestors: Option<Vec<String>>,
    /// The type of the asset. Example: `compute.googleapis.com/Disk` See [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more information.
    #[serde(rename="assetType")]
    
    pub asset_type: Option<String>,
    /// A representation of the IAM policy set on a Google Cloud resource. There can be a maximum of one IAM policy set on any given resource. In addition, IAM policies inherit their granted access scope from any policies set on parent resources in the resource hierarchy. Therefore, the effectively policy is the union of both the policy set on this resource and each policy set on all of the resource's ancestry resource levels in the hierarchy. See [this topic](https://cloud.google.com/iam/help/allow-policies/inheritance) for more information.
    #[serde(rename="iamPolicy")]
    
    pub iam_policy: Option<Policy>,
    /// The full name of the asset. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1` See [Resource names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information.
    
    pub name: Option<String>,
    /// A representation of an [organization policy](https://cloud.google.com/resource-manager/docs/organization-policy/overview#organization_policy). There can be more than one organization policy with different constraints set on a given resource.
    #[serde(rename="orgPolicy")]
    
    pub org_policy: Option<Vec<GoogleCloudOrgpolicyV1Policy>>,
    /// A representation of runtime OS Inventory information. See [this topic](https://cloud.google.com/compute/docs/instances/os-inventory-management) for more information.
    #[serde(rename="osInventory")]
    
    pub os_inventory: Option<Inventory>,
    /// One related asset of the current asset.
    #[serde(rename="relatedAsset")]
    
    pub related_asset: Option<RelatedAsset>,
    /// DEPRECATED. This field only presents for the purpose of backward-compatibility. The server will never generate responses with this field. The related assets of the asset of one relationship type. One asset only represents one type of relationship.
    #[serde(rename="relatedAssets")]
    
    pub related_assets: Option<RelatedAssets>,
    /// A representation of the resource.
    
    pub resource: Option<Resource>,
    /// Please also refer to the [service perimeter user guide](https://cloud.google.com/vpc-service-controls/docs/overview).
    #[serde(rename="servicePerimeter")]
    
    pub service_perimeter: Option<GoogleIdentityAccesscontextmanagerV1ServicePerimeter>,
    /// The last update timestamp of an asset. update_time is updated when create/update/delete operation is performed.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Resource for Asset {}


/// Attached resource representation, which is defined by the corresponding service provider. It represents an attached resource's payload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AttachedResource {
    /// The type of this attached resource. Example: `osconfig.googleapis.com/Inventory` You can find the supported attached asset types of each resource in this table: `https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types`
    #[serde(rename="assetType")]
    
    pub asset_type: Option<String>,
    /// Versioned resource representations of this attached resource. This is repeated because there could be multiple versions of the attached resource representations during version migration.
    #[serde(rename="versionedResources")]
    
    pub versioned_resources: Option<Vec<VersionedResource>>,
}

impl client::Part for AttachedResource {}


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


/// Batch get assets history response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch get assets history](MethodBatchGetAssetsHistoryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetAssetsHistoryResponse {
    /// A list of assets with valid time windows.
    
    pub assets: Option<Vec<TemporalAsset>>,
}

impl client::ResponseResult for BatchGetAssetsHistoryResponse {}


/// A response message for AssetService.BatchGetEffectiveIamPolicies.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch get effective iam policies](EffectiveIamPolicyBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetEffectiveIamPoliciesResponse {
    /// The effective policies for a batch of resources. Note that the results order is the same as the order of BatchGetEffectiveIamPoliciesRequest.names. When a resource does not have any effective IAM policies, its corresponding policy_result will contain empty EffectiveIamPolicy.policies.
    #[serde(rename="policyResults")]
    
    pub policy_results: Option<Vec<EffectiveIamPolicy>>,
}

impl client::ResponseResult for BatchGetEffectiveIamPoliciesResponse {}


/// A BigQuery destination for exporting assets to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigQueryDestination {
    /// Required. The BigQuery dataset in format “projects/projectId/datasets/datasetId”, to which the snapshot result should be exported. If this dataset does not exist, the export call returns an INVALID_ARGUMENT error. Setting the `contentType` for `exportAssets` determines the [schema](https://cloud.google.com/asset-inventory/docs/exporting-to-bigquery#bigquery-schema) of the BigQuery table. Setting `separateTablesPerAssetType` to `TRUE` also influences the schema.
    
    pub dataset: Option<String>,
    /// If the destination table already exists and this flag is `TRUE`, the table will be overwritten by the contents of assets snapshot. If the flag is `FALSE` or unset and the destination table already exists, the export call returns an INVALID_ARGUMEMT error.
    
    pub force: Option<bool>,
    /// [partition_spec] determines whether to export to partitioned table(s) and how to partition the data. If [partition_spec] is unset or [partition_spec.partition_key] is unset or `PARTITION_KEY_UNSPECIFIED`, the snapshot results will be exported to non-partitioned table(s). [force] will decide whether to overwrite existing table(s). If [partition_spec] is specified. First, the snapshot results will be written to partitioned table(s) with two additional timestamp columns, readTime and requestTime, one of which will be the partition key. Secondly, in the case when any destination table already exists, it will first try to update existing table's schema as necessary by appending additional columns. Then, if [force] is `TRUE`, the corresponding partition will be overwritten by the snapshot results (data in different partitions will remain intact); if [force] is unset or `FALSE`, it will append the data. An error will be returned if the schema update or data appension fails.
    #[serde(rename="partitionSpec")]
    
    pub partition_spec: Option<PartitionSpec>,
    /// If this flag is `TRUE`, the snapshot results will be written to one or multiple tables, each of which contains results of one asset type. The [force] and [partition_spec] fields will apply to each of them. Field [table] will be concatenated with "_" and the asset type names (see https://cloud.google.com/asset-inventory/docs/supported-asset-types for supported asset types) to construct per-asset-type table names, in which all non-alphanumeric characters like "." and "/" will be substituted by "_". Example: if field [table] is "mytable" and snapshot results contain "storage.googleapis.com/Bucket" assets, the corresponding table name will be "mytable_storage_googleapis_com_Bucket". If any of these tables does not exist, a new table with the concatenated name will be created. When [content_type] in the ExportAssetsRequest is `RESOURCE`, the schema of each table will include RECORD-type columns mapped to the nested fields in the Asset.resource.data field of that asset type (up to the 15 nested level BigQuery supports (https://cloud.google.com/bigquery/docs/nested-repeated#limitations)). The fields in >15 nested levels will be stored in JSON format string as a child column of its parent RECORD column. If error occurs when exporting to any table, the whole export call will return an error but the export results that already succeed will persist. Example: if exporting to table_type_A succeeds when exporting to table_type_B fails during one export call, the results in table_type_A will persist and there will not be partial results persisting in a table.
    #[serde(rename="separateTablesPerAssetType")]
    
    pub separate_tables_per_asset_type: Option<bool>,
    /// Required. The BigQuery table to which the snapshot result should be written. If this table does not exist, a new table with the given name will be created.
    
    pub table: Option<String>,
}

impl client::Part for BigQueryDestination {}


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


/// The IAM conditions context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConditionContext {
    /// The hypothetical access timestamp to evaluate IAM conditions. Note that this value must not be earlier than the current time; otherwise, an INVALID_ARGUMENT error will be returned.
    #[serde(rename="accessTime")]
    
    pub access_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for ConditionContext {}


/// The Condition evaluation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConditionEvaluation {
    /// The evaluation result.
    #[serde(rename="evaluationValue")]
    
    pub evaluation_value: Option<ConditionEvaluationEvaluationValueEnum>,
}

impl client::Part for ConditionEvaluation {}


/// Create asset feed request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create feeds](FeedCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateFeedRequest {
    /// Required. The feed details. The field `name` must be empty and it will be generated in the format of: projects/project_number/feeds/feed_id folders/folder_number/feeds/feed_id organizations/organization_number/feeds/feed_id
    
    pub feed: Option<Feed>,
    /// Required. This is the client-assigned asset feed identifier and it needs to be unique under a specific parent project/folder/organization.
    #[serde(rename="feedId")]
    
    pub feed_id: Option<String>,
}

impl client::RequestValue for CreateFeedRequest {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for Date {}


/// The effective IAM policies on one resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EffectiveIamPolicy {
    /// The [full_resource_name] (https://cloud.google.com/asset-inventory/docs/resource-name-format) for which the policies are computed. This is one of the BatchGetEffectiveIamPoliciesRequest.names the caller provides in the request.
    #[serde(rename="fullResourceName")]
    
    pub full_resource_name: Option<String>,
    /// The effective policies for the full_resource_name. These policies include the policy set on the full_resource_name and those set on its parents and ancestors up to the BatchGetEffectiveIamPoliciesRequest.scope. Note that these policies are not filtered according to the resource type of the full_resource_name. These policies are hierarchically ordered by PolicyInfo.attached_resource starting from full_resource_name itself to its parents and ancestors, such that policies[i]'s PolicyInfo.attached_resource is the child of policies[i+1]'s PolicyInfo.attached_resource, if policies[i+1] exists.
    
    pub policies: Option<Vec<PolicyInfo>>,
}

impl client::Part for EffectiveIamPolicy {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete feeds](FeedDeleteCall) (response)
/// * [delete saved queries](SavedQueryDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Explanation about the IAM policy search result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Explanation {
    /// The map from roles to their included permissions that match the permission query (i.e., a query containing `policy.role.permissions:`). Example: if query `policy.role.permissions:compute.disk.get` matches a policy binding that contains owner role, the matched_permissions will be `{"roles/owner": ["compute.disk.get"]}`. The roles can also be found in the returned `policy` bindings. Note that the map is populated only for requests with permission queries.
    #[serde(rename="matchedPermissions")]
    
    pub matched_permissions: Option<HashMap<String, Permissions>>,
}

impl client::Part for Explanation {}


/// Export asset request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [export assets](MethodExportAssetCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportAssetsRequest {
    /// A list of asset types to take a snapshot for. For example: "compute.googleapis.com/Disk". Regular expressions are also supported. For example: * "compute.googleapis.com.*" snapshots resources whose asset type starts with "compute.googleapis.com". * ".*Instance" snapshots resources whose asset type ends with "Instance". * ".*Instance.*" snapshots resources whose asset type contains "Instance". See [RE2](https://github.com/google/re2/wiki/Syntax) for all supported regular expression syntax. If the regular expression does not match any supported asset type, an INVALID_ARGUMENT error will be returned. If specified, only matching assets will be returned, otherwise, it will snapshot all asset types. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all supported asset types.
    #[serde(rename="assetTypes")]
    
    pub asset_types: Option<Vec<String>>,
    /// Asset content type. If not specified, no content but the asset name will be returned.
    #[serde(rename="contentType")]
    
    pub content_type: Option<ExportAssetsRequestContentTypeEnum>,
    /// Required. Output configuration indicating where the results will be output to.
    #[serde(rename="outputConfig")]
    
    pub output_config: Option<OutputConfig>,
    /// Timestamp to take an asset snapshot. This can only be set to a timestamp between the current time and the current time minus 35 days (inclusive). If not specified, the current time will be used. Due to delays in resource data collection and indexing, there is a volatile window during which running the same query may get different results.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A list of relationship types to export, for example: `INSTANCE_TO_INSTANCEGROUP`. This field should only be specified if content_type=RELATIONSHIP. * If specified: it snapshots specified relationships. It returns an error if any of the [relationship_types] doesn't belong to the supported relationship types of the [asset_types] or if any of the [asset_types] doesn't belong to the source types of the [relationship_types]. * Otherwise: it snapshots the supported relationships for all [asset_types] or returns an error if any of the [asset_types] has no relationship support. An unspecified asset types field means all supported asset_types. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all supported asset types and relationship types.
    #[serde(rename="relationshipTypes")]
    
    pub relationship_types: Option<Vec<String>>,
}

impl client::RequestValue for ExportAssetsRequest {}


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


/// An asset feed used to export asset updates to a destinations. An asset feed filter controls what updates are exported. The asset feed must be created within a project, organization, or folder. Supported destinations are: Pub/Sub topics.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create feeds](FeedCreateCall) (response)
/// * [delete feeds](FeedDeleteCall) (none)
/// * [get feeds](FeedGetCall) (response)
/// * [list feeds](FeedListCall) (none)
/// * [patch feeds](FeedPatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Feed {
    /// A list of the full names of the assets to receive updates. You must specify either or both of asset_names and asset_types. Only asset updates matching specified asset_names or asset_types are exported to the feed. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`. For a list of the full names for supported asset types, see [Resource name format](https://cloud.google.com/asset-inventory/docs/resource-name-format).
    #[serde(rename="assetNames")]
    
    pub asset_names: Option<Vec<String>>,
    /// A list of types of the assets to receive updates. You must specify either or both of asset_names and asset_types. Only asset updates matching specified asset_names or asset_types are exported to the feed. Example: `"compute.googleapis.com/Disk"` For a list of all supported asset types, see [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types).
    #[serde(rename="assetTypes")]
    
    pub asset_types: Option<Vec<String>>,
    /// A condition which determines whether an asset update should be published. If specified, an asset will be returned only when the expression evaluates to true. When set, `expression` field in the `Expr` must be a valid [CEL expression] (https://github.com/google/cel-spec) on a TemporalAsset with name `temporal_asset`. Example: a Feed with expression ("temporal_asset.deleted == true") will only publish Asset deletions. Other fields of `Expr` are optional. See our [user guide](https://cloud.google.com/asset-inventory/docs/monitoring-asset-changes-with-condition) for detailed instructions.
    
    pub condition: Option<Expr>,
    /// Asset content type. If not specified, no content but the asset name and type will be returned.
    #[serde(rename="contentType")]
    
    pub content_type: Option<FeedContentTypeEnum>,
    /// Required. Feed output configuration defining where the asset updates are published to.
    #[serde(rename="feedOutputConfig")]
    
    pub feed_output_config: Option<FeedOutputConfig>,
    /// Required. The format will be projects/{project_number}/feeds/{client-assigned_feed_identifier} or folders/{folder_number}/feeds/{client-assigned_feed_identifier} or organizations/{organization_number}/feeds/{client-assigned_feed_identifier} The client-assigned feed identifier must be unique within the parent project/folder/organization.
    
    pub name: Option<String>,
    /// A list of relationship types to output, for example: `INSTANCE_TO_INSTANCEGROUP`. This field should only be specified if content_type=RELATIONSHIP. * If specified: it outputs specified relationship updates on the [asset_names] or the [asset_types]. It returns an error if any of the [relationship_types] doesn't belong to the supported relationship types of the [asset_names] or [asset_types], or any of the [asset_names] or the [asset_types] doesn't belong to the source types of the [relationship_types]. * Otherwise: it outputs the supported relationships of the types of [asset_names] and [asset_types] or returns an error if any of the [asset_names] or the [asset_types] has no replationship support. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all supported asset types and relationship types.
    #[serde(rename="relationshipTypes")]
    
    pub relationship_types: Option<Vec<String>>,
}

impl client::Resource for Feed {}
impl client::ResponseResult for Feed {}


/// Output configuration for asset feed destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FeedOutputConfig {
    /// Destination on Pub/Sub.
    #[serde(rename="pubsubDestination")]
    
    pub pubsub_destination: Option<PubsubDestination>,
}

impl client::Part for FeedOutputConfig {}


/// A Cloud Storage location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcsDestination {
    /// The URI of the Cloud Storage object. It's the same URI that is used by gsutil. Example: "gs://bucket_name/object_name". See [Viewing and Editing Object Metadata](https://cloud.google.com/storage/docs/viewing-editing-metadata) for more information. If the specified Cloud Storage object already exists and there is no [hold](https://cloud.google.com/storage/docs/object-holds), it will be overwritten with the exported result.
    
    pub uri: Option<String>,
    /// The URI prefix of all generated Cloud Storage objects. Example: "gs://bucket_name/object_name_prefix". Each object URI is in format: "gs://bucket_name/object_name_prefix// and only contains assets for that type. starts from 0. Example: "gs://bucket_name/object_name_prefix/compute.googleapis.com/Disk/0" is the first shard of output objects containing all compute.googleapis.com/Disk assets. An INVALID_ARGUMENT error will be returned if file with the same name "gs://bucket_name/object_name_prefix" already exists.
    #[serde(rename="uriPrefix")]
    
    pub uri_prefix: Option<String>,
}

impl client::Part for GcsDestination {}


/// An IAM role or permission under analysis.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1Access {
    /// The analysis state of this access.
    #[serde(rename="analysisState")]
    
    pub analysis_state: Option<IamPolicyAnalysisState>,
    /// The permission.
    
    pub permission: Option<String>,
    /// The role.
    
    pub role: Option<String>,
}

impl client::Part for GoogleCloudAssetV1Access {}


/// An access control list, derived from the above IAM policy binding, which contains a set of resources and accesses. May include one item from each set to compose an access control entry. NOTICE that there could be multiple access control lists for one IAM policy binding. The access control lists are created based on resource and access combinations. For example, assume we have the following cases in one IAM policy binding: - Permission P1 and P2 apply to resource R1 and R2; - Permission P3 applies to resource R2 and R3; This will result in the following access control lists: - AccessControlList 1: [R1, R2], [P1, P2] - AccessControlList 2: [R2, R3], [P3]
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1AccessControlList {
    /// The accesses that match one of the following conditions: - The access_selector, if it is specified in request; - Otherwise, access specifiers reachable from the policy binding's role.
    
    pub accesses: Option<Vec<GoogleCloudAssetV1Access>>,
    /// Condition evaluation for this AccessControlList, if there is a condition defined in the above IAM policy binding.
    #[serde(rename="conditionEvaluation")]
    
    pub condition_evaluation: Option<ConditionEvaluation>,
    /// Resource edges of the graph starting from the policy attached resource to any descendant resources. The Edge.source_node contains the full resource name of a parent resource and Edge.target_node contains the full resource name of a child resource. This field is present only if the output_resource_edges option is enabled in request.
    #[serde(rename="resourceEdges")]
    
    pub resource_edges: Option<Vec<GoogleCloudAssetV1Edge>>,
    /// The resources that match one of the following conditions: - The resource_selector, if it is specified in request; - Otherwise, resources reachable from the policy attached resource.
    
    pub resources: Option<Vec<GoogleCloudAssetV1Resource>>,
}

impl client::Part for GoogleCloudAssetV1AccessControlList {}


/// Represents a Google Cloud asset(resource or IAM policy) governed by the organization policies of the AnalyzeOrgPolicyGovernedAssetsRequest.constraint.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedAsset {
    /// The consolidated policy for the analyzed asset. The consolidated policy is computed by merging and evaluating AnalyzeOrgPolicyGovernedAssetsResponse.GovernedAsset.policy_bundle. The evaluation will respect the organization policy [hierarchy rules](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-hierarchy).
    #[serde(rename="consolidatedPolicy")]
    
    pub consolidated_policy: Option<AnalyzerOrgPolicy>,
    /// An IAM policy governed by the organization policies of the AnalyzeOrgPolicyGovernedAssetsRequest.constraint.
    #[serde(rename="governedIamPolicy")]
    
    pub governed_iam_policy: Option<GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedIamPolicy>,
    /// A Google Cloud resource governed by the organization policies of the AnalyzeOrgPolicyGovernedAssetsRequest.constraint.
    #[serde(rename="governedResource")]
    
    pub governed_resource: Option<GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedResource>,
    /// The ordered list of all organization policies from the AnalyzeOrgPoliciesResponse.OrgPolicyResult.consolidated_policy.attached_resource to the scope specified in the request. If the constraint is defined with default policy, it will also appear in the list.
    #[serde(rename="policyBundle")]
    
    pub policy_bundle: Option<Vec<AnalyzerOrgPolicy>>,
}

impl client::Part for GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedAsset {}


/// The IAM policies governed by the organization policies of the AnalyzeOrgPolicyGovernedAssetsRequest.constraint.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedIamPolicy {
    /// The full resource name of the resource associated with this IAM policy. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`. See [Cloud Asset Inventory Resource Name Format](https://cloud.google.com/asset-inventory/docs/resource-name-format) for more information.
    #[serde(rename="attachedResource")]
    
    pub attached_resource: Option<String>,
    /// The folder(s) that this IAM policy belongs to, in the form of folders/{FOLDER_NUMBER}. This field is available when the IAM policy belongs(directly or cascadingly) to one or more folders.
    
    pub folders: Option<Vec<String>>,
    /// The organization that this IAM policy belongs to, in the form of organizations/{ORGANIZATION_NUMBER}. This field is available when the IAM policy belongs(directly or cascadingly) to an organization.
    
    pub organization: Option<String>,
    /// The IAM policy directly set on the given resource.
    
    pub policy: Option<Policy>,
    /// The project that this IAM policy belongs to, in the form of projects/{PROJECT_NUMBER}. This field is available when the IAM policy belongs to a project.
    
    pub project: Option<String>,
}

impl client::Part for GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedIamPolicy {}


/// The Google Cloud resources governed by the organization policies of the AnalyzeOrgPolicyGovernedAssetsRequest.constraint.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedResource {
    /// The folder(s) that this resource belongs to, in the form of folders/{FOLDER_NUMBER}. This field is available when the resource belongs(directly or cascadingly) to one or more folders.
    
    pub folders: Option<Vec<String>>,
    /// The [full resource name] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of the Google Cloud resource.
    #[serde(rename="fullResourceName")]
    
    pub full_resource_name: Option<String>,
    /// The organization that this resource belongs to, in the form of organizations/{ORGANIZATION_NUMBER}. This field is available when the resource belongs(directly or cascadingly) to an organization.
    
    pub organization: Option<String>,
    /// The [full resource name] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of the parent of AnalyzeOrgPolicyGovernedAssetsResponse.GovernedResource.full_resource_name.
    
    pub parent: Option<String>,
    /// The project that this resource belongs to, in the form of projects/{PROJECT_NUMBER}. This field is available when the resource belongs to a project.
    
    pub project: Option<String>,
}

impl client::Part for GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedResource {}


/// A BigQuery destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1BigQueryDestination {
    /// Required. The BigQuery dataset in format "projects/projectId/datasets/datasetId", to which the analysis results should be exported. If this dataset does not exist, the export call will return an INVALID_ARGUMENT error.
    
    pub dataset: Option<String>,
    /// The partition key for BigQuery partitioned table.
    #[serde(rename="partitionKey")]
    
    pub partition_key: Option<GoogleCloudAssetV1BigQueryDestinationPartitionKeyEnum>,
    /// Required. The prefix of the BigQuery tables to which the analysis results will be written. Tables will be created based on this table_prefix if not exist: * _analysis table will contain export operation's metadata. * _analysis_result will contain all the IamPolicyAnalysisResult. When [partition_key] is specified, both tables will be partitioned based on the [partition_key].
    #[serde(rename="tablePrefix")]
    
    pub table_prefix: Option<String>,
    /// Optional. Specifies the action that occurs if the destination table or partition already exists. The following values are supported: * WRITE_TRUNCATE: If the table or partition already exists, BigQuery overwrites the entire table or all the partitions data. * WRITE_APPEND: If the table or partition already exists, BigQuery appends the data to the table or the latest partition. * WRITE_EMPTY: If the table already exists and contains data, an error is returned. The default value is WRITE_APPEND. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Details are at https://cloud.google.com/bigquery/docs/loading-data-local#appending_to_or_overwriting_a_table_using_a_local_file.
    #[serde(rename="writeDisposition")]
    
    pub write_disposition: Option<String>,
}

impl client::Part for GoogleCloudAssetV1BigQueryDestination {}


/// A `Constraint` that is either enforced or not. For example a constraint `constraints/compute.disableSerialPortAccess`. If it is enforced on a VM instance, serial port connections will not be opened to that instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1BooleanConstraint { _never_set: Option<bool> }

impl client::Part for GoogleCloudAssetV1BooleanConstraint {}


/// The definition of a constraint.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1Constraint {
    /// Defines this constraint as being a BooleanConstraint.
    #[serde(rename="booleanConstraint")]
    
    pub boolean_constraint: Option<GoogleCloudAssetV1BooleanConstraint>,
    /// The evaluation behavior of this constraint in the absence of 'Policy'.
    #[serde(rename="constraintDefault")]
    
    pub constraint_default: Option<GoogleCloudAssetV1ConstraintConstraintDefaultEnum>,
    /// Detailed description of what this `Constraint` controls as well as how and where it is enforced.
    
    pub description: Option<String>,
    /// The human readable name of the constraint.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Defines this constraint as being a ListConstraint.
    #[serde(rename="listConstraint")]
    
    pub list_constraint: Option<GoogleCloudAssetV1ListConstraint>,
    /// The unique name of the constraint. Format of the name should be * `constraints/{constraint_name}` For example, `constraints/compute.disableSerialPortAccess`.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudAssetV1Constraint {}


/// The definition of a custom constraint.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1CustomConstraint {
    /// Allow or deny type.
    #[serde(rename="actionType")]
    
    pub action_type: Option<GoogleCloudAssetV1CustomConstraintActionTypeEnum>,
    /// Organization Policy condition/expression. For example: `resource.instanceName.matches("[production|test]_.*_(\d)+")'` or, `resource.management.auto_upgrade == true`
    
    pub condition: Option<String>,
    /// Detailed information about this custom policy constraint.
    
    pub description: Option<String>,
    /// One line display name for the UI.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// All the operations being applied for this constraint.
    #[serde(rename="methodTypes")]
    
    pub method_types: Option<Vec<GoogleCloudAssetV1CustomConstraintMethodTypesEnum>>,
    /// Name of the constraint. This is unique within the organization. Format of the name should be * `organizations/{organization_id}/customConstraints/{custom_constraint_id}` Example : "organizations/123/customConstraints/custom.createOnlyE2TypeVms"
    
    pub name: Option<String>,
    /// The Resource Instance type on which this policy applies to. Format will be of the form : "/" Example: * `compute.googleapis.com/Instance`.
    #[serde(rename="resourceTypes")]
    
    pub resource_types: Option<Vec<String>>,
}

impl client::Part for GoogleCloudAssetV1CustomConstraint {}


/// A directional edge.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1Edge {
    /// The source node of the edge. For example, it could be a full resource name for a resource node or an email of an identity.
    #[serde(rename="sourceNode")]
    
    pub source_node: Option<String>,
    /// The target node of the edge. For example, it could be a full resource name for a resource node or an email of an identity.
    #[serde(rename="targetNode")]
    
    pub target_node: Option<String>,
}

impl client::Part for GoogleCloudAssetV1Edge {}


/// A Cloud Storage location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1GcsDestination {
    /// Required. The URI of the Cloud Storage object. It's the same URI that is used by gsutil. Example: "gs://bucket_name/object_name". See [Viewing and Editing Object Metadata](https://cloud.google.com/storage/docs/viewing-editing-metadata) for more information. If the specified Cloud Storage object already exists and there is no [hold](https://cloud.google.com/storage/docs/object-holds), it will be overwritten with the analysis result.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudAssetV1GcsDestination {}


/// The organization/folder/project resource governed by organization policies of AnalyzeOrgPolicyGovernedContainersRequest.constraint.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1GovernedContainer {
    /// The consolidated organization policy for the analyzed resource. The consolidated organization policy is computed by merging and evaluating AnalyzeOrgPolicyGovernedContainersResponse.GovernedContainer.policy_bundle. The evaluation will respect the organization policy [hierarchy rules](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-hierarchy).
    #[serde(rename="consolidatedPolicy")]
    
    pub consolidated_policy: Option<AnalyzerOrgPolicy>,
    /// The [full resource name] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of an organization/folder/project resource.
    #[serde(rename="fullResourceName")]
    
    pub full_resource_name: Option<String>,
    /// The [full resource name] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of the parent of AnalyzeOrgPolicyGovernedContainersResponse.GovernedContainer.full_resource_name.
    
    pub parent: Option<String>,
    /// The ordered list of all organization policies from the AnalyzeOrgPoliciesResponse.OrgPolicyResult.consolidated_policy.attached_resource. to the scope specified in the request. If the constraint is defined with default policy, it will also appear in the list.
    #[serde(rename="policyBundle")]
    
    pub policy_bundle: Option<Vec<AnalyzerOrgPolicy>>,
}

impl client::Part for GoogleCloudAssetV1GovernedContainer {}


/// An identity under analysis.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1Identity {
    /// The analysis state of this identity.
    #[serde(rename="analysisState")]
    
    pub analysis_state: Option<IamPolicyAnalysisState>,
    /// The identity name in any form of members appear in [IAM policy binding](https://cloud.google.com/iam/reference/rest/v1/Binding), such as: - user:foo@google.com - group:group1@google.com - serviceAccount:s1@prj1.iam.gserviceaccount.com - projectOwner:some_project_id - domain:google.com - allUsers - etc.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudAssetV1Identity {}


/// The identities and group edges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1IdentityList {
    /// Group identity edges of the graph starting from the binding's group members to any node of the identities. The Edge.source_node contains a group, such as `group:parent@google.com`. The Edge.target_node contains a member of the group, such as `group:child@google.com` or `user:foo@google.com`. This field is present only if the output_group_edges option is enabled in request.
    #[serde(rename="groupEdges")]
    
    pub group_edges: Option<Vec<GoogleCloudAssetV1Edge>>,
    /// Only the identities that match one of the following conditions will be presented: - The identity_selector, if it is specified in request; - Otherwise, identities reachable from the policy binding's members.
    
    pub identities: Option<Vec<GoogleCloudAssetV1Identity>>,
}

impl client::Part for GoogleCloudAssetV1IdentityList {}


/// A `Constraint` that allows or disallows a list of string values, which are configured by an organization's policy administrator with a `Policy`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1ListConstraint {
    /// Indicates whether values grouped into categories can be used in `Policy.allowed_values` and `Policy.denied_values`. For example, `"in:Python"` would match any value in the 'Python' group.
    #[serde(rename="supportsIn")]
    
    pub supports_in: Option<bool>,
    /// Indicates whether subtrees of Cloud Resource Manager resource hierarchy can be used in `Policy.allowed_values` and `Policy.denied_values`. For example, `"under:folders/123"` would match any resource under the 'folders/123' folder.
    #[serde(rename="supportsUnder")]
    
    pub supports_under: Option<bool>,
}

impl client::Part for GoogleCloudAssetV1ListConstraint {}


/// BigQuery destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1QueryAssetsOutputConfigBigQueryDestination {
    /// Required. The BigQuery dataset where the query results will be saved. It has the format of "projects/{projectId}/datasets/{datasetId}".
    
    pub dataset: Option<String>,
    /// Required. The BigQuery table where the query results will be saved. If this table does not exist, a new table with the given name will be created.
    
    pub table: Option<String>,
    /// Specifies the action that occurs if the destination table or partition already exists. The following values are supported: * WRITE_TRUNCATE: If the table or partition already exists, BigQuery overwrites the entire table or all the partitions data. * WRITE_APPEND: If the table or partition already exists, BigQuery appends the data to the table or the latest partition. * WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_EMPTY.
    #[serde(rename="writeDisposition")]
    
    pub write_disposition: Option<String>,
}

impl client::Part for GoogleCloudAssetV1QueryAssetsOutputConfigBigQueryDestination {}


/// A Google Cloud resource under analysis.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1Resource {
    /// The analysis state of this resource.
    #[serde(rename="analysisState")]
    
    pub analysis_state: Option<IamPolicyAnalysisState>,
    /// The [full resource name](https://cloud.google.com/asset-inventory/docs/resource-name-format)
    #[serde(rename="fullResourceName")]
    
    pub full_resource_name: Option<String>,
}

impl client::Part for GoogleCloudAssetV1Resource {}


/// Represents a rule defined in an organization policy
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1Rule {
    /// Setting this to true means that all values are allowed. This field can be set only in Policies for list constraints.
    #[serde(rename="allowAll")]
    
    pub allow_all: Option<bool>,
    /// The evaluating condition for this rule.
    
    pub condition: Option<Expr>,
    /// Setting this to true means that all values are denied. This field can be set only in Policies for list constraints.
    #[serde(rename="denyAll")]
    
    pub deny_all: Option<bool>,
    /// If `true`, then the `Policy` is enforced. If `false`, then any configuration is acceptable. This field can be set only in Policies for boolean constraints.
    
    pub enforce: Option<bool>,
    /// List of values to be used for this PolicyRule. This field can be set only in Policies for list constraints.
    
    pub values: Option<GoogleCloudAssetV1StringValues>,
}

impl client::Part for GoogleCloudAssetV1Rule {}


/// The string values for the list constraints.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudAssetV1StringValues {
    /// List of values allowed at this resource.
    #[serde(rename="allowedValues")]
    
    pub allowed_values: Option<Vec<String>>,
    /// List of values denied at this resource.
    #[serde(rename="deniedValues")]
    
    pub denied_values: Option<Vec<String>>,
}

impl client::Part for GoogleCloudAssetV1StringValues {}


/// Used in `policy_type` to specify how `boolean_policy` will behave at this resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV1BooleanPolicy {
    /// If `true`, then the `Policy` is enforced. If `false`, then any configuration is acceptable. Suppose you have a `Constraint` `constraints/compute.disableSerialPortAccess` with `constraint_default` set to `ALLOW`. A `Policy` for that `Constraint` exhibits the following behavior: - If the `Policy` at this resource has enforced set to `false`, serial port connection attempts will be allowed. - If the `Policy` at this resource has enforced set to `true`, serial port connection attempts will be refused. - If the `Policy` at this resource is `RestoreDefault`, serial port connection attempts will be allowed. - If no `Policy` is set at this resource or anywhere higher in the resource hierarchy, serial port connection attempts will be allowed. - If no `Policy` is set at this resource, but one exists higher in the resource hierarchy, the behavior is as if the`Policy` were set at this resource. The following examples demonstrate the different possible layerings: Example 1 (nearest `Constraint` wins): `organizations/foo` has a `Policy` with: {enforced: false} `projects/bar` has no `Policy` set. The constraint at `projects/bar` and `organizations/foo` will not be enforced. Example 2 (enforcement gets replaced): `organizations/foo` has a `Policy` with: {enforced: false} `projects/bar` has a `Policy` with: {enforced: true} The constraint at `organizations/foo` is not enforced. The constraint at `projects/bar` is enforced. Example 3 (RestoreDefault): `organizations/foo` has a `Policy` with: {enforced: true} `projects/bar` has a `Policy` with: {RestoreDefault: {}} The constraint at `organizations/foo` is enforced. The constraint at `projects/bar` is not enforced, because `constraint_default` for the `Constraint` is `ALLOW`.
    
    pub enforced: Option<bool>,
}

impl client::Part for GoogleCloudOrgpolicyV1BooleanPolicy {}


/// Used in `policy_type` to specify how `list_policy` behaves at this resource. `ListPolicy` can define specific values and subtrees of Cloud Resource Manager resource hierarchy (`Organizations`, `Folders`, `Projects`) that are allowed or denied by setting the `allowed_values` and `denied_values` fields. This is achieved by using the `under:` and optional `is:` prefixes. The `under:` prefix is used to denote resource subtree values. The `is:` prefix is used to denote specific values, and is required only if the value contains a ":". Values prefixed with "is:" are treated the same as values with no prefix. Ancestry subtrees must be in one of the following formats: - "projects/", e.g. "projects/tokyo-rain-123" - "folders/", e.g. "folders/1234" - "organizations/", e.g. "organizations/1234" The `supports_under` field of the associated `Constraint` defines whether ancestry prefixes can be used. You can set `allowed_values` and `denied_values` in the same `Policy` if `all_values` is `ALL_VALUES_UNSPECIFIED`. `ALLOW` or `DENY` are used to allow or deny all values. If `all_values` is set to either `ALLOW` or `DENY`, `allowed_values` and `denied_values` must be unset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV1ListPolicy {
    /// The policy all_values state.
    #[serde(rename="allValues")]
    
    pub all_values: Option<GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum>,
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

impl client::Part for GoogleCloudOrgpolicyV1ListPolicy {}


/// Defines a Cloud Organization `Policy` which is used to specify `Constraints` for configurations of Cloud Platform resources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV1Policy {
    /// For boolean `Constraints`, whether to enforce the `Constraint` or not.
    #[serde(rename="booleanPolicy")]
    
    pub boolean_policy: Option<GoogleCloudOrgpolicyV1BooleanPolicy>,
    /// The name of the `Constraint` the `Policy` is configuring, for example, `constraints/serviceuser.services`. A [list of available constraints](https://cloud.google.com/resource-manager/docs/organization-policy/org-policy-constraints) is available. Immutable after creation.
    
    pub constraint: Option<String>,
    /// An opaque tag indicating the current version of the `Policy`, used for concurrency control. When the `Policy` is returned from either a `GetPolicy` or a `ListOrgPolicy` request, this `etag` indicates the version of the current `Policy` to use when executing a read-modify-write loop. When the `Policy` is returned from a `GetEffectivePolicy` request, the `etag` will be unset. When the `Policy` is used in a `SetOrgPolicy` method, use the `etag` value that was returned from a `GetOrgPolicy` request as part of a read-modify-write loop for concurrency control. Not setting the `etag`in a `SetOrgPolicy` request will result in an unconditional write of the `Policy`.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// List of values either allowed or disallowed.
    #[serde(rename="listPolicy")]
    
    pub list_policy: Option<GoogleCloudOrgpolicyV1ListPolicy>,
    /// Restores the default behavior of the constraint; independent of `Constraint` type.
    #[serde(rename="restoreDefault")]
    
    pub restore_default: Option<GoogleCloudOrgpolicyV1RestoreDefault>,
    /// The time stamp the `Policy` was previously updated. This is set by the server, not specified by the caller, and represents the last time a call to `SetOrgPolicy` was made for that `Policy`. Any value set by the client will be ignored.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Version of the `Policy`. Default version is 0;
    
    pub version: Option<i32>,
}

impl client::Part for GoogleCloudOrgpolicyV1Policy {}


/// Ignores policies set above this resource and restores the `constraint_default` enforcement behavior of the specific `Constraint` at this resource. Suppose that `constraint_default` is set to `ALLOW` for the `Constraint` `constraints/serviceuser.services`. Suppose that organization foo.com sets a `Policy` at their Organization resource node that restricts the allowed service activations to deny all service activations. They could then set a `Policy` with the `policy_type` `restore_default` on several experimental projects, restoring the `constraint_default` enforcement of the `Constraint` for only those projects, allowing those projects to have all services activated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV1RestoreDefault { _never_set: Option<bool> }

impl client::Part for GoogleCloudOrgpolicyV1RestoreDefault {}


/// An `AccessLevel` is a label that can be applied to requests to Google Cloud services, along with a list of requirements necessary for the label to be applied.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1AccessLevel {
    /// A `BasicLevel` composed of `Conditions`.
    
    pub basic: Option<GoogleIdentityAccesscontextmanagerV1BasicLevel>,
    /// A `CustomLevel` written in the Common Expression Language.
    
    pub custom: Option<GoogleIdentityAccesscontextmanagerV1CustomLevel>,
    /// Description of the `AccessLevel` and its use. Does not affect behavior.
    
    pub description: Option<String>,
    /// Resource name for the `AccessLevel`. Format: `accessPolicies/{access_policy}/accessLevels/{access_level}`. The `access_level` component must begin with a letter, followed by alphanumeric characters or `_`. Its maximum length is 50 characters. After you create an `AccessLevel`, you cannot change its `name`.
    
    pub name: Option<String>,
    /// Human readable title. Must be unique within the Policy.
    
    pub title: Option<String>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1AccessLevel {}


/// `AccessPolicy` is a container for `AccessLevels` (which define the necessary attributes to use Google Cloud services) and `ServicePerimeters` (which define regions of services able to freely pass data within a perimeter). An access policy is globally visible within an organization, and the restrictions it specifies apply to all projects within an organization.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1AccessPolicy {
    /// Output only. An opaque identifier for the current version of the `AccessPolicy`. This will always be a strongly validated etag, meaning that two Access Polices will be identical if and only if their etags are identical. Clients should not expect this to be in any specific format.
    
    pub etag: Option<String>,
    /// Output only. Resource name of the `AccessPolicy`. Format: `accessPolicies/{access_policy}`
    
    pub name: Option<String>,
    /// Required. The parent of this `AccessPolicy` in the Cloud Resource Hierarchy. Currently immutable once created. Format: `organizations/{organization_id}`
    
    pub parent: Option<String>,
    /// The scopes of a policy define which resources an ACM policy can restrict, and where ACM resources can be referenced. For example, a policy with scopes=["folders/123"] has the following behavior: - vpcsc perimeters can only restrict projects within folders/123 - access levels can only be referenced by resources within folders/123. If empty, there are no limitations on which resources can be restricted by an ACM policy, and there are no limitations on where ACM resources can be referenced. Only one policy can include a given scope (attempting to create a second policy which includes "folders/123" will result in an error). Currently, scopes cannot be modified after a policy is created. Currently, policies can only have a single scope. Format: list of `folders/{folder_number}` or `projects/{project_number}`
    
    pub scopes: Option<Vec<String>>,
    /// Required. Human readable title. Does not affect behavior.
    
    pub title: Option<String>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1AccessPolicy {}


/// Identification for an API Operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1ApiOperation {
    /// API methods or permissions to allow. Method or permission must belong to the service specified by `service_name` field. A single MethodSelector entry with `*` specified for the `method` field will allow all methods AND permissions for the service specified in `service_name`.
    #[serde(rename="methodSelectors")]
    
    pub method_selectors: Option<Vec<GoogleIdentityAccesscontextmanagerV1MethodSelector>>,
    /// The name of the API whose methods or permissions the IngressPolicy or EgressPolicy want to allow. A single ApiOperation with `service_name` field set to `*` will allow all methods AND permissions for all services.
    #[serde(rename="serviceName")]
    
    pub service_name: Option<String>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1ApiOperation {}


/// `BasicLevel` is an `AccessLevel` using a set of recommended features.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1BasicLevel {
    /// How the `conditions` list should be combined to determine if a request is granted this `AccessLevel`. If AND is used, each `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. If OR is used, at least one `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. Default behavior is AND.
    #[serde(rename="combiningFunction")]
    
    pub combining_function: Option<GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum>,
    /// Required. A list of requirements for the `AccessLevel` to be granted.
    
    pub conditions: Option<Vec<GoogleIdentityAccesscontextmanagerV1Condition>>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1BasicLevel {}


/// A condition necessary for an `AccessLevel` to be granted. The Condition is an AND over its fields. So a Condition is true if: 1) the request IP is from one of the listed subnetworks AND 2) the originating device complies with the listed device policy AND 3) all listed access levels are granted AND 4) the request was sent at a time allowed by the DateTimeRestriction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1Condition {
    /// Device specific restrictions, all restrictions must hold for the Condition to be true. If not specified, all devices are allowed.
    #[serde(rename="devicePolicy")]
    
    pub device_policy: Option<GoogleIdentityAccesscontextmanagerV1DevicePolicy>,
    /// CIDR block IP subnetwork specification. May be IPv4 or IPv6. Note that for a CIDR IP address block, the specified IP address portion must be properly truncated (i.e. all the host bits must be zero) or the input is considered malformed. For example, "192.0.2.0/24" is accepted but "192.0.2.1/24" is not. Similarly, for IPv6, "2001:db8::/32" is accepted whereas "2001:db8::1/32" is not. The originating IP of a request must be in one of the listed subnets in order for this Condition to be true. If empty, all IP addresses are allowed.
    #[serde(rename="ipSubnetworks")]
    
    pub ip_subnetworks: Option<Vec<String>>,
    /// The request must be made by one of the provided user or service accounts. Groups are not supported. Syntax: `user:{emailid}` `serviceAccount:{emailid}` If not specified, a request may come from any user.
    
    pub members: Option<Vec<String>>,
    /// Whether to negate the Condition. If true, the Condition becomes a NAND over its non-empty fields, each field must be false for the Condition overall to be satisfied. Defaults to false.
    
    pub negate: Option<bool>,
    /// The request must originate from one of the provided countries/regions. Must be valid ISO 3166-1 alpha-2 codes.
    
    pub regions: Option<Vec<String>>,
    /// A list of other access levels defined in the same `Policy`, referenced by resource name. Referencing an `AccessLevel` which does not exist is an error. All access levels listed must be granted for the Condition to be true. Example: "`accessPolicies/MY_POLICY/accessLevels/LEVEL_NAME"`
    #[serde(rename="requiredAccessLevels")]
    
    pub required_access_levels: Option<Vec<String>>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1Condition {}


/// `CustomLevel` is an `AccessLevel` using the Cloud Common Expression Language to represent the necessary conditions for the level to apply to a request. See CEL spec at: https://github.com/google/cel-spec
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1CustomLevel {
    /// Required. A Cloud CEL expression evaluating to a boolean.
    
    pub expr: Option<Expr>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1CustomLevel {}


/// `DevicePolicy` specifies device specific restrictions necessary to acquire a given access level. A `DevicePolicy` specifies requirements for requests from devices to be granted access levels, it does not do any enforcement on the device. `DevicePolicy` acts as an AND over all specified fields, and each repeated field is an OR over its elements. Any unset fields are ignored. For example, if the proto is { os_type : DESKTOP_WINDOWS, os_type : DESKTOP_LINUX, encryption_status: ENCRYPTED}, then the DevicePolicy will be true for requests originating from encrypted Linux desktops and encrypted Windows desktops.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1DevicePolicy {
    /// Allowed device management levels, an empty list allows all management levels.
    #[serde(rename="allowedDeviceManagementLevels")]
    
    pub allowed_device_management_levels: Option<Vec<GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum>>,
    /// Allowed encryptions statuses, an empty list allows all statuses.
    #[serde(rename="allowedEncryptionStatuses")]
    
    pub allowed_encryption_statuses: Option<Vec<GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum>>,
    /// Allowed OS versions, an empty list allows all types and all versions.
    #[serde(rename="osConstraints")]
    
    pub os_constraints: Option<Vec<GoogleIdentityAccesscontextmanagerV1OsConstraint>>,
    /// Whether the device needs to be approved by the customer admin.
    #[serde(rename="requireAdminApproval")]
    
    pub require_admin_approval: Option<bool>,
    /// Whether the device needs to be corp owned.
    #[serde(rename="requireCorpOwned")]
    
    pub require_corp_owned: Option<bool>,
    /// Whether or not screenlock is required for the DevicePolicy to be true. Defaults to `false`.
    #[serde(rename="requireScreenlock")]
    
    pub require_screenlock: Option<bool>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1DevicePolicy {}


/// Defines the conditions under which an EgressPolicy matches a request. Conditions based on information about the source of the request. Note that if the destination of the request is also protected by a ServicePerimeter, then that ServicePerimeter must have an IngressPolicy which allows access in order for this request to succeed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1EgressFrom {
    /// A list of identities that are allowed access through this [EgressPolicy]. Should be in the format of email address. The email address should represent individual user or service account only.
    
    pub identities: Option<Vec<String>>,
    /// Specifies the type of identities that are allowed access to outside the perimeter. If left unspecified, then members of `identities` field will be allowed access.
    #[serde(rename="identityType")]
    
    pub identity_type: Option<GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1EgressFrom {}


/// Policy for egress from perimeter. EgressPolicies match requests based on `egress_from` and `egress_to` stanzas. For an EgressPolicy to match, both `egress_from` and `egress_to` stanzas must be matched. If an EgressPolicy matches a request, the request is allowed to span the ServicePerimeter boundary. For example, an EgressPolicy can be used to allow VMs on networks within the ServicePerimeter to access a defined set of projects outside the perimeter in certain contexts (e.g. to read data from a Cloud Storage bucket or query against a BigQuery dataset). EgressPolicies are concerned with the *resources* that a request relates as well as the API services and API actions being used. They do not related to the direction of data movement. More detailed documentation for this concept can be found in the descriptions of EgressFrom and EgressTo.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1EgressPolicy {
    /// Defines conditions on the source of a request causing this EgressPolicy to apply.
    #[serde(rename="egressFrom")]
    
    pub egress_from: Option<GoogleIdentityAccesscontextmanagerV1EgressFrom>,
    /// Defines the conditions on the ApiOperation and destination resources that cause this EgressPolicy to apply.
    #[serde(rename="egressTo")]
    
    pub egress_to: Option<GoogleIdentityAccesscontextmanagerV1EgressTo>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1EgressPolicy {}


/// Defines the conditions under which an EgressPolicy matches a request. Conditions are based on information about the ApiOperation intended to be performed on the `resources` specified. Note that if the destination of the request is also protected by a ServicePerimeter, then that ServicePerimeter must have an IngressPolicy which allows access in order for this request to succeed. The request must match `operations` AND `resources` fields in order to be allowed egress out of the perimeter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1EgressTo {
    /// A list of external resources that are allowed to be accessed. Only AWS and Azure resources are supported. For Amazon S3, the supported format is s3://BUCKET_NAME. For Azure Storage, the supported format is azure://myaccount.blob.core.windows.net/CONTAINER_NAME. A request matches if it contains an external resource in this list (Example: s3://bucket/path). Currently '*' is not allowed.
    #[serde(rename="externalResources")]
    
    pub external_resources: Option<Vec<String>>,
    /// A list of ApiOperations allowed to be performed by the sources specified in the corresponding EgressFrom. A request matches if it uses an operation/service in this list.
    
    pub operations: Option<Vec<GoogleIdentityAccesscontextmanagerV1ApiOperation>>,
    /// A list of resources, currently only projects in the form `projects/`, that are allowed to be accessed by sources defined in the corresponding EgressFrom. A request matches if it contains a resource in this list. If `*` is specified for `resources`, then this EgressTo rule will authorize access to all resources outside the perimeter.
    
    pub resources: Option<Vec<String>>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1EgressTo {}


/// Defines the conditions under which an IngressPolicy matches a request. Conditions are based on information about the source of the request. The request must satisfy what is defined in `sources` AND identity related fields in order to match.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1IngressFrom {
    /// A list of identities that are allowed access through this ingress policy. Should be in the format of email address. The email address should represent individual user or service account only.
    
    pub identities: Option<Vec<String>>,
    /// Specifies the type of identities that are allowed access from outside the perimeter. If left unspecified, then members of `identities` field will be allowed access.
    #[serde(rename="identityType")]
    
    pub identity_type: Option<GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum>,
    /// Sources that this IngressPolicy authorizes access from.
    
    pub sources: Option<Vec<GoogleIdentityAccesscontextmanagerV1IngressSource>>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1IngressFrom {}


/// Policy for ingress into ServicePerimeter. IngressPolicies match requests based on `ingress_from` and `ingress_to` stanzas. For an ingress policy to match, both the `ingress_from` and `ingress_to` stanzas must be matched. If an IngressPolicy matches a request, the request is allowed through the perimeter boundary from outside the perimeter. For example, access from the internet can be allowed either based on an AccessLevel or, for traffic hosted on Google Cloud, the project of the source network. For access from private networks, using the project of the hosting network is required. Individual ingress policies can be limited by restricting which services and/or actions they match using the `ingress_to` field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1IngressPolicy {
    /// Defines the conditions on the source of a request causing this IngressPolicy to apply.
    #[serde(rename="ingressFrom")]
    
    pub ingress_from: Option<GoogleIdentityAccesscontextmanagerV1IngressFrom>,
    /// Defines the conditions on the ApiOperation and request destination that cause this IngressPolicy to apply.
    #[serde(rename="ingressTo")]
    
    pub ingress_to: Option<GoogleIdentityAccesscontextmanagerV1IngressTo>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1IngressPolicy {}


/// The source that IngressPolicy authorizes access from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1IngressSource {
    /// An AccessLevel resource name that allow resources within the ServicePerimeters to be accessed from the internet. AccessLevels listed must be in the same policy as this ServicePerimeter. Referencing a nonexistent AccessLevel will cause an error. If no AccessLevel names are listed, resources within the perimeter can only be accessed via Google Cloud calls with request origins within the perimeter. Example: `accessPolicies/MY_POLICY/accessLevels/MY_LEVEL`. If a single `*` is specified for `access_level`, then all IngressSources will be allowed.
    #[serde(rename="accessLevel")]
    
    pub access_level: Option<String>,
    /// A Google Cloud resource that is allowed to ingress the perimeter. Requests from these resources will be allowed to access perimeter data. Currently only projects and VPCs are allowed. Project format: `projects/{project_number}` VPC format: `//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}`. The project may be in any Google Cloud organization, not just the organization that the perimeter is defined in. `*` is not allowed, the case of allowing all Google Cloud resources only is not supported.
    
    pub resource: Option<String>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1IngressSource {}


/// Defines the conditions under which an IngressPolicy matches a request. Conditions are based on information about the ApiOperation intended to be performed on the target resource of the request. The request must satisfy what is defined in `operations` AND `resources` in order to match.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1IngressTo {
    /// A list of ApiOperations allowed to be performed by the sources specified in corresponding IngressFrom in this ServicePerimeter.
    
    pub operations: Option<Vec<GoogleIdentityAccesscontextmanagerV1ApiOperation>>,
    /// A list of resources, currently only projects in the form `projects/`, protected by this ServicePerimeter that are allowed to be accessed by sources defined in the corresponding IngressFrom. If a single `*` is specified, then access to all resources inside the perimeter are allowed.
    
    pub resources: Option<Vec<String>>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1IngressTo {}


/// An allowed method or permission of a service specified in ApiOperation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1MethodSelector {
    /// Value for `method` should be a valid method name for the corresponding `service_name` in ApiOperation. If `*` used as value for `method`, then ALL methods and permissions are allowed.
    
    pub method: Option<String>,
    /// Value for `permission` should be a valid Cloud IAM permission for the corresponding `service_name` in ApiOperation.
    
    pub permission: Option<String>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1MethodSelector {}


/// A restriction on the OS type and version of devices making requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1OsConstraint {
    /// The minimum allowed OS version. If not set, any version of this OS satisfies the constraint. Format: `"major.minor.patch"`. Examples: `"10.5.301"`, `"9.2.1"`.
    #[serde(rename="minimumVersion")]
    
    pub minimum_version: Option<String>,
    /// Required. The allowed OS type.
    #[serde(rename="osType")]
    
    pub os_type: Option<GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum>,
    /// Only allows requests from devices with a verified Chrome OS. Verifications includes requirements that the device is enterprise-managed, conformant to domain policies, and the caller has permission to call the API targeted by the request.
    #[serde(rename="requireVerifiedChromeOs")]
    
    pub require_verified_chrome_os: Option<bool>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1OsConstraint {}


/// `ServicePerimeter` describes a set of Google Cloud resources which can freely import and export data amongst themselves, but not export outside of the `ServicePerimeter`. If a request with a source within this `ServicePerimeter` has a target outside of the `ServicePerimeter`, the request will be blocked. Otherwise the request is allowed. There are two types of Service Perimeter - Regular and Bridge. Regular Service Perimeters cannot overlap, a single Google Cloud project can only belong to a single regular Service Perimeter. Service Perimeter Bridges can contain only Google Cloud projects as members, a single Google Cloud project may belong to multiple Service Perimeter Bridges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1ServicePerimeter {
    /// Description of the `ServicePerimeter` and its use. Does not affect behavior.
    
    pub description: Option<String>,
    /// Resource name for the `ServicePerimeter`. Format: `accessPolicies/{access_policy}/servicePerimeters/{service_perimeter}`. The `service_perimeter` component must begin with a letter, followed by alphanumeric characters or `_`. After you create a `ServicePerimeter`, you cannot change its `name`.
    
    pub name: Option<String>,
    /// Perimeter type indicator. A single project is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, the restricted service list as well as access level lists must be empty.
    #[serde(rename="perimeterType")]
    
    pub perimeter_type: Option<GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum>,
    /// Proposed (or dry run) ServicePerimeter configuration. This configuration allows to specify and test ServicePerimeter configuration without enforcing actual access restrictions. Only allowed to be set when the "use_explicit_dry_run_spec" flag is set.
    
    pub spec: Option<GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig>,
    /// Current ServicePerimeter configuration. Specifies sets of resources, restricted services and access levels that determine perimeter content and boundaries.
    
    pub status: Option<GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig>,
    /// Human readable title. Must be unique within the Policy.
    
    pub title: Option<String>,
    /// Use explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists for all Service Perimeters, and that spec is identical to the status for those Service Perimeters. When this flag is set, it inhibits the generation of the implicit spec, thereby allowing the user to explicitly provide a configuration ("spec") to use in a dry-run version of the Service Perimeter. This allows the user to test changes to the enforced config ("status") without actually enforcing them. This testing is done through analyzing the differences between currently enforced and suggested restrictions. use_explicit_dry_run_spec must bet set to True if any of the fields in the spec are set to non-default values.
    #[serde(rename="useExplicitDryRunSpec")]
    
    pub use_explicit_dry_run_spec: Option<bool>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1ServicePerimeter {}


/// `ServicePerimeterConfig` specifies a set of Google Cloud resources that describe specific Service Perimeter configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig {
    /// A list of `AccessLevel` resource names that allow resources within the `ServicePerimeter` to be accessed from the internet. `AccessLevels` listed must be in the same policy as this `ServicePerimeter`. Referencing a nonexistent `AccessLevel` is a syntax error. If no `AccessLevel` names are listed, resources within the perimeter can only be accessed via Google Cloud calls with request origins within the perimeter. Example: `"accessPolicies/MY_POLICY/accessLevels/MY_LEVEL"`. For Service Perimeter Bridge, must be empty.
    #[serde(rename="accessLevels")]
    
    pub access_levels: Option<Vec<String>>,
    /// List of EgressPolicies to apply to the perimeter. A perimeter may have multiple EgressPolicies, each of which is evaluated separately. Access is granted if any EgressPolicy grants it. Must be empty for a perimeter bridge.
    #[serde(rename="egressPolicies")]
    
    pub egress_policies: Option<Vec<GoogleIdentityAccesscontextmanagerV1EgressPolicy>>,
    /// List of IngressPolicies to apply to the perimeter. A perimeter may have multiple IngressPolicies, each of which is evaluated separately. Access is granted if any Ingress Policy grants it. Must be empty for a perimeter bridge.
    #[serde(rename="ingressPolicies")]
    
    pub ingress_policies: Option<Vec<GoogleIdentityAccesscontextmanagerV1IngressPolicy>>,
    /// A list of Google Cloud resources that are inside of the service perimeter. Currently only projects and VPCs are allowed. Project format: `projects/{project_number}` VPC format: `//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}`.
    
    pub resources: Option<Vec<String>>,
    /// Google Cloud services that are subject to the Service Perimeter restrictions. For example, if `storage.googleapis.com` is specified, access to the storage buckets inside the perimeter must meet the perimeter's access restrictions.
    #[serde(rename="restrictedServices")]
    
    pub restricted_services: Option<Vec<String>>,
    /// Configuration for APIs allowed within Perimeter.
    #[serde(rename="vpcAccessibleServices")]
    
    pub vpc_accessible_services: Option<GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig {}


/// Specifies how APIs are allowed to communicate within the Service Perimeter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices {
    /// The list of APIs usable within the Service Perimeter. Must be empty unless 'enable_restriction' is True. You can specify a list of individual services, as well as include the 'RESTRICTED-SERVICES' value, which automatically includes all of the services protected by the perimeter.
    #[serde(rename="allowedServices")]
    
    pub allowed_services: Option<Vec<String>>,
    /// Whether to restrict API calls within the Service Perimeter to the list of APIs specified in 'allowed_services'.
    #[serde(rename="enableRestriction")]
    
    pub enable_restriction: Option<bool>,
}

impl client::Part for GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices {}


/// An analysis message to group the query and results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IamPolicyAnalysis {
    /// The analysis query.
    #[serde(rename="analysisQuery")]
    
    pub analysis_query: Option<IamPolicyAnalysisQuery>,
    /// A list of IamPolicyAnalysisResult that matches the analysis query, or empty if no result is found.
    #[serde(rename="analysisResults")]
    
    pub analysis_results: Option<Vec<IamPolicyAnalysisResult>>,
    /// Represents whether all entries in the analysis_results have been fully explored to answer the query.
    #[serde(rename="fullyExplored")]
    
    pub fully_explored: Option<bool>,
    /// A list of non-critical errors happened during the query handling.
    #[serde(rename="nonCriticalErrors")]
    
    pub non_critical_errors: Option<Vec<IamPolicyAnalysisState>>,
}

impl client::Part for IamPolicyAnalysis {}


/// Output configuration for export IAM policy analysis destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IamPolicyAnalysisOutputConfig {
    /// Destination on BigQuery.
    #[serde(rename="bigqueryDestination")]
    
    pub bigquery_destination: Option<GoogleCloudAssetV1BigQueryDestination>,
    /// Destination on Cloud Storage.
    #[serde(rename="gcsDestination")]
    
    pub gcs_destination: Option<GoogleCloudAssetV1GcsDestination>,
}

impl client::Part for IamPolicyAnalysisOutputConfig {}


/// IAM policy analysis query message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IamPolicyAnalysisQuery {
    /// Optional. Specifies roles or permissions for analysis. This is optional.
    #[serde(rename="accessSelector")]
    
    pub access_selector: Option<AccessSelector>,
    /// Optional. The hypothetical context for IAM conditions evaluation.
    #[serde(rename="conditionContext")]
    
    pub condition_context: Option<ConditionContext>,
    /// Optional. Specifies an identity for analysis.
    #[serde(rename="identitySelector")]
    
    pub identity_selector: Option<IdentitySelector>,
    /// Optional. The query options.
    
    pub options: Option<Options>,
    /// Optional. Specifies a resource for analysis.
    #[serde(rename="resourceSelector")]
    
    pub resource_selector: Option<ResourceSelector>,
    /// Required. The relative name of the root asset. Only resources and IAM policies within the scope will be analyzed. This can only be an organization number (such as "organizations/123"), a folder number (such as "folders/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345"). To know how to get organization id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id). To know how to get folder or project id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-folders#viewing_or_listing_folders_and_projects).
    
    pub scope: Option<String>,
}

impl client::Part for IamPolicyAnalysisQuery {}


/// IAM Policy analysis result, consisting of one IAM policy binding and derived access control lists.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IamPolicyAnalysisResult {
    /// The access control lists derived from the iam_binding that match or potentially match resource and access selectors specified in the request.
    #[serde(rename="accessControlLists")]
    
    pub access_control_lists: Option<Vec<GoogleCloudAssetV1AccessControlList>>,
    /// The [full resource name](https://cloud.google.com/asset-inventory/docs/resource-name-format) of the resource to which the iam_binding policy attaches.
    #[serde(rename="attachedResourceFullName")]
    
    pub attached_resource_full_name: Option<String>,
    /// Represents whether all analyses on the iam_binding have successfully finished.
    #[serde(rename="fullyExplored")]
    
    pub fully_explored: Option<bool>,
    /// The IAM policy binding under analysis.
    #[serde(rename="iamBinding")]
    
    pub iam_binding: Option<Binding>,
    /// The identity list derived from members of the iam_binding that match or potentially match identity selector specified in the request.
    #[serde(rename="identityList")]
    
    pub identity_list: Option<GoogleCloudAssetV1IdentityList>,
}

impl client::Part for IamPolicyAnalysisResult {}


/// Represents the detailed state of an entity under analysis, such as a resource, an identity or an access.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IamPolicyAnalysisState {
    /// The human-readable description of the cause of failure.
    
    pub cause: Option<String>,
    /// The Google standard error code that best describes the state. For example: - OK means the analysis on this entity has been successfully finished; - PERMISSION_DENIED means an access denied error is encountered; - DEADLINE_EXCEEDED means the analysis on this entity hasn't been started in time;
    
    pub code: Option<IamPolicyAnalysisStateCodeEnum>,
}

impl client::Part for IamPolicyAnalysisState {}


/// A result of IAM Policy search, containing information of an IAM policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IamPolicySearchResult {
    /// The type of the resource associated with this IAM policy. Example: `compute.googleapis.com/Disk`. To search against the `asset_type`: * specify the `asset_types` field in your search request.
    #[serde(rename="assetType")]
    
    pub asset_type: Option<String>,
    /// Explanation about the IAM policy search result. It contains additional information to explain why the search result matches the query.
    
    pub explanation: Option<Explanation>,
    /// The folder(s) that the IAM policy belongs to, in the form of folders/{FOLDER_NUMBER}. This field is available when the IAM policy belongs to one or more folders. To search against `folders`: * use a field query. Example: `folders:(123 OR 456)` * use a free text query. Example: `123` * specify the `scope` field as this folder in your search request.
    
    pub folders: Option<Vec<String>>,
    /// The organization that the IAM policy belongs to, in the form of organizations/{ORGANIZATION_NUMBER}. This field is available when the IAM policy belongs to an organization. To search against `organization`: * use a field query. Example: `organization:123` * use a free text query. Example: `123` * specify the `scope` field as this organization in your search request.
    
    pub organization: Option<String>,
    /// The IAM policy directly set on the given resource. Note that the original IAM policy can contain multiple bindings. This only contains the bindings that match the given query. For queries that don't contain a constrain on policies (e.g., an empty query), this contains all the bindings. To search against the `policy` bindings: * use a field query: - query by the policy contained members. Example: `policy:amy@gmail.com` - query by the policy contained roles. Example: `policy:roles/compute.admin` - query by the policy contained roles' included permissions. Example: `policy.role.permissions:compute.instances.create`
    
    pub policy: Option<Policy>,
    /// The project that the associated Google Cloud resource belongs to, in the form of projects/{PROJECT_NUMBER}. If an IAM policy is set on a resource (like VM instance, Cloud Storage bucket), the project field will indicate the project that contains the resource. If an IAM policy is set on a folder or orgnization, this field will be empty. To search against the `project`: * specify the `scope` field as this project in your search request.
    
    pub project: Option<String>,
    /// The full resource name of the resource associated with this IAM policy. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`. See [Cloud Asset Inventory Resource Name Format](https://cloud.google.com/asset-inventory/docs/resource-name-format) for more information. To search against the `resource`: * use a field query. Example: `resource:organizations/123`
    
    pub resource: Option<String>,
}

impl client::Part for IamPolicySearchResult {}


/// Specifies an identity for which to determine resource access, based on roles assigned either directly to them or to the groups they belong to, directly or indirectly.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentitySelector {
    /// Required. The identity appear in the form of principals in [IAM policy binding](https://cloud.google.com/iam/reference/rest/v1/Binding). The examples of supported forms are: "user:mike@example.com", "group:admins@example.com", "domain:google.com", "serviceAccount:my-project-id@appspot.gserviceaccount.com". Notice that wildcard characters (such as * and ?) are not supported. You must give a specific identity.
    
    pub identity: Option<String>,
}

impl client::Part for IdentitySelector {}


/// This API resource represents the available inventory data for a Compute Engine virtual machine (VM) instance at a given point in time. You can use this API resource to determine the inventory data of your VM. For more information, see [Information provided by OS inventory management](https://cloud.google.com/compute/docs/instances/os-inventory-management#data-collected).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Inventory {
    /// Inventory items related to the VM keyed by an opaque unique identifier for each inventory item. The identifier is unique to each distinct and addressable inventory item and will change, when there is a new package version.
    
    pub items: Option<HashMap<String, Item>>,
    /// Output only. The `Inventory` API resource name. Format: `projects/{project_number}/locations/{location}/instances/{instance_id}/inventory`
    
    pub name: Option<String>,
    /// Base level operating system information for the VM.
    #[serde(rename="osInfo")]
    
    pub os_info: Option<OsInfo>,
    /// Output only. Timestamp of the last reported inventory for the VM.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Inventory {}


/// A single piece of inventory on a VM.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    /// Software package available to be installed on the VM instance.
    #[serde(rename="availablePackage")]
    
    pub available_package: Option<SoftwarePackage>,
    /// When this inventory item was first detected.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Identifier for this item, unique across items for this VM.
    
    pub id: Option<String>,
    /// Software package present on the VM instance.
    #[serde(rename="installedPackage")]
    
    pub installed_package: Option<SoftwarePackage>,
    /// The origin of this inventory item.
    #[serde(rename="originType")]
    
    pub origin_type: Option<ItemOriginTypeEnum>,
    /// The specific type of inventory, correlating to its specific details.
    #[serde(rename="type")]
    
    pub type_: Option<ItemTypeEnum>,
    /// When this inventory item was last modified.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Item {}


/// ListAssets response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list assets](AssetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAssetsResponse {
    /// Assets.
    
    pub assets: Option<Vec<Asset>>,
    /// Token to retrieve the next page of results. It expires 72 hours after the page token for the first page is generated. Set to empty if there are no remaining results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Time the snapshot was taken.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for ListAssetsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list feeds](FeedListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFeedsResponse {
    /// A list of feeds.
    
    pub feeds: Option<Vec<Feed>>,
}

impl client::ResponseResult for ListFeedsResponse {}


/// Response of listing saved queries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list saved queries](SavedQueryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSavedQueriesResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of savedQueries.
    #[serde(rename="savedQueries")]
    
    pub saved_queries: Option<Vec<SavedQuery>>,
}

impl client::ResponseResult for ListSavedQueriesResponse {}


/// A message to group the analysis information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MoveAnalysis {
    /// Analysis result of moving the target resource.
    
    pub analysis: Option<MoveAnalysisResult>,
    /// The user friendly display name of the analysis. E.g. IAM, organization policy etc.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Description of error encountered when performing the analysis.
    
    pub error: Option<Status>,
}

impl client::Part for MoveAnalysis {}


/// An analysis result including blockers and warnings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MoveAnalysisResult {
    /// Blocking information that would prevent the target resource from moving to the specified destination at runtime.
    
    pub blockers: Option<Vec<MoveImpact>>,
    /// Warning information indicating that moving the target resource to the specified destination might be unsafe. This can include important policy information and configuration changes, but will not block moves at runtime.
    
    pub warnings: Option<Vec<MoveImpact>>,
}

impl client::Part for MoveAnalysisResult {}


/// A message to group impacts of moving the target resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MoveImpact {
    /// User friendly impact detail in a free form message.
    
    pub detail: Option<String>,
}

impl client::Part for MoveImpact {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get operations](OperationGetCall) (response)
/// * [analyze iam policy longrunning](MethodAnalyzeIamPolicyLongrunningCall) (response)
/// * [export assets](MethodExportAssetCall) (response)
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


/// Contains query options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Options {
    /// Optional. If true, the response will include access analysis from identities to resources via service account impersonation. This is a very expensive operation, because many derived queries will be executed. We highly recommend you use AssetService.AnalyzeIamPolicyLongrunning RPC instead. For example, if the request analyzes for which resources user A has permission P, and there's an IAM policy states user A has iam.serviceAccounts.getAccessToken permission to a service account SA, and there's another IAM policy states service account SA has permission P to a Google Cloud folder F, then user A potentially has access to the Google Cloud folder F. And those advanced analysis results will be included in AnalyzeIamPolicyResponse.service_account_impersonation_analysis. Another example, if the request analyzes for who has permission P to a Google Cloud folder F, and there's an IAM policy states user A has iam.serviceAccounts.actAs permission to a service account SA, and there's another IAM policy states service account SA has permission P to the Google Cloud folder F, then user A potentially has access to the Google Cloud folder F. And those advanced analysis results will be included in AnalyzeIamPolicyResponse.service_account_impersonation_analysis. Only the following permissions are considered in this analysis: * `iam.serviceAccounts.actAs` * `iam.serviceAccounts.signBlob` * `iam.serviceAccounts.signJwt` * `iam.serviceAccounts.getAccessToken` * `iam.serviceAccounts.getOpenIdToken` * `iam.serviceAccounts.implicitDelegation` Default is false.
    #[serde(rename="analyzeServiceAccountImpersonation")]
    
    pub analyze_service_account_impersonation: Option<bool>,
    /// Optional. If true, the identities section of the result will expand any Google groups appearing in an IAM policy binding. If IamPolicyAnalysisQuery.identity_selector is specified, the identity in the result will be determined by the selector, and this flag is not allowed to set. If true, the default max expansion per group is 1000 for AssetService.AnalyzeIamPolicy][]. Default is false.
    #[serde(rename="expandGroups")]
    
    pub expand_groups: Option<bool>,
    /// Optional. If true and IamPolicyAnalysisQuery.resource_selector is not specified, the resource section of the result will expand any resource attached to an IAM policy to include resources lower in the resource hierarchy. For example, if the request analyzes for which resources user A has permission P, and the results include an IAM policy with P on a Google Cloud folder, the results will also include resources in that folder with permission P. If true and IamPolicyAnalysisQuery.resource_selector is specified, the resource section of the result will expand the specified resource to include resources lower in the resource hierarchy. Only project or lower resources are supported. Folder and organization resources cannot be used together with this option. For example, if the request analyzes for which users have permission P on a Google Cloud project with this option enabled, the results will include all users who have permission P on that project or any lower resource. If true, the default max expansion per resource is 1000 for AssetService.AnalyzeIamPolicy][] and 100000 for AssetService.AnalyzeIamPolicyLongrunning][]. Default is false.
    #[serde(rename="expandResources")]
    
    pub expand_resources: Option<bool>,
    /// Optional. If true, the access section of result will expand any roles appearing in IAM policy bindings to include their permissions. If IamPolicyAnalysisQuery.access_selector is specified, the access section of the result will be determined by the selector, and this flag is not allowed to set. Default is false.
    #[serde(rename="expandRoles")]
    
    pub expand_roles: Option<bool>,
    /// Optional. If true, the result will output the relevant membership relationships between groups and other groups, and between groups and principals. Default is false.
    #[serde(rename="outputGroupEdges")]
    
    pub output_group_edges: Option<bool>,
    /// Optional. If true, the result will output the relevant parent/child relationships between resources. Default is false.
    #[serde(rename="outputResourceEdges")]
    
    pub output_resource_edges: Option<bool>,
}

impl client::Part for Options {}


/// The organization policy result to the query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrgPolicyResult {
    /// The consolidated organization policy for the analyzed resource. The consolidated organization policy is computed by merging and evaluating AnalyzeOrgPoliciesResponse.policy_bundle. The evaluation will respect the organization policy [hierarchy rules](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-hierarchy).
    #[serde(rename="consolidatedPolicy")]
    
    pub consolidated_policy: Option<AnalyzerOrgPolicy>,
    /// The ordered list of all organization policies from the AnalyzeOrgPoliciesResponse.OrgPolicyResult.consolidated_policy.attached_resource. to the scope specified in the request. If the constraint is defined with default policy, it will also appear in the list.
    #[serde(rename="policyBundle")]
    
    pub policy_bundle: Option<Vec<AnalyzerOrgPolicy>>,
}

impl client::Part for OrgPolicyResult {}


/// Operating system information for the VM.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OsInfo {
    /// The system architecture of the operating system.
    
    pub architecture: Option<String>,
    /// The VM hostname.
    
    pub hostname: Option<String>,
    /// The kernel release of the operating system.
    #[serde(rename="kernelRelease")]
    
    pub kernel_release: Option<String>,
    /// The kernel version of the operating system.
    #[serde(rename="kernelVersion")]
    
    pub kernel_version: Option<String>,
    /// The operating system long name. For example 'Debian GNU/Linux 9' or 'Microsoft Window Server 2019 Datacenter'.
    #[serde(rename="longName")]
    
    pub long_name: Option<String>,
    /// The current version of the OS Config agent running on the VM.
    #[serde(rename="osconfigAgentVersion")]
    
    pub osconfig_agent_version: Option<String>,
    /// The operating system short name. For example, 'windows' or 'debian'.
    #[serde(rename="shortName")]
    
    pub short_name: Option<String>,
    /// The version of the operating system.
    
    pub version: Option<String>,
}

impl client::Part for OsInfo {}


/// Output configuration for export assets destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Destination on BigQuery. The output table stores the fields in asset Protobuf as columns in BigQuery.
    #[serde(rename="bigqueryDestination")]
    
    pub bigquery_destination: Option<BigQueryDestination>,
    /// Destination on Cloud Storage.
    #[serde(rename="gcsDestination")]
    
    pub gcs_destination: Option<GcsDestination>,
}

impl client::Part for OutputConfig {}


/// Specifications of BigQuery partitioned table as export destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartitionSpec {
    /// The partition key for BigQuery partitioned table.
    #[serde(rename="partitionKey")]
    
    pub partition_key: Option<PartitionSpecPartitionKeyEnum>,
}

impl client::Part for PartitionSpec {}


/// IAM permissions
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Permissions {
    /// A list of permissions. A sample permission string: `compute.disk.get`.
    
    pub permissions: Option<Vec<String>>,
}

impl client::Part for Permissions {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { "bindings": [ { "role": "roles/resourcemanager.organizationAdmin", "members": [ "user:mike@example.com", "group:admins@example.com", "domain:google.com", "serviceAccount:my-project-id@appspot.gserviceaccount.com" ] }, { "role": "roles/resourcemanager.organizationViewer", "members": [ "user:eve@example.com" ], "condition": { "title": "expirable access", "description": "Does not grant access after Sep 2020", "expression": "request.time < timestamp('2020-10-01T00:00:00.000Z')", } } ], "etag": "BwWWja0YfJA=", "version": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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

impl client::Part for Policy {}


/// The IAM policy and its attached resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PolicyInfo {
    /// The full resource name the policy is directly attached to.
    #[serde(rename="attachedResource")]
    
    pub attached_resource: Option<String>,
    /// The IAM policy that's directly attached to the attached_resource.
    
    pub policy: Option<Policy>,
}

impl client::Part for PolicyInfo {}


/// A Pub/Sub destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PubsubDestination {
    /// The name of the Pub/Sub topic to publish to. Example: `projects/PROJECT_ID/topics/TOPIC_ID`.
    
    pub topic: Option<String>,
}

impl client::Part for PubsubDestination {}


/// Output configuration query assets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryAssetsOutputConfig {
    /// BigQuery destination where the query results will be saved.
    #[serde(rename="bigqueryDestination")]
    
    pub bigquery_destination: Option<GoogleCloudAssetV1QueryAssetsOutputConfigBigQueryDestination>,
}

impl client::Part for QueryAssetsOutputConfig {}


/// QueryAssets request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query assets](MethodQueryAssetCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryAssetsRequest {
    /// Optional. Reference to the query job, which is from the `QueryAssetsResponse` of previous `QueryAssets` call.
    #[serde(rename="jobReference")]
    
    pub job_reference: Option<String>,
    /// Optional. Destination where the query results will be saved. When this field is specified, the query results won't be saved in the [QueryAssetsResponse.query_result]. Instead [QueryAssetsResponse.output_config] will be set. Meanwhile, [QueryAssetsResponse.job_reference] will be set and can be used to check the status of the query job when passed to a following [QueryAssets] API call.
    #[serde(rename="outputConfig")]
    
    pub output_config: Option<QueryAssetsOutputConfig>,
    /// Optional. The maximum number of rows to return in the results. Responses are limited to 10 MB and 1000 rows. By default, the maximum row count is 1000. When the byte or row count limit is reached, the rest of the query results will be paginated. The field will be ignored when [output_config] is specified.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional. A page token received from previous `QueryAssets`. The field will be ignored when [output_config] is specified.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Optional. Queries cloud assets as they appeared at the specified point in time.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. [start_time] is required. [start_time] must be less than [end_time] Defaults [end_time] to now if [start_time] is set and [end_time] isn't. Maximum permitted time range is 7 days.
    #[serde(rename="readTimeWindow")]
    
    pub read_time_window: Option<TimeWindow>,
    /// Optional. A SQL statement that's compatible with [BigQuery Standard SQL](http://cloud/bigquery/docs/reference/standard-sql/enabling-standard-sql).
    
    pub statement: Option<String>,
    /// Optional. Specifies the maximum amount of time that the client is willing to wait for the query to complete. By default, this limit is 5 min for the first query, and 1 minute for the following queries. If the query is complete, the `done` field in the `QueryAssetsResponse` is true, otherwise false. Like BigQuery [jobs.query API](https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/query#queryrequest) The call is not guaranteed to wait for the specified timeout; it typically returns after around 200 seconds (200,000 milliseconds), even if the query is not complete. The field will be ignored when [output_config] is specified.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub timeout: Option<client::chrono::Duration>,
}

impl client::RequestValue for QueryAssetsRequest {}


/// QueryAssets response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query assets](MethodQueryAssetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryAssetsResponse {
    /// The query response, which can be either an `error` or a valid `response`. If `done` == `false` and the query result is being saved in a output, the output_config field will be set. If `done` == `true`, exactly one of `error`, `query_result` or `output_config` will be set.
    
    pub done: Option<bool>,
    /// Error status.
    
    pub error: Option<Status>,
    /// Reference to a query job.
    #[serde(rename="jobReference")]
    
    pub job_reference: Option<String>,
    /// Output configuration which indicates instead of being returned in API response on the fly, the query result will be saved in a specific output.
    #[serde(rename="outputConfig")]
    
    pub output_config: Option<QueryAssetsOutputConfig>,
    /// Result of the query.
    #[serde(rename="queryResult")]
    
    pub query_result: Option<QueryResult>,
}

impl client::ResponseResult for QueryAssetsResponse {}


/// The query content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryContent {
    /// An IAM Policy Analysis query, which could be used in the AssetService.AnalyzeIamPolicy RPC or the AssetService.AnalyzeIamPolicyLongrunning RPC.
    #[serde(rename="iamPolicyAnalysisQuery")]
    
    pub iam_policy_analysis_query: Option<IamPolicyAnalysisQuery>,
}

impl client::Part for QueryContent {}


/// Execution results of the query. The result is formatted as rows represented by BigQuery compatible [schema]. When pagination is necessary, it will contains the page token to retrieve the results of following pages.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryResult {
    /// Token to retrieve the next page of the results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Each row hold a query result in the format of `Struct`.
    
    pub rows: Option<Vec<HashMap<String, json::Value>>>,
    /// Describes the format of the [rows].
    
    pub schema: Option<TableSchema>,
    /// Total rows of the whole query results.
    #[serde(rename="totalRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_rows: Option<i64>,
}

impl client::Part for QueryResult {}


/// An asset identifier in Google Cloud which contains its name, type and ancestors. An asset can be any resource in the Google Cloud [resource hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy), a resource outside the Google Cloud resource hierarchy (such as Google Kubernetes Engine clusters and objects), or a policy (e.g. IAM policy). See [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelatedAsset {
    /// The ancestors of an asset in Google Cloud [resource hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy), represented as a list of relative resource names. An ancestry path starts with the closest ancestor in the hierarchy and ends at root. Example: `["projects/123456789", "folders/5432", "organizations/1234"]`
    
    pub ancestors: Option<Vec<String>>,
    /// The full name of the asset. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1` See [Resource names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information.
    
    pub asset: Option<String>,
    /// The type of the asset. Example: `compute.googleapis.com/Disk` See [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more information.
    #[serde(rename="assetType")]
    
    pub asset_type: Option<String>,
    /// The unique identifier of the relationship type. Example: `INSTANCE_TO_INSTANCEGROUP`
    #[serde(rename="relationshipType")]
    
    pub relationship_type: Option<String>,
}

impl client::Part for RelatedAsset {}


/// DEPRECATED. This message only presents for the purpose of backward-compatibility. The server will never populate this message in responses. The detailed related assets with the `relationship_type`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelatedAssets {
    /// The peer resources of the relationship.
    
    pub assets: Option<Vec<RelatedAsset>>,
    /// The detailed relationship attributes.
    #[serde(rename="relationshipAttributes")]
    
    pub relationship_attributes: Option<RelationshipAttributes>,
}

impl client::Part for RelatedAssets {}


/// The detailed related resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelatedResource {
    /// The type of the asset. Example: `compute.googleapis.com/Instance`
    #[serde(rename="assetType")]
    
    pub asset_type: Option<String>,
    /// The full resource name of the related resource. Example: `//compute.googleapis.com/projects/my_proj_123/zones/instance/instance123`
    #[serde(rename="fullResourceName")]
    
    pub full_resource_name: Option<String>,
}

impl client::Part for RelatedResource {}


/// The related resources of the primary resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelatedResources {
    /// The detailed related resources of the primary resource.
    #[serde(rename="relatedResources")]
    
    pub related_resources: Option<Vec<RelatedResource>>,
}

impl client::Part for RelatedResources {}


/// DEPRECATED. This message only presents for the purpose of backward-compatibility. The server will never populate this message in responses. The relationship attributes which include `type`, `source_resource_type`, `target_resource_type` and `action`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipAttributes {
    /// The detail of the relationship, e.g. `contains`, `attaches`
    
    pub action: Option<String>,
    /// The source asset type. Example: `compute.googleapis.com/Instance`
    #[serde(rename="sourceResourceType")]
    
    pub source_resource_type: Option<String>,
    /// The target asset type. Example: `compute.googleapis.com/Disk`
    #[serde(rename="targetResourceType")]
    
    pub target_resource_type: Option<String>,
    /// The unique identifier of the relationship type. Example: `INSTANCE_TO_INSTANCEGROUP`
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for RelationshipAttributes {}


/// A representation of a Google Cloud resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Resource {
    /// The content of the resource, in which some sensitive fields are removed and may not be present.
    
    pub data: Option<HashMap<String, json::Value>>,
    /// The URL of the discovery document containing the resource's JSON schema. Example: `https://www.googleapis.com/discovery/v1/apis/compute/v1/rest` This value is unspecified for resources that do not have an API based on a discovery document, such as Cloud Bigtable.
    #[serde(rename="discoveryDocumentUri")]
    
    pub discovery_document_uri: Option<String>,
    /// The JSON schema name listed in the discovery document. Example: `Project` This value is unspecified for resources that do not have an API based on a discovery document, such as Cloud Bigtable.
    #[serde(rename="discoveryName")]
    
    pub discovery_name: Option<String>,
    /// The location of the resource in Google Cloud, such as its zone and region. For more information, see https://cloud.google.com/about/locations/.
    
    pub location: Option<String>,
    /// The full name of the immediate parent of this resource. See [Resource Names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information. For Google Cloud assets, this value is the parent resource defined in the [IAM policy hierarchy](https://cloud.google.com/iam/docs/overview#policy_hierarchy). Example: `//cloudresourcemanager.googleapis.com/projects/my_project_123` For third-party assets, this field may be set differently.
    
    pub parent: Option<String>,
    /// The REST URL for accessing the resource. An HTTP `GET` request using this URL returns the resource itself. Example: `https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123` This value is unspecified for resources without a REST API.
    #[serde(rename="resourceUrl")]
    
    pub resource_url: Option<String>,
    /// The API version. Example: `v1`
    
    pub version: Option<String>,
}

impl client::Part for Resource {}


/// A result of Resource Search, containing information of a cloud resource. Next ID: 31
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceSearchResult {
    /// The additional searchable attributes of this resource. The attributes may vary from one resource type to another. Examples: `projectId` for Project, `dnsName` for DNS ManagedZone. This field contains a subset of the resource metadata fields that are returned by the List or Get APIs provided by the corresponding Google Cloud service (e.g., Compute Engine). see [API references and supported searchable attributes](https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types) to see which fields are included. You can search values of these fields through free text search. However, you should not consume the field programically as the field names and values may change as the Google Cloud service updates to a new incompatible API version. To search against the `additional_attributes`: * Use a free text query to match the attributes values. Example: to search `additional_attributes = { dnsName: "foobar" }`, you can issue a query `foobar`.
    #[serde(rename="additionalAttributes")]
    
    pub additional_attributes: Option<HashMap<String, json::Value>>,
    /// The type of this resource. Example: `compute.googleapis.com/Disk`. To search against the `asset_type`: * Specify the `asset_type` field in your search request.
    #[serde(rename="assetType")]
    
    pub asset_type: Option<String>,
    /// Attached resources of this resource. For example, an OSConfig Inventory is an attached resource of a Compute Instance. This field is repeated because a resource could have multiple attached resources. This `attached_resources` field is not searchable. Some attributes of the attached resources are exposed in `additional_attributes` field, so as to allow users to search on them.
    #[serde(rename="attachedResources")]
    
    pub attached_resources: Option<Vec<AttachedResource>>,
    /// The create timestamp of this resource, at which the resource was created. The granularity is in seconds. Timestamp.nanos will always be 0. This field is available only when the resource's Protobuf contains it. To search against `create_time`: * Use a field query. - value in seconds since unix epoch. Example: `createTime > 1609459200` - value in date string. Example: `createTime > 2021-01-01` - value in date-time string (must be quoted). Example: `createTime > "2021-01-01T00:00:00"`
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// One or more paragraphs of text description of this resource. Maximum length could be up to 1M bytes. This field is available only when the resource's Protobuf contains it. To search against the `description`: * Use a field query. Example: `description:"important instance"` * Use a free text query. Example: `"important instance"`
    
    pub description: Option<String>,
    /// The display name of this resource. This field is available only when the resource's Protobuf contains it. To search against the `display_name`: * Use a field query. Example: `displayName:"My Instance"` * Use a free text query. Example: `"My Instance"`
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The folder(s) that this resource belongs to, in the form of folders/{FOLDER_NUMBER}. This field is available when the resource belongs to one or more folders. To search against `folders`: * Use a field query. Example: `folders:(123 OR 456)` * Use a free text query. Example: `123` * Specify the `scope` field as this folder in your search request.
    
    pub folders: Option<Vec<String>>,
    /// The Cloud KMS [CryptoKey](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys) name or [CryptoKeyVersion](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions) name. This field only presents for the purpose of backward compatibility. Please use the `kms_keys` field to retrieve Cloud KMS key information. This field is available only when the resource's Protobuf contains it and will only be populated for [these resource types](https://cloud.google.com/asset-inventory/docs/legacy-field-names#resource_types_with_the_to_be_deprecated_kmskey_field) for backward compatible purposes. To search against the `kms_key`: * Use a field query. Example: `kmsKey:key` * Use a free text query. Example: `key`
    #[serde(rename="kmsKey")]
    
    pub kms_key: Option<String>,
    /// The Cloud KMS [CryptoKey](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys) names or [CryptoKeyVersion](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions) names. This field is available only when the resource's Protobuf contains it. To search against the `kms_keys`: * Use a field query. Example: `kmsKeys:key` * Use a free text query. Example: `key`
    #[serde(rename="kmsKeys")]
    
    pub kms_keys: Option<Vec<String>>,
    /// Labels associated with this resource. See [Labelling and grouping Google Cloud resources](https://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-google-cloud-platform-resources) for more information. This field is available only when the resource's Protobuf contains it. To search against the `labels`: * Use a field query: - query on any label's key or value. Example: `labels:prod` - query by a given label. Example: `labels.env:prod` - query by a given label's existence. Example: `labels.env:*` * Use a free text query. Example: `prod`
    
    pub labels: Option<HashMap<String, String>>,
    /// Location can be `global`, regional like `us-east1`, or zonal like `us-west1-b`. This field is available only when the resource's Protobuf contains it. To search against the `location`: * Use a field query. Example: `location:us-west*` * Use a free text query. Example: `us-west*`
    
    pub location: Option<String>,
    /// The full resource name of this resource. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`. See [Cloud Asset Inventory Resource Name Format](https://cloud.google.com/asset-inventory/docs/resource-name-format) for more information. To search against the `name`: * Use a field query. Example: `name:instance1` * Use a free text query. Example: `instance1`
    
    pub name: Option<String>,
    /// Network tags associated with this resource. Like labels, network tags are a type of annotations used to group Google Cloud resources. See [Labelling Google Cloud resources](https://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-google-cloud-platform-resources) for more information. This field is available only when the resource's Protobuf contains it. To search against the `network_tags`: * Use a field query. Example: `networkTags:internal` * Use a free text query. Example: `internal`
    #[serde(rename="networkTags")]
    
    pub network_tags: Option<Vec<String>>,
    /// The organization that this resource belongs to, in the form of organizations/{ORGANIZATION_NUMBER}. This field is available when the resource belongs to an organization. To search against `organization`: * Use a field query. Example: `organization:123` * Use a free text query. Example: `123` * Specify the `scope` field as this organization in your search request.
    
    pub organization: Option<String>,
    /// The type of this resource's immediate parent, if there is one. To search against the `parent_asset_type`: * Use a field query. Example: `parentAssetType:"cloudresourcemanager.googleapis.com/Project"` * Use a free text query. Example: `cloudresourcemanager.googleapis.com/Project`
    #[serde(rename="parentAssetType")]
    
    pub parent_asset_type: Option<String>,
    /// The full resource name of this resource's parent, if it has one. To search against the `parent_full_resource_name`: * Use a field query. Example: `parentFullResourceName:"project-name"` * Use a free text query. Example: `project-name`
    #[serde(rename="parentFullResourceName")]
    
    pub parent_full_resource_name: Option<String>,
    /// The project that this resource belongs to, in the form of projects/{PROJECT_NUMBER}. This field is available when the resource belongs to a project. To search against `project`: * Use a field query. Example: `project:12345` * Use a free text query. Example: `12345` * Specify the `scope` field as this project in your search request.
    
    pub project: Option<String>,
    /// A map of related resources of this resource, keyed by the relationship type. A relationship type is in the format of {SourceType}_{ACTION}_{DestType}. Example: `DISK_TO_INSTANCE`, `DISK_TO_NETWORK`, `INSTANCE_TO_INSTANCEGROUP`. See [supported relationship types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#supported_relationship_types).
    
    pub relationships: Option<HashMap<String, RelatedResources>>,
    /// The state of this resource. Different resources types have different state definitions that are mapped from various fields of different resource types. This field is available only when the resource's Protobuf contains it. Example: If the resource is an instance provided by Compute Engine, its state will include PROVISIONING, STAGING, RUNNING, STOPPING, SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED. See `status` definition in [API Reference](https://cloud.google.com/compute/docs/reference/rest/v1/instances). If the resource is a project provided by Resource Manager, its state will include LIFECYCLE_STATE_UNSPECIFIED, ACTIVE, DELETE_REQUESTED and DELETE_IN_PROGRESS. See `lifecycleState` definition in [API Reference](https://cloud.google.com/resource-manager/reference/rest/v1/projects). To search against the `state`: * Use a field query. Example: `state:RUNNING` * Use a free text query. Example: `RUNNING`
    
    pub state: Option<String>,
    /// TagKey namespaced names, in the format of {ORG_ID}/{TAG_KEY_SHORT_NAME}. To search against the `tagKeys`: * Use a field query. Example: - `tagKeys:"123456789/env*"` - `tagKeys="123456789/env"` - `tagKeys:"env"` * Use a free text query. Example: - `env`
    #[serde(rename="tagKeys")]
    
    pub tag_keys: Option<Vec<String>>,
    /// TagValue IDs, in the format of tagValues/{TAG_VALUE_ID}. To search against the `tagValueIds`: * Use a field query. Example: - `tagValueIds:"456"` - `tagValueIds="tagValues/456"` * Use a free text query. Example: - `456`
    #[serde(rename="tagValueIds")]
    
    pub tag_value_ids: Option<Vec<String>>,
    /// TagValue namespaced names, in the format of {ORG_ID}/{TAG_KEY_SHORT_NAME}/{TAG_VALUE_SHORT_NAME}. To search against the `tagValues`: * Use a field query. Example: - `tagValues:"env"` - `tagValues:"env/prod"` - `tagValues:"123456789/env/prod*"` - `tagValues="123456789/env/prod"` * Use a free text query. Example: - `prod`
    #[serde(rename="tagValues")]
    
    pub tag_values: Option<Vec<String>>,
    /// The last update timestamp of this resource, at which the resource was last modified or deleted. The granularity is in seconds. Timestamp.nanos will always be 0. This field is available only when the resource's Protobuf contains it. To search against `update_time`: * Use a field query. - value in seconds since unix epoch. Example: `updateTime < 1609459200` - value in date string. Example: `updateTime < 2021-01-01` - value in date-time string (must be quoted). Example: `updateTime < "2021-01-01T00:00:00"`
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Versioned resource representations of this resource. This is repeated because there could be multiple versions of resource representations during version migration. This `versioned_resources` field is not searchable. Some attributes of the resource representations are exposed in `additional_attributes` field, so as to allow users to search on them.
    #[serde(rename="versionedResources")]
    
    pub versioned_resources: Option<Vec<VersionedResource>>,
}

impl client::Part for ResourceSearchResult {}


/// Specifies the resource to analyze for access policies, which may be set directly on the resource, or on ancestors such as organizations, folders or projects.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceSelector {
    /// Required. The [full resource name] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of a resource of [supported resource types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#analyzable_asset_types).
    #[serde(rename="fullResourceName")]
    
    pub full_resource_name: Option<String>,
}

impl client::Part for ResourceSelector {}


/// A saved query which can be shared with others or used later.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create saved queries](SavedQueryCreateCall) (request|response)
/// * [get saved queries](SavedQueryGetCall) (response)
/// * [patch saved queries](SavedQueryPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SavedQuery {
    /// The query content.
    
    pub content: Option<QueryContent>,
    /// Output only. The create time of this saved query.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The account's email address who has created this saved query.
    
    pub creator: Option<String>,
    /// The description of this saved query. This value should be fewer than 255 characters.
    
    pub description: Option<String>,
    /// Labels applied on the resource. This value should not contain more than 10 entries. The key and value of each entry must be non-empty and fewer than 64 characters.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The last update time of this saved query.
    #[serde(rename="lastUpdateTime")]
    
    pub last_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The account's email address who has updated this saved query most recently.
    #[serde(rename="lastUpdater")]
    
    pub last_updater: Option<String>,
    /// The resource name of the saved query. The format must be: * projects/project_number/savedQueries/saved_query_id * folders/folder_number/savedQueries/saved_query_id * organizations/organization_number/savedQueries/saved_query_id
    
    pub name: Option<String>,
}

impl client::RequestValue for SavedQuery {}
impl client::ResponseResult for SavedQuery {}


/// Search all IAM policies response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search all iam policies](MethodSearchAllIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchAllIamPoliciesResponse {
    /// Set if there are more results than those appearing in this response; to get the next set of results, call this method again, using this value as the `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of IAM policies that match the search query. Related information such as the associated resource is returned along with the policy.
    
    pub results: Option<Vec<IamPolicySearchResult>>,
}

impl client::ResponseResult for SearchAllIamPoliciesResponse {}


/// Search all resources response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search all resources](MethodSearchAllResourceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchAllResourcesResponse {
    /// If there are more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of Resources that match the search query. It contains the resource standard metadata information.
    
    pub results: Option<Vec<ResourceSearchResult>>,
}

impl client::ResponseResult for SearchAllResourcesResponse {}


/// Software package information of the operating system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SoftwarePackage {
    /// Details of an APT package. For details about the apt package manager, see https://wiki.debian.org/Apt.
    #[serde(rename="aptPackage")]
    
    pub apt_package: Option<VersionedPackage>,
    /// Details of a COS package.
    #[serde(rename="cosPackage")]
    
    pub cos_package: Option<VersionedPackage>,
    /// Details of a Googet package. For details about the googet package manager, see https://github.com/google/googet.
    #[serde(rename="googetPackage")]
    
    pub googet_package: Option<VersionedPackage>,
    /// Details of a Windows Quick Fix engineering package. See https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering for info in Windows Quick Fix Engineering.
    #[serde(rename="qfePackage")]
    
    pub qfe_package: Option<WindowsQuickFixEngineeringPackage>,
    /// Details of Windows Application.
    #[serde(rename="windowsApplication")]
    
    pub windows_application: Option<WindowsApplication>,
    /// Details of a Windows Update package. See https://docs.microsoft.com/en-us/windows/win32/api/_wua/ for information about Windows Update.
    #[serde(rename="wuaPackage")]
    
    pub wua_package: Option<WindowsUpdatePackage>,
    /// Yum package info. For details about the yum package manager, see https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/ch-yum.
    #[serde(rename="yumPackage")]
    
    pub yum_package: Option<VersionedPackage>,
    /// Details of a Zypper package. For details about the Zypper package manager, see https://en.opensuse.org/SDB:Zypper_manual.
    #[serde(rename="zypperPackage")]
    
    pub zypper_package: Option<VersionedPackage>,
    /// Details of a Zypper patch. For details about the Zypper package manager, see https://en.opensuse.org/SDB:Zypper_manual.
    #[serde(rename="zypperPatch")]
    
    pub zypper_patch: Option<ZypperPatch>,
}

impl client::Part for SoftwarePackage {}


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


/// A field in TableSchema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableFieldSchema {
    /// The field name. The name must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_), and must start with a letter or underscore. The maximum length is 128 characters.
    
    pub field: Option<String>,
    /// Describes the nested schema fields if the type property is set to RECORD.
    
    pub fields: Option<Vec<TableFieldSchema>>,
    /// The field mode. Possible values include NULLABLE, REQUIRED and REPEATED. The default value is NULLABLE.
    
    pub mode: Option<String>,
    /// The field data type. Possible values include * STRING * BYTES * INTEGER * FLOAT * BOOLEAN * TIMESTAMP * DATE * TIME * DATETIME * GEOGRAPHY, * NUMERIC, * BIGNUMERIC, * RECORD (where RECORD indicates that the field contains a nested schema).
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for TableFieldSchema {}


/// BigQuery Compatible table schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableSchema {
    /// Describes the fields in a table.
    
    pub fields: Option<Vec<TableFieldSchema>>,
}

impl client::Part for TableSchema {}


/// An asset in Google Cloud and its temporal metadata, including the time window when it was observed and its status during that window.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TemporalAsset {
    /// An asset in Google Cloud.
    
    pub asset: Option<Asset>,
    /// Whether the asset has been deleted or not.
    
    pub deleted: Option<bool>,
    /// Prior copy of the asset. Populated if prior_asset_state is PRESENT. Currently this is only set for responses in Real-Time Feed.
    #[serde(rename="priorAsset")]
    
    pub prior_asset: Option<Asset>,
    /// State of prior_asset.
    #[serde(rename="priorAssetState")]
    
    pub prior_asset_state: Option<TemporalAssetPriorAssetStateEnum>,
    /// The time window when the asset data and state was observed.
    
    pub window: Option<TimeWindow>,
}

impl client::Part for TemporalAsset {}


/// A time window specified by its `start_time` and `end_time`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeWindow {
    /// End time of the time window (inclusive). If not specified, the current timestamp is used instead.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Start time of the time window (exclusive).
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TimeWindow {}


/// Update asset feed request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [patch feeds](FeedPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateFeedRequest {
    /// Required. The new values of feed details. It must match an existing feed and the field `name` must be in the format of: projects/project_number/feeds/feed_id or folders/folder_number/feeds/feed_id or organizations/organization_number/feeds/feed_id.
    
    pub feed: Option<Feed>,
    /// Required. Only updates the `feed` fields indicated by this mask. The field mask must not be empty, and it must not contain fields that are immutable or only set by the server.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for UpdateFeedRequest {}


/// Information related to the a standard versioned package. This includes package info for APT, Yum, Zypper, and Googet package managers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VersionedPackage {
    /// The system architecture this package is intended for.
    
    pub architecture: Option<String>,
    /// The name of the package.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// The version of the package.
    
    pub version: Option<String>,
}

impl client::Part for VersionedPackage {}


/// Resource representation as defined by the corresponding service providing the resource for a given API version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VersionedResource {
    /// JSON representation of the resource as defined by the corresponding service providing this resource. Example: If the resource is an instance provided by Compute Engine, this field will contain the JSON representation of the instance as defined by Compute Engine: `https://cloud.google.com/compute/docs/reference/rest/v1/instances`. You can find the resource definition for each supported resource type in this table: `https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types`
    
    pub resource: Option<HashMap<String, json::Value>>,
    /// API version of the resource. Example: If the resource is an instance provided by Compute Engine v1 API as defined in `https://cloud.google.com/compute/docs/reference/rest/v1/instances`, version will be "v1".
    
    pub version: Option<String>,
}

impl client::Part for VersionedResource {}


/// Contains information about a Windows application that is retrieved from the Windows Registry. For more information about these fields, see: https://docs.microsoft.com/en-us/windows/win32/msi/uninstall-registry-key
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WindowsApplication {
    /// The name of the application or product.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The version of the product or application in string format.
    #[serde(rename="displayVersion")]
    
    pub display_version: Option<String>,
    /// The internet address for technical support.
    #[serde(rename="helpLink")]
    
    pub help_link: Option<String>,
    /// The last time this product received service. The value of this property is replaced each time a patch is applied or removed from the product or the command-line option is used to repair the product.
    #[serde(rename="installDate")]
    
    pub install_date: Option<Date>,
    /// The name of the manufacturer for the product or application.
    
    pub publisher: Option<String>,
}

impl client::Part for WindowsApplication {}


/// Information related to a Quick Fix Engineering package. Fields are taken from Windows QuickFixEngineering Interface and match the source names: https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WindowsQuickFixEngineeringPackage {
    /// A short textual description of the QFE update.
    
    pub caption: Option<String>,
    /// A textual description of the QFE update.
    
    pub description: Option<String>,
    /// Unique identifier associated with a particular QFE update.
    #[serde(rename="hotFixId")]
    
    pub hot_fix_id: Option<String>,
    /// Date that the QFE update was installed. Mapped from installed_on field.
    #[serde(rename="installTime")]
    
    pub install_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for WindowsQuickFixEngineeringPackage {}


/// Categories specified by the Windows Update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WindowsUpdateCategory {
    /// The identifier of the windows update category.
    
    pub id: Option<String>,
    /// The name of the windows update category.
    
    pub name: Option<String>,
}

impl client::Part for WindowsUpdateCategory {}


/// Details related to a Windows Update package. Field data and names are taken from Windows Update API IUpdate Interface: https://docs.microsoft.com/en-us/windows/win32/api/_wua/ Descriptive fields like title, and description are localized based on the locale of the VM being updated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WindowsUpdatePackage {
    /// The categories that are associated with this update package.
    
    pub categories: Option<Vec<WindowsUpdateCategory>>,
    /// The localized description of the update package.
    
    pub description: Option<String>,
    /// A collection of Microsoft Knowledge Base article IDs that are associated with the update package.
    #[serde(rename="kbArticleIds")]
    
    pub kb_article_ids: Option<Vec<String>>,
    /// The last published date of the update, in (UTC) date and time.
    #[serde(rename="lastDeploymentChangeTime")]
    
    pub last_deployment_change_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A collection of URLs that provide more information about the update package.
    #[serde(rename="moreInfoUrls")]
    
    pub more_info_urls: Option<Vec<String>>,
    /// The revision number of this update package.
    #[serde(rename="revisionNumber")]
    
    pub revision_number: Option<i32>,
    /// A hyperlink to the language-specific support information for the update.
    #[serde(rename="supportUrl")]
    
    pub support_url: Option<String>,
    /// The localized title of the update package.
    
    pub title: Option<String>,
    /// Gets the identifier of an update package. Stays the same across revisions.
    #[serde(rename="updateId")]
    
    pub update_id: Option<String>,
}

impl client::Part for WindowsUpdatePackage {}


/// Details related to a Zypper Patch.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ZypperPatch {
    /// The category of the patch.
    
    pub category: Option<String>,
    /// The name of the patch.
    #[serde(rename="patchName")]
    
    pub patch_name: Option<String>,
    /// The severity specified for this patch
    
    pub severity: Option<String>,
    /// Any summary information provided about this patch.
    
    pub summary: Option<String>,
}

impl client::Part for ZypperPatch {}


