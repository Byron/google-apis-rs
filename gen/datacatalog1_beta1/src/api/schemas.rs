use super::*;
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


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries tags delete projects](ProjectLocationEntryGroupEntryTagDeleteCall) (response)
/// * [locations entry groups entries delete projects](ProjectLocationEntryGroupEntryDeleteCall) (response)
/// * [locations entry groups tags delete projects](ProjectLocationEntryGroupTagDeleteCall) (response)
/// * [locations entry groups delete projects](ProjectLocationEntryGroupDeleteCall) (response)
/// * [locations tag templates fields delete projects](ProjectLocationTagTemplateFieldDeleteCall) (response)
/// * [locations tag templates delete projects](ProjectLocationTagTemplateDeleteCall) (response)
/// * [locations taxonomies policy tags delete projects](ProjectLocationTaxonomyPolicyTagDeleteCall) (response)
/// * [locations taxonomies delete projects](ProjectLocationTaxonomyDeleteCall) (response)
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


/// Request message for `GetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries get iam policy projects](ProjectLocationEntryGroupEntryGetIamPolicyCall) (request)
/// * [locations entry groups get iam policy projects](ProjectLocationEntryGroupGetIamPolicyCall) (request)
/// * [locations tag templates get iam policy projects](ProjectLocationTagTemplateGetIamPolicyCall) (request)
/// * [locations taxonomies policy tags get iam policy projects](ProjectLocationTaxonomyPolicyTagGetIamPolicyCall) (request)
/// * [locations taxonomies get iam policy projects](ProjectLocationTaxonomyGetIamPolicyCall) (request)
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


/// Spec for a group of BigQuery tables with name pattern `[prefix]YYYYMMDD`. Context: https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1BigQueryDateShardedSpec {
    /// Output only. The Data Catalog resource name of the dataset entry the current table belongs to, for example, `projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}`.
    
    pub dataset: Option<String>,
    /// Output only. Total number of shards.
    #[serde(rename="shardCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub shard_count: Option<i64>,
    /// Output only. The table name prefix of the shards. The name of any given shard is `[table_prefix]YYYYMMDD`, for example, for shard `MyTable20180101`, the `table_prefix` is `MyTable`.
    #[serde(rename="tablePrefix")]
    
    pub table_prefix: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1BigQueryDateShardedSpec {}


/// Describes a BigQuery table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1BigQueryTableSpec {
    /// Output only. The table source type.
    #[serde(rename="tableSourceType")]
    
    pub table_source_type: Option<GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum>,
    /// Spec of a BigQuery table. This field should only be populated if `table_source_type` is `BIGQUERY_TABLE`.
    #[serde(rename="tableSpec")]
    
    pub table_spec: Option<GoogleCloudDatacatalogV1beta1TableSpec>,
    /// Table view specification. This field should only be populated if `table_source_type` is `BIGQUERY_VIEW`.
    #[serde(rename="viewSpec")]
    
    pub view_spec: Option<GoogleCloudDatacatalogV1beta1ViewSpec>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1BigQueryTableSpec {}


/// Representation of a column within a schema. Columns could be nested inside other columns.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1ColumnSchema {
    /// Required. Name of the column.
    
    pub column: Option<String>,
    /// Optional. Description of the column. Default value is an empty string.
    
    pub description: Option<String>,
    /// Optional. A column's mode indicates whether the values in this column are required, nullable, etc. Only `NULLABLE`, `REQUIRED` and `REPEATED` are supported. Default mode is `NULLABLE`.
    
    pub mode: Option<String>,
    /// Optional. Schema of sub-columns. A column can have zero or more sub-columns.
    
    pub subcolumns: Option<Vec<GoogleCloudDatacatalogV1beta1ColumnSchema>>,
    /// Required. Type of the column.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1ColumnSchema {}


/// Entry Metadata. A Data Catalog Entry resource represents another resource in Google Cloud Platform (such as a BigQuery dataset or a Pub/Sub topic), or outside of Google Cloud Platform. Clients can use the `linked_resource` field in the Entry resource to refer to the original resource ID of the source system. An Entry resource contains resource details, such as its schema. An Entry can also be used to attach flexible metadata, such as a Tag.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [lookup entries](EntryLookupCall) (response)
/// * [locations entry groups entries create projects](ProjectLocationEntryGroupEntryCreateCall) (request|response)
/// * [locations entry groups entries get projects](ProjectLocationEntryGroupEntryGetCall) (response)
/// * [locations entry groups entries patch projects](ProjectLocationEntryGroupEntryPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1Entry {
    /// Specification for a group of BigQuery tables with name pattern `[prefix]YYYYMMDD`. Context: https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding.
    #[serde(rename="bigqueryDateShardedSpec")]
    
    pub bigquery_date_sharded_spec: Option<GoogleCloudDatacatalogV1beta1BigQueryDateShardedSpec>,
    /// Specification that applies to a BigQuery table. This is only valid on entries of type `TABLE`.
    #[serde(rename="bigqueryTableSpec")]
    
    pub bigquery_table_spec: Option<GoogleCloudDatacatalogV1beta1BigQueryTableSpec>,
    /// Entry description, which can consist of several sentences or paragraphs that describe entry contents. Default value is an empty string.
    
    pub description: Option<String>,
    /// Display information such as title and description. A short name to identify the entry, for example, "Analytics Data - Jan 2011". Default value is an empty string.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Specification that applies to a Cloud Storage fileset. This is only valid on entries of type FILESET.
    #[serde(rename="gcsFilesetSpec")]
    
    pub gcs_fileset_spec: Option<GoogleCloudDatacatalogV1beta1GcsFilesetSpec>,
    /// Output only. This field indicates the entry's source system that Data Catalog integrates with, such as BigQuery or Pub/Sub.
    #[serde(rename="integratedSystem")]
    
    pub integrated_system: Option<GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum>,
    /// The resource this metadata entry refers to. For Google Cloud Platform resources, `linked_resource` is the [full name of the resource](https://cloud.google.com/apis/design/resource_names#full_resource_name). For example, the `linked_resource` for a table resource from BigQuery is: * //bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId Output only when Entry is of type in the EntryType enum. For entries with user_specified_type, this field is optional and defaults to an empty string.
    #[serde(rename="linkedResource")]
    
    pub linked_resource: Option<String>,
    /// Output only. The Data Catalog resource name of the entry in URL format. Example: * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id} Note that this Entry and its child resources may not actually be stored in the location in this name.
    
    pub name: Option<String>,
    /// Schema of the entry. An entry might not have any schema attached to it.
    
    pub schema: Option<GoogleCloudDatacatalogV1beta1Schema>,
    /// Output only. Timestamps about the underlying resource, not about this Data Catalog entry. Output only when Entry is of type in the EntryType enum. For entries with user_specified_type, this field is optional and defaults to an empty timestamp.
    #[serde(rename="sourceSystemTimestamps")]
    
    pub source_system_timestamps: Option<GoogleCloudDatacatalogV1beta1SystemTimestamps>,
    /// The type of the entry. Only used for Entries with types in the EntryType enum.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudDatacatalogV1beta1EntryTypeEnum>,
    /// Output only. Statistics on the usage level of the resource.
    #[serde(rename="usageSignal")]
    
    pub usage_signal: Option<GoogleCloudDatacatalogV1beta1UsageSignal>,
    /// This field indicates the entry's source system that Data Catalog does not integrate with. `user_specified_system` strings must begin with a letter or underscore and can only contain letters, numbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long.
    #[serde(rename="userSpecifiedSystem")]
    
    pub user_specified_system: Option<String>,
    /// Entry type if it does not fit any of the input-allowed values listed in `EntryType` enum above. When creating an entry, users should check the enum values first, if nothing matches the entry to be created, then provide a custom value, for example "my_special_type". `user_specified_type` strings must begin with a letter or underscore and can only contain letters, numbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long. Currently, only FILESET enum value is allowed. All other entries created through Data Catalog must use `user_specified_type`.
    #[serde(rename="userSpecifiedType")]
    
    pub user_specified_type: Option<String>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1beta1Entry {}
impl client::ResponseResult for GoogleCloudDatacatalogV1beta1Entry {}


/// EntryGroup Metadata. An EntryGroup resource represents a logical grouping of zero or more Data Catalog Entry resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups create projects](ProjectLocationEntryGroupCreateCall) (request|response)
/// * [locations entry groups get projects](ProjectLocationEntryGroupGetCall) (response)
/// * [locations entry groups patch projects](ProjectLocationEntryGroupPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1EntryGroup {
    /// Output only. Timestamps about this EntryGroup. Default value is empty timestamps.
    #[serde(rename="dataCatalogTimestamps")]
    
    pub data_catalog_timestamps: Option<GoogleCloudDatacatalogV1beta1SystemTimestamps>,
    /// Entry group description, which can consist of several sentences or paragraphs that describe entry group contents. Default value is an empty string.
    
    pub description: Option<String>,
    /// A short name to identify the entry group, for example, "analytics data - jan 2011". Default value is an empty string.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The resource name of the entry group in URL format. Example: * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id} Note that this EntryGroup and its child resources may not actually be stored in the location in this name.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1beta1EntryGroup {}
impl client::ResponseResult for GoogleCloudDatacatalogV1beta1EntryGroup {}


/// Response message for ExportTaxonomies.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations taxonomies export projects](ProjectLocationTaxonomyExportCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1ExportTaxonomiesResponse {
    /// List of taxonomies and policy tags in a tree structure.
    
    pub taxonomies: Option<Vec<GoogleCloudDatacatalogV1beta1SerializedTaxonomy>>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1beta1ExportTaxonomiesResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1FieldType {
    /// Represents an enum type.
    #[serde(rename="enumType")]
    
    pub enum_type: Option<GoogleCloudDatacatalogV1beta1FieldTypeEnumType>,
    /// Represents primitive types - string, bool etc.
    #[serde(rename="primitiveType")]
    
    pub primitive_type: Option<GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1FieldType {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1FieldTypeEnumType {
    /// no description provided
    #[serde(rename="allowedValues")]
    
    pub allowed_values: Option<Vec<GoogleCloudDatacatalogV1beta1FieldTypeEnumTypeEnumValue>>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1FieldTypeEnumType {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1FieldTypeEnumTypeEnumValue {
    /// Required. The display name of the enum value. Must not be an empty string.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1FieldTypeEnumTypeEnumValue {}


/// Specifications of a single file in Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1GcsFileSpec {
    /// Required. The full file path. Example: `gs://bucket_name/a/b.txt`.
    #[serde(rename="filePath")]
    
    pub file_path: Option<String>,
    /// Output only. Timestamps about the Cloud Storage file.
    #[serde(rename="gcsTimestamps")]
    
    pub gcs_timestamps: Option<GoogleCloudDatacatalogV1beta1SystemTimestamps>,
    /// Output only. The size of the file, in bytes.
    #[serde(rename="sizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size_bytes: Option<i64>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1GcsFileSpec {}


/// Describes a Cloud Storage fileset entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1GcsFilesetSpec {
    /// Required. Patterns to identify a set of files in Google Cloud Storage. See [Cloud Storage documentation](https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames) for more information. Note that bucket wildcards are currently not supported. Examples of valid file_patterns: * `gs://bucket_name/dir/*`: matches all files within `bucket_name/dir` directory. * `gs://bucket_name/dir/**`: matches all files in `bucket_name/dir` spanning all subdirectories. * `gs://bucket_name/file*`: matches files prefixed by `file` in `bucket_name` * `gs://bucket_name/??.txt`: matches files with two characters followed by `.txt` in `bucket_name` * `gs://bucket_name/[aeiou].txt`: matches files that contain a single vowel character followed by `.txt` in `bucket_name` * `gs://bucket_name/[a-m].txt`: matches files that contain `a`, `b`, ... or `m` followed by `.txt` in `bucket_name` * `gs://bucket_name/a/*/b`: matches all files in `bucket_name` that match `a/*/b` pattern, such as `a/c/b`, `a/d/b` * `gs://another_bucket/a.txt`: matches `gs://another_bucket/a.txt` You can combine wildcards to provide more powerful matches, for example: * `gs://bucket_name/[a-m]??.j*g`
    #[serde(rename="filePatterns")]
    
    pub file_patterns: Option<Vec<String>>,
    /// Output only. Sample files contained in this fileset, not all files contained in this fileset are represented here.
    #[serde(rename="sampleGcsFileSpecs")]
    
    pub sample_gcs_file_specs: Option<Vec<GoogleCloudDatacatalogV1beta1GcsFileSpec>>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1GcsFilesetSpec {}


/// Request message for ImportTaxonomies.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations taxonomies import projects](ProjectLocationTaxonomyImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1ImportTaxonomiesRequest {
    /// Inline source used for taxonomies to be imported.
    #[serde(rename="inlineSource")]
    
    pub inline_source: Option<GoogleCloudDatacatalogV1beta1InlineSource>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1beta1ImportTaxonomiesRequest {}


/// Response message for ImportTaxonomies.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations taxonomies import projects](ProjectLocationTaxonomyImportCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1ImportTaxonomiesResponse {
    /// Taxonomies that were imported.
    
    pub taxonomies: Option<Vec<GoogleCloudDatacatalogV1beta1Taxonomy>>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1beta1ImportTaxonomiesResponse {}


/// Inline source used for taxonomies import.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1InlineSource {
    /// Required. Taxonomies to be imported.
    
    pub taxonomies: Option<Vec<GoogleCloudDatacatalogV1beta1SerializedTaxonomy>>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1InlineSource {}


/// Response message for ListEntries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries list projects](ProjectLocationEntryGroupEntryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1ListEntriesResponse {
    /// Entry details.
    
    pub entries: Option<Vec<GoogleCloudDatacatalogV1beta1Entry>>,
    /// Token to retrieve the next page of results. It is set to empty if no items remain in results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1beta1ListEntriesResponse {}


/// Response message for ListEntryGroups.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups list projects](ProjectLocationEntryGroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1ListEntryGroupsResponse {
    /// EntryGroup details.
    #[serde(rename="entryGroups")]
    
    pub entry_groups: Option<Vec<GoogleCloudDatacatalogV1beta1EntryGroup>>,
    /// Token to retrieve the next page of results. It is set to empty if no items remain in results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1beta1ListEntryGroupsResponse {}


/// Response message for ListPolicyTags.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations taxonomies policy tags list projects](ProjectLocationTaxonomyPolicyTagListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1ListPolicyTagsResponse {
    /// Token used to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The policy tags that are in the requested taxonomy.
    #[serde(rename="policyTags")]
    
    pub policy_tags: Option<Vec<GoogleCloudDatacatalogV1beta1PolicyTag>>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1beta1ListPolicyTagsResponse {}


/// Response message for ListTags.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries tags list projects](ProjectLocationEntryGroupEntryTagListCall) (response)
/// * [locations entry groups tags list projects](ProjectLocationEntryGroupTagListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1ListTagsResponse {
    /// Token to retrieve the next page of results. It is set to empty if no items remain in results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Tag details.
    
    pub tags: Option<Vec<GoogleCloudDatacatalogV1beta1Tag>>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1beta1ListTagsResponse {}


/// Response message for ListTaxonomies.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations taxonomies list projects](ProjectLocationTaxonomyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1ListTaxonomiesResponse {
    /// Token used to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Taxonomies that the project contains.
    
    pub taxonomies: Option<Vec<GoogleCloudDatacatalogV1beta1Taxonomy>>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1beta1ListTaxonomiesResponse {}


/// Denotes one policy tag in a taxonomy (e.g. ssn). Policy Tags can be defined in a hierarchy. For example, consider the following hierarchy: Geolocation -> (LatLong, City, ZipCode). PolicyTag “Geolocation” contains three child policy tags: “LatLong”, “City”, and “ZipCode”.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations taxonomies policy tags create projects](ProjectLocationTaxonomyPolicyTagCreateCall) (request|response)
/// * [locations taxonomies policy tags get projects](ProjectLocationTaxonomyPolicyTagGetCall) (response)
/// * [locations taxonomies policy tags patch projects](ProjectLocationTaxonomyPolicyTagPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1PolicyTag {
    /// Output only. Resource names of child policy tags of this policy tag.
    #[serde(rename="childPolicyTags")]
    
    pub child_policy_tags: Option<Vec<String>>,
    /// Description of this policy tag. It must: contain only unicode characters, tabs, newlines, carriage returns and page breaks; and be at most 2000 bytes long when encoded in UTF-8. If not set, defaults to an empty description. If not set, defaults to an empty description.
    
    pub description: Option<String>,
    /// Required. User defined name of this policy tag. It must: be unique within the parent taxonomy; contain only unicode letters, numbers, underscores, dashes and spaces; not start or end with spaces; and be at most 200 bytes long when encoded in UTF-8.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Resource name of this policy tag, whose format is: "projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{id}".
    
    pub name: Option<String>,
    /// Resource name of this policy tag's parent policy tag (e.g. for the "LatLong" policy tag in the example above, this field contains the resource name of the "Geolocation" policy tag). If empty, it means this policy tag is a top level policy tag (e.g. this field is empty for the "Geolocation" policy tag in the example above). If not set, defaults to an empty string.
    #[serde(rename="parentPolicyTag")]
    
    pub parent_policy_tag: Option<String>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1beta1PolicyTag {}
impl client::ResponseResult for GoogleCloudDatacatalogV1beta1PolicyTag {}


/// Request message for RenameTagTemplateFieldEnumValue.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations tag templates fields enum values rename projects](ProjectLocationTagTemplateFieldEnumValueRenameCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1RenameTagTemplateFieldEnumValueRequest {
    /// Required. The new display name of the enum value. For example, `my_new_enum_value`.
    #[serde(rename="newEnumValueDisplayName")]
    
    pub new_enum_value_display_name: Option<String>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1beta1RenameTagTemplateFieldEnumValueRequest {}


/// Request message for RenameTagTemplateField.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations tag templates fields rename projects](ProjectLocationTagTemplateFieldRenameCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1RenameTagTemplateFieldRequest {
    /// Required. The new ID of this tag template field. For example, `my_new_field`.
    #[serde(rename="newTagTemplateFieldId")]
    
    pub new_tag_template_field_id: Option<String>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1beta1RenameTagTemplateFieldRequest {}


/// Represents a schema (e.g. BigQuery, GoogleSQL, Avro schema).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1Schema {
    /// Required. Schema of columns. A maximum of 10,000 columns and sub-columns can be specified.
    
    pub columns: Option<Vec<GoogleCloudDatacatalogV1beta1ColumnSchema>>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1Schema {}


/// Request message for SearchCatalog.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search catalog](CatalogSearchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1SearchCatalogRequest {
    /// Specifies the ordering of results, currently supported case-sensitive choices are: * `relevance`, only supports descending * `last_modified_timestamp [asc|desc]`, defaults to descending if not specified * `default` that can only be descending If not specified, defaults to `relevance` descending.
    #[serde(rename="orderBy")]
    
    pub order_by: Option<String>,
    /// Number of results in the search page. If <=0 then defaults to 10. Max limit for page_size is 1000. Throws an invalid argument for page_size > 1000.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional. Pagination token returned in an earlier SearchCatalogResponse.next_page_token, which indicates that this is a continuation of a prior SearchCatalogRequest call, and that the system should return the next page of data. If empty, the first page is returned.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Optional. The query string in search query syntax. An empty query string will result in all data assets (in the specified scope) that the user has access to. Query strings can be simple as "x" or more qualified as: * name:x * column:x * description:y Note: Query tokens need to have a minimum of 3 characters for substring matching to work correctly. See [Data Catalog Search Syntax](https://cloud.google.com/data-catalog/docs/how-to/search-reference) for more information.
    
    pub query: Option<String>,
    /// Required. The scope of this search request. A `scope` that has empty `include_org_ids`, `include_project_ids` AND false `include_gcp_public_datasets` is considered invalid. Data Catalog will return an error in such a case.
    
    pub scope: Option<GoogleCloudDatacatalogV1beta1SearchCatalogRequestScope>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1beta1SearchCatalogRequest {}


/// The criteria that select the subspace used for query matching.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1SearchCatalogRequestScope {
    /// If `true`, include Google Cloud Platform (GCP) public datasets in the search results. Info on GCP public datasets is available at https://cloud.google.com/public-datasets/. By default, GCP public datasets are excluded.
    #[serde(rename="includeGcpPublicDatasets")]
    
    pub include_gcp_public_datasets: Option<bool>,
    /// The list of organization IDs to search within. To find your organization ID, follow instructions in https://cloud.google.com/resource-manager/docs/creating-managing-organization.
    #[serde(rename="includeOrgIds")]
    
    pub include_org_ids: Option<Vec<String>>,
    /// The list of project IDs to search within. To learn more about the distinction between project names/IDs/numbers, go to https://cloud.google.com/docs/overview/#projects.
    #[serde(rename="includeProjectIds")]
    
    pub include_project_ids: Option<Vec<String>>,
    /// Optional. The list of locations to search within. 1. If empty, search will be performed in all locations; 2. If any of the locations are NOT in the valid locations list, error will be returned; 3. Otherwise, search only the given locations for matching results. Typical usage is to leave this field empty. When a location is unreachable as returned in the `SearchCatalogResponse.unreachable` field, users can repeat the search request with this parameter set to get additional information on the error. Valid locations: * asia-east1 * asia-east2 * asia-northeast1 * asia-northeast2 * asia-northeast3 * asia-south1 * asia-southeast1 * australia-southeast1 * eu * europe-north1 * europe-west1 * europe-west2 * europe-west3 * europe-west4 * europe-west6 * global * northamerica-northeast1 * southamerica-east1 * us * us-central1 * us-east1 * us-east4 * us-west1 * us-west2
    #[serde(rename="restrictedLocations")]
    
    pub restricted_locations: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1SearchCatalogRequestScope {}


/// Response message for SearchCatalog.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search catalog](CatalogSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1SearchCatalogResponse {
    /// The token that can be used to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Search results.
    
    pub results: Option<Vec<GoogleCloudDatacatalogV1beta1SearchCatalogResult>>,
    /// Unreachable locations. Search result does not include data from those locations. Users can get additional information on the error by repeating the search request with a more restrictive parameter -- setting the value for `SearchDataCatalogRequest.scope.restricted_locations`.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1beta1SearchCatalogResponse {}


/// A result that appears in the response of a search request. Each result captures details of one entry that matches the search.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1SearchCatalogResult {
    /// The full name of the cloud resource the entry belongs to. See: https://cloud.google.com/apis/design/resource_names#full_resource_name. Example: * `//bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId`
    #[serde(rename="linkedResource")]
    
    pub linked_resource: Option<String>,
    /// Last-modified timestamp of the entry from the managing system.
    #[serde(rename="modifyTime")]
    
    pub modify_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The relative resource name of the resource in URL format. Examples: * `projects/{project_id}/locations/{location_id}/entryGroups/{entry_group_id}/entries/{entry_id}` * `projects/{project_id}/tagTemplates/{tag_template_id}`
    #[serde(rename="relativeResourceName")]
    
    pub relative_resource_name: Option<String>,
    /// Sub-type of the search result. This is a dot-delimited description of the resource's full type, and is the same as the value callers would provide in the "type" search facet. Examples: `entry.table`, `entry.dataStream`, `tagTemplate`.
    #[serde(rename="searchResultSubtype")]
    
    pub search_result_subtype: Option<String>,
    /// Type of the search result. This field can be used to determine which Get method to call to fetch the full resource.
    #[serde(rename="searchResultType")]
    
    pub search_result_type: Option<GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1SearchCatalogResult {}


/// Message representing one policy tag when exported as a nested proto.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1SerializedPolicyTag {
    /// Children of the policy tag if any.
    #[serde(rename="childPolicyTags")]
    
    pub child_policy_tags: Option<Vec<GoogleCloudDatacatalogV1beta1SerializedPolicyTag>>,
    /// Description of the serialized policy tag. The length of the description is limited to 2000 bytes when encoded in UTF-8. If not set, defaults to an empty description.
    
    pub description: Option<String>,
    /// Required. Display name of the policy tag. Max 200 bytes when encoded in UTF-8.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Resource name of the policy tag. This field will be ignored when calling ImportTaxonomies.
    #[serde(rename="policyTag")]
    
    pub policy_tag: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1SerializedPolicyTag {}


/// Message capturing a taxonomy and its policy tag hierarchy as a nested proto. Used for taxonomy import/export and mutation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1SerializedTaxonomy {
    /// A list of policy types that are activated for a taxonomy.
    #[serde(rename="activatedPolicyTypes")]
    
    pub activated_policy_types: Option<Vec<GoogleCloudDatacatalogV1beta1SerializedTaxonomyActivatedPolicyTypesEnum>>,
    /// Description of the serialized taxonomy. The length of the description is limited to 2000 bytes when encoded in UTF-8. If not set, defaults to an empty description.
    
    pub description: Option<String>,
    /// Required. Display name of the taxonomy. Max 200 bytes when encoded in UTF-8.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Top level policy tags associated with the taxonomy if any.
    #[serde(rename="policyTags")]
    
    pub policy_tags: Option<Vec<GoogleCloudDatacatalogV1beta1SerializedPolicyTag>>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1SerializedTaxonomy {}


/// Timestamps about this resource according to a particular system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1SystemTimestamps {
    /// The creation time of the resource within the given system.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The expiration time of the resource within the given system. Currently only apllicable to BigQuery resources.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The last-modified time of the resource within the given system.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1SystemTimestamps {}


/// Normal BigQuery table spec.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1TableSpec {
    /// Output only. If the table is a dated shard, i.e., with name pattern `[prefix]YYYYMMDD`, `grouped_entry` is the Data Catalog resource name of the date sharded grouped entry, for example, `projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}`. Otherwise, `grouped_entry` is empty.
    #[serde(rename="groupedEntry")]
    
    pub grouped_entry: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1TableSpec {}


/// Tags are used to attach custom metadata to Data Catalog resources. Tags conform to the specifications within their tag template. See [Data Catalog IAM](https://cloud.google.com/data-catalog/docs/concepts/iam) for information on the permissions needed to create or view tags.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries tags create projects](ProjectLocationEntryGroupEntryTagCreateCall) (request|response)
/// * [locations entry groups entries tags patch projects](ProjectLocationEntryGroupEntryTagPatchCall) (request|response)
/// * [locations entry groups tags create projects](ProjectLocationEntryGroupTagCreateCall) (request|response)
/// * [locations entry groups tags patch projects](ProjectLocationEntryGroupTagPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1Tag {
    /// Resources like Entry can have schemas associated with them. This scope allows users to attach tags to an individual column based on that schema. For attaching a tag to a nested column, use `.` to separate the column names. Example: * `outer_column.inner_column`
    
    pub column: Option<String>,
    /// Required. This maps the ID of a tag field to the value of and additional information about that field. Valid field IDs are defined by the tag's template. A tag must have at least 1 field and at most 500 fields.
    
    pub fields: Option<HashMap<String, GoogleCloudDatacatalogV1beta1TagField>>,
    /// The resource name of the tag in URL format. Example: * projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}/tags/{tag_id} where `tag_id` is a system-generated identifier. Note that this Tag may not actually be stored in the location in this name.
    
    pub name: Option<String>,
    /// Required. The resource name of the tag template that this tag uses. Example: * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id} This field cannot be modified after creation.
    
    pub template: Option<String>,
    /// Output only. The display name of the tag template.
    #[serde(rename="templateDisplayName")]
    
    pub template_display_name: Option<String>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1beta1Tag {}
impl client::ResponseResult for GoogleCloudDatacatalogV1beta1Tag {}


/// Contains the value and supporting information for a field within a Tag.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1TagField {
    /// Holds the value for a tag field with boolean type.
    #[serde(rename="boolValue")]
    
    pub bool_value: Option<bool>,
    /// Output only. The display name of this field.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Holds the value for a tag field with double type.
    #[serde(rename="doubleValue")]
    
    pub double_value: Option<f64>,
    /// Holds the value for a tag field with enum type. This value must be one of the allowed values in the definition of this enum.
    #[serde(rename="enumValue")]
    
    pub enum_value: Option<GoogleCloudDatacatalogV1beta1TagFieldEnumValue>,
    /// Output only. The order of this field with respect to other fields in this tag. It can be set in Tag. For example, a higher value can indicate a more important field. The value can be negative. Multiple fields can have the same order, and field orders within a tag do not have to be sequential.
    
    pub order: Option<i32>,
    /// Holds the value for a tag field with string type.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
    /// Holds the value for a tag field with timestamp type.
    #[serde(rename="timestampValue")]
    
    pub timestamp_value: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1TagField {}


/// Holds an enum value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1TagFieldEnumValue {
    /// The display name of the enum value.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1TagFieldEnumValue {}


/// A tag template defines a tag, which can have one or more typed fields. The template is used to create and attach the tag to GCP resources. [Tag template roles](https://cloud.google.com/iam/docs/understanding-roles#data-catalog-roles) provide permissions to create, edit, and use the template. See, for example, the [TagTemplate User](https://cloud.google.com/data-catalog/docs/how-to/template-user) role, which includes permission to use the tag template to tag resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations tag templates create projects](ProjectLocationTagTemplateCreateCall) (request|response)
/// * [locations tag templates get projects](ProjectLocationTagTemplateGetCall) (response)
/// * [locations tag templates patch projects](ProjectLocationTagTemplatePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1TagTemplate {
    /// The display name for this template. Defaults to an empty string.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. Map of tag template field IDs to the settings for the field. This map is an exhaustive list of the allowed fields. This map must contain at least one field and at most 500 fields. The keys to this map are tag template field IDs. Field IDs can contain letters (both uppercase and lowercase), numbers (0-9) and underscores (_). Field IDs must be at least 1 character long and at most 64 characters long. Field IDs must start with a letter or underscore.
    
    pub fields: Option<HashMap<String, GoogleCloudDatacatalogV1beta1TagTemplateField>>,
    /// The resource name of the tag template in URL format. Example: * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id} Note that this TagTemplate and its child resources may not actually be stored in the location in this name.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1beta1TagTemplate {}
impl client::ResponseResult for GoogleCloudDatacatalogV1beta1TagTemplate {}


/// The template for an individual field within a tag template.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations tag templates fields enum values rename projects](ProjectLocationTagTemplateFieldEnumValueRenameCall) (response)
/// * [locations tag templates fields create projects](ProjectLocationTagTemplateFieldCreateCall) (request|response)
/// * [locations tag templates fields patch projects](ProjectLocationTagTemplateFieldPatchCall) (request|response)
/// * [locations tag templates fields rename projects](ProjectLocationTagTemplateFieldRenameCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1TagTemplateField {
    /// The description for this field. Defaults to an empty string.
    
    pub description: Option<String>,
    /// The display name for this field. Defaults to an empty string.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Whether this is a required field. Defaults to false.
    #[serde(rename="isRequired")]
    
    pub is_required: Option<bool>,
    /// Output only. The resource name of the tag template field in URL format. Example: * projects/{project_id}/locations/{location}/tagTemplates/{tag_template}/fields/{field} Note that this TagTemplateField may not actually be stored in the location in this name.
    
    pub name: Option<String>,
    /// The order of this field with respect to other fields in this tag template. A higher value indicates a more important field. The value can be negative. Multiple fields can have the same order, and field orders within a tag do not have to be sequential.
    
    pub order: Option<i32>,
    /// Required. The type of value this tag field can contain.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudDatacatalogV1beta1FieldType>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1beta1TagTemplateField {}
impl client::ResponseResult for GoogleCloudDatacatalogV1beta1TagTemplateField {}


/// A taxonomy is a collection of policy tags that classify data along a common axis. For instance a data *sensitivity* taxonomy could contain policy tags denoting PII such as age, zipcode, and SSN. A data *origin* taxonomy could contain policy tags to distinguish user data, employee data, partner data, public data.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations taxonomies create projects](ProjectLocationTaxonomyCreateCall) (request|response)
/// * [locations taxonomies get projects](ProjectLocationTaxonomyGetCall) (response)
/// * [locations taxonomies patch projects](ProjectLocationTaxonomyPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1Taxonomy {
    /// Optional. A list of policy types that are activated for this taxonomy. If not set, defaults to an empty list.
    #[serde(rename="activatedPolicyTypes")]
    
    pub activated_policy_types: Option<Vec<GoogleCloudDatacatalogV1beta1TaxonomyActivatedPolicyTypesEnum>>,
    /// Optional. Description of this taxonomy. It must: contain only unicode characters, tabs, newlines, carriage returns and page breaks; and be at most 2000 bytes long when encoded in UTF-8. If not set, defaults to an empty description.
    
    pub description: Option<String>,
    /// Required. User defined name of this taxonomy. It must: contain only unicode letters, numbers, underscores, dashes and spaces; not start or end with spaces; and be at most 200 bytes long when encoded in UTF-8. The taxonomy display name must be unique within an organization.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Resource name of this taxonomy, whose format is: "projects/{project_number}/locations/{location_id}/taxonomies/{id}".
    
    pub name: Option<String>,
    /// Output only. Number of policy tags contained in this taxonomy.
    #[serde(rename="policyTagCount")]
    
    pub policy_tag_count: Option<i32>,
    /// Output only. Identity of the service which owns the Taxonomy. This field is only populated when the taxonomy is created by a GCP service. Currently only 'DATAPLEX' is supported.
    
    pub service: Option<GoogleCloudDatacatalogV1beta1TaxonomyService>,
    /// Output only. Timestamps about this taxonomy. Only create_time and update_time are used.
    #[serde(rename="taxonomyTimestamps")]
    
    pub taxonomy_timestamps: Option<GoogleCloudDatacatalogV1beta1SystemTimestamps>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1beta1Taxonomy {}
impl client::ResponseResult for GoogleCloudDatacatalogV1beta1Taxonomy {}


/// The source system of the Taxonomy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1TaxonomyService {
    /// P4SA Identity of the service.
    
    pub identity: Option<String>,
    /// The GCP service name.
    
    pub name: Option<GoogleCloudDatacatalogV1beta1TaxonomyServiceNameEnum>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1TaxonomyService {}


/// The set of all usage signals that we store in Data Catalog.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1UsageSignal {
    /// The timestamp of the end of the usage statistics duration.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Usage statistics over each of the pre-defined time ranges, supported strings for time ranges are {"24H", "7D", "30D"}.
    #[serde(rename="usageWithinTimeRange")]
    
    pub usage_within_time_range: Option<HashMap<String, GoogleCloudDatacatalogV1beta1UsageStats>>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1UsageSignal {}


/// Detailed counts on the entry's usage. Caveats: - Only BigQuery tables have usage stats - The usage stats only include BigQuery query jobs - The usage stats might be underestimated, e.g. wildcard table references are not yet counted in usage computation https://cloud.google.com/bigquery/docs/querying-wildcard-tables
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1UsageStats {
    /// The number of times that the underlying entry was attempted to be used but was cancelled by the user.
    #[serde(rename="totalCancellations")]
    
    pub total_cancellations: Option<f32>,
    /// The number of times that the underlying entry was successfully used.
    #[serde(rename="totalCompletions")]
    
    pub total_completions: Option<f32>,
    /// Total time spent (in milliseconds) during uses the resulted in completions.
    #[serde(rename="totalExecutionTimeForCompletionsMillis")]
    
    pub total_execution_time_for_completions_millis: Option<f32>,
    /// The number of times that the underlying entry was attempted to be used but failed.
    #[serde(rename="totalFailures")]
    
    pub total_failures: Option<f32>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1UsageStats {}


/// Table view specification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1beta1ViewSpec {
    /// Output only. The query that defines the table view.
    #[serde(rename="viewQuery")]
    
    pub view_query: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1beta1ViewSpec {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries get iam policy projects](ProjectLocationEntryGroupEntryGetIamPolicyCall) (response)
/// * [locations entry groups get iam policy projects](ProjectLocationEntryGroupGetIamPolicyCall) (response)
/// * [locations entry groups set iam policy projects](ProjectLocationEntryGroupSetIamPolicyCall) (response)
/// * [locations tag templates get iam policy projects](ProjectLocationTagTemplateGetIamPolicyCall) (response)
/// * [locations tag templates set iam policy projects](ProjectLocationTagTemplateSetIamPolicyCall) (response)
/// * [locations taxonomies policy tags get iam policy projects](ProjectLocationTaxonomyPolicyTagGetIamPolicyCall) (response)
/// * [locations taxonomies policy tags set iam policy projects](ProjectLocationTaxonomyPolicyTagSetIamPolicyCall) (response)
/// * [locations taxonomies get iam policy projects](ProjectLocationTaxonomyGetIamPolicyCall) (response)
/// * [locations taxonomies set iam policy projects](ProjectLocationTaxonomySetIamPolicyCall) (response)
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


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups set iam policy projects](ProjectLocationEntryGroupSetIamPolicyCall) (request)
/// * [locations tag templates set iam policy projects](ProjectLocationTagTemplateSetIamPolicyCall) (request)
/// * [locations taxonomies policy tags set iam policy projects](ProjectLocationTaxonomyPolicyTagSetIamPolicyCall) (request)
/// * [locations taxonomies set iam policy projects](ProjectLocationTaxonomySetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries test iam permissions projects](ProjectLocationEntryGroupEntryTestIamPermissionCall) (request)
/// * [locations entry groups test iam permissions projects](ProjectLocationEntryGroupTestIamPermissionCall) (request)
/// * [locations tag templates test iam permissions projects](ProjectLocationTagTemplateTestIamPermissionCall) (request)
/// * [locations taxonomies policy tags test iam permissions projects](ProjectLocationTaxonomyPolicyTagTestIamPermissionCall) (request)
/// * [locations taxonomies test iam permissions projects](ProjectLocationTaxonomyTestIamPermissionCall) (request)
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
/// * [locations entry groups entries test iam permissions projects](ProjectLocationEntryGroupEntryTestIamPermissionCall) (response)
/// * [locations entry groups test iam permissions projects](ProjectLocationEntryGroupTestIamPermissionCall) (response)
/// * [locations tag templates test iam permissions projects](ProjectLocationTagTemplateTestIamPermissionCall) (response)
/// * [locations taxonomies policy tags test iam permissions projects](ProjectLocationTaxonomyPolicyTagTestIamPermissionCall) (response)
/// * [locations taxonomies test iam permissions projects](ProjectLocationTaxonomyTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


