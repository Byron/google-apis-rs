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
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
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


/// Specification for the BigQuery connection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1BigQueryConnectionSpec {
    /// Specification for the BigQuery connection to a Cloud SQL instance.
    #[serde(rename="cloudSql")]
    
    pub cloud_sql: Option<GoogleCloudDatacatalogV1CloudSqlBigQueryConnectionSpec>,
    /// The type of the BigQuery connection.
    #[serde(rename="connectionType")]
    
    pub connection_type: Option<GoogleCloudDatacatalogV1BigQueryConnectionSpecConnectionTypeEnum>,
    /// True if there are credentials attached to the BigQuery connection; false otherwise.
    #[serde(rename="hasCredential")]
    
    pub has_credential: Option<bool>,
}

impl client::Part for GoogleCloudDatacatalogV1BigQueryConnectionSpec {}


/// Specification for a group of BigQuery tables with the `[prefix]YYYYMMDD` name pattern. For more information, see [Introduction to partitioned tables] (https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1BigQueryDateShardedSpec {
    /// Output only. The Data Catalog resource name of the dataset entry the current table belongs to. For example: `projects/{PROJECT_ID}/locations/{LOCATION}/entrygroups/{ENTRY_GROUP_ID}/entries/{ENTRY_ID}`.
    
    pub dataset: Option<String>,
    /// Output only. BigQuery resource name of the latest shard.
    #[serde(rename="latestShardResource")]
    
    pub latest_shard_resource: Option<String>,
    /// Output only. Total number of shards.
    #[serde(rename="shardCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub shard_count: Option<i64>,
    /// Output only. The table name prefix of the shards. The name of any given shard is `[table_prefix]YYYYMMDD`. For example, for the `MyTable20180101` shard, the `table_prefix` is `MyTable`.
    #[serde(rename="tablePrefix")]
    
    pub table_prefix: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1BigQueryDateShardedSpec {}


/// Fields specific for BigQuery routines.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1BigQueryRoutineSpec {
    /// Paths of the imported libraries.
    #[serde(rename="importedLibraries")]
    
    pub imported_libraries: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDatacatalogV1BigQueryRoutineSpec {}


/// Describes a BigQuery table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1BigQueryTableSpec {
    /// Output only. The table source type.
    #[serde(rename="tableSourceType")]
    
    pub table_source_type: Option<GoogleCloudDatacatalogV1BigQueryTableSpecTableSourceTypeEnum>,
    /// Specification of a BigQuery table. Populated only if the `table_source_type` is `BIGQUERY_TABLE`.
    #[serde(rename="tableSpec")]
    
    pub table_spec: Option<GoogleCloudDatacatalogV1TableSpec>,
    /// Table view specification. Populated only if the `table_source_type` is `BIGQUERY_VIEW`.
    #[serde(rename="viewSpec")]
    
    pub view_spec: Option<GoogleCloudDatacatalogV1ViewSpec>,
}

impl client::Part for GoogleCloudDatacatalogV1BigQueryTableSpec {}


/// Business Context of the entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1BusinessContext {
    /// Contact people for the entry.
    
    pub contacts: Option<GoogleCloudDatacatalogV1Contacts>,
    /// Entry overview fields for rich text descriptions of entries.
    #[serde(rename="entryOverview")]
    
    pub entry_overview: Option<GoogleCloudDatacatalogV1EntryOverview>,
}

impl client::Part for GoogleCloudDatacatalogV1BusinessContext {}


/// Specification for the BigQuery connection to a Cloud SQL instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1CloudSqlBigQueryConnectionSpec {
    /// Database name.
    
    pub database: Option<String>,
    /// Cloud SQL instance ID in the format of `project:location:instance`.
    #[serde(rename="instanceId")]
    
    pub instance_id: Option<String>,
    /// Type of the Cloud SQL database.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudDatacatalogV1CloudSqlBigQueryConnectionSpecTypeEnum>,
}

impl client::Part for GoogleCloudDatacatalogV1CloudSqlBigQueryConnectionSpec {}


/// A column within a schema. Columns can be nested inside other columns.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1ColumnSchema {
    /// Required. Name of the column. Must be a UTF-8 string without dots (.). The maximum size is 64 bytes.
    
    pub column: Option<String>,
    /// Optional. Default value for the column.
    #[serde(rename="defaultValue")]
    
    pub default_value: Option<String>,
    /// Optional. Description of the column. Default value is an empty string. The description must be a UTF-8 string with the maximum size of 2000 bytes.
    
    pub description: Option<String>,
    /// Optional. Garbage collection policy for the column or column family. Applies to systems like Cloud Bigtable.
    #[serde(rename="gcRule")]
    
    pub gc_rule: Option<String>,
    /// Optional. Most important inclusion of this column.
    #[serde(rename="highestIndexingType")]
    
    pub highest_indexing_type: Option<GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum>,
    /// Looker specific column info of this column.
    #[serde(rename="lookerColumnSpec")]
    
    pub looker_column_spec: Option<GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpec>,
    /// Optional. A column's mode indicates whether values in this column are required, nullable, or repeated. Only `NULLABLE`, `REQUIRED`, and `REPEATED` values are supported. Default mode is `NULLABLE`.
    
    pub mode: Option<String>,
    /// Optional. Ordinal position
    #[serde(rename="ordinalPosition")]
    
    pub ordinal_position: Option<i32>,
    /// Optional. Schema of sub-columns. A column can have zero or more sub-columns.
    
    pub subcolumns: Option<Vec<GoogleCloudDatacatalogV1ColumnSchema>>,
    /// Required. Type of the column. Must be a UTF-8 string with the maximum size of 128 bytes.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1ColumnSchema {}


/// Column info specific to Looker System.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpec {
    /// Looker specific column type of this column.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum>,
}

impl client::Part for GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpec {}


/// Common statistics on the entry's usage. They can be set on any system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1CommonUsageStats {
    /// View count in source system.
    #[serde(rename="viewCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub view_count: Option<i64>,
}

impl client::Part for GoogleCloudDatacatalogV1CommonUsageStats {}


/// Contact people for the entry.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries modify entry contacts projects](ProjectLocationEntryGroupEntryModifyEntryContactCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1Contacts {
    /// The list of contact people for the entry.
    
    pub people: Option<Vec<GoogleCloudDatacatalogV1ContactsPerson>>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1Contacts {}


/// A contact person for the entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1ContactsPerson {
    /// Designation of the person, for example, Data Steward.
    
    pub designation: Option<String>,
    /// Email of the person in the format of `john.doe@xyz`, ``, or `John Doe`.
    
    pub email: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1ContactsPerson {}


/// Cross-regional source used to import an existing taxonomy into a different region.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1CrossRegionalSource {
    /// Required. The resource name of the source taxonomy to import.
    
    pub taxonomy: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1CrossRegionalSource {}


/// Physical location of an entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1DataSource {
    /// Full name of a resource as defined by the service. For example: `//bigquery.googleapis.com/projects/{PROJECT_ID}/locations/{LOCATION}/datasets/{DATASET_ID}/tables/{TABLE_ID}`
    
    pub resource: Option<String>,
    /// Service that physically stores the data.
    
    pub service: Option<GoogleCloudDatacatalogV1DataSourceServiceEnum>,
    /// Output only. Data Catalog entry name, if applicable.
    #[serde(rename="sourceEntry")]
    
    pub source_entry: Option<String>,
    /// Detailed properties of the underlying storage.
    #[serde(rename="storageProperties")]
    
    pub storage_properties: Option<GoogleCloudDatacatalogV1StorageProperties>,
}

impl client::Part for GoogleCloudDatacatalogV1DataSource {}


/// Specification that applies to a data source connection. Valid only for entries with the `DATA_SOURCE_CONNECTION` type. Only one of internal specs can be set at the time, and cannot be changed later.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1DataSourceConnectionSpec {
    /// Output only. Fields specific to BigQuery connections.
    #[serde(rename="bigqueryConnectionSpec")]
    
    pub bigquery_connection_spec: Option<GoogleCloudDatacatalogV1BigQueryConnectionSpec>,
}

impl client::Part for GoogleCloudDatacatalogV1DataSourceConnectionSpec {}


/// Specification that applies to a table resource. Valid only for entries with the `TABLE` type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1DatabaseTableSpec {
    /// Spec what aplies to tables that are actually views. Not set for "real" tables.
    #[serde(rename="databaseViewSpec")]
    
    pub database_view_spec: Option<GoogleCloudDatacatalogV1DatabaseTableSpecDatabaseViewSpec>,
    /// Output only. Fields specific to a Dataplex table and present only in the Dataplex table entries.
    #[serde(rename="dataplexTable")]
    
    pub dataplex_table: Option<GoogleCloudDatacatalogV1DataplexTableSpec>,
    /// Type of this table.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudDatacatalogV1DatabaseTableSpecTypeEnum>,
}

impl client::Part for GoogleCloudDatacatalogV1DatabaseTableSpec {}


/// Specification that applies to database view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1DatabaseTableSpecDatabaseViewSpec {
    /// Name of a singular table this view reflects one to one.
    #[serde(rename="baseTable")]
    
    pub base_table: Option<String>,
    /// SQL query used to generate this view.
    #[serde(rename="sqlQuery")]
    
    pub sql_query: Option<String>,
    /// Type of this view.
    #[serde(rename="viewType")]
    
    pub view_type: Option<GoogleCloudDatacatalogV1DatabaseTableSpecDatabaseViewSpecViewTypeEnum>,
}

impl client::Part for GoogleCloudDatacatalogV1DatabaseTableSpecDatabaseViewSpec {}


/// External table registered by Dataplex. Dataplex publishes data discovered from an asset into multiple other systems (BigQuery, DPMS) in form of tables. We call them "external tables". External tables are also synced into the Data Catalog. This message contains pointers to those external tables (fully qualified name, resource name et cetera) within the Data Catalog.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1DataplexExternalTable {
    /// Name of the Data Catalog entry representing the external table.
    #[serde(rename="dataCatalogEntry")]
    
    pub data_catalog_entry: Option<String>,
    /// Fully qualified name (FQN) of the external table.
    #[serde(rename="fullyQualifiedName")]
    
    pub fully_qualified_name: Option<String>,
    /// Google Cloud resource name of the external table.
    #[serde(rename="googleCloudResource")]
    
    pub google_cloud_resource: Option<String>,
    /// Service in which the external table is registered.
    
    pub system: Option<GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum>,
}

impl client::Part for GoogleCloudDatacatalogV1DataplexExternalTable {}


/// Entry specyfication for a Dataplex fileset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1DataplexFilesetSpec {
    /// Common Dataplex fields.
    #[serde(rename="dataplexSpec")]
    
    pub dataplex_spec: Option<GoogleCloudDatacatalogV1DataplexSpec>,
}

impl client::Part for GoogleCloudDatacatalogV1DataplexFilesetSpec {}


/// Common Dataplex fields.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1DataplexSpec {
    /// Fully qualified resource name of an asset in Dataplex, to which the underlying data source (Cloud Storage bucket or BigQuery dataset) of the entity is attached.
    
    pub asset: Option<String>,
    /// Compression format of the data, e.g., zip, gzip etc.
    #[serde(rename="compressionFormat")]
    
    pub compression_format: Option<String>,
    /// Format of the data.
    #[serde(rename="dataFormat")]
    
    pub data_format: Option<GoogleCloudDatacatalogV1PhysicalSchema>,
    /// Project ID of the underlying Cloud Storage or BigQuery data. Note that this may not be the same project as the correspondingly Dataplex lake / zone / asset.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1DataplexSpec {}


/// Entry specification for a Dataplex table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1DataplexTableSpec {
    /// Common Dataplex fields.
    #[serde(rename="dataplexSpec")]
    
    pub dataplex_spec: Option<GoogleCloudDatacatalogV1DataplexSpec>,
    /// List of external tables registered by Dataplex in other systems based on the same underlying data. External tables allow to query this data in those systems.
    #[serde(rename="externalTables")]
    
    pub external_tables: Option<Vec<GoogleCloudDatacatalogV1DataplexExternalTable>>,
    /// Indicates if the table schema is managed by the user or not.
    #[serde(rename="userManaged")]
    
    pub user_managed: Option<bool>,
}

impl client::Part for GoogleCloudDatacatalogV1DataplexTableSpec {}


/// Entry metadata. A Data Catalog entry represents another resource in Google Cloud Platform (such as a BigQuery dataset or a Pub/Sub topic) or outside of it. You can use the `linked_resource` field in the entry resource to refer to the original resource ID of the source system. An entry resource contains resource details, for example, its schema. Additionally, you can attach flexible metadata to an entry in the form of a Tag.
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
pub struct GoogleCloudDatacatalogV1Entry {
    /// Output only. Specification for a group of BigQuery tables with the `[prefix]YYYYMMDD` name pattern. For more information, see [Introduction to partitioned tables] (https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding).
    #[serde(rename="bigqueryDateShardedSpec")]
    
    pub bigquery_date_sharded_spec: Option<GoogleCloudDatacatalogV1BigQueryDateShardedSpec>,
    /// Output only. Specification that applies to a BigQuery table. Valid only for entries with the `TABLE` type.
    #[serde(rename="bigqueryTableSpec")]
    
    pub bigquery_table_spec: Option<GoogleCloudDatacatalogV1BigQueryTableSpec>,
    /// Business Context of the entry. Not supported for BigQuery datasets
    #[serde(rename="businessContext")]
    
    pub business_context: Option<GoogleCloudDatacatalogV1BusinessContext>,
    /// Output only. Physical location of the entry.
    #[serde(rename="dataSource")]
    
    pub data_source: Option<GoogleCloudDatacatalogV1DataSource>,
    /// Specification that applies to a data source connection. Valid only for entries with the `DATA_SOURCE_CONNECTION` type.
    #[serde(rename="dataSourceConnectionSpec")]
    
    pub data_source_connection_spec: Option<GoogleCloudDatacatalogV1DataSourceConnectionSpec>,
    /// Specification that applies to a table resource. Valid only for entries with the `TABLE` or `EXPLORE` type.
    #[serde(rename="databaseTableSpec")]
    
    pub database_table_spec: Option<GoogleCloudDatacatalogV1DatabaseTableSpec>,
    /// Entry description that can consist of several sentences or paragraphs that describe entry contents. The description must not contain Unicode non-characters as well as C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF). The maximum size is 2000 bytes when encoded in UTF-8. Default value is an empty string.
    
    pub description: Option<String>,
    /// Display name of an entry. The maximum size is 500 bytes when encoded in UTF-8. Default value is an empty string.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Specification that applies to a fileset resource. Valid only for entries with the `FILESET` type.
    #[serde(rename="filesetSpec")]
    
    pub fileset_spec: Option<GoogleCloudDatacatalogV1FilesetSpec>,
    /// Fully qualified name (FQN) of the resource. Set automatically for entries representing resources from synced systems. Settable only during creation and read-only afterwards. Can be used for search and lookup of the entries. FQNs take two forms: * For non-regionalized resources: `{SYSTEM}:{PROJECT}.{PATH_TO_RESOURCE_SEPARATED_WITH_DOTS}` * For regionalized resources: `{SYSTEM}:{PROJECT}.{LOCATION_ID}.{PATH_TO_RESOURCE_SEPARATED_WITH_DOTS}` Example for a DPMS table: `dataproc_metastore:{PROJECT_ID}.{LOCATION_ID}.{INSTANCE_ID}.{DATABASE_ID}.{TABLE_ID}`
    #[serde(rename="fullyQualifiedName")]
    
    pub fully_qualified_name: Option<String>,
    /// Specification that applies to a Cloud Storage fileset. Valid only for entries with the `FILESET` type.
    #[serde(rename="gcsFilesetSpec")]
    
    pub gcs_fileset_spec: Option<GoogleCloudDatacatalogV1GcsFilesetSpec>,
    /// Output only. Indicates the entry's source system that Data Catalog integrates with, such as BigQuery, Pub/Sub, or Dataproc Metastore.
    #[serde(rename="integratedSystem")]
    
    pub integrated_system: Option<GoogleCloudDatacatalogV1EntryIntegratedSystemEnum>,
    /// Cloud labels attached to the entry. In Data Catalog, you can create and modify labels attached only to custom entries. Synced entries have unmodifiable labels that come from the source system.
    
    pub labels: Option<HashMap<String, String>>,
    /// The resource this metadata entry refers to. For Google Cloud Platform resources, `linked_resource` is the [Full Resource Name] (https://cloud.google.com/apis/design/resource_names#full_resource_name). For example, the `linked_resource` for a table resource from BigQuery is: `//bigquery.googleapis.com/projects/{PROJECT_ID}/datasets/{DATASET_ID}/tables/{TABLE_ID}` Output only when the entry is one of the types in the `EntryType` enum. For entries with a `user_specified_type`, this field is optional and defaults to an empty string. The resource string must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), periods (.), colons (:), slashes (/), dashes (-), and hashes (#). The maximum size is 200 bytes when encoded in UTF-8.
    #[serde(rename="linkedResource")]
    
    pub linked_resource: Option<String>,
    /// Specification that applies to Looker sysstem. Only settable when `user_specified_system` is equal to `LOOKER`
    #[serde(rename="lookerSystemSpec")]
    
    pub looker_system_spec: Option<GoogleCloudDatacatalogV1LookerSystemSpec>,
    /// Output only. The resource name of an entry in URL format. Note: The entry itself and its child resources might not be stored in the location specified in its name.
    
    pub name: Option<String>,
    /// Output only. Additional information related to the entry. Private to the current user.
    #[serde(rename="personalDetails")]
    
    pub personal_details: Option<GoogleCloudDatacatalogV1PersonalDetails>,
    /// Specification that applies to a user-defined function or procedure. Valid only for entries with the `ROUTINE` type.
    #[serde(rename="routineSpec")]
    
    pub routine_spec: Option<GoogleCloudDatacatalogV1RoutineSpec>,
    /// Schema of the entry. An entry might not have any schema attached to it.
    
    pub schema: Option<GoogleCloudDatacatalogV1Schema>,
    /// Timestamps from the underlying resource, not from the Data Catalog entry. Output only when the entry has a system listed in the `IntegratedSystem` enum. For entries with `user_specified_system`, this field is optional and defaults to an empty timestamp.
    #[serde(rename="sourceSystemTimestamps")]
    
    pub source_system_timestamps: Option<GoogleCloudDatacatalogV1SystemTimestamps>,
    /// Specification that applies to a relational database system. Only settable when `user_specified_system` is equal to `SQL_DATABASE`
    #[serde(rename="sqlDatabaseSystemSpec")]
    
    pub sql_database_system_spec: Option<GoogleCloudDatacatalogV1SqlDatabaseSystemSpec>,
    /// The type of the entry. Only used for entries with types listed in the `EntryType` enum. Currently, only `FILESET` enum value is allowed. All other entries created in Data Catalog must use the `user_specified_type`.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudDatacatalogV1EntryTypeEnum>,
    /// Resource usage statistics.
    #[serde(rename="usageSignal")]
    
    pub usage_signal: Option<GoogleCloudDatacatalogV1UsageSignal>,
    /// Indicates the entry's source system that Data Catalog doesn't automatically integrate with. The `user_specified_system` string has the following limitations: * Is case insensitive. * Must begin with a letter or underscore. * Can only contain letters, numbers, and underscores. * Must be at least 1 character and at most 64 characters long.
    #[serde(rename="userSpecifiedSystem")]
    
    pub user_specified_system: Option<String>,
    /// Custom entry type that doesn't match any of the values allowed for input and listed in the `EntryType` enum. When creating an entry, first check the type values in the enum. If there are no appropriate types for the new entry, provide a custom value, for example, `my_special_type`. The `user_specified_type` string has the following limitations: * Is case insensitive. * Must begin with a letter or underscore. * Can only contain letters, numbers, and underscores. * Must be at least 1 character and at most 64 characters long.
    #[serde(rename="userSpecifiedType")]
    
    pub user_specified_type: Option<String>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1Entry {}
impl client::ResponseResult for GoogleCloudDatacatalogV1Entry {}


/// Entry group metadata. An `EntryGroup` resource represents a logical grouping of zero or more Data Catalog Entry resources.
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
pub struct GoogleCloudDatacatalogV1EntryGroup {
    /// Output only. Timestamps of the entry group. Default value is empty.
    #[serde(rename="dataCatalogTimestamps")]
    
    pub data_catalog_timestamps: Option<GoogleCloudDatacatalogV1SystemTimestamps>,
    /// Entry group description. Can consist of several sentences or paragraphs that describe the entry group contents. Default value is an empty string.
    
    pub description: Option<String>,
    /// A short name to identify the entry group, for example, "analytics data - jan 2011". Default value is an empty string.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The resource name of the entry group in URL format. Note: The entry group itself and its child resources might not be stored in the location specified in its name.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1EntryGroup {}
impl client::ResponseResult for GoogleCloudDatacatalogV1EntryGroup {}


/// Entry overview fields for rich text descriptions of entries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries modify entry overview projects](ProjectLocationEntryGroupEntryModifyEntryOverviewCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1EntryOverview {
    /// Entry overview with support for rich text. The overview must only contain Unicode characters, and should be formatted using HTML. The maximum length is 10 MiB as this value holds HTML descriptions including encoded images. The maximum length of the text without images is 100 KiB.
    
    pub overview: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1EntryOverview {}


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
pub struct GoogleCloudDatacatalogV1ExportTaxonomiesResponse {
    /// List of taxonomies and policy tags as nested protocol buffers.
    
    pub taxonomies: Option<Vec<GoogleCloudDatacatalogV1SerializedTaxonomy>>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1ExportTaxonomiesResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1FieldType {
    /// An enum type.
    #[serde(rename="enumType")]
    
    pub enum_type: Option<GoogleCloudDatacatalogV1FieldTypeEnumType>,
    /// Primitive types, such as string, boolean, etc.
    #[serde(rename="primitiveType")]
    
    pub primitive_type: Option<GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum>,
}

impl client::Part for GoogleCloudDatacatalogV1FieldType {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1FieldTypeEnumType {
    /// The set of allowed values for this enum. This set must not be empty and can include up to 100 allowed values. The display names of the values in this set must not be empty and must be case-insensitively unique within this set. The order of items in this set is preserved. This field can be used to create, remove, and reorder enum values. To rename enum values, use the `RenameTagTemplateFieldEnumValue` method.
    #[serde(rename="allowedValues")]
    
    pub allowed_values: Option<Vec<GoogleCloudDatacatalogV1FieldTypeEnumTypeEnumValue>>,
}

impl client::Part for GoogleCloudDatacatalogV1FieldTypeEnumType {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1FieldTypeEnumTypeEnumValue {
    /// Required. The display name of the enum value. Must not be an empty string. The name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and can't start or end with spaces. The maximum length is 200 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1FieldTypeEnumTypeEnumValue {}


/// Specification that applies to a fileset. Valid only for entries with the 'FILESET' type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1FilesetSpec {
    /// Fields specific to a Dataplex fileset and present only in the Dataplex fileset entries.
    #[serde(rename="dataplexFileset")]
    
    pub dataplex_fileset: Option<GoogleCloudDatacatalogV1DataplexFilesetSpec>,
}

impl client::Part for GoogleCloudDatacatalogV1FilesetSpec {}


/// Specification of a single file in Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1GcsFileSpec {
    /// Required. Full file path. Example: `gs://bucket_name/a/b.txt`.
    #[serde(rename="filePath")]
    
    pub file_path: Option<String>,
    /// Output only. Creation, modification, and expiration timestamps of a Cloud Storage file.
    #[serde(rename="gcsTimestamps")]
    
    pub gcs_timestamps: Option<GoogleCloudDatacatalogV1SystemTimestamps>,
    /// Output only. File size in bytes.
    #[serde(rename="sizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size_bytes: Option<i64>,
}

impl client::Part for GoogleCloudDatacatalogV1GcsFileSpec {}


/// Describes a Cloud Storage fileset entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1GcsFilesetSpec {
    /// Required. Patterns to identify a set of files in Google Cloud Storage. For more information, see [Wildcard Names] (https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames). Note: Currently, bucket wildcards are not supported. Examples of valid `file_patterns`: * `gs://bucket_name/dir/*`: matches all files in `bucket_name/dir` directory * `gs://bucket_name/dir/**`: matches all files in `bucket_name/dir` and all subdirectories * `gs://bucket_name/file*`: matches files prefixed by `file` in `bucket_name` * `gs://bucket_name/??.txt`: matches files with two characters followed by `.txt` in `bucket_name` * `gs://bucket_name/[aeiou].txt`: matches files that contain a single vowel character followed by `.txt` in `bucket_name` * `gs://bucket_name/[a-m].txt`: matches files that contain `a`, `b`, ... or `m` followed by `.txt` in `bucket_name` * `gs://bucket_name/a/*/b`: matches all files in `bucket_name` that match the `a/*/b` pattern, such as `a/c/b`, `a/d/b` * `gs://another_bucket/a.txt`: matches `gs://another_bucket/a.txt` You can combine wildcards to match complex sets of files, for example: `gs://bucket_name/[a-m]??.j*g`
    #[serde(rename="filePatterns")]
    
    pub file_patterns: Option<Vec<String>>,
    /// Output only. Sample files contained in this fileset, not all files contained in this fileset are represented here.
    #[serde(rename="sampleGcsFileSpecs")]
    
    pub sample_gcs_file_specs: Option<Vec<GoogleCloudDatacatalogV1GcsFileSpec>>,
}

impl client::Part for GoogleCloudDatacatalogV1GcsFilesetSpec {}


/// Request message for ImportEntries method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries import projects](ProjectLocationEntryGroupEntryImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1ImportEntriesRequest {
    /// Path to a Cloud Storage bucket that contains a dump ready for ingestion.
    #[serde(rename="gcsBucketPath")]
    
    pub gcs_bucket_path: Option<String>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1ImportEntriesRequest {}


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
pub struct GoogleCloudDatacatalogV1ImportTaxonomiesRequest {
    /// Cross-regional source taxonomy to import.
    #[serde(rename="crossRegionalSource")]
    
    pub cross_regional_source: Option<GoogleCloudDatacatalogV1CrossRegionalSource>,
    /// Inline source taxonomy to import.
    #[serde(rename="inlineSource")]
    
    pub inline_source: Option<GoogleCloudDatacatalogV1InlineSource>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1ImportTaxonomiesRequest {}


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
pub struct GoogleCloudDatacatalogV1ImportTaxonomiesResponse {
    /// Imported taxonomies.
    
    pub taxonomies: Option<Vec<GoogleCloudDatacatalogV1Taxonomy>>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1ImportTaxonomiesResponse {}


/// Inline source containing taxonomies to import.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1InlineSource {
    /// Required. Taxonomies to import.
    
    pub taxonomies: Option<Vec<GoogleCloudDatacatalogV1SerializedTaxonomy>>,
}

impl client::Part for GoogleCloudDatacatalogV1InlineSource {}


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
pub struct GoogleCloudDatacatalogV1ListEntriesResponse {
    /// Entry details.
    
    pub entries: Option<Vec<GoogleCloudDatacatalogV1Entry>>,
    /// Pagination token of the next results page. Empty if there are no more items in results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1ListEntriesResponse {}


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
pub struct GoogleCloudDatacatalogV1ListEntryGroupsResponse {
    /// Entry group details.
    #[serde(rename="entryGroups")]
    
    pub entry_groups: Option<Vec<GoogleCloudDatacatalogV1EntryGroup>>,
    /// Pagination token to specify in the next call to retrieve the next page of results. Empty if there are no more items.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1ListEntryGroupsResponse {}


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
pub struct GoogleCloudDatacatalogV1ListPolicyTagsResponse {
    /// Pagination token of the next results page. Empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The policy tags that belong to the taxonomy.
    #[serde(rename="policyTags")]
    
    pub policy_tags: Option<Vec<GoogleCloudDatacatalogV1PolicyTag>>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1ListPolicyTagsResponse {}


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
pub struct GoogleCloudDatacatalogV1ListTagsResponse {
    /// Pagination token of the next results page. Empty if there are no more items in results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Tag details.
    
    pub tags: Option<Vec<GoogleCloudDatacatalogV1Tag>>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1ListTagsResponse {}


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
pub struct GoogleCloudDatacatalogV1ListTaxonomiesResponse {
    /// Pagination token of the next results page. Empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Taxonomies that the project contains.
    
    pub taxonomies: Option<Vec<GoogleCloudDatacatalogV1Taxonomy>>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1ListTaxonomiesResponse {}


/// Specification that applies to entries that are part `LOOKER` system (user_specified_type)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1LookerSystemSpec {
    /// Name of the parent Looker Instance. Empty if it does not exist.
    #[serde(rename="parentInstanceDisplayName")]
    
    pub parent_instance_display_name: Option<String>,
    /// ID of the parent Looker Instance. Empty if it does not exist. Example value: `someinstance.looker.com`
    #[serde(rename="parentInstanceId")]
    
    pub parent_instance_id: Option<String>,
    /// Name of the parent Model. Empty if it does not exist.
    #[serde(rename="parentModelDisplayName")]
    
    pub parent_model_display_name: Option<String>,
    /// ID of the parent Model. Empty if it does not exist.
    #[serde(rename="parentModelId")]
    
    pub parent_model_id: Option<String>,
    /// Name of the parent View. Empty if it does not exist.
    #[serde(rename="parentViewDisplayName")]
    
    pub parent_view_display_name: Option<String>,
    /// ID of the parent View. Empty if it does not exist.
    #[serde(rename="parentViewId")]
    
    pub parent_view_id: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1LookerSystemSpec {}


/// Request message for ModifyEntryContacts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries modify entry contacts projects](ProjectLocationEntryGroupEntryModifyEntryContactCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1ModifyEntryContactsRequest {
    /// Required. The new value for the Contacts.
    
    pub contacts: Option<GoogleCloudDatacatalogV1Contacts>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1ModifyEntryContactsRequest {}


/// Request message for ModifyEntryOverview.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries modify entry overview projects](ProjectLocationEntryGroupEntryModifyEntryOverviewCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1ModifyEntryOverviewRequest {
    /// Required. The new value for the Entry Overview.
    #[serde(rename="entryOverview")]
    
    pub entry_overview: Option<GoogleCloudDatacatalogV1EntryOverview>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1ModifyEntryOverviewRequest {}


/// Entry metadata relevant only to the user and private to them.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1PersonalDetails {
    /// Set if the entry is starred; unset otherwise.
    #[serde(rename="starTime")]
    
    pub star_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// True if the entry is starred by the user; false otherwise.
    
    pub starred: Option<bool>,
}

impl client::Part for GoogleCloudDatacatalogV1PersonalDetails {}


/// Native schema used by a resource represented as an entry. Used by query engines for deserializing and parsing source data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1PhysicalSchema {
    /// Schema in Avro JSON format.
    
    pub avro: Option<GoogleCloudDatacatalogV1PhysicalSchemaAvroSchema>,
    /// Marks a CSV-encoded data source.
    
    pub csv: Option<GoogleCloudDatacatalogV1PhysicalSchemaCsvSchema>,
    /// Marks an ORC-encoded data source.
    
    pub orc: Option<GoogleCloudDatacatalogV1PhysicalSchemaOrcSchema>,
    /// Marks a Parquet-encoded data source.
    
    pub parquet: Option<GoogleCloudDatacatalogV1PhysicalSchemaParquetSchema>,
    /// Schema in protocol buffer format.
    
    pub protobuf: Option<GoogleCloudDatacatalogV1PhysicalSchemaProtobufSchema>,
    /// Schema in Thrift format.
    
    pub thrift: Option<GoogleCloudDatacatalogV1PhysicalSchemaThriftSchema>,
}

impl client::Part for GoogleCloudDatacatalogV1PhysicalSchema {}


/// Schema in Avro JSON format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1PhysicalSchemaAvroSchema {
    /// JSON source of the Avro schema.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1PhysicalSchemaAvroSchema {}


/// Marks a CSV-encoded data source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1PhysicalSchemaCsvSchema { _never_set: Option<bool> }

impl client::Part for GoogleCloudDatacatalogV1PhysicalSchemaCsvSchema {}


/// Marks an ORC-encoded data source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1PhysicalSchemaOrcSchema { _never_set: Option<bool> }

impl client::Part for GoogleCloudDatacatalogV1PhysicalSchemaOrcSchema {}


/// Marks a Parquet-encoded data source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1PhysicalSchemaParquetSchema { _never_set: Option<bool> }

impl client::Part for GoogleCloudDatacatalogV1PhysicalSchemaParquetSchema {}


/// Schema in protocol buffer format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1PhysicalSchemaProtobufSchema {
    /// Protocol buffer source of the schema.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1PhysicalSchemaProtobufSchema {}


/// Schema in Thrift format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1PhysicalSchemaThriftSchema {
    /// Thrift IDL source of the schema.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1PhysicalSchemaThriftSchema {}


/// Denotes one policy tag in a taxonomy, for example, SSN. Policy tags can be defined in a hierarchy. For example: `+ Geolocation + LatLong + City + ZipCode` Where the “Geolocation” policy tag contains three children.
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
pub struct GoogleCloudDatacatalogV1PolicyTag {
    /// Output only. Resource names of child policy tags of this policy tag.
    #[serde(rename="childPolicyTags")]
    
    pub child_policy_tags: Option<Vec<String>>,
    /// Description of this policy tag. If not set, defaults to empty. The description must contain only Unicode characters, tabs, newlines, carriage returns and page breaks, and be at most 2000 bytes long when encoded in UTF-8.
    
    pub description: Option<String>,
    /// Required. User-defined name of this policy tag. The name can't start or end with spaces and must be unique within the parent taxonomy, contain only Unicode letters, numbers, underscores, dashes and spaces, and be at most 200 bytes long when encoded in UTF-8.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Resource name of this policy tag in the URL format. The policy tag manager generates unique taxonomy IDs and policy tag IDs.
    
    pub name: Option<String>,
    /// Resource name of this policy tag's parent policy tag. If empty, this is a top level tag. If not set, defaults to an empty string. For example, for the "LatLong" policy tag in the example above, this field contains the resource name of the "Geolocation" policy tag, and, for "Geolocation", this field is empty.
    #[serde(rename="parentPolicyTag")]
    
    pub parent_policy_tag: Option<String>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1PolicyTag {}
impl client::ResponseResult for GoogleCloudDatacatalogV1PolicyTag {}


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
pub struct GoogleCloudDatacatalogV1RenameTagTemplateFieldEnumValueRequest {
    /// Required. The new display name of the enum value. For example, `my_new_enum_value`.
    #[serde(rename="newEnumValueDisplayName")]
    
    pub new_enum_value_display_name: Option<String>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1RenameTagTemplateFieldEnumValueRequest {}


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
pub struct GoogleCloudDatacatalogV1RenameTagTemplateFieldRequest {
    /// Required. The new ID of this tag template field. For example, `my_new_field`.
    #[serde(rename="newTagTemplateFieldId")]
    
    pub new_tag_template_field_id: Option<String>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1RenameTagTemplateFieldRequest {}


/// Request message for ReplaceTaxonomy.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations taxonomies replace projects](ProjectLocationTaxonomyReplaceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1ReplaceTaxonomyRequest {
    /// Required. Taxonomy to update along with its child policy tags.
    #[serde(rename="serializedTaxonomy")]
    
    pub serialized_taxonomy: Option<GoogleCloudDatacatalogV1SerializedTaxonomy>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1ReplaceTaxonomyRequest {}


/// Specification that applies to a routine. Valid only for entries with the `ROUTINE` type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1RoutineSpec {
    /// Fields specific for BigQuery routines.
    #[serde(rename="bigqueryRoutineSpec")]
    
    pub bigquery_routine_spec: Option<GoogleCloudDatacatalogV1BigQueryRoutineSpec>,
    /// The body of the routine.
    #[serde(rename="definitionBody")]
    
    pub definition_body: Option<String>,
    /// The language the routine is written in. The exact value depends on the source system. For BigQuery routines, possible values are: * `SQL` * `JAVASCRIPT`
    
    pub language: Option<String>,
    /// Return type of the argument. The exact value depends on the source system and the language.
    #[serde(rename="returnType")]
    
    pub return_type: Option<String>,
    /// Arguments of the routine.
    #[serde(rename="routineArguments")]
    
    pub routine_arguments: Option<Vec<GoogleCloudDatacatalogV1RoutineSpecArgument>>,
    /// The type of the routine.
    #[serde(rename="routineType")]
    
    pub routine_type: Option<GoogleCloudDatacatalogV1RoutineSpecRoutineTypeEnum>,
}

impl client::Part for GoogleCloudDatacatalogV1RoutineSpec {}


/// Input or output argument of a function or stored procedure.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1RoutineSpecArgument {
    /// Specifies whether the argument is input or output.
    
    pub mode: Option<GoogleCloudDatacatalogV1RoutineSpecArgumentModeEnum>,
    /// The name of the argument. A return argument of a function might not have a name.
    
    pub name: Option<String>,
    /// Type of the argument. The exact value depends on the source system and the language.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1RoutineSpecArgument {}


/// Represents a schema, for example, a BigQuery, GoogleSQL, or Avro schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1Schema {
    /// The unified GoogleSQL-like schema of columns. The overall maximum number of columns and nested columns is 10,000. The maximum nested depth is 15 levels.
    
    pub columns: Option<Vec<GoogleCloudDatacatalogV1ColumnSchema>>,
}

impl client::Part for GoogleCloudDatacatalogV1Schema {}


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
pub struct GoogleCloudDatacatalogV1SearchCatalogRequest {
    /// Specifies the order of results. Currently supported case-sensitive values are: * `relevance` that can only be descending * `last_modified_timestamp [asc|desc]` with descending (`desc`) as default * `default` that can only be descending If this parameter is omitted, it defaults to the descending `relevance`.
    #[serde(rename="orderBy")]
    
    pub order_by: Option<String>,
    /// Number of results to return in a single search page. Can't be negative or 0, defaults to 10 in this case. The maximum number is 1000. If exceeded, throws an "invalid argument" exception.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional. Pagination token that, if specified, returns the next page of search results. If empty, returns the first page. This token is returned in the SearchCatalogResponse.next_page_token field of the response to a previous SearchCatalogRequest call.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Optional. The query string with a minimum of 3 characters and specific syntax. For more information, see [Data Catalog search syntax](https://cloud.google.com/data-catalog/docs/how-to/search-reference). An empty query string returns all data assets (in the specified scope) that you have access to. A query string can be a simple `xyz` or qualified by predicates: * `name:x` * `column:y` * `description:z`
    
    pub query: Option<String>,
    /// Required. The scope of this search request. The `scope` is invalid if `include_org_ids`, `include_project_ids` are empty AND `include_gcp_public_datasets` is set to `false`. In this case, the request returns an error.
    
    pub scope: Option<GoogleCloudDatacatalogV1SearchCatalogRequestScope>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1SearchCatalogRequest {}


/// The criteria that select the subspace used for query matching.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1SearchCatalogRequestScope {
    /// If `true`, include Google Cloud Platform (GCP) public datasets in search results. By default, they are excluded. See [Google Cloud Public Datasets](https://cloud.google.com/public-datasets) for more information.
    #[serde(rename="includeGcpPublicDatasets")]
    
    pub include_gcp_public_datasets: Option<bool>,
    /// The list of organization IDs to search within. To find your organization ID, follow the steps from \[Creating and managing organizations\] (/resource-manager/docs/creating-managing-organization).
    #[serde(rename="includeOrgIds")]
    
    pub include_org_ids: Option<Vec<String>>,
    /// The list of project IDs to search within. For more information on the distinction between project names, IDs, and numbers, see [Projects](https://cloud.google.com/docs/overview/#projects).
    #[serde(rename="includeProjectIds")]
    
    pub include_project_ids: Option<Vec<String>>,
    /// Optional. This field is deprecated. The search mechanism for public and private tag templates is the same.
    #[serde(rename="includePublicTagTemplates")]
    
    pub include_public_tag_templates: Option<bool>,
    /// Optional. The list of locations to search within. If empty, all locations are searched. Returns an error if any location in the list isn't one of the [Supported regions](https://cloud.google.com/data-catalog/docs/concepts/regions#supported_regions). If a location is unreachable, its name is returned in the `SearchCatalogResponse.unreachable` field. To get additional information on the error, repeat the search request and set the location name as the value of this parameter.
    #[serde(rename="restrictedLocations")]
    
    pub restricted_locations: Option<Vec<String>>,
    /// Optional. If `true`, search only among starred entries. By default, all results are returned, starred or not.
    #[serde(rename="starredOnly")]
    
    pub starred_only: Option<bool>,
}

impl client::Part for GoogleCloudDatacatalogV1SearchCatalogRequestScope {}


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
pub struct GoogleCloudDatacatalogV1SearchCatalogResponse {
    /// Pagination token that can be used in subsequent calls to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Search results.
    
    pub results: Option<Vec<GoogleCloudDatacatalogV1SearchCatalogResult>>,
    /// Unreachable locations. Search results don't include data from those locations. To get additional information on an error, repeat the search request and restrict it to specific locations by setting the `SearchCatalogRequest.scope.restricted_locations` parameter.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleCloudDatacatalogV1SearchCatalogResponse {}


/// Result in the response to a search request. Each result captures details of one entry that matches the search.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1SearchCatalogResult {
    /// Entry description that can consist of several sentences or paragraphs that describe entry contents.
    
    pub description: Option<String>,
    /// The display name of the result.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Fully qualified name (FQN) of the resource. FQNs take two forms: * For non-regionalized resources: `{SYSTEM}:{PROJECT}.{PATH_TO_RESOURCE_SEPARATED_WITH_DOTS}` * For regionalized resources: `{SYSTEM}:{PROJECT}.{LOCATION_ID}.{PATH_TO_RESOURCE_SEPARATED_WITH_DOTS}` Example for a DPMS table: `dataproc_metastore:PROJECT_ID.LOCATION_ID.INSTANCE_ID.DATABASE_ID.TABLE_ID`
    #[serde(rename="fullyQualifiedName")]
    
    pub fully_qualified_name: Option<String>,
    /// Output only. The source system that Data Catalog automatically integrates with, such as BigQuery, Cloud Pub/Sub, or Dataproc Metastore.
    #[serde(rename="integratedSystem")]
    
    pub integrated_system: Option<GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum>,
    /// The full name of the Google Cloud resource the entry belongs to. For more information, see \[Full Resource Name\] (/apis/design/resource_names#full_resource_name). Example: `//bigquery.googleapis.com/projects/PROJECT_ID/datasets/DATASET_ID/tables/TABLE_ID`
    #[serde(rename="linkedResource")]
    
    pub linked_resource: Option<String>,
    /// The last modification timestamp of the entry in the source system.
    #[serde(rename="modifyTime")]
    
    pub modify_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The relative name of the resource in URL format. Examples: * `projects/{PROJECT_ID}/locations/{LOCATION_ID}/entryGroups/{ENTRY_GROUP_ID}/entries/{ENTRY_ID}` * `projects/{PROJECT_ID}/tagTemplates/{TAG_TEMPLATE_ID}`
    #[serde(rename="relativeResourceName")]
    
    pub relative_resource_name: Option<String>,
    /// Sub-type of the search result. A dot-delimited full type of the resource. The same type you specify in the `type` search predicate. Examples: `entry.table`, `entry.dataStream`, `tagTemplate`.
    #[serde(rename="searchResultSubtype")]
    
    pub search_result_subtype: Option<String>,
    /// Type of the search result. You can use this field to determine which get method to call to fetch the full resource.
    #[serde(rename="searchResultType")]
    
    pub search_result_type: Option<GoogleCloudDatacatalogV1SearchCatalogResultSearchResultTypeEnum>,
    /// Custom source system that you can manually integrate Data Catalog with.
    #[serde(rename="userSpecifiedSystem")]
    
    pub user_specified_system: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1SearchCatalogResult {}


/// A nested protocol buffer that represents a policy tag and all its descendants.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1SerializedPolicyTag {
    /// Children of the policy tag, if any.
    #[serde(rename="childPolicyTags")]
    
    pub child_policy_tags: Option<Vec<GoogleCloudDatacatalogV1SerializedPolicyTag>>,
    /// Description of the serialized policy tag. At most 2000 bytes when encoded in UTF-8. If not set, defaults to an empty description.
    
    pub description: Option<String>,
    /// Required. Display name of the policy tag. At most 200 bytes when encoded in UTF-8.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Resource name of the policy tag. This field is ignored when calling `ImportTaxonomies`.
    #[serde(rename="policyTag")]
    
    pub policy_tag: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1SerializedPolicyTag {}


/// A nested protocol buffer that represents a taxonomy and the hierarchy of its policy tags. Used for taxonomy replacement, import, and export.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1SerializedTaxonomy {
    /// A list of policy types that are activated per taxonomy.
    #[serde(rename="activatedPolicyTypes")]
    
    pub activated_policy_types: Option<Vec<GoogleCloudDatacatalogV1SerializedTaxonomyActivatedPolicyTypesEnum>>,
    /// Description of the serialized taxonomy. At most 2000 bytes when encoded in UTF-8. If not set, defaults to an empty description.
    
    pub description: Option<String>,
    /// Required. Display name of the taxonomy. At most 200 bytes when encoded in UTF-8.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Top level policy tags associated with the taxonomy, if any.
    #[serde(rename="policyTags")]
    
    pub policy_tags: Option<Vec<GoogleCloudDatacatalogV1SerializedPolicyTag>>,
}

impl client::Part for GoogleCloudDatacatalogV1SerializedTaxonomy {}


/// Specification that applies to entries that are part `SQL_DATABASE` system (user_specified_type)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1SqlDatabaseSystemSpec {
    /// Version of the database engine.
    #[serde(rename="databaseVersion")]
    
    pub database_version: Option<String>,
    /// Host of the SQL database enum InstanceHost { UNDEFINED = 0; SELF_HOSTED = 1; CLOUD_SQL = 2; AMAZON_RDS = 3; AZURE_SQL = 4; } Host of the enclousing database instance.
    #[serde(rename="instanceHost")]
    
    pub instance_host: Option<String>,
    /// SQL Database Engine. enum SqlEngine { UNDEFINED = 0; MY_SQL = 1; POSTGRE_SQL = 2; SQL_SERVER = 3; } Engine of the enclosing database instance.
    #[serde(rename="sqlEngine")]
    
    pub sql_engine: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1SqlDatabaseSystemSpec {}


/// Request message for StarEntry.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries star projects](ProjectLocationEntryGroupEntryStarCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1StarEntryRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDatacatalogV1StarEntryRequest {}


/// Response message for StarEntry. Empty for now
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries star projects](ProjectLocationEntryGroupEntryStarCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1StarEntryResponse { _never_set: Option<bool> }

impl client::ResponseResult for GoogleCloudDatacatalogV1StarEntryResponse {}


/// Details the properties of the underlying storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1StorageProperties {
    /// Patterns to identify a set of files for this fileset. Examples of a valid `file_pattern`: * `gs://bucket_name/dir/*`: matches all files in the `bucket_name/dir` directory * `gs://bucket_name/dir/**`: matches all files in the `bucket_name/dir` and all subdirectories recursively * `gs://bucket_name/file*`: matches files prefixed by `file` in `bucket_name` * `gs://bucket_name/??.txt`: matches files with two characters followed by `.txt` in `bucket_name` * `gs://bucket_name/[aeiou].txt`: matches files that contain a single vowel character followed by `.txt` in `bucket_name` * `gs://bucket_name/[a-m].txt`: matches files that contain `a`, `b`, ... or `m` followed by `.txt` in `bucket_name` * `gs://bucket_name/a/*/b`: matches all files in `bucket_name` that match the `a/*/b` pattern, such as `a/c/b`, `a/d/b` * `gs://another_bucket/a.txt`: matches `gs://another_bucket/a.txt`
    #[serde(rename="filePattern")]
    
    pub file_pattern: Option<Vec<String>>,
    /// File type in MIME format, for example, `text/plain`.
    #[serde(rename="fileType")]
    
    pub file_type: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1StorageProperties {}


/// Timestamps associated with this resource in a particular system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1SystemTimestamps {
    /// Creation timestamp of the resource within the given system.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Expiration timestamp of the resource within the given system. Currently only applicable to BigQuery resources.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Timestamp of the last modification of the resource or its metadata within a given system. Note: Depending on the source system, not every modification updates this timestamp. For example, BigQuery timestamps every metadata modification but not data or permission changes.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudDatacatalogV1SystemTimestamps {}


/// Normal BigQuery table specification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1TableSpec {
    /// Output only. If the table is date-sharded, that is, it matches the `[prefix]YYYYMMDD` name pattern, this field is the Data Catalog resource name of the date-sharded grouped entry. For example: `projects/{PROJECT_ID}/locations/{LOCATION}/entrygroups/{ENTRY_GROUP_ID}/entries/{ENTRY_ID}`. Otherwise, `grouped_entry` is empty.
    #[serde(rename="groupedEntry")]
    
    pub grouped_entry: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1TableSpec {}


/// Tags contain custom metadata and are attached to Data Catalog resources. Tags conform with the specification of their tag template. See [Data Catalog IAM](https://cloud.google.com/data-catalog/docs/concepts/iam) for information on the permissions needed to create or view tags.
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
pub struct GoogleCloudDatacatalogV1Tag {
    /// Resources like entry can have schemas associated with them. This scope allows you to attach tags to an individual column based on that schema. To attach a tag to a nested column, separate column names with a dot (`.`). Example: `column.nested_column`.
    
    pub column: Option<String>,
    /// Required. Maps the ID of a tag field to its value and additional information about that field. Tag template defines valid field IDs. A tag must have at least 1 field and at most 500 fields.
    
    pub fields: Option<HashMap<String, GoogleCloudDatacatalogV1TagField>>,
    /// The resource name of the tag in URL format where tag ID is a system-generated identifier. Note: The tag itself might not be stored in the location specified in its name.
    
    pub name: Option<String>,
    /// Required. The resource name of the tag template this tag uses. Example: `projects/{PROJECT_ID}/locations/{LOCATION}/tagTemplates/{TAG_TEMPLATE_ID}` This field cannot be modified after creation.
    
    pub template: Option<String>,
    /// Output only. The display name of the tag template.
    #[serde(rename="templateDisplayName")]
    
    pub template_display_name: Option<String>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1Tag {}
impl client::ResponseResult for GoogleCloudDatacatalogV1Tag {}


/// Contains the value and additional information on a field within a Tag.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1TagField {
    /// The value of a tag field with a boolean type.
    #[serde(rename="boolValue")]
    
    pub bool_value: Option<bool>,
    /// Output only. The display name of this field.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The value of a tag field with a double type.
    #[serde(rename="doubleValue")]
    
    pub double_value: Option<f64>,
    /// The value of a tag field with an enum type. This value must be one of the allowed values listed in this enum.
    #[serde(rename="enumValue")]
    
    pub enum_value: Option<GoogleCloudDatacatalogV1TagFieldEnumValue>,
    /// Output only. The order of this field with respect to other fields in this tag. Can be set by Tag. For example, a higher value can indicate a more important field. The value can be negative. Multiple fields can have the same order, and field orders within a tag don't have to be sequential.
    
    pub order: Option<i32>,
    /// The value of a tag field with a rich text type. The maximum length is 10 MiB as this value holds HTML descriptions including encoded images. The maximum length of the text without images is 100 KiB.
    #[serde(rename="richtextValue")]
    
    pub richtext_value: Option<String>,
    /// The value of a tag field with a string type. The maximum length is 2000 UTF-8 characters.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
    /// The value of a tag field with a timestamp type.
    #[serde(rename="timestampValue")]
    
    pub timestamp_value: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudDatacatalogV1TagField {}


/// An enum value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1TagFieldEnumValue {
    /// The display name of the enum value.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1TagFieldEnumValue {}


/// A tag template defines a tag that can have one or more typed fields. The template is used to create tags that are attached to GCP resources. \[Tag template roles\] (https://cloud.google.com/iam/docs/understanding-roles#data-catalog-roles) provide permissions to create, edit, and use the template. For example, see the \[TagTemplate User\] (https://cloud.google.com/data-catalog/docs/how-to/template-user) role that includes a permission to use the tag template to tag resources.
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
pub struct GoogleCloudDatacatalogV1TagTemplate {
    /// Display name for this template. Defaults to an empty string. The name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and can't start or end with spaces. The maximum length is 200 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. Map of tag template field IDs to the settings for the field. This map is an exhaustive list of the allowed fields. The map must contain at least one field and at most 500 fields. The keys to this map are tag template field IDs. The IDs have the following limitations: * Can contain uppercase and lowercase letters, numbers (0-9) and underscores (_). * Must be at least 1 character and at most 64 characters long. * Must start with a letter or underscore.
    
    pub fields: Option<HashMap<String, GoogleCloudDatacatalogV1TagTemplateField>>,
    /// Indicates whether tags created with this template are public. Public tags do not require tag template access to appear in ListTags API response. Additionally, you can search for a public tag by value with a simple search query in addition to using a ``tag:`` predicate.
    #[serde(rename="isPubliclyReadable")]
    
    pub is_publicly_readable: Option<bool>,
    /// The resource name of the tag template in URL format. Note: The tag template itself and its child resources might not be stored in the location specified in its name.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1TagTemplate {}
impl client::ResponseResult for GoogleCloudDatacatalogV1TagTemplate {}


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
pub struct GoogleCloudDatacatalogV1TagTemplateField {
    /// The description for this field. Defaults to an empty string.
    
    pub description: Option<String>,
    /// The display name for this field. Defaults to an empty string. The name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and can't start or end with spaces. The maximum length is 200 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// If true, this field is required. Defaults to false.
    #[serde(rename="isRequired")]
    
    pub is_required: Option<bool>,
    /// Output only. The resource name of the tag template field in URL format. Example: `projects/{PROJECT_ID}/locations/{LOCATION}/tagTemplates/{TAG_TEMPLATE}/fields/{FIELD}` Note: The tag template field itself might not be stored in the location specified in its name. The name must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_), and must start with a letter or underscore. The maximum length is 64 characters.
    
    pub name: Option<String>,
    /// The order of this field with respect to other fields in this tag template. For example, a higher value can indicate a more important field. The value can be negative. Multiple fields can have the same order and field orders within a tag don't have to be sequential.
    
    pub order: Option<i32>,
    /// Required. The type of value this tag field can contain.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudDatacatalogV1FieldType>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1TagTemplateField {}
impl client::ResponseResult for GoogleCloudDatacatalogV1TagTemplateField {}


/// A taxonomy is a collection of hierarchical policy tags that classify data along a common axis. For example, a “data sensitivity” taxonomy might contain the following policy tags: `+ PII + Account number + Age + SSN + Zipcode + Financials + Revenue` A “data origin” taxonomy might contain the following policy tags: `+ User data + Employee data + Partner data + Public data`
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations taxonomies create projects](ProjectLocationTaxonomyCreateCall) (request|response)
/// * [locations taxonomies get projects](ProjectLocationTaxonomyGetCall) (response)
/// * [locations taxonomies patch projects](ProjectLocationTaxonomyPatchCall) (request|response)
/// * [locations taxonomies replace projects](ProjectLocationTaxonomyReplaceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1Taxonomy {
    /// Optional. A list of policy types that are activated for this taxonomy. If not set, defaults to an empty list.
    #[serde(rename="activatedPolicyTypes")]
    
    pub activated_policy_types: Option<Vec<GoogleCloudDatacatalogV1TaxonomyActivatedPolicyTypesEnum>>,
    /// Optional. Description of this taxonomy. If not set, defaults to empty. The description must contain only Unicode characters, tabs, newlines, carriage returns, and page breaks, and be at most 2000 bytes long when encoded in UTF-8.
    
    pub description: Option<String>,
    /// Required. User-defined name of this taxonomy. The name can't start or end with spaces, must contain only Unicode letters, numbers, underscores, dashes, and spaces, and be at most 200 bytes long when encoded in UTF-8. The taxonomy display name must be unique within an organization.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Resource name of this taxonomy in URL format. Note: Policy tag manager generates unique taxonomy IDs.
    
    pub name: Option<String>,
    /// Output only. Number of policy tags in this taxonomy.
    #[serde(rename="policyTagCount")]
    
    pub policy_tag_count: Option<i32>,
    /// Output only. Identity of the service which owns the Taxonomy. This field is only populated when the taxonomy is created by a GCP service. Currently only 'DATAPLEX' is supported.
    
    pub service: Option<GoogleCloudDatacatalogV1TaxonomyService>,
    /// Output only. Creation and modification timestamps of this taxonomy.
    #[serde(rename="taxonomyTimestamps")]
    
    pub taxonomy_timestamps: Option<GoogleCloudDatacatalogV1SystemTimestamps>,
}

impl client::RequestValue for GoogleCloudDatacatalogV1Taxonomy {}
impl client::ResponseResult for GoogleCloudDatacatalogV1Taxonomy {}


/// The source system of the Taxonomy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1TaxonomyService {
    /// P4SA Identity of the service.
    
    pub identity: Option<String>,
    /// The GCP service name.
    
    pub name: Option<GoogleCloudDatacatalogV1TaxonomyServiceNameEnum>,
}

impl client::Part for GoogleCloudDatacatalogV1TaxonomyService {}


/// Request message for UnstarEntry.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries unstar projects](ProjectLocationEntryGroupEntryUnstarCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1UnstarEntryRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDatacatalogV1UnstarEntryRequest {}


/// Response message for UnstarEntry. Empty for now
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries unstar projects](ProjectLocationEntryGroupEntryUnstarCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1UnstarEntryResponse { _never_set: Option<bool> }

impl client::ResponseResult for GoogleCloudDatacatalogV1UnstarEntryResponse {}


/// The set of all usage signals that Data Catalog stores. Note: Usually, these signals are updated daily. In rare cases, an update may fail but will be performed again on the next day.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1UsageSignal {
    /// Common usage statistics over each of the predefined time ranges. Supported time ranges are `{"24H", "7D", "30D", "Lifetime"}`.
    #[serde(rename="commonUsageWithinTimeRange")]
    
    pub common_usage_within_time_range: Option<HashMap<String, GoogleCloudDatacatalogV1CommonUsageStats>>,
    /// Favorite count in the source system.
    #[serde(rename="favoriteCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub favorite_count: Option<i64>,
    /// The end timestamp of the duration of usage statistics.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. BigQuery usage statistics over each of the predefined time ranges. Supported time ranges are `{"24H", "7D", "30D"}`.
    #[serde(rename="usageWithinTimeRange")]
    
    pub usage_within_time_range: Option<HashMap<String, GoogleCloudDatacatalogV1UsageStats>>,
}

impl client::Part for GoogleCloudDatacatalogV1UsageSignal {}


/// Detailed statistics on the entry's usage. Usage statistics have the following limitations: - Only BigQuery tables have them. - They only include BigQuery query jobs. - They might be underestimated because wildcard table references are not yet counted. For more information, see [Querying multiple tables using a wildcard table] (https://cloud.google.com/bigquery/docs/querying-wildcard-tables)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1UsageStats {
    /// The number of cancelled attempts to use the underlying entry.
    #[serde(rename="totalCancellations")]
    
    pub total_cancellations: Option<f32>,
    /// The number of successful uses of the underlying entry.
    #[serde(rename="totalCompletions")]
    
    pub total_completions: Option<f32>,
    /// Total time spent only on successful uses, in milliseconds.
    #[serde(rename="totalExecutionTimeForCompletionsMillis")]
    
    pub total_execution_time_for_completions_millis: Option<f32>,
    /// The number of failed attempts to use the underlying entry.
    #[serde(rename="totalFailures")]
    
    pub total_failures: Option<f32>,
}

impl client::Part for GoogleCloudDatacatalogV1UsageStats {}


/// Table view specification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatacatalogV1ViewSpec {
    /// Output only. The query that defines the table view.
    #[serde(rename="viewQuery")]
    
    pub view_query: Option<String>,
}

impl client::Part for GoogleCloudDatacatalogV1ViewSpec {}


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


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations entry groups entries import projects](ProjectLocationEntryGroupEntryImportCall) (response)
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


