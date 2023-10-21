use super::*;
/// Defines a aggregation that produces a single result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Aggregation {
    /// Optional. Optional name of the field to store the result of the aggregation into. If not provided, Firestore will pick a default name following the format `field_`. For example: ``` AGGREGATE COUNT_UP_TO(1) AS count_up_to_1, COUNT_UP_TO(2), COUNT_UP_TO(3) AS count_up_to_3, COUNT_UP_TO(4) OVER ( ... ); ``` becomes: ``` AGGREGATE COUNT_UP_TO(1) AS count_up_to_1, COUNT_UP_TO(2) AS field_1, COUNT_UP_TO(3) AS count_up_to_3, COUNT_UP_TO(4) AS field_2 OVER ( ... ); ``` Requires: * Must be unique across all aggregation aliases. * Conform to document field name limitations.
    
    pub alias: Option<String>,
    /// Count aggregator.
    
    pub count: Option<Count>,
}

impl client::Part for Aggregation {}


/// The result of a single bucket from a Firestore aggregation query. The keys of `aggregate_fields` are the same for all results in an aggregation query, unlike document queries which can have different fields present for each result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregationResult {
    /// The result of the aggregation functions, ex: `COUNT(*) AS total_docs`. The key is the alias assigned to the aggregation function on input and the size of this map equals the number of aggregation functions in the query.
    #[serde(rename="aggregateFields")]
    
    pub aggregate_fields: Option<HashMap<String, Value>>,
}

impl client::Part for AggregationResult {}


/// An array value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArrayValue {
    /// Values in the array.
    
    pub values: Option<Vec<Value>>,
}

impl client::Part for ArrayValue {}


/// The request for Firestore.BatchGetDocuments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents batch get projects](ProjectDatabaseDocumentBatchGetCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetDocumentsRequest {
    /// The names of the documents to retrieve. In the format: `projects/{project_id}/databases/{database_id}/documents/{document_path}`. The request will fail if any of the document is not a child resource of the given `database`. Duplicate names will be elided.
    
    pub documents: Option<Vec<String>>,
    /// The fields to return. If not set, returns all fields. If a document has a field that is not present in this mask, that field will not be returned in the response.
    
    pub mask: Option<DocumentMask>,
    /// Starts a new transaction and reads the documents. Defaults to a read-only transaction. The new transaction ID will be returned as the first response in the stream.
    #[serde(rename="newTransaction")]
    
    pub new_transaction: Option<TransactionOptions>,
    /// Reads documents as they were at the given time. This may not be older than 270 seconds.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Reads documents in a transaction.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub transaction: Option<Vec<u8>>,
}

impl client::RequestValue for BatchGetDocumentsRequest {}


/// The streamed response for Firestore.BatchGetDocuments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents batch get projects](ProjectDatabaseDocumentBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetDocumentsResponse {
    /// A document that was requested.
    
    pub found: Option<Document>,
    /// A document name that was requested but does not exist. In the format: `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    
    pub missing: Option<String>,
    /// The time at which the document was read. This may be monotically increasing, in this case the previous documents in the result stream are guaranteed not to have changed between their read_time and this one.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The transaction that was started as part of this request. Will only be set in the first response, and only if BatchGetDocumentsRequest.new_transaction was set in the request.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub transaction: Option<Vec<u8>>,
}

impl client::ResponseResult for BatchGetDocumentsResponse {}


/// The request for Firestore.BatchWrite.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents batch write projects](ProjectDatabaseDocumentBatchWriteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchWriteRequest {
    /// Labels associated with this batch write.
    
    pub labels: Option<HashMap<String, String>>,
    /// The writes to apply. Method does not apply writes atomically and does not guarantee ordering. Each write succeeds or fails independently. You cannot write to the same document more than once per request.
    
    pub writes: Option<Vec<Write>>,
}

impl client::RequestValue for BatchWriteRequest {}


/// The response from Firestore.BatchWrite.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents batch write projects](ProjectDatabaseDocumentBatchWriteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchWriteResponse {
    /// The status of applying the writes. This i-th write status corresponds to the i-th write in the request.
    
    pub status: Option<Vec<Status>>,
    /// The result of applying the writes. This i-th write result corresponds to the i-th write in the request.
    #[serde(rename="writeResults")]
    
    pub write_results: Option<Vec<WriteResult>>,
}

impl client::ResponseResult for BatchWriteResponse {}


/// The request for Firestore.BeginTransaction.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents begin transaction projects](ProjectDatabaseDocumentBeginTransactionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BeginTransactionRequest {
    /// The options for the transaction. Defaults to a read-write transaction.
    
    pub options: Option<TransactionOptions>,
}

impl client::RequestValue for BeginTransactionRequest {}


/// The response for Firestore.BeginTransaction.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents begin transaction projects](ProjectDatabaseDocumentBeginTransactionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BeginTransactionResponse {
    /// The transaction that was started.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub transaction: Option<Vec<u8>>,
}

impl client::ResponseResult for BeginTransactionResponse {}


/// A selection of a collection, such as `messages as m1`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CollectionSelector {
    /// When false, selects only collections that are immediate children of the `parent` specified in the containing `RunQueryRequest`. When true, selects all descendant collections.
    #[serde(rename="allDescendants")]
    
    pub all_descendants: Option<bool>,
    /// The collection ID. When set, selects only collections with this ID.
    #[serde(rename="collectionId")]
    
    pub collection_id: Option<String>,
}

impl client::Part for CollectionSelector {}


/// The request for Firestore.Commit.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents commit projects](ProjectDatabaseDocumentCommitCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommitRequest {
    /// If set, applies all writes in this transaction, and commits it.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub transaction: Option<Vec<u8>>,
    /// The writes to apply. Always executed atomically and in order.
    
    pub writes: Option<Vec<Write>>,
}

impl client::RequestValue for CommitRequest {}


/// The response for Firestore.Commit.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents commit projects](ProjectDatabaseDocumentCommitCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommitResponse {
    /// The time at which the commit occurred. Any read with an equal or greater `read_time` is guaranteed to see the effects of the commit.
    #[serde(rename="commitTime")]
    
    pub commit_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The result of applying the writes. This i-th write result corresponds to the i-th write in the request.
    #[serde(rename="writeResults")]
    
    pub write_results: Option<Vec<WriteResult>>,
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


/// Count of documents that match the query. The `COUNT(*)` aggregation function operates on the entire document so it does not require a field reference.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Count {
    /// Optional. Optional constraint on the maximum number of documents to count. This provides a way to set an upper bound on the number of documents to scan, limiting latency and cost. Unspecified is interpreted as no bound. High-Level Example: ``` AGGREGATE COUNT_UP_TO(1000) OVER ( SELECT * FROM k ); ``` Requires: * Must be greater than zero when present.
    #[serde(rename="upTo")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub up_to: Option<i64>,
}

impl client::Part for Count {}


/// A position in a query result set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cursor {
    /// If the position is just before or just after the given values, relative to the sort order defined by the query.
    
    pub before: Option<bool>,
    /// The values that represent a position, in the order they appear in the order by clause of a query. Can contain fewer values than specified in the order by clause.
    
    pub values: Option<Vec<Value>>,
}

impl client::Part for Cursor {}


/// A Firestore document. Must not exceed 1 MiB - 4 bytes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents create document projects](ProjectDatabaseDocumentCreateDocumentCall) (request|response)
/// * [databases documents get projects](ProjectDatabaseDocumentGetCall) (response)
/// * [databases documents patch projects](ProjectDatabaseDocumentPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    /// Output only. The time at which the document was created. This value increases monotonically when a document is deleted then recreated. It can also be compared to values from other documents and the `read_time` of a query.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The document's fields. The map keys represent field names. A simple field name contains only characters `a` to `z`, `A` to `Z`, `0` to `9`, or `_`, and must not start with `0` to `9`. For example, `foo_bar_17`. Field names matching the regular expression `__.*__` are reserved. Reserved field names are forbidden except in certain documented contexts. The map keys, represented as UTF-8, must not exceed 1,500 bytes and cannot be empty. Field paths may be used in other contexts to refer to structured fields defined here. For `map_value`, the field path is represented by the simple or quoted field names of the containing fields, delimited by `.`. For example, the structured field `"foo" : { map_value: { "x&y" : { string_value: "hello" }}}` would be represented by the field path `foo.x&y`. Within a field path, a quoted field name starts and ends with `` ` `` and may contain any character. Some characters, including `` ` ``, must be escaped using a `\`. For example, `` `x&y` `` represents `x&y` and `` `bak\`tik` `` represents `` bak`tik ``.
    
    pub fields: Option<HashMap<String, Value>>,
    /// The resource name of the document, for example `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    
    pub name: Option<String>,
    /// Output only. The time at which the document was last changed. This value is initially set to the `create_time` then increases monotonically with each change to the document. It can also be compared to values from other documents and the `read_time` of a query.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Document {}
impl client::ResponseResult for Document {}


/// A Document has changed. May be the result of multiple writes, including deletes, that ultimately resulted in a new value for the Document. Multiple DocumentChange messages may be returned for the same logical change, if multiple targets are affected.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentChange {
    /// The new state of the Document. If `mask` is set, contains only fields that were updated or added.
    
    pub document: Option<Document>,
    /// A set of target IDs for targets that no longer match this document.
    #[serde(rename="removedTargetIds")]
    
    pub removed_target_ids: Option<Vec<i32>>,
    /// A set of target IDs of targets that match this document.
    #[serde(rename="targetIds")]
    
    pub target_ids: Option<Vec<i32>>,
}

impl client::Part for DocumentChange {}


/// A Document has been deleted. May be the result of multiple writes, including updates, the last of which deleted the Document. Multiple DocumentDelete messages may be returned for the same logical delete, if multiple targets are affected.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentDelete {
    /// The resource name of the Document that was deleted.
    
    pub document: Option<String>,
    /// The read timestamp at which the delete was observed. Greater or equal to the `commit_time` of the delete.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A set of target IDs for targets that previously matched this entity.
    #[serde(rename="removedTargetIds")]
    
    pub removed_target_ids: Option<Vec<i32>>,
}

impl client::Part for DocumentDelete {}


/// A set of field paths on a document. Used to restrict a get or update operation on a document to a subset of its fields. This is different from standard field masks, as this is always scoped to a Document, and takes in account the dynamic nature of Value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentMask {
    /// The list of field paths in the mask. See Document.fields for a field path syntax reference.
    #[serde(rename="fieldPaths")]
    
    pub field_paths: Option<Vec<String>>,
}

impl client::Part for DocumentMask {}


/// A Document has been removed from the view of the targets. Sent if the document is no longer relevant to a target and is out of view. Can be sent instead of a DocumentDelete or a DocumentChange if the server can not send the new value of the document. Multiple DocumentRemove messages may be returned for the same logical write or delete, if multiple targets are affected.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentRemove {
    /// The resource name of the Document that has gone out of view.
    
    pub document: Option<String>,
    /// The read timestamp at which the remove was observed. Greater or equal to the `commit_time` of the change/delete/remove.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A set of target IDs for targets that previously matched this document.
    #[serde(rename="removedTargetIds")]
    
    pub removed_target_ids: Option<Vec<i32>>,
}

impl client::Part for DocumentRemove {}


/// A transformation of a document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentTransform {
    /// The name of the document to transform.
    
    pub document: Option<String>,
    /// The list of transformations to apply to the fields of the document, in order. This must not be empty.
    #[serde(rename="fieldTransforms")]
    
    pub field_transforms: Option<Vec<FieldTransform>>,
}

impl client::Part for DocumentTransform {}


/// A target specified by a set of documents names.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentsTarget {
    /// The names of the documents to retrieve. In the format: `projects/{project_id}/databases/{database_id}/documents/{document_path}`. The request will fail if any of the document is not a child resource of the given `database`. Duplicate names will be elided.
    
    pub documents: Option<Vec<String>>,
}

impl client::Part for DocumentsTarget {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases collection groups indexes delete projects](ProjectDatabaseCollectionGroupIndexDeleteCall) (response)
/// * [databases documents delete projects](ProjectDatabaseDocumentDeleteCall) (response)
/// * [databases documents rollback projects](ProjectDatabaseDocumentRollbackCall) (response)
/// * [databases operations cancel projects](ProjectDatabaseOperationCancelCall) (response)
/// * [databases operations delete projects](ProjectDatabaseOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// A digest of all the documents that match a given target.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExistenceFilter {
    /// The total count of documents that match target_id. If different from the count of documents in the client that match, the client must manually determine which documents no longer match the target.
    
    pub count: Option<i32>,
    /// The target ID to which this filter applies.
    #[serde(rename="targetId")]
    
    pub target_id: Option<i32>,
}

impl client::Part for ExistenceFilter {}


/// A filter on a specific field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FieldFilter {
    /// The field to filter by.
    
    pub field: Option<FieldReference>,
    /// The operator to filter by.
    
    pub op: Option<String>,
    /// The value to compare to.
    
    pub value: Option<Value>,
}

impl client::Part for FieldFilter {}


/// A reference to a field in a document, ex: `stats.operations`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FieldReference {
    /// The relative path of the document being referenced. Requires: * Conform to document field name limitations.
    #[serde(rename="fieldPath")]
    
    pub field_path: Option<String>,
}

impl client::Part for FieldReference {}


/// A transformation of a field of the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FieldTransform {
    /// Append the given elements in order if they are not already present in the current field value. If the field is not an array, or if the field does not yet exist, it is first set to the empty array. Equivalent numbers of different types (e.g. 3L and 3.0) are considered equal when checking if a value is missing. NaN is equal to NaN, and Null is equal to Null. If the input contains multiple equivalent values, only the first will be considered. The corresponding transform_result will be the null value.
    #[serde(rename="appendMissingElements")]
    
    pub append_missing_elements: Option<ArrayValue>,
    /// The path of the field. See Document.fields for the field path syntax reference.
    #[serde(rename="fieldPath")]
    
    pub field_path: Option<String>,
    /// Adds the given value to the field's current value. This must be an integer or a double value. If the field is not an integer or double, or if the field does not yet exist, the transformation will set the field to the given value. If either of the given value or the current field value are doubles, both values will be interpreted as doubles. Double arithmetic and representation of double values follow IEEE 754 semantics. If there is positive/negative integer overflow, the field is resolved to the largest magnitude positive/negative integer.
    
    pub increment: Option<Value>,
    /// Sets the field to the maximum of its current value and the given value. This must be an integer or a double value. If the field is not an integer or double, or if the field does not yet exist, the transformation will set the field to the given value. If a maximum operation is applied where the field and the input value are of mixed types (that is - one is an integer and one is a double) the field takes on the type of the larger operand. If the operands are equivalent (e.g. 3 and 3.0), the field does not change. 0, 0.0, and -0.0 are all zero. The maximum of a zero stored value and zero input value is always the stored value. The maximum of any numeric value x and NaN is NaN.
    
    pub maximum: Option<Value>,
    /// Sets the field to the minimum of its current value and the given value. This must be an integer or a double value. If the field is not an integer or double, or if the field does not yet exist, the transformation will set the field to the input value. If a minimum operation is applied where the field and the input value are of mixed types (that is - one is an integer and one is a double) the field takes on the type of the smaller operand. If the operands are equivalent (e.g. 3 and 3.0), the field does not change. 0, 0.0, and -0.0 are all zero. The minimum of a zero stored value and zero input value is always the stored value. The minimum of any numeric value x and NaN is NaN.
    
    pub minimum: Option<Value>,
    /// Remove all of the given elements from the array in the field. If the field is not an array, or if the field does not yet exist, it is set to the empty array. Equivalent numbers of the different types (e.g. 3L and 3.0) are considered equal when deciding whether an element should be removed. NaN is equal to NaN, and Null is equal to Null. This will remove all equivalent values if there are duplicates. The corresponding transform_result will be the null value.
    #[serde(rename="removeAllFromArray")]
    
    pub remove_all_from_array: Option<ArrayValue>,
    /// Sets the field to the given server value.
    #[serde(rename="setToServerValue")]
    
    pub set_to_server_value: Option<String>,
}

impl client::Part for FieldTransform {}


/// A filter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Filter {
    /// A composite filter.
    #[serde(rename="compositeFilter")]
    
    pub composite_filter: Option<CompositeFilter>,
    /// A filter on a document field.
    #[serde(rename="fieldFilter")]
    
    pub field_filter: Option<FieldFilter>,
    /// A filter that takes exactly one argument.
    #[serde(rename="unaryFilter")]
    
    pub unary_filter: Option<UnaryFilter>,
}

impl client::Part for Filter {}


/// A Cloud Firestore Database. Currently only one database is allowed per cloud project; this database must have a `database_id` of ‘(default)’.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases create projects](ProjectDatabaseCreateCall) (request)
/// * [databases get projects](ProjectDatabaseGetCall) (response)
/// * [databases patch projects](ProjectDatabasePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirestoreAdminV1Database {
    /// The App Engine integration mode to use for this database.
    #[serde(rename="appEngineIntegrationMode")]
    
    pub app_engine_integration_mode: Option<String>,
    /// The concurrency control mode to use for this database.
    #[serde(rename="concurrencyMode")]
    
    pub concurrency_mode: Option<String>,
    /// This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Output only. The key_prefix for this database. This key_prefix is used, in combination with the project id ("~") to construct the application id that is returned from the Cloud Datastore APIs in Google App Engine first generation runtimes. This value may be empty in which case the appid to use for URL-encoded keys is the project_id (eg: foo instead of v~foo).
    #[serde(rename="keyPrefix")]
    
    pub key_prefix: Option<String>,
    /// The location of the database. Available databases are listed at https://cloud.google.com/firestore/docs/locations.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// The resource name of the Database. Format: `projects/{project}/databases/{database}`
    
    pub name: Option<String>,
    /// The type of the database. See https://cloud.google.com/datastore/docs/firestore-or-datastore for information about how to choose.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for GoogleFirestoreAdminV1Database {}
impl client::ResponseResult for GoogleFirestoreAdminV1Database {}


/// The request for FirestoreAdmin.ExportDocuments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases export documents projects](ProjectDatabaseExportDocumentCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirestoreAdminV1ExportDocumentsRequest {
    /// Which collection ids to export. Unspecified means all collections.
    #[serde(rename="collectionIds")]
    
    pub collection_ids: Option<Vec<String>>,
    /// An empty list represents all namespaces. This is the preferred usage for databases that don't use namespaces. An empty string element represents the default namespace. This should be used if the database has data in non-default namespaces, but doesn't want to include them. Each namespace in this list must be unique.
    #[serde(rename="namespaceIds")]
    
    pub namespace_ids: Option<Vec<String>>,
    /// The output URI. Currently only supports Google Cloud Storage URIs of the form: `gs://BUCKET_NAME[/NAMESPACE_PATH]`, where `BUCKET_NAME` is the name of the Google Cloud Storage bucket and `NAMESPACE_PATH` is an optional Google Cloud Storage namespace path. When choosing a name, be sure to consider Google Cloud Storage naming guidelines: https://cloud.google.com/storage/docs/naming. If the URI is a bucket (without a namespace path), a prefix will be generated based on the start time.
    #[serde(rename="outputUriPrefix")]
    
    pub output_uri_prefix: Option<String>,
}

impl client::RequestValue for GoogleFirestoreAdminV1ExportDocumentsRequest {}


/// Represents a single field in the database. Fields are grouped by their “Collection Group”, which represent all collections in the database with the same id.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases collection groups fields get projects](ProjectDatabaseCollectionGroupFieldGetCall) (response)
/// * [databases collection groups fields patch projects](ProjectDatabaseCollectionGroupFieldPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirestoreAdminV1Field {
    /// The index configuration for this field. If unset, field indexing will revert to the configuration defined by the `ancestor_field`. To explicitly remove all indexes for this field, specify an index config with an empty list of indexes.
    #[serde(rename="indexConfig")]
    
    pub index_config: Option<GoogleFirestoreAdminV1IndexConfig>,
    /// Required. A field name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_path}` A field path may be a simple field name, e.g. `address` or a path to fields within map_value , e.g. `address.city`, or a special field path. The only valid special field is `*`, which represents any field. Field paths may be quoted using ` (backtick). The only character that needs to be escaped within a quoted field path is the backtick character itself, escaped using a backslash. Special characters in field paths that must be quoted include: `*`, `.`, ``` (backtick), `[`, `]`, as well as any ascii symbolic characters. Examples: (Note: Comments here are written in markdown syntax, so there is an additional layer of backticks to represent a code block) `\`address.city\`` represents a field named `address.city`, not the map key `city` in the field `address`. `\`*\`` represents a field named `*`, not any field. A special `Field` contains the default indexing settings for all fields. This field's resource name is: `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*` Indexes defined on this `Field` will be applied to all fields which do not have their own `Field` index configuration.
    
    pub name: Option<String>,
    /// The TTL configuration for this `Field`. Setting or unsetting this will enable or disable the TTL for documents that have this `Field`.
    #[serde(rename="ttlConfig")]
    
    pub ttl_config: Option<GoogleFirestoreAdminV1TtlConfig>,
}

impl client::RequestValue for GoogleFirestoreAdminV1Field {}
impl client::ResponseResult for GoogleFirestoreAdminV1Field {}


/// The request for FirestoreAdmin.ImportDocuments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases import documents projects](ProjectDatabaseImportDocumentCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirestoreAdminV1ImportDocumentsRequest {
    /// Which collection ids to import. Unspecified means all collections included in the import.
    #[serde(rename="collectionIds")]
    
    pub collection_ids: Option<Vec<String>>,
    /// Location of the exported files. This must match the output_uri_prefix of an ExportDocumentsResponse from an export that has completed successfully. See: google.firestore.admin.v1.ExportDocumentsResponse.output_uri_prefix.
    #[serde(rename="inputUriPrefix")]
    
    pub input_uri_prefix: Option<String>,
    /// An empty list represents all namespaces. This is the preferred usage for databases that don't use namespaces. An empty string element represents the default namespace. This should be used if the database has data in non-default namespaces, but doesn't want to include them. Each namespace in this list must be unique.
    #[serde(rename="namespaceIds")]
    
    pub namespace_ids: Option<Vec<String>>,
}

impl client::RequestValue for GoogleFirestoreAdminV1ImportDocumentsRequest {}


/// Cloud Firestore indexes enable simple and complex queries against documents in a database.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases collection groups indexes create projects](ProjectDatabaseCollectionGroupIndexCreateCall) (request)
/// * [databases collection groups indexes get projects](ProjectDatabaseCollectionGroupIndexGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirestoreAdminV1Index {
    /// The API scope supported by this index.
    #[serde(rename="apiScope")]
    
    pub api_scope: Option<String>,
    /// The fields supported by this index. For composite indexes, this requires a minimum of 2 and a maximum of 100 fields. The last field entry is always for the field path `__name__`. If, on creation, `__name__` was not specified as the last field, it will be added automatically with the same direction as that of the last field defined. If the final field in a composite index is not directional, the `__name__` will be ordered ASCENDING (unless explicitly specified). For single field indexes, this will always be exactly one entry with a field path equal to the field path of the associated field.
    
    pub fields: Option<Vec<GoogleFirestoreAdminV1IndexField>>,
    /// Output only. A server defined name for this index. The form of this name for composite indexes will be: `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/indexes/{composite_index_id}` For single field indexes, this field will be empty.
    
    pub name: Option<String>,
    /// Indexes with a collection query scope specified allow queries against a collection that is the child of a specific document, specified at query time, and that has the same collection id. Indexes with a collection group query scope specified allow queries against all collections descended from a specific document, specified at query time, and that have the same collection id as this index.
    #[serde(rename="queryScope")]
    
    pub query_scope: Option<String>,
    /// Output only. The serving state of the index.
    
    pub state: Option<String>,
}

impl client::RequestValue for GoogleFirestoreAdminV1Index {}
impl client::ResponseResult for GoogleFirestoreAdminV1Index {}


/// The index configuration for this field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirestoreAdminV1IndexConfig {
    /// Output only. Specifies the resource name of the `Field` from which this field's index configuration is set (when `uses_ancestor_config` is true), or from which it *would* be set if this field had no index configuration (when `uses_ancestor_config` is false).
    #[serde(rename="ancestorField")]
    
    pub ancestor_field: Option<String>,
    /// The indexes supported for this field.
    
    pub indexes: Option<Vec<GoogleFirestoreAdminV1Index>>,
    /// Output only When true, the `Field`'s index configuration is in the process of being reverted. Once complete, the index config will transition to the same state as the field specified by `ancestor_field`, at which point `uses_ancestor_config` will be `true` and `reverting` will be `false`.
    
    pub reverting: Option<bool>,
    /// Output only. When true, the `Field`'s index configuration is set from the configuration specified by the `ancestor_field`. When false, the `Field`'s index configuration is defined explicitly.
    #[serde(rename="usesAncestorConfig")]
    
    pub uses_ancestor_config: Option<bool>,
}

impl client::Part for GoogleFirestoreAdminV1IndexConfig {}


/// A field in an index. The field_path describes which field is indexed, the value_mode describes how the field value is indexed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirestoreAdminV1IndexField {
    /// Indicates that this field supports operations on `array_value`s.
    #[serde(rename="arrayConfig")]
    
    pub array_config: Option<String>,
    /// Can be __name__. For single field indexes, this must match the name of the field or may be omitted.
    #[serde(rename="fieldPath")]
    
    pub field_path: Option<String>,
    /// Indicates that this field supports ordering by the specified order or comparing using =, !=, <, <=, >, >=.
    
    pub order: Option<String>,
}

impl client::Part for GoogleFirestoreAdminV1IndexField {}


/// The list of databases for a project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases list projects](ProjectDatabaseListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirestoreAdminV1ListDatabasesResponse {
    /// The databases in the project.
    
    pub databases: Option<Vec<GoogleFirestoreAdminV1Database>>,
}

impl client::ResponseResult for GoogleFirestoreAdminV1ListDatabasesResponse {}


/// The response for FirestoreAdmin.ListFields.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases collection groups fields list projects](ProjectDatabaseCollectionGroupFieldListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirestoreAdminV1ListFieldsResponse {
    /// The requested fields.
    
    pub fields: Option<Vec<GoogleFirestoreAdminV1Field>>,
    /// A page token that may be used to request another page of results. If blank, this is the last page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleFirestoreAdminV1ListFieldsResponse {}


/// The response for FirestoreAdmin.ListIndexes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases collection groups indexes list projects](ProjectDatabaseCollectionGroupIndexListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirestoreAdminV1ListIndexesResponse {
    /// The requested indexes.
    
    pub indexes: Option<Vec<GoogleFirestoreAdminV1Index>>,
    /// A page token that may be used to request another page of results. If blank, this is the last page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleFirestoreAdminV1ListIndexesResponse {}


/// The TTL (time-to-live) configuration for documents that have this `Field` set. Storing a timestamp value into a TTL-enabled field will be treated as the document's absolute expiration time. Timestamp values in the past indicate that the document is eligible for immediate expiration. Using any other data type or leaving the field absent will disable expiration for the individual document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirestoreAdminV1TtlConfig {
    /// Output only. The state of the TTL configuration.
    
    pub state: Option<String>,
}

impl client::Part for GoogleFirestoreAdminV1TtlConfig {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases operations cancel projects](ProjectDatabaseOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningCancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleLongrunningCancelOperationRequest {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases operations list projects](ProjectDatabaseOperationListCall) (response)
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
/// * [databases collection groups fields patch projects](ProjectDatabaseCollectionGroupFieldPatchCall) (response)
/// * [databases collection groups indexes create projects](ProjectDatabaseCollectionGroupIndexCreateCall) (response)
/// * [databases operations get projects](ProjectDatabaseOperationGetCall) (response)
/// * [databases create projects](ProjectDatabaseCreateCall) (response)
/// * [databases delete projects](ProjectDatabaseDeleteCall) (response)
/// * [databases export documents projects](ProjectDatabaseExportDocumentCall) (response)
/// * [databases import documents projects](ProjectDatabaseImportDocumentCall) (response)
/// * [databases patch projects](ProjectDatabasePatchCall) (response)
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


/// The request for Firestore.ListCollectionIds.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents list collection ids projects](ProjectDatabaseDocumentListCollectionIdCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCollectionIdsRequest {
    /// The maximum number of results to return.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// A page token. Must be a value from ListCollectionIdsResponse.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Reads documents as they were at the given time. This may not be older than 270 seconds.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ListCollectionIdsRequest {}


/// The response from Firestore.ListCollectionIds.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents list collection ids projects](ProjectDatabaseDocumentListCollectionIdCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCollectionIdsResponse {
    /// The collection ids.
    #[serde(rename="collectionIds")]
    
    pub collection_ids: Option<Vec<String>>,
    /// A page token that may be used to continue the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCollectionIdsResponse {}


/// The response for Firestore.ListDocuments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents list projects](ProjectDatabaseDocumentListCall) (response)
/// * [databases documents list documents projects](ProjectDatabaseDocumentListDocumentCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDocumentsResponse {
    /// The Documents found.
    
    pub documents: Option<Vec<Document>>,
    /// A token to retrieve the next page of documents. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDocumentsResponse {}


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


/// A request for Firestore.Listen
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents listen projects](ProjectDatabaseDocumentListenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListenRequest {
    /// A target to add to this stream.
    #[serde(rename="addTarget")]
    
    pub add_target: Option<Target>,
    /// Labels associated with this target change.
    
    pub labels: Option<HashMap<String, String>>,
    /// The ID of a target to remove from this stream.
    #[serde(rename="removeTarget")]
    
    pub remove_target: Option<i32>,
}

impl client::RequestValue for ListenRequest {}


/// The response for Firestore.Listen.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents listen projects](ProjectDatabaseDocumentListenCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListenResponse {
    /// A Document has changed.
    #[serde(rename="documentChange")]
    
    pub document_change: Option<DocumentChange>,
    /// A Document has been deleted.
    #[serde(rename="documentDelete")]
    
    pub document_delete: Option<DocumentDelete>,
    /// A Document has been removed from a target (because it is no longer relevant to that target).
    #[serde(rename="documentRemove")]
    
    pub document_remove: Option<DocumentRemove>,
    /// A filter to apply to the set of documents previously returned for the given target. Returned when documents may have been removed from the given target, but the exact documents are unknown.
    
    pub filter: Option<ExistenceFilter>,
    /// Targets have changed.
    #[serde(rename="targetChange")]
    
    pub target_change: Option<TargetChange>,
}

impl client::ResponseResult for ListenResponse {}


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


/// A map value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MapValue {
    /// The map's fields. The map keys represent field names. Field names matching the regular expression `__.*__` are reserved. Reserved field names are forbidden except in certain documented contexts. The map keys, represented as UTF-8, must not exceed 1,500 bytes and cannot be empty.
    
    pub fields: Option<HashMap<String, Value>>,
}

impl client::Part for MapValue {}


/// An order on a field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    /// The direction to order by. Defaults to `ASCENDING`.
    
    pub direction: Option<String>,
    /// The field to order by.
    
    pub field: Option<FieldReference>,
}

impl client::Part for Order {}


/// The request for Firestore.PartitionQuery.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents partition query projects](ProjectDatabaseDocumentPartitionQueryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartitionQueryRequest {
    /// The maximum number of partitions to return in this call, subject to `partition_count`. For example, if `partition_count` = 10 and `page_size` = 8, the first call to PartitionQuery will return up to 8 partitions and a `next_page_token` if more results exist. A second call to PartitionQuery will return up to 2 partitions, to complete the total of 10 specified in `partition_count`.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// The `next_page_token` value returned from a previous call to PartitionQuery that may be used to get an additional set of results. There are no ordering guarantees between sets of results. Thus, using multiple sets of results will require merging the different result sets. For example, two subsequent calls using a page_token may return: * cursor B, cursor M, cursor Q * cursor A, cursor U, cursor W To obtain a complete result set ordered with respect to the results of the query supplied to PartitionQuery, the results sets should be merged: cursor A, cursor B, cursor M, cursor Q, cursor U, cursor W
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// The desired maximum number of partition points. The partitions may be returned across multiple pages of results. The number must be positive. The actual number of partitions returned may be fewer. For example, this may be set to one fewer than the number of parallel queries to be run, or in running a data pipeline job, one fewer than the number of workers or compute instances available.
    #[serde(rename="partitionCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partition_count: Option<i64>,
    /// Reads documents as they were at the given time. This may not be older than 270 seconds.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A structured query. Query must specify collection with all descendants and be ordered by name ascending. Other filters, order bys, limits, offsets, and start/end cursors are not supported.
    #[serde(rename="structuredQuery")]
    
    pub structured_query: Option<StructuredQuery>,
}

impl client::RequestValue for PartitionQueryRequest {}


/// The response for Firestore.PartitionQuery.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents partition query projects](ProjectDatabaseDocumentPartitionQueryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartitionQueryResponse {
    /// A page token that may be used to request an additional set of results, up to the number specified by `partition_count` in the PartitionQuery request. If blank, there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Partition results. Each partition is a split point that can be used by RunQuery as a starting or end point for the query results. The RunQuery requests must be made with the same query supplied to this PartitionQuery request. The partition cursors will be ordered according to same ordering as the results of the query supplied to PartitionQuery. For example, if a PartitionQuery request returns partition cursors A and B, running the following three queries will return the entire result set of the original query: * query, end_at A * query, start_at A, end_at B * query, start_at B An empty result may indicate that the query has too few results to be partitioned.
    
    pub partitions: Option<Vec<Cursor>>,
}

impl client::ResponseResult for PartitionQueryResponse {}


/// A precondition on a document, used for conditional operations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Precondition {
    /// When set to `true`, the target document must exist. When set to `false`, the target document must not exist.
    
    pub exists: Option<bool>,
    /// When set, the target document must exist and have been last updated at that time. Timestamp must be microsecond aligned.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Precondition {}


/// The projection of document's fields to return.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Projection {
    /// The fields to return. If empty, all fields are returned. To only return the name of the document, use `['__name__']`.
    
    pub fields: Option<Vec<FieldReference>>,
}

impl client::Part for Projection {}


/// A target specified by a query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryTarget {
    /// The parent resource name. In the format: `projects/{project_id}/databases/{database_id}/documents` or `projects/{project_id}/databases/{database_id}/documents/{document_path}`. For example: `projects/my-project/databases/my-database/documents` or `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`
    
    pub parent: Option<String>,
    /// A structured query.
    #[serde(rename="structuredQuery")]
    
    pub structured_query: Option<StructuredQuery>,
}

impl client::Part for QueryTarget {}


/// Options for a transaction that can only be used to read documents.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReadOnly {
    /// Reads documents at the given time. This may not be older than 60 seconds.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for ReadOnly {}


/// Options for a transaction that can be used to read and write documents.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReadWrite {
    /// An optional transaction to retry.
    #[serde(rename="retryTransaction")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub retry_transaction: Option<Vec<u8>>,
}

impl client::Part for ReadWrite {}


/// The request for Firestore.Rollback.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents rollback projects](ProjectDatabaseDocumentRollbackCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollbackRequest {
    /// Required. The transaction to roll back.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub transaction: Option<Vec<u8>>,
}

impl client::RequestValue for RollbackRequest {}


/// The request for Firestore.RunAggregationQuery.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents run aggregation query projects](ProjectDatabaseDocumentRunAggregationQueryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunAggregationQueryRequest {
    /// Starts a new transaction as part of the query, defaulting to read-only. The new transaction ID will be returned as the first response in the stream.
    #[serde(rename="newTransaction")]
    
    pub new_transaction: Option<TransactionOptions>,
    /// Executes the query at the given timestamp. Requires: * Cannot be more than 270 seconds in the past.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// An aggregation query.
    #[serde(rename="structuredAggregationQuery")]
    
    pub structured_aggregation_query: Option<StructuredAggregationQuery>,
    /// Run the aggregation within an already active transaction. The value here is the opaque transaction ID to execute the query in.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub transaction: Option<Vec<u8>>,
}

impl client::RequestValue for RunAggregationQueryRequest {}


/// The response for Firestore.RunAggregationQuery.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents run aggregation query projects](ProjectDatabaseDocumentRunAggregationQueryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunAggregationQueryResponse {
    /// The time at which the aggregate value is valid for.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A single aggregation result. Not present when reporting partial progress.
    
    pub result: Option<AggregationResult>,
    /// The transaction that was started as part of this request. Only present on the first response when the request requested to start a new transaction.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub transaction: Option<Vec<u8>>,
}

impl client::ResponseResult for RunAggregationQueryResponse {}


/// The request for Firestore.RunQuery.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents run query projects](ProjectDatabaseDocumentRunQueryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunQueryRequest {
    /// Starts a new transaction and reads the documents. Defaults to a read-only transaction. The new transaction ID will be returned as the first response in the stream.
    #[serde(rename="newTransaction")]
    
    pub new_transaction: Option<TransactionOptions>,
    /// Reads documents as they were at the given time. This may not be older than 270 seconds.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A structured query.
    #[serde(rename="structuredQuery")]
    
    pub structured_query: Option<StructuredQuery>,
    /// Run the query within an already active transaction. The value here is the opaque transaction ID to execute the query in.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub transaction: Option<Vec<u8>>,
}

impl client::RequestValue for RunQueryRequest {}


/// The response for Firestore.RunQuery.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents run query projects](ProjectDatabaseDocumentRunQueryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunQueryResponse {
    /// A query result, not set when reporting partial progress.
    
    pub document: Option<Document>,
    /// If present, Firestore has completely finished the request and no more documents will be returned.
    
    pub done: Option<bool>,
    /// The time at which the document was read. This may be monotonically increasing; in this case, the previous documents in the result stream are guaranteed not to have changed between their `read_time` and this one. If the query returns no results, a response with `read_time` and no `document` will be sent, and this represents the time at which the query was run.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The number of results that have been skipped due to an offset between the last response and the current response.
    #[serde(rename="skippedResults")]
    
    pub skipped_results: Option<i32>,
    /// The transaction that was started as part of this request. Can only be set in the first response, and only if RunQueryRequest.new_transaction was set in the request. If set, no other fields will be set in this response.
    
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


/// Firestore query for running an aggregation over a StructuredQuery.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StructuredAggregationQuery {
    /// Optional. Series of aggregations to apply over the results of the `structured_query`. Requires: * A minimum of one and maximum of five aggregations per query.
    
    pub aggregations: Option<Vec<Aggregation>>,
    /// Nested structured query.
    #[serde(rename="structuredQuery")]
    
    pub structured_query: Option<StructuredQuery>,
}

impl client::Part for StructuredAggregationQuery {}


/// A Firestore query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StructuredQuery {
    /// A potential prefix of a position in the result set to end the query at. This is similar to `START_AT` but with it controlling the end position rather than the start position. Requires: * The number of values cannot be greater than the number of fields specified in the `ORDER BY` clause.
    #[serde(rename="endAt")]
    
    pub end_at: Option<Cursor>,
    /// The collections to query.
    
    pub from: Option<Vec<CollectionSelector>>,
    /// The maximum number of results to return. Applies after all other constraints. Requires: * The value must be greater than or equal to zero if specified.
    
    pub limit: Option<i32>,
    /// The number of documents to skip before returning the first result. This applies after the constraints specified by the `WHERE`, `START AT`, & `END AT` but before the `LIMIT` clause. Requires: * The value must be greater than or equal to zero if specified.
    
    pub offset: Option<i32>,
    /// The order to apply to the query results. Firestore allows callers to provide a full ordering, a partial ordering, or no ordering at all. In all cases, Firestore guarantees a stable ordering through the following rules: * The `order_by` is required to reference all fields used with an inequality filter. * All fields that are required to be in the `order_by` but are not already present are appended in lexicographical ordering of the field name. * If an order on `__name__` is not specified, it is appended by default. Fields are appended with the same sort direction as the last order specified, or 'ASCENDING' if no order was specified. For example: * `ORDER BY a` becomes `ORDER BY a ASC, __name__ ASC` * `ORDER BY a DESC` becomes `ORDER BY a DESC, __name__ DESC` * `WHERE a > 1` becomes `WHERE a > 1 ORDER BY a ASC, __name__ ASC` * `WHERE __name__ > ... AND a > 1` becomes `WHERE __name__ > ... AND a > 1 ORDER BY a ASC, __name__ ASC`
    #[serde(rename="orderBy")]
    
    pub order_by: Option<Vec<Order>>,
    /// The projection to return.
    
    pub select: Option<Projection>,
    /// A potential prefix of a position in the result set to start the query at. The ordering of the result set is based on the `ORDER BY` clause of the original query. ``` SELECT * FROM k WHERE a = 1 AND b > 2 ORDER BY b ASC, __name__ ASC; ``` This query's results are ordered by `(b ASC, __name__ ASC)`. Cursors can reference either the full ordering or a prefix of the location, though it cannot reference more fields than what are in the provided `ORDER BY`. Continuing off the example above, attaching the following start cursors will have varying impact: - `START BEFORE (2, /k/123)`: start the query right before `a = 1 AND b > 2 AND __name__ > /k/123`. - `START AFTER (10)`: start the query right after `a = 1 AND b > 10`. Unlike `OFFSET` which requires scanning over the first N results to skip, a start cursor allows the query to begin at a logical position. This position is not required to match an actual result, it will scan forward from this position to find the next document. Requires: * The number of values cannot be greater than the number of fields specified in the `ORDER BY` clause.
    #[serde(rename="startAt")]
    
    pub start_at: Option<Cursor>,
    /// The filter to apply.
    #[serde(rename="where")]
    
    pub where_: Option<Filter>,
}

impl client::Part for StructuredQuery {}


/// A specification of a set of documents to listen to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Target {
    /// A target specified by a set of document names.
    
    pub documents: Option<DocumentsTarget>,
    /// If the target should be removed once it is current and consistent.
    
    pub once: Option<bool>,
    /// A target specified by a query.
    
    pub query: Option<QueryTarget>,
    /// Start listening after a specific `read_time`. The client must know the state of matching documents at this time.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A resume token from a prior TargetChange for an identical target. Using a resume token with a different target is unsupported and may fail.
    #[serde(rename="resumeToken")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub resume_token: Option<Vec<u8>>,
    /// The target ID that identifies the target on the stream. Must be a positive number and non-zero.
    #[serde(rename="targetId")]
    
    pub target_id: Option<i32>,
}

impl client::Part for Target {}


/// Targets being watched have changed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetChange {
    /// The error that resulted in this change, if applicable.
    
    pub cause: Option<Status>,
    /// The consistent `read_time` for the given `target_ids` (omitted when the target_ids are not at a consistent snapshot). The stream is guaranteed to send a `read_time` with `target_ids` empty whenever the entire stream reaches a new consistent snapshot. ADD, CURRENT, and RESET messages are guaranteed to (eventually) result in a new consistent snapshot (while NO_CHANGE and REMOVE messages are not). For a given stream, `read_time` is guaranteed to be monotonically increasing.
    #[serde(rename="readTime")]
    
    pub read_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A token that can be used to resume the stream for the given `target_ids`, or all targets if `target_ids` is empty. Not set on every target change.
    #[serde(rename="resumeToken")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub resume_token: Option<Vec<u8>>,
    /// The type of change that occurred.
    #[serde(rename="targetChangeType")]
    
    pub target_change_type: Option<String>,
    /// The target IDs of targets that have changed. If empty, the change applies to all targets. The order of the target IDs is not defined.
    #[serde(rename="targetIds")]
    
    pub target_ids: Option<Vec<i32>>,
}

impl client::Part for TargetChange {}


/// Options for creating a new transaction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransactionOptions {
    /// The transaction can only be used for read operations.
    #[serde(rename="readOnly")]
    
    pub read_only: Option<ReadOnly>,
    /// The transaction can be used for both read and write operations.
    #[serde(rename="readWrite")]
    
    pub read_write: Option<ReadWrite>,
}

impl client::Part for TransactionOptions {}


/// A filter with a single operand.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnaryFilter {
    /// The field to which to apply the operator.
    
    pub field: Option<FieldReference>,
    /// The unary operator to apply.
    
    pub op: Option<String>,
}

impl client::Part for UnaryFilter {}


/// A message that can hold any of the supported value types.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Value {
    /// An array value. Cannot directly contain another array value, though can contain an map which contains another array.
    #[serde(rename="arrayValue")]
    
    pub array_value: Option<ArrayValue>,
    /// A boolean value.
    #[serde(rename="booleanValue")]
    
    pub boolean_value: Option<bool>,
    /// A bytes value. Must not exceed 1 MiB - 89 bytes. Only the first 1,500 bytes are considered by queries.
    #[serde(rename="bytesValue")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub bytes_value: Option<Vec<u8>>,
    /// A double value.
    #[serde(rename="doubleValue")]
    
    pub double_value: Option<f64>,
    /// A geo point value representing a point on the surface of Earth.
    #[serde(rename="geoPointValue")]
    
    pub geo_point_value: Option<LatLng>,
    /// An integer value.
    #[serde(rename="integerValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub integer_value: Option<i64>,
    /// A map value.
    #[serde(rename="mapValue")]
    
    pub map_value: Option<MapValue>,
    /// A null value.
    #[serde(rename="nullValue")]
    
    pub null_value: Option<String>,
    /// A reference to a document. For example: `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    #[serde(rename="referenceValue")]
    
    pub reference_value: Option<String>,
    /// A string value. The string, represented as UTF-8, must not exceed 1 MiB - 89 bytes. Only the first 1,500 bytes of the UTF-8 representation are considered by queries.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
    /// A timestamp value. Precise only to microseconds. When stored, any additional precision is rounded down.
    #[serde(rename="timestampValue")]
    
    pub timestamp_value: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Value {}


/// A write on a document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Write {
    /// An optional precondition on the document. The write will fail if this is set and not met by the target document.
    #[serde(rename="currentDocument")]
    
    pub current_document: Option<Precondition>,
    /// A document name to delete. In the format: `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    
    pub delete: Option<String>,
    /// Applies a transformation to a document.
    
    pub transform: Option<DocumentTransform>,
    /// A document to write.
    
    pub update: Option<Document>,
    /// The fields to update in this write. This field can be set only when the operation is `update`. If the mask is not set for an `update` and the document exists, any existing data will be overwritten. If the mask is set and the document on the server has fields not covered by the mask, they are left unchanged. Fields referenced in the mask, but not present in the input document, are deleted from the document on the server. The field paths in this mask must not contain a reserved field name.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<DocumentMask>,
    /// The transforms to perform after update. This field can be set only when the operation is `update`. If present, this write is equivalent to performing `update` and `transform` to the same document atomically and in order.
    #[serde(rename="updateTransforms")]
    
    pub update_transforms: Option<Vec<FieldTransform>>,
}

impl client::Part for Write {}


/// The request for Firestore.Write. The first request creates a stream, or resumes an existing one from a token. When creating a new stream, the server replies with a response containing only an ID and a token, to use in the next request. When resuming a stream, the server first streams any responses later than the given token, then a response containing only an up-to-date token, to use in the next request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents write projects](ProjectDatabaseDocumentWriteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WriteRequest {
    /// Labels associated with this write request.
    
    pub labels: Option<HashMap<String, String>>,
    /// The ID of the write stream to resume. This may only be set in the first message. When left empty, a new write stream will be created.
    #[serde(rename="streamId")]
    
    pub stream_id: Option<String>,
    /// A stream token that was previously sent by the server. The client should set this field to the token from the most recent WriteResponse it has received. This acknowledges that the client has received responses up to this token. After sending this token, earlier tokens may not be used anymore. The server may close the stream if there are too many unacknowledged responses. Leave this field unset when creating a new stream. To resume a stream at a specific point, set this field and the `stream_id` field. Leave this field unset when creating a new stream.
    #[serde(rename="streamToken")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub stream_token: Option<Vec<u8>>,
    /// The writes to apply. Always executed atomically and in order. This must be empty on the first request. This may be empty on the last request. This must not be empty on all other requests.
    
    pub writes: Option<Vec<Write>>,
}

impl client::RequestValue for WriteRequest {}


/// The response for Firestore.Write.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [databases documents write projects](ProjectDatabaseDocumentWriteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WriteResponse {
    /// The time at which the commit occurred. Any read with an equal or greater `read_time` is guaranteed to see the effects of the write.
    #[serde(rename="commitTime")]
    
    pub commit_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The ID of the stream. Only set on the first message, when a new stream was created.
    #[serde(rename="streamId")]
    
    pub stream_id: Option<String>,
    /// A token that represents the position of this response in the stream. This can be used by a client to resume the stream at this point. This field is always set.
    #[serde(rename="streamToken")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub stream_token: Option<Vec<u8>>,
    /// The result of applying the writes. This i-th write result corresponds to the i-th write in the request.
    #[serde(rename="writeResults")]
    
    pub write_results: Option<Vec<WriteResult>>,
}

impl client::ResponseResult for WriteResponse {}


/// The result of applying a write.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WriteResult {
    /// The results of applying each DocumentTransform.FieldTransform, in the same order.
    #[serde(rename="transformResults")]
    
    pub transform_results: Option<Vec<Value>>,
    /// The last update time of the document after applying the write. Not set after a `delete`. If the write did not actually change the document, this will be the previous update_time.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for WriteResult {}


