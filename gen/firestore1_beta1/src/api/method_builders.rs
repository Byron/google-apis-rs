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
/// extern crate google_firestore1_beta1 as firestore1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use firestore1_beta1::{Firestore, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Firestore::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `databases_documents_batch_get(...)`, `databases_documents_batch_write(...)`, `databases_documents_begin_transaction(...)`, `databases_documents_commit(...)`, `databases_documents_create_document(...)`, `databases_documents_delete(...)`, `databases_documents_get(...)`, `databases_documents_list(...)`, `databases_documents_list_collection_ids(...)`, `databases_documents_list_documents(...)`, `databases_documents_listen(...)`, `databases_documents_partition_query(...)`, `databases_documents_patch(...)`, `databases_documents_rollback(...)`, `databases_documents_run_aggregation_query(...)`, `databases_documents_run_query(...)`, `databases_documents_write(...)`, `databases_export_documents(...)`, `databases_import_documents(...)`, `databases_indexes_create(...)`, `databases_indexes_delete(...)`, `databases_indexes_get(...)` and `databases_indexes_list(...)`
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
    /// Creates the specified index. A newly created index's initial state is `CREATING`. On completion of the returned google.longrunning.Operation, the state will be `READY`. If the index already exists, the call will return an `ALREADY_EXISTS` status. During creation, the process could result in an error, in which case the index will move to the `ERROR` state. The process can be recovered by fixing the data that caused the error, removing the index with delete, then re-creating the index with create. Indexes with a single field cannot be created.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the database this index will apply to. For example: `projects/{project_id}/databases/{database_id}`
    pub fn databases_indexes_create(&self, request: GoogleFirestoreAdminV1beta1Index, parent: &str) -> ProjectDatabaseIndexCreateCall<'a, S> {
        ProjectDatabaseIndexCreateCall {
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
    /// Deletes an index.
    /// 
    /// # Arguments
    ///
    /// * `name` - The index name. For example: `projects/{project_id}/databases/{database_id}/indexes/{index_id}`
    pub fn databases_indexes_delete(&self, name: &str) -> ProjectDatabaseIndexDeleteCall<'a, S> {
        ProjectDatabaseIndexDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an index.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the index. For example: `projects/{project_id}/databases/{database_id}/indexes/{index_id}`
    pub fn databases_indexes_get(&self, name: &str) -> ProjectDatabaseIndexGetCall<'a, S> {
        ProjectDatabaseIndexGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the indexes that match the specified filters.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The database name. For example: `projects/{project_id}/databases/{database_id}`
    pub fn databases_indexes_list(&self, parent: &str) -> ProjectDatabaseIndexListCall<'a, S> {
        ProjectDatabaseIndexListCall {
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
    /// Exports a copy of all or a subset of documents from Google Cloud Firestore to another storage system, such as Google Cloud Storage. Recent updates to documents may not be reflected in the export. The export occurs in the background and its progress can be monitored and managed via the Operation resource that is created. The output of an export may only be used once the associated operation is done. If an export operation is cancelled before completion it may leave partial data behind in Google Cloud Storage.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Database to export. Should be of the form: `projects/{project_id}/databases/{database_id}`.
    pub fn databases_export_documents(&self, request: GoogleFirestoreAdminV1beta1ExportDocumentsRequest, name: &str) -> ProjectDatabaseExportDocumentCall<'a, S> {
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
    /// Imports documents into Google Cloud Firestore. Existing documents with the same name are overwritten. The import occurs in the background and its progress can be monitored and managed via the Operation resource that is created. If an ImportDocuments operation is cancelled, it is possible that a subset of the data has already been imported to Cloud Firestore.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Database to import into. Should be of the form: `projects/{project_id}/databases/{database_id}`.
    pub fn databases_import_documents(&self, request: GoogleFirestoreAdminV1beta1ImportDocumentsRequest, name: &str) -> ProjectDatabaseImportDocumentCall<'a, S> {
        ProjectDatabaseImportDocumentCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



