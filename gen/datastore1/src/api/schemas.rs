use super::*;
/// Defines a aggregation that produces a single result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Aggregation {
    /// Optional. Optional name of the property to store the result of the aggregation. If not provided, Datastore will pick a default name following the format `property_`. For example: ``` AGGREGATE COUNT_UP_TO(1) AS count_up_to_1, COUNT_UP_TO(2), COUNT_UP_TO(3) AS count_up_to_3, COUNT_UP_TO(4) OVER ( ... ); ``` becomes: ``` AGGREGATE COUNT_UP_TO(1) AS count_up_to_1, COUNT_UP_TO(2) AS property_1, COUNT_UP_TO(3) AS count_up_to_3, COUNT_UP_TO(4) AS property_2 OVER ( ... ); ``` Requires: * Must be unique across all aggregation aliases. * Conform to entity property name limitations.
    
    pub alias: Option<String>,
    /// Count aggregator.
    
    pub count: Option<Count>,
}

impl client::Part for Aggregation {}


/// Datastore query for running an aggregation over a Query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregationQuery {
    /// Optional. Series of aggregations to apply over the results of the `nested_query`. Requires: * A minimum of one and maximum of five aggregations per query.
    
    pub aggregations: Option<Vec<Aggregation>>,
    /// Nested query for aggregation
    #[serde(rename="nestedQuery")]
    
    pub nested_query: Option<Query>,
}

impl client::Part for AggregationQuery {}


/// The result of a single bucket from a Datastore aggregation query. The keys of `aggregate_properties` are the same for all results in an aggregation query, unlike entity queries which can have different fields present for each result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregationResult {
    /// The result of the aggregation functions, ex: `COUNT(*) AS total_entities`. The key is the alias assigned to the aggregation function on input and the size of this map equals the number of aggregation functions in the query.
    #[serde(rename="aggregateProperties")]
    
    pub aggregate_properties: Option<HashMap<String, Value>>,
}

impl client::Part for AggregationResult {}


/// A batch of aggregation results produced by an aggregation query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregationResultBatch {
    /// The aggregation results for this batch.
    #[serde(rename="aggregationResults")]
    
    pub aggregation_results: Option<Vec<AggregationResult>>,
    /// The state of the query after the current batch. Only COUNT(*) aggregations are supported in the initial launch. Therefore, expected result type is limited to `NO_MORE_RESULTS`.
    #[serde(rename="moreResults")]
    
    pub more_results: Option<String>,
    /// Read timestamp this batch was returned from. In a single transaction, subsequent query result batches for the same query can have a greater timestamp. Each batch's read timestamp is valid for all preceding batches.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for AggregationResultBatch {}


/// The request for Datastore.AllocateIds.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [allocate ids projects](ProjectAllocateIdCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AllocateIdsRequest {
    /// The ID of the database against which to make the request. '(default)' is not allowed; please use empty string '' to refer the default database.
    #[serde(rename="databaseId")]
    
    pub database_id: Option<String>,
    /// Required. A list of keys with incomplete key paths for which to allocate IDs. No key may be reserved/read-only.
    
    pub keys: Option<Vec<Key>>,
}

impl client::RequestValue for AllocateIdsRequest {}


/// The response for Datastore.AllocateIds.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [allocate ids projects](ProjectAllocateIdCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AllocateIdsResponse {
    /// The keys specified in the request (in the same order), each with its key path completed with a newly allocated ID.
    
    pub keys: Option<Vec<Key>>,
}

impl client::ResponseResult for AllocateIdsResponse {}


/// An array value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArrayValue {
    /// Values in the array. The order of values in an array is preserved as long as all values have identical settings for 'exclude_from_indexes'.
    
    pub values: Option<Vec<Value>>,
}

impl client::Part for ArrayValue {}


/// The request for Datastore.BeginTransaction.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [begin transaction projects](ProjectBeginTransactionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BeginTransactionRequest {
    /// The ID of the database against which to make the request. '(default)' is not allowed; please use empty string '' to refer the default database.
    #[serde(rename="databaseId")]
    
    pub database_id: Option<String>,
    /// Options for a new transaction.
    #[serde(rename="transactionOptions")]
    
    pub transaction_options: Option<TransactionOptions>,
}

impl client::RequestValue for BeginTransactionRequest {}


/// The response for Datastore.BeginTransaction.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [begin transaction projects](ProjectBeginTransactionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BeginTransactionResponse {
    /// The transaction identifier (always present).
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub transaction: Option<Vec<u8>>,
}

impl client::ResponseResult for BeginTransactionResponse {}


/// The request for Datastore.Commit.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [commit projects](ProjectCommitCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommitRequest {
    /// The ID of the database against which to make the request. '(default)' is not allowed; please use empty string '' to refer the default database.
    #[serde(rename="databaseId")]
    
    pub database_id: Option<String>,
    /// The type of commit to perform. Defaults to `TRANSACTIONAL`.
    
    pub mode: Option<String>,
    /// The mutations to perform. When mode is `TRANSACTIONAL`, mutations affecting a single entity are applied in order. The following sequences of mutations affecting a single entity are not permitted in a single `Commit` request: - `insert` followed by `insert` - `update` followed by `insert` - `upsert` followed by `insert` - `delete` followed by `update` When mode is `NON_TRANSACTIONAL`, no two mutations may affect a single entity.
    
    pub mutations: Option<Vec<Mutation>>,
    /// Options for beginning a new transaction for this request. The transaction is committed when the request completes. If specified, TransactionOptions.mode must be TransactionOptions.ReadWrite.
    #[serde(rename="singleUseTransaction")]
    
    pub single_use_transaction: Option<TransactionOptions>,
    /// The identifier of the transaction associated with the commit. A transaction identifier is returned by a call to Datastore.BeginTransaction.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub transaction: Option<Vec<u8>>,
}

impl client::RequestValue for CommitRequest {}


/// The response for Datastore.Commit.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [commit projects](ProjectCommitCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommitResponse {
    /// The transaction commit timestamp. Not set for non-transactional commits.
    #[serde(rename="commitTime")]
    
    pub commit_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The number of index entries updated during the commit, or zero if none were updated.
    #[serde(rename="indexUpdates")]
    
    pub index_updates: Option<i32>,
    /// The result of performing the mutations. The i-th mutation result corresponds to the i-th mutation in the request.
    #[serde(rename="mutationResults")]
    
    pub mutation_results: Option<Vec<MutationResult>>,
}

impl client::ResponseResult for CommitResponse {}


/// A filter that merges multiple other filters using the given operator.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompositeFilter {
    /// The list of filters to combine. Requires: * At least one filter is present.
    
    pub filters: Option<Vec<Filter>>,
    /// The operator for combining multiple filters.
    
    pub op: Option<String>,
}

impl client::Part for CompositeFilter {}


/// Count of entities that match the query. The `COUNT(*)` aggregation function operates on the entire entity so it does not require a field reference.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Count {
    /// Optional. Optional constraint on the maximum number of entities to count. This provides a way to set an upper bound on the number of entities to scan, limiting latency and cost. Unspecified is interpreted as no bound. If a zero value is provided, a count result of zero should always be expected. High-Level Example: ``` AGGREGATE COUNT_UP_TO(1000) OVER ( SELECT * FROM k ); ``` Requires: * Must be non-negative when present.
    #[serde(rename="upTo")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub up_to: Option<i64>,
}

impl client::Part for Count {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations cancel projects](ProjectOperationCancelCall) (response)
/// * [operations delete projects](ProjectOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// A Datastore data object. An entity is limited to 1 megabyte when stored. That _roughly_ corresponds to a limit of 1 megabyte for the serialized form of this message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    /// The entity's key. An entity must have a key, unless otherwise documented (for example, an entity in `Value.entity_value` may have no key). An entity's kind is its key path's last element's kind, or null if it has no key.
    
    pub key: Option<Key>,
    /// The entity's properties. The map's keys are property names. A property name matching regex `__.*__` is reserved. A reserved property name is forbidden in certain documented contexts. The name must not contain more than 500 characters. The name cannot be `""`.
    
    pub properties: Option<HashMap<String, Value>>,
}

impl client::Part for Entity {}


/// The result of fetching an entity from Datastore.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityResult {
    /// The time at which the entity was created. This field is set for `FULL` entity results. If this entity is missing, this field will not be set.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A cursor that points to the position after the result entity. Set only when the `EntityResult` is part of a `QueryResultBatch` message.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub cursor: Option<Vec<u8>>,
    /// The resulting entity.
    
    pub entity: Option<Entity>,
    /// The time at which the entity was last changed. This field is set for `FULL` entity results. If this entity is missing, this field will not be set.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The version of the entity, a strictly positive number that monotonically increases with changes to the entity. This field is set for `FULL` entity results. For missing entities in `LookupResponse`, this is the version of the snapshot that was used to look up the entity, and it is always set except for eventually consistent reads.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
}

impl client::Part for EntityResult {}


/// A holder for any type of filter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Filter {
    /// A composite filter.
    #[serde(rename="compositeFilter")]
    
    pub composite_filter: Option<CompositeFilter>,
    /// A filter on a property.
    #[serde(rename="propertyFilter")]
    
    pub property_filter: Option<PropertyFilter>,
}

impl client::Part for Filter {}


/// Identifies a subset of entities in a project. This is specified as combinations of kinds and namespaces (either or both of which may be all, as described in the following examples). Example usage: Entire project: kinds=[], namespace_ids=[] Kinds Foo and Bar in all namespaces: kinds=['Foo', 'Bar'], namespace_ids=[] Kinds Foo and Bar only in the default namespace: kinds=['Foo', 'Bar'], namespace_ids=[''] Kinds Foo and Bar in both the default and Baz namespaces: kinds=['Foo', 'Bar'], namespace_ids=['', 'Baz'] The entire Baz namespace: kinds=[], namespace_ids=['Baz']
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleDatastoreAdminV1EntityFilter {
    /// If empty, then this represents all kinds.
    
    pub kinds: Option<Vec<String>>,
    /// An empty list represents all namespaces. This is the preferred usage for projects that don't use namespaces. An empty string element represents the default namespace. This should be used if the project has data in non-default namespaces, but doesn't want to include them. Each namespace in this list must be unique.
    #[serde(rename="namespaceIds")]
    
    pub namespace_ids: Option<Vec<String>>,
}

impl client::Part for GoogleDatastoreAdminV1EntityFilter {}


/// The request for google.datastore.admin.v1.DatastoreAdmin.ExportEntities.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [export projects](ProjectExportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleDatastoreAdminV1ExportEntitiesRequest {
    /// Description of what data from the project is included in the export.
    #[serde(rename="entityFilter")]
    
    pub entity_filter: Option<GoogleDatastoreAdminV1EntityFilter>,
    /// Client-assigned labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. Location for the export metadata and data files. The full resource URL of the external storage location. Currently, only Google Cloud Storage is supported. So output_url_prefix should be of the form: `gs://BUCKET_NAME[/NAMESPACE_PATH]`, where `BUCKET_NAME` is the name of the Cloud Storage bucket and `NAMESPACE_PATH` is an optional Cloud Storage namespace path (this is not a Cloud Datastore namespace). For more information about Cloud Storage namespace paths, see [Object name considerations](https://cloud.google.com/storage/docs/naming#object-considerations). The resulting files will be nested deeper than the specified URL prefix. The final output URL will be provided in the google.datastore.admin.v1.ExportEntitiesResponse.output_url field. That value should be used for subsequent ImportEntities operations. By nesting the data files deeper, the same Cloud Storage bucket can be used in multiple ExportEntities operations without conflict.
    #[serde(rename="outputUrlPrefix")]
    
    pub output_url_prefix: Option<String>,
}

impl client::RequestValue for GoogleDatastoreAdminV1ExportEntitiesRequest {}


/// The request for google.datastore.admin.v1.DatastoreAdmin.ImportEntities.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [import projects](ProjectImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleDatastoreAdminV1ImportEntitiesRequest {
    /// Optionally specify which kinds/namespaces are to be imported. If provided, the list must be a subset of the EntityFilter used in creating the export, otherwise a FAILED_PRECONDITION error will be returned. If no filter is specified then all entities from the export are imported.
    #[serde(rename="entityFilter")]
    
    pub entity_filter: Option<GoogleDatastoreAdminV1EntityFilter>,
    /// Required. The full resource URL of the external storage location. Currently, only Google Cloud Storage is supported. So input_url should be of the form: `gs://BUCKET_NAME[/NAMESPACE_PATH]/OVERALL_EXPORT_METADATA_FILE`, where `BUCKET_NAME` is the name of the Cloud Storage bucket, `NAMESPACE_PATH` is an optional Cloud Storage namespace path (this is not a Cloud Datastore namespace), and `OVERALL_EXPORT_METADATA_FILE` is the metadata file written by the ExportEntities operation. For more information about Cloud Storage namespace paths, see [Object name considerations](https://cloud.google.com/storage/docs/naming#object-considerations). For more information, see google.datastore.admin.v1.ExportEntitiesResponse.output_url.
    #[serde(rename="inputUrl")]
    
    pub input_url: Option<String>,
    /// Client-assigned labels.
    
    pub labels: Option<HashMap<String, String>>,
}

impl client::RequestValue for GoogleDatastoreAdminV1ImportEntitiesRequest {}


/// Datastore composite index definition.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [indexes create projects](ProjectIndexCreateCall) (request)
/// * [indexes get projects](ProjectIndexGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleDatastoreAdminV1Index {
    /// Required. The index's ancestor mode. Must not be ANCESTOR_MODE_UNSPECIFIED.
    
    pub ancestor: Option<String>,
    /// Output only. The resource ID of the index.
    #[serde(rename="indexId")]
    
    pub index_id: Option<String>,
    /// Required. The entity kind to which this index applies.
    
    pub kind: Option<String>,
    /// Output only. Project ID.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Required. An ordered sequence of property names and their index attributes. Requires: * A maximum of 100 properties.
    
    pub properties: Option<Vec<GoogleDatastoreAdminV1IndexedProperty>>,
    /// Output only. The state of the index.
    
    pub state: Option<String>,
}

impl client::RequestValue for GoogleDatastoreAdminV1Index {}
impl client::ResponseResult for GoogleDatastoreAdminV1Index {}


/// A property of an index.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleDatastoreAdminV1IndexedProperty {
    /// Required. The indexed property's direction. Must not be DIRECTION_UNSPECIFIED.
    
    pub direction: Option<String>,
    /// Required. The property name to index.
    
    pub name: Option<String>,
}

impl client::Part for GoogleDatastoreAdminV1IndexedProperty {}


/// The response for google.datastore.admin.v1.DatastoreAdmin.ListIndexes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [indexes list projects](ProjectIndexListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleDatastoreAdminV1ListIndexesResponse {
    /// The indexes.
    
    pub indexes: Option<Vec<GoogleDatastoreAdminV1Index>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleDatastoreAdminV1ListIndexesResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations list projects](ProjectOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<GoogleLongrunningOperation>>,
}

impl client::ResponseResult for GoogleLongrunningListOperationsResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [indexes create projects](ProjectIndexCreateCall) (response)
/// * [indexes delete projects](ProjectIndexDeleteCall) (response)
/// * [operations get projects](ProjectOperationGetCall) (response)
/// * [export projects](ProjectExportCall) (response)
/// * [import projects](ProjectImportCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
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

impl client::ResponseResult for GoogleLongrunningOperation {}


/// A [GQL query](https://cloud.google.com/datastore/docs/apis/gql/gql_reference).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GqlQuery {
    /// When false, the query string must not contain any literals and instead must bind all values. For example, `SELECT * FROM Kind WHERE a = 'string literal'` is not allowed, while `SELECT * FROM Kind WHERE a = @value` is.
    #[serde(rename="allowLiterals")]
    
    pub allow_literals: Option<bool>,
    /// For each non-reserved named binding site in the query string, there must be a named parameter with that name, but not necessarily the inverse. Key must match regex `A-Za-z_$*`, must not match regex `__.*__`, and must not be `""`.
    #[serde(rename="namedBindings")]
    
    pub named_bindings: Option<HashMap<String, GqlQueryParameter>>,
    /// Numbered binding site @1 references the first numbered parameter, effectively using 1-based indexing, rather than the usual 0. For each binding site numbered i in `query_string`, there must be an i-th numbered parameter. The inverse must also be true.
    #[serde(rename="positionalBindings")]
    
    pub positional_bindings: Option<Vec<GqlQueryParameter>>,
    /// A string of the format described [here](https://cloud.google.com/datastore/docs/apis/gql/gql_reference).
    #[serde(rename="queryString")]
    
    pub query_string: Option<String>,
}

impl client::Part for GqlQuery {}


/// A binding parameter for a GQL query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GqlQueryParameter {
    /// A query cursor. Query cursors are returned in query result batches.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub cursor: Option<Vec<u8>>,
    /// A value parameter.
    
    pub value: Option<Value>,
}

impl client::Part for GqlQueryParameter {}


/// A unique identifier for an entity. If a key's partition ID or any of its path kinds or names are reserved/read-only, the key is reserved/read-only. A reserved/read-only key is forbidden in certain documented contexts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Key {
    /// Entities are partitioned into subsets, currently identified by a project ID and namespace ID. Queries are scoped to a single partition.
    #[serde(rename="partitionId")]
    
    pub partition_id: Option<PartitionId>,
    /// The entity path. An entity path consists of one or more elements composed of a kind and a string or numerical identifier, which identify entities. The first element identifies a _root entity_, the second element identifies a _child_ of the root entity, the third element identifies a child of the second entity, and so forth. The entities identified by all prefixes of the path are called the element's _ancestors_. An entity path is always fully complete: *all* of the entity's ancestors are required to be in the path along with the entity identifier itself. The only exception is that in some documented cases, the identifier in the last path element (for the entity) itself may be omitted. For example, the last path element of the key of `Mutation.insert` may have no identifier. A path can never be empty, and a path can have at most 100 elements.
    
    pub path: Option<Vec<PathElement>>,
}

impl client::Part for Key {}


/// A representation of a kind.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KindExpression {
    /// The name of the kind.
    
    pub name: Option<String>,
}

impl client::Part for KindExpression {}


/// An object that represents a latitude/longitude pair. This is expressed as a pair of doubles to represent degrees latitude and degrees longitude. Unless specified otherwise, this object must conform to the WGS84 standard. Values must be within normalized ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    
    pub latitude: Option<f64>,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    
    pub longitude: Option<f64>,
}

impl client::Part for LatLng {}


/// The request for Datastore.Lookup.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [lookup projects](ProjectLookupCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LookupRequest {
    /// The ID of the database against which to make the request. '(default)' is not allowed; please use empty string '' to refer the default database.
    #[serde(rename="databaseId")]
    
    pub database_id: Option<String>,
    /// Required. Keys of entities to look up.
    
    pub keys: Option<Vec<Key>>,
    /// The options for this lookup request.
    #[serde(rename="readOptions")]
    
    pub read_options: Option<ReadOptions>,
}

impl client::RequestValue for LookupRequest {}


/// The response for Datastore.Lookup.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [lookup projects](ProjectLookupCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LookupResponse {
    /// A list of keys that were not looked up due to resource constraints. The order of results in this field is undefined and has no relation to the order of the keys in the input.
    
    pub deferred: Option<Vec<Key>>,
    /// Entities found as `ResultType.FULL` entities. The order of results in this field is undefined and has no relation to the order of the keys in the input.
    
    pub found: Option<Vec<EntityResult>>,
    /// Entities not found as `ResultType.KEY_ONLY` entities. The order of results in this field is undefined and has no relation to the order of the keys in the input.
    
    pub missing: Option<Vec<EntityResult>>,
    /// The time at which these entities were read or found missing.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The identifier of the transaction that was started as part of this Lookup request. Set only when ReadOptions.begin_transaction was set in LookupRequest.read_options.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub transaction: Option<Vec<u8>>,
}

impl client::ResponseResult for LookupResponse {}


/// A mutation to apply to an entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Mutation {
    /// The version of the entity that this mutation is being applied to. If this does not match the current version on the server, the mutation conflicts.
    #[serde(rename="baseVersion")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub base_version: Option<i64>,
    /// The key of the entity to delete. The entity may or may not already exist. Must have a complete key path and must not be reserved/read-only.
    
    pub delete: Option<Key>,
    /// The entity to insert. The entity must not already exist. The entity key's final path element may be incomplete.
    
    pub insert: Option<Entity>,
    /// The entity to update. The entity must already exist. Must have a complete key path.
    
    pub update: Option<Entity>,
    /// The update time of the entity that this mutation is being applied to. If this does not match the current update time on the server, the mutation conflicts.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The entity to upsert. The entity may or may not already exist. The entity key's final path element may be incomplete.
    
    pub upsert: Option<Entity>,
}

impl client::Part for Mutation {}


/// The result of applying a mutation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MutationResult {
    /// Whether a conflict was detected for this mutation. Always false when a conflict detection strategy field is not set in the mutation.
    #[serde(rename="conflictDetected")]
    
    pub conflict_detected: Option<bool>,
    /// The create time of the entity. This field will not be set after a 'delete'.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The automatically allocated key. Set only when the mutation allocated a key.
    
    pub key: Option<Key>,
    /// The update time of the entity on the server after processing the mutation. If the mutation doesn't change anything on the server, then the timestamp will be the update timestamp of the current entity. This field will not be set after a 'delete'.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The version of the entity on the server after processing the mutation. If the mutation doesn't change anything on the server, then the version will be the version of the current entity or, if no entity is present, a version that is strictly greater than the version of any previous entity and less than the version of any possible future entity.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
}

impl client::Part for MutationResult {}


/// A partition ID identifies a grouping of entities. The grouping is always by project and namespace, however the namespace ID may be empty. A partition ID contains several dimensions: project ID and namespace ID. Partition dimensions: - May be `""`. - Must be valid UTF-8 bytes. - Must have values that match regex `[A-Za-z\d\.\-_]{1,100}` If the value of any dimension matches regex `__.*__`, the partition is reserved/read-only. A reserved/read-only partition ID is forbidden in certain documented contexts. Foreign partition IDs (in which the project ID does not match the context project ID ) are discouraged. Reads and writes of foreign partition IDs may fail if the project is not in an active state.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartitionId {
    /// If not empty, the ID of the database to which the entities belong.
    #[serde(rename="databaseId")]
    
    pub database_id: Option<String>,
    /// If not empty, the ID of the namespace to which the entities belong.
    #[serde(rename="namespaceId")]
    
    pub namespace_id: Option<String>,
    /// The ID of the project to which the entities belong.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::Part for PartitionId {}


/// A (kind, ID/name) pair used to construct a key path. If either name or ID is set, the element is complete. If neither is set, the element is incomplete.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PathElement {
    /// The auto-allocated ID of the entity. Never equal to zero. Values less than zero are discouraged and may not be supported in the future.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// The kind of the entity. A kind matching regex `__.*__` is reserved/read-only. A kind must not contain more than 1500 bytes when UTF-8 encoded. Cannot be `""`. Must be valid UTF-8 bytes. Legacy values that are not valid UTF-8 are encoded as `__bytes__` where `` is the base-64 encoding of the bytes.
    
    pub kind: Option<String>,
    /// The name of the entity. A name matching regex `__.*__` is reserved/read-only. A name must not be more than 1500 bytes when UTF-8 encoded. Cannot be `""`. Must be valid UTF-8 bytes. Legacy values that are not valid UTF-8 are encoded as `__bytes__` where `` is the base-64 encoding of the bytes.
    
    pub name: Option<String>,
}

impl client::Part for PathElement {}


/// A representation of a property in a projection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Projection {
    /// The property to project.
    
    pub property: Option<PropertyReference>,
}

impl client::Part for Projection {}


/// A filter on a specific property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PropertyFilter {
    /// The operator to filter by.
    
    pub op: Option<String>,
    /// The property to filter by.
    
    pub property: Option<PropertyReference>,
    /// The value to compare the property to.
    
    pub value: Option<Value>,
}

impl client::Part for PropertyFilter {}


/// The desired order for a specific property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PropertyOrder {
    /// The direction to order by. Defaults to `ASCENDING`.
    
    pub direction: Option<String>,
    /// The property to order by.
    
    pub property: Option<PropertyReference>,
}

impl client::Part for PropertyOrder {}


/// A reference to a property relative to the kind expressions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PropertyReference {
    /// The name of the property. If name includes "."s, it may be interpreted as a property name path.
    
    pub name: Option<String>,
}

impl client::Part for PropertyReference {}


/// A query for entities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Query {
    /// The properties to make distinct. The query results will contain the first result for each distinct combination of values for the given properties (if empty, all results are returned).
    #[serde(rename="distinctOn")]
    
    pub distinct_on: Option<Vec<PropertyReference>>,
    /// An ending point for the query results. Query cursors are returned in query result batches and [can only be used to limit the same query](https://cloud.google.com/datastore/docs/concepts/queries#cursors_limits_and_offsets).
    #[serde(rename="endCursor")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub end_cursor: Option<Vec<u8>>,
    /// The filter to apply.
    
    pub filter: Option<Filter>,
    /// The kinds to query (if empty, returns entities of all kinds). Currently at most 1 kind may be specified.
    
    pub kind: Option<Vec<KindExpression>>,
    /// The maximum number of results to return. Applies after all other constraints. Optional. Unspecified is interpreted as no limit. Must be >= 0 if specified.
    
    pub limit: Option<i32>,
    /// The number of results to skip. Applies before limit, but after all other constraints. Optional. Must be >= 0 if specified.
    
    pub offset: Option<i32>,
    /// The order to apply to the query results (if empty, order is unspecified).
    
    pub order: Option<Vec<PropertyOrder>>,
    /// The projection to return. Defaults to returning all properties.
    
    pub projection: Option<Vec<Projection>>,
    /// A starting point for the query results. Query cursors are returned in query result batches and [can only be used to continue the same query](https://cloud.google.com/datastore/docs/concepts/queries#cursors_limits_and_offsets).
    #[serde(rename="startCursor")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub start_cursor: Option<Vec<u8>>,
}

impl client::Part for Query {}


/// A batch of results produced by a query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryResultBatch {
    /// A cursor that points to the position after the last result in the batch.
    #[serde(rename="endCursor")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub end_cursor: Option<Vec<u8>>,
    /// The result type for every entity in `entity_results`.
    #[serde(rename="entityResultType")]
    
    pub entity_result_type: Option<String>,
    /// The results for this batch.
    #[serde(rename="entityResults")]
    
    pub entity_results: Option<Vec<EntityResult>>,
    /// The state of the query after the current batch.
    #[serde(rename="moreResults")]
    
    pub more_results: Option<String>,
    /// Read timestamp this batch was returned from. This applies to the range of results from the query's `start_cursor` (or the beginning of the query if no cursor was given) to this batch's `end_cursor` (not the query's `end_cursor`). In a single transaction, subsequent query result batches for the same query can have a greater timestamp. Each batch's read timestamp is valid for all preceding batches. This value will not be set for eventually consistent queries in Cloud Datastore.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A cursor that points to the position after the last skipped result. Will be set when `skipped_results` != 0.
    #[serde(rename="skippedCursor")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub skipped_cursor: Option<Vec<u8>>,
    /// The number of results skipped, typically because of an offset.
    #[serde(rename="skippedResults")]
    
    pub skipped_results: Option<i32>,
    /// The version number of the snapshot this batch was returned from. This applies to the range of results from the query's `start_cursor` (or the beginning of the query if no cursor was given) to this batch's `end_cursor` (not the query's `end_cursor`). In a single transaction, subsequent query result batches for the same query can have a greater snapshot version number. Each batch's snapshot version is valid for all preceding batches. The value will be zero for eventually consistent queries.
    #[serde(rename="snapshotVersion")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub snapshot_version: Option<i64>,
}

impl client::Part for QueryResultBatch {}


/// Options specific to read-only transactions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReadOnly {
    /// Reads entities at the given time. This may not be older than 60 seconds.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for ReadOnly {}


/// The options shared by read requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReadOptions {
    /// Options for beginning a new transaction for this request. The new transaction identifier will be returned in the corresponding response as either LookupResponse.transaction or RunQueryResponse.transaction.
    #[serde(rename="newTransaction")]
    
    pub new_transaction: Option<TransactionOptions>,
    /// The non-transactional read consistency to use.
    #[serde(rename="readConsistency")]
    
    pub read_consistency: Option<String>,
    /// Reads entities as they were at the given time. This may not be older than 270 seconds. This value is only supported for Cloud Firestore in Datastore mode.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The identifier of the transaction in which to read. A transaction identifier is returned by a call to Datastore.BeginTransaction.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub transaction: Option<Vec<u8>>,
}

impl client::Part for ReadOptions {}


/// Options specific to read / write transactions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReadWrite {
    /// The transaction identifier of the transaction being retried.
    #[serde(rename="previousTransaction")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub previous_transaction: Option<Vec<u8>>,
}

impl client::Part for ReadWrite {}


/// The request for Datastore.ReserveIds.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reserve ids projects](ProjectReserveIdCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReserveIdsRequest {
    /// The ID of the database against which to make the request. '(default)' is not allowed; please use empty string '' to refer the default database.
    #[serde(rename="databaseId")]
    
    pub database_id: Option<String>,
    /// Required. A list of keys with complete key paths whose numeric IDs should not be auto-allocated.
    
    pub keys: Option<Vec<Key>>,
}

impl client::RequestValue for ReserveIdsRequest {}


/// The response for Datastore.ReserveIds.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reserve ids projects](ProjectReserveIdCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReserveIdsResponse { _never_set: Option<bool> }

impl client::ResponseResult for ReserveIdsResponse {}


/// The request for Datastore.Rollback.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rollback projects](ProjectRollbackCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollbackRequest {
    /// The ID of the database against which to make the request. '(default)' is not allowed; please use empty string '' to refer the default database.
    #[serde(rename="databaseId")]
    
    pub database_id: Option<String>,
    /// Required. The transaction identifier, returned by a call to Datastore.BeginTransaction.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub transaction: Option<Vec<u8>>,
}

impl client::RequestValue for RollbackRequest {}


/// The response for Datastore.Rollback. (an empty message).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rollback projects](ProjectRollbackCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollbackResponse { _never_set: Option<bool> }

impl client::ResponseResult for RollbackResponse {}


/// The request for Datastore.RunAggregationQuery.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run aggregation query projects](ProjectRunAggregationQueryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunAggregationQueryRequest {
    /// The query to run.
    #[serde(rename="aggregationQuery")]
    
    pub aggregation_query: Option<AggregationQuery>,
    /// The ID of the database against which to make the request. '(default)' is not allowed; please use empty string '' to refer the default database.
    #[serde(rename="databaseId")]
    
    pub database_id: Option<String>,
    /// The GQL query to run. This query must be an aggregation query.
    #[serde(rename="gqlQuery")]
    
    pub gql_query: Option<GqlQuery>,
    /// Entities are partitioned into subsets, identified by a partition ID. Queries are scoped to a single partition. This partition ID is normalized with the standard default context partition ID.
    #[serde(rename="partitionId")]
    
    pub partition_id: Option<PartitionId>,
    /// The options for this query.
    #[serde(rename="readOptions")]
    
    pub read_options: Option<ReadOptions>,
}

impl client::RequestValue for RunAggregationQueryRequest {}


/// The response for Datastore.RunAggregationQuery.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run aggregation query projects](ProjectRunAggregationQueryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunAggregationQueryResponse {
    /// A batch of aggregation results. Always present.
    
    pub batch: Option<AggregationResultBatch>,
    /// The parsed form of the `GqlQuery` from the request, if it was set.
    
    pub query: Option<AggregationQuery>,
    /// The identifier of the transaction that was started as part of this RunAggregationQuery request. Set only when ReadOptions.begin_transaction was set in RunAggregationQueryRequest.read_options.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub transaction: Option<Vec<u8>>,
}

impl client::ResponseResult for RunAggregationQueryResponse {}


/// The request for Datastore.RunQuery.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run query projects](ProjectRunQueryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunQueryRequest {
    /// The ID of the database against which to make the request. '(default)' is not allowed; please use empty string '' to refer the default database.
    #[serde(rename="databaseId")]
    
    pub database_id: Option<String>,
    /// The GQL query to run. This query must be a non-aggregation query.
    #[serde(rename="gqlQuery")]
    
    pub gql_query: Option<GqlQuery>,
    /// Entities are partitioned into subsets, identified by a partition ID. Queries are scoped to a single partition. This partition ID is normalized with the standard default context partition ID.
    #[serde(rename="partitionId")]
    
    pub partition_id: Option<PartitionId>,
    /// The query to run.
    
    pub query: Option<Query>,
    /// The options for this query.
    #[serde(rename="readOptions")]
    
    pub read_options: Option<ReadOptions>,
}

impl client::RequestValue for RunQueryRequest {}


/// The response for Datastore.RunQuery.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run query projects](ProjectRunQueryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunQueryResponse {
    /// A batch of query results (always present).
    
    pub batch: Option<QueryResultBatch>,
    /// The parsed form of the `GqlQuery` from the request, if it was set.
    
    pub query: Option<Query>,
    /// The identifier of the transaction that was started as part of this RunQuery request. Set only when ReadOptions.begin_transaction was set in RunQueryRequest.read_options.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub transaction: Option<Vec<u8>>,
}

impl client::ResponseResult for RunQueryResponse {}


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


/// Options for beginning a new transaction. Transactions can be created explicitly with calls to Datastore.BeginTransaction or implicitly by setting ReadOptions.new_transaction in read requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransactionOptions {
    /// The transaction should only allow reads.
    #[serde(rename="readOnly")]
    
    pub read_only: Option<ReadOnly>,
    /// The transaction should allow both reads and writes.
    #[serde(rename="readWrite")]
    
    pub read_write: Option<ReadWrite>,
}

impl client::Part for TransactionOptions {}


/// A message that can hold any of the supported value types and associated metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Value {
    /// An array value. Cannot contain another array value. A `Value` instance that sets field `array_value` must not set fields `meaning` or `exclude_from_indexes`.
    #[serde(rename="arrayValue")]
    
    pub array_value: Option<ArrayValue>,
    /// A blob value. May have at most 1,000,000 bytes. When `exclude_from_indexes` is false, may have at most 1500 bytes. In JSON requests, must be base64-encoded.
    #[serde(rename="blobValue")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub blob_value: Option<Vec<u8>>,
    /// A boolean value.
    #[serde(rename="booleanValue")]
    
    pub boolean_value: Option<bool>,
    /// A double value.
    #[serde(rename="doubleValue")]
    
    pub double_value: Option<f64>,
    /// An entity value. - May have no key. - May have a key with an incomplete key path. - May have a reserved/read-only key.
    #[serde(rename="entityValue")]
    
    pub entity_value: Option<Entity>,
    /// If the value should be excluded from all indexes including those defined explicitly.
    #[serde(rename="excludeFromIndexes")]
    
    pub exclude_from_indexes: Option<bool>,
    /// A geo point value representing a point on the surface of Earth.
    #[serde(rename="geoPointValue")]
    
    pub geo_point_value: Option<LatLng>,
    /// An integer value.
    #[serde(rename="integerValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub integer_value: Option<i64>,
    /// A key value.
    #[serde(rename="keyValue")]
    
    pub key_value: Option<Key>,
    /// The `meaning` field should only be populated for backwards compatibility.
    
    pub meaning: Option<i32>,
    /// A null value.
    #[serde(rename="nullValue")]
    
    pub null_value: Option<String>,
    /// A UTF-8 encoded string value. When `exclude_from_indexes` is false (it is indexed) , may have at most 1500 bytes. Otherwise, may be set to at most 1,000,000 bytes.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
    /// A timestamp value. When stored in the Datastore, precise only to microseconds; any additional precision is rounded down.
    #[serde(rename="timestampValue")]
    
    pub timestamp_value: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Value {}


