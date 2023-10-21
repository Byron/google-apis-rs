use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Firestore`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_firestore1 as firestore1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use firestore1::{Firestore, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Firestore::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `databases_collection_groups_fields_get(...)`, `databases_collection_groups_fields_list(...)`, `databases_collection_groups_fields_patch(...)`, `databases_collection_groups_indexes_create(...)`, `databases_collection_groups_indexes_delete(...)`, `databases_collection_groups_indexes_get(...)`, `databases_collection_groups_indexes_list(...)`, `databases_create(...)`, `databases_delete(...)`, `databases_documents_batch_get(...)`, `databases_documents_batch_write(...)`, `databases_documents_begin_transaction(...)`, `databases_documents_commit(...)`, `databases_documents_create_document(...)`, `databases_documents_delete(...)`, `databases_documents_get(...)`, `databases_documents_list(...)`, `databases_documents_list_collection_ids(...)`, `databases_documents_list_documents(...)`, `databases_documents_listen(...)`, `databases_documents_partition_query(...)`, `databases_documents_patch(...)`, `databases_documents_rollback(...)`, `databases_documents_run_aggregation_query(...)`, `databases_documents_run_query(...)`, `databases_documents_write(...)`, `databases_export_documents(...)`, `databases_get(...)`, `databases_import_documents(...)`, `databases_list(...)`, `databases_operations_cancel(...)`, `databases_operations_delete(...)`, `databases_operations_get(...)`, `databases_operations_list(...)`, `databases_patch(...)`, `locations_get(...)` and `locations_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Firestore<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the metadata and configuration for a Field.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_id}`
    pub fn databases_collection_groups_fields_get(&self, name: &str) -> ProjectDatabaseCollectionGroupFieldGetCall<'a, S> {
        ProjectDatabaseCollectionGroupFieldGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the field configuration and metadata for this database. Currently, FirestoreAdmin.ListFields only supports listing fields that have been explicitly overridden. To issue this query, call FirestoreAdmin.ListFields with the filter set to `indexConfig.usesAncestorConfig:false` .
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. A parent name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}`
    pub fn databases_collection_groups_fields_list(&self, parent: &str) -> ProjectDatabaseCollectionGroupFieldListCall<'a, S> {
        ProjectDatabaseCollectionGroupFieldListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a field configuration. Currently, field updates apply only to single field index configuration. However, calls to FirestoreAdmin.UpdateField should provide a field mask to avoid changing any configuration that the caller isn't aware of. The field mask should be specified as: `{ paths: "index_config" }`. This call returns a google.longrunning.Operation which may be used to track the status of the field update. The metadata for the operation will be the type FieldOperationMetadata. To configure the default field settings for the database, use the special `Field` with resource name: `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. A field name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_path}` A field path may be a simple field name, e.g. `address` or a path to fields within map_value , e.g. `address.city`, or a special field path. The only valid special field is `*`, which represents any field. Field paths may be quoted using ` (backtick). The only character that needs to be escaped within a quoted field path is the backtick character itself, escaped using a backslash. Special characters in field paths that must be quoted include: `*`, `.`, ``` (backtick), `[`, `]`, as well as any ascii symbolic characters. Examples: (Note: Comments here are written in markdown syntax, so there is an additional layer of backticks to represent a code block) `\`address.city\`` represents a field named `address.city`, not the map key `city` in the field `address`. `\`*\`` represents a field named `*`, not any field. A special `Field` contains the default indexing settings for all fields. This field's resource name is: `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*` Indexes defined on this `Field` will be applied to all fields which do not have their own `Field` index configuration.
    pub fn databases_collection_groups_fields_patch(&self, request: GoogleFirestoreAdminV1Field, name: &str) -> ProjectDatabaseCollectionGroupFieldPatchCall<'a, S> {
        ProjectDatabaseCollectionGroupFieldPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a composite index. This returns a google.longrunning.Operation which may be used to track the status of the creation. The metadata for the operation will be the type IndexOperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. A parent name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}`
    pub fn databases_collection_groups_indexes_create(&self, request: GoogleFirestoreAdminV1Index, parent: &str) -> ProjectDatabaseCollectionGroupIndexCreateCall<'a, S> {
        ProjectDatabaseCollectionGroupIndexCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a composite index.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/indexes/{index_id}`
    pub fn databases_collection_groups_indexes_delete(&self, name: &str) -> ProjectDatabaseCollectionGroupIndexDeleteCall<'a, S> {
        ProjectDatabaseCollectionGroupIndexDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a composite index.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/indexes/{index_id}`
    pub fn databases_collection_groups_indexes_get(&self, name: &str) -> ProjectDatabaseCollectionGroupIndexGetCall<'a, S> {
        ProjectDatabaseCollectionGroupIndexGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists composite indexes.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. A parent name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}`
    pub fn databases_collection_groups_indexes_list(&self, parent: &str) -> ProjectDatabaseCollectionGroupIndexListCall<'a, S> {
        ProjectDatabaseCollectionGroupIndexListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets multiple documents. Documents returned by this method are not guaranteed to be returned in the same order that they were requested.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `database` - Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`.
    pub fn databases_documents_batch_get(&self, request: BatchGetDocumentsRequest, database: &str) -> ProjectDatabaseDocumentBatchGetCall<'a, S> {
        ProjectDatabaseDocumentBatchGetCall {
            hub: self.hub,
            _request: request,
            _database: database.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Applies a batch of write operations. The BatchWrite method does not apply the write operations atomically and can apply them out of order. Method does not allow more than one write per document. Each write succeeds or fails independently. See the BatchWriteResponse for the success status of each write. If you require an atomically applied set of writes, use Commit instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `database` - Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`.
    pub fn databases_documents_batch_write(&self, request: BatchWriteRequest, database: &str) -> ProjectDatabaseDocumentBatchWriteCall<'a, S> {
        ProjectDatabaseDocumentBatchWriteCall {
            hub: self.hub,
            _request: request,
            _database: database.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts a new transaction.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `database` - Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`.
    pub fn databases_documents_begin_transaction(&self, request: BeginTransactionRequest, database: &str) -> ProjectDatabaseDocumentBeginTransactionCall<'a, S> {
        ProjectDatabaseDocumentBeginTransactionCall {
            hub: self.hub,
            _request: request,
            _database: database.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Commits a transaction, while optionally updating documents.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `database` - Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`.
    pub fn databases_documents_commit(&self, request: CommitRequest, database: &str) -> ProjectDatabaseDocumentCommitCall<'a, S> {
        ProjectDatabaseDocumentCommitCall {
            hub: self.hub,
            _request: request,
            _database: database.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new document.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource. For example: `projects/{project_id}/databases/{database_id}/documents` or `projects/{project_id}/databases/{database_id}/documents/chatrooms/{chatroom_id}`
    /// * `collectionId` - Required. The collection ID, relative to `parent`, to list. For example: `chatrooms`.
    pub fn databases_documents_create_document(&self, request: Document, parent: &str, collection_id: &str) -> ProjectDatabaseDocumentCreateDocumentCall<'a, S> {
        ProjectDatabaseDocumentCreateDocumentCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _collection_id: collection_id.to_string(),
            _mask_field_paths: Default::default(),
            _document_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a document.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Document to delete. In the format: `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    pub fn databases_documents_delete(&self, name: &str) -> ProjectDatabaseDocumentDeleteCall<'a, S> {
        ProjectDatabaseDocumentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _current_document_update_time: Default::default(),
            _current_document_exists: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single document.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Document to get. In the format: `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    pub fn databases_documents_get(&self, name: &str) -> ProjectDatabaseDocumentGetCall<'a, S> {
        ProjectDatabaseDocumentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _transaction: Default::default(),
            _read_time: Default::default(),
            _mask_field_paths: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists documents.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name. In the format: `projects/{project_id}/databases/{database_id}/documents` or `projects/{project_id}/databases/{database_id}/documents/{document_path}`. For example: `projects/my-project/databases/my-database/documents` or `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`
    /// * `collectionId` - Optional. The collection ID, relative to `parent`, to list. For example: `chatrooms` or `messages`. This is optional, and when not provided, Firestore will list documents from all collections under the provided `parent`.
    pub fn databases_documents_list(&self, parent: &str, collection_id: &str) -> ProjectDatabaseDocumentListCall<'a, S> {
        ProjectDatabaseDocumentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _collection_id: collection_id.to_string(),
            _transaction: Default::default(),
            _show_missing: Default::default(),
            _read_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _mask_field_paths: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the collection IDs underneath a document.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent document. In the format: `projects/{project_id}/databases/{database_id}/documents/{document_path}`. For example: `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`
    pub fn databases_documents_list_collection_ids(&self, request: ListCollectionIdsRequest, parent: &str) -> ProjectDatabaseDocumentListCollectionIdCall<'a, S> {
        ProjectDatabaseDocumentListCollectionIdCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists documents.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name. In the format: `projects/{project_id}/databases/{database_id}/documents` or `projects/{project_id}/databases/{database_id}/documents/{document_path}`. For example: `projects/my-project/databases/my-database/documents` or `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`
    /// * `collectionId` - Optional. The collection ID, relative to `parent`, to list. For example: `chatrooms` or `messages`. This is optional, and when not provided, Firestore will list documents from all collections under the provided `parent`.
    pub fn databases_documents_list_documents(&self, parent: &str, collection_id: &str) -> ProjectDatabaseDocumentListDocumentCall<'a, S> {
        ProjectDatabaseDocumentListDocumentCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _collection_id: collection_id.to_string(),
            _transaction: Default::default(),
            _show_missing: Default::default(),
            _read_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _mask_field_paths: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Listens to changes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `database` - Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`.
    pub fn databases_documents_listen(&self, request: ListenRequest, database: &str) -> ProjectDatabaseDocumentListenCall<'a, S> {
        ProjectDatabaseDocumentListenCall {
            hub: self.hub,
            _request: request,
            _database: database.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Partitions a query by returning partition cursors that can be used to run the query in parallel. The returned partition cursors are split points that can be used by RunQuery as starting/end points for the query results.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name. In the format: `projects/{project_id}/databases/{database_id}/documents`. Document resource names are not supported; only database resource names can be specified.
    pub fn databases_documents_partition_query(&self, request: PartitionQueryRequest, parent: &str) -> ProjectDatabaseDocumentPartitionQueryCall<'a, S> {
        ProjectDatabaseDocumentPartitionQueryCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates or inserts a document.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the document, for example `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    pub fn databases_documents_patch(&self, request: Document, name: &str) -> ProjectDatabaseDocumentPatchCall<'a, S> {
        ProjectDatabaseDocumentPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask_field_paths: Default::default(),
            _mask_field_paths: Default::default(),
            _current_document_update_time: Default::default(),
            _current_document_exists: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rolls back a transaction.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `database` - Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`.
    pub fn databases_documents_rollback(&self, request: RollbackRequest, database: &str) -> ProjectDatabaseDocumentRollbackCall<'a, S> {
        ProjectDatabaseDocumentRollbackCall {
            hub: self.hub,
            _request: request,
            _database: database.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs an aggregation query. Rather than producing Document results like Firestore.RunQuery, this API allows running an aggregation to produce a series of AggregationResult server-side. High-Level Example: ``` -- Return the number of documents in table given a filter. SELECT COUNT(*) FROM ( SELECT * FROM k where a = true ); ```
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name. In the format: `projects/{project_id}/databases/{database_id}/documents` or `projects/{project_id}/databases/{database_id}/documents/{document_path}`. For example: `projects/my-project/databases/my-database/documents` or `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`
    pub fn databases_documents_run_aggregation_query(&self, request: RunAggregationQueryRequest, parent: &str) -> ProjectDatabaseDocumentRunAggregationQueryCall<'a, S> {
        ProjectDatabaseDocumentRunAggregationQueryCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs a query.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name. In the format: `projects/{project_id}/databases/{database_id}/documents` or `projects/{project_id}/databases/{database_id}/documents/{document_path}`. For example: `projects/my-project/databases/my-database/documents` or `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`
    pub fn databases_documents_run_query(&self, request: RunQueryRequest, parent: &str) -> ProjectDatabaseDocumentRunQueryCall<'a, S> {
        ProjectDatabaseDocumentRunQueryCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Streams batches of document updates and deletes, in order.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `database` - Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`. This is only required in the first message.
    pub fn databases_documents_write(&self, request: WriteRequest, database: &str) -> ProjectDatabaseDocumentWriteCall<'a, S> {
        ProjectDatabaseDocumentWriteCall {
            hub: self.hub,
            _request: request,
            _database: database.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn databases_operations_cancel(&self, request: GoogleLongrunningCancelOperationRequest, name: &str) -> ProjectDatabaseOperationCancelCall<'a, S> {
        ProjectDatabaseOperationCancelCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn databases_operations_delete(&self, name: &str) -> ProjectDatabaseOperationDeleteCall<'a, S> {
        ProjectDatabaseOperationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn databases_operations_get(&self, name: &str) -> ProjectDatabaseOperationGetCall<'a, S> {
        ProjectDatabaseOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn databases_operations_list(&self, name: &str) -> ProjectDatabaseOperationListCall<'a, S> {
        ProjectDatabaseOperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a database.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. A parent name of the form `projects/{project_id}`
    pub fn databases_create(&self, request: GoogleFirestoreAdminV1Database, parent: &str) -> ProjectDatabaseCreateCall<'a, S> {
        ProjectDatabaseCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _database_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a database.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the form `projects/{project_id}/databases/{database_id}`
    pub fn databases_delete(&self, name: &str) -> ProjectDatabaseDeleteCall<'a, S> {
        ProjectDatabaseDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _free_id: Default::default(),
            _etag: Default::default(),
            _allow_missing: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports a copy of all or a subset of documents from Google Cloud Firestore to another storage system, such as Google Cloud Storage. Recent updates to documents may not be reflected in the export. The export occurs in the background and its progress can be monitored and managed via the Operation resource that is created. The output of an export may only be used once the associated operation is done. If an export operation is cancelled before completion it may leave partial data behind in Google Cloud Storage. For more details on export behavior and output format, refer to: https://cloud.google.com/firestore/docs/manage-data/export-import
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Database to export. Should be of the form: `projects/{project_id}/databases/{database_id}`.
    pub fn databases_export_documents(&self, request: GoogleFirestoreAdminV1ExportDocumentsRequest, name: &str) -> ProjectDatabaseExportDocumentCall<'a, S> {
        ProjectDatabaseExportDocumentCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a database.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the form `projects/{project_id}/databases/{database_id}`
    pub fn databases_get(&self, name: &str) -> ProjectDatabaseGetCall<'a, S> {
        ProjectDatabaseGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Imports documents into Google Cloud Firestore. Existing documents with the same name are overwritten. The import occurs in the background and its progress can be monitored and managed via the Operation resource that is created. If an ImportDocuments operation is cancelled, it is possible that a subset of the data has already been imported to Cloud Firestore.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Database to import into. Should be of the form: `projects/{project_id}/databases/{database_id}`.
    pub fn databases_import_documents(&self, request: GoogleFirestoreAdminV1ImportDocumentsRequest, name: &str) -> ProjectDatabaseImportDocumentCall<'a, S> {
        ProjectDatabaseImportDocumentCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all the databases in the project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. A parent name of the form `projects/{project_id}`
    pub fn databases_list(&self, parent: &str) -> ProjectDatabaseListCall<'a, S> {
        ProjectDatabaseListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a database.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the Database. Format: `projects/{project}/databases/{database}`
    pub fn databases_patch(&self, request: GoogleFirestoreAdminV1Database, name: &str) -> ProjectDatabasePatchCall<'a, S> {
        ProjectDatabasePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Resource name for the location.
    pub fn locations_get(&self, name: &str) -> ProjectLocationGetCall<'a, S> {
        ProjectLocationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists information about the supported locations for this service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource that owns the locations collection, if applicable.
    pub fn locations_list(&self, name: &str) -> ProjectLocationListCall<'a, S> {
        ProjectLocationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



