use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Translate`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_translate3 as translate3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use translate3::{Translate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Translate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `detect_language(...)`, `get_supported_languages(...)`, `locations_batch_translate_document(...)`, `locations_batch_translate_text(...)`, `locations_datasets_create(...)`, `locations_datasets_delete(...)`, `locations_datasets_examples_list(...)`, `locations_datasets_export_data(...)`, `locations_datasets_get(...)`, `locations_datasets_import_data(...)`, `locations_datasets_list(...)`, `locations_detect_language(...)`, `locations_get(...)`, `locations_get_supported_languages(...)`, `locations_glossaries_create(...)`, `locations_glossaries_delete(...)`, `locations_glossaries_get(...)`, `locations_glossaries_glossary_entries_create(...)`, `locations_glossaries_glossary_entries_delete(...)`, `locations_glossaries_glossary_entries_get(...)`, `locations_glossaries_glossary_entries_list(...)`, `locations_glossaries_glossary_entries_patch(...)`, `locations_glossaries_list(...)`, `locations_glossaries_patch(...)`, `locations_list(...)`, `locations_models_create(...)`, `locations_models_delete(...)`, `locations_models_get(...)`, `locations_models_list(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_operations_wait(...)`, `locations_translate_document(...)`, `locations_translate_text(...)` and `translate_text(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Translate<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists sentence pairs in the dataset.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent dataset. In form of `projects/{project-number-or-id}/locations/{location-id}/datasets/{dataset-id}`
    pub fn locations_datasets_examples_list(&self, parent: &str) -> ProjectLocationDatasetExampleListCall<'a, S> {
        ProjectLocationDatasetExampleListCall {
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
    /// Creates a Dataset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project name.
    pub fn locations_datasets_create(&self, request: Dataset, parent: &str) -> ProjectLocationDatasetCreateCall<'a, S> {
        ProjectLocationDatasetCreateCall {
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
    /// Deletes a dataset and all of its contents.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the dataset to delete.
    pub fn locations_datasets_delete(&self, name: &str) -> ProjectLocationDatasetDeleteCall<'a, S> {
        ProjectLocationDatasetDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports dataset's data to the provided output location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `dataset` - Required. Name of the dataset. In form of `projects/{project-number-or-id}/locations/{location-id}/datasets/{dataset-id}`
    pub fn locations_datasets_export_data(&self, request: ExportDataRequest, dataset: &str) -> ProjectLocationDatasetExportDataCall<'a, S> {
        ProjectLocationDatasetExportDataCall {
            hub: self.hub,
            _request: request,
            _dataset: dataset.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a Dataset.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the dataset to retrieve.
    pub fn locations_datasets_get(&self, name: &str) -> ProjectLocationDatasetGetCall<'a, S> {
        ProjectLocationDatasetGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Import sentence pairs into translation Dataset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `dataset` - Required. Name of the dataset. In form of `projects/{project-number-or-id}/locations/{location-id}/datasets/{dataset-id}`
    pub fn locations_datasets_import_data(&self, request: ImportDataRequest, dataset: &str) -> ProjectLocationDatasetImportDataCall<'a, S> {
        ProjectLocationDatasetImportDataCall {
            hub: self.hub,
            _request: request,
            _dataset: dataset.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists datasets.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent project. In form of `projects/{project-number-or-id}/locations/{location-id}`
    pub fn locations_datasets_list(&self, parent: &str) -> ProjectLocationDatasetListCall<'a, S> {
        ProjectLocationDatasetListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a glossary entry.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the glossary to create the entry under.
    pub fn locations_glossaries_glossary_entries_create(&self, request: GlossaryEntry, parent: &str) -> ProjectLocationGlossaryGlossaryEntryCreateCall<'a, S> {
        ProjectLocationGlossaryGlossaryEntryCreateCall {
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
    /// Deletes a single entry from the glossary
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the glossary entry to delete
    pub fn locations_glossaries_glossary_entries_delete(&self, name: &str) -> ProjectLocationGlossaryGlossaryEntryDeleteCall<'a, S> {
        ProjectLocationGlossaryGlossaryEntryDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single glossary entry by the given id.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the glossary entry to get
    pub fn locations_glossaries_glossary_entries_get(&self, name: &str) -> ProjectLocationGlossaryGlossaryEntryGetCall<'a, S> {
        ProjectLocationGlossaryGlossaryEntryGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the entries for the glossary.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent glossary resource name for listing the glossary's entries.
    pub fn locations_glossaries_glossary_entries_list(&self, parent: &str) -> ProjectLocationGlossaryGlossaryEntryListCall<'a, S> {
        ProjectLocationGlossaryGlossaryEntryListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a glossary entry.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the entry. Format: "projects/*/locations/*/glossaries/*/glossaryEntries/*"
    pub fn locations_glossaries_glossary_entries_patch(&self, request: GlossaryEntry, name: &str) -> ProjectLocationGlossaryGlossaryEntryPatchCall<'a, S> {
        ProjectLocationGlossaryGlossaryEntryPatchCall {
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
    /// Creates a glossary and returns the long-running operation. Returns NOT_FOUND, if the project doesn't exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project name.
    pub fn locations_glossaries_create(&self, request: Glossary, parent: &str) -> ProjectLocationGlossaryCreateCall<'a, S> {
        ProjectLocationGlossaryCreateCall {
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
    /// Deletes a glossary, or cancels glossary construction if the glossary isn't created yet. Returns NOT_FOUND, if the glossary doesn't exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the glossary to delete.
    pub fn locations_glossaries_delete(&self, name: &str) -> ProjectLocationGlossaryDeleteCall<'a, S> {
        ProjectLocationGlossaryDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a glossary. Returns NOT_FOUND, if the glossary doesn't exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the glossary to retrieve.
    pub fn locations_glossaries_get(&self, name: &str) -> ProjectLocationGlossaryGetCall<'a, S> {
        ProjectLocationGlossaryGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists glossaries in a project. Returns NOT_FOUND, if the project doesn't exist.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the project from which to list all of the glossaries.
    pub fn locations_glossaries_list(&self, parent: &str) -> ProjectLocationGlossaryListCall<'a, S> {
        ProjectLocationGlossaryListCall {
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
    /// Updates a glossary. A LRO is used since the update can be async if the glossary's entry file is updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the glossary. Glossary names have the form `projects/{project-number-or-id}/locations/{location-id}/glossaries/{glossary-id}`.
    pub fn locations_glossaries_patch(&self, request: Glossary, name: &str) -> ProjectLocationGlossaryPatchCall<'a, S> {
        ProjectLocationGlossaryPatchCall {
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
    /// Creates a Model.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project name, in form of `projects/{project}/locations/{location}`
    pub fn locations_models_create(&self, request: Model, parent: &str) -> ProjectLocationModelCreateCall<'a, S> {
        ProjectLocationModelCreateCall {
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
    /// Deletes a model.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the model to delete.
    pub fn locations_models_delete(&self, name: &str) -> ProjectLocationModelDeleteCall<'a, S> {
        ProjectLocationModelDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a model.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the model to retrieve.
    pub fn locations_models_get(&self, name: &str) -> ProjectLocationModelGetCall<'a, S> {
        ProjectLocationModelGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists models.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent project. In form of `projects/{project-number-or-id}/locations/{location-id}`
    pub fn locations_models_list(&self, parent: &str) -> ProjectLocationModelListCall<'a, S> {
        ProjectLocationModelListCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn locations_operations_cancel(&self, request: CancelOperationRequest, name: &str) -> ProjectLocationOperationCancelCall<'a, S> {
        ProjectLocationOperationCancelCall {
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
    pub fn locations_operations_delete(&self, name: &str) -> ProjectLocationOperationDeleteCall<'a, S> {
        ProjectLocationOperationDeleteCall {
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
    pub fn locations_operations_get(&self, name: &str) -> ProjectLocationOperationGetCall<'a, S> {
        ProjectLocationOperationGetCall {
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
    pub fn locations_operations_list(&self, name: &str) -> ProjectLocationOperationListCall<'a, S> {
        ProjectLocationOperationListCall {
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
    /// Waits until the specified long-running operation is done or reaches at most a specified timeout, returning the latest state. If the operation is already done, the latest state is immediately returned. If the timeout specified is greater than the default HTTP/RPC timeout, the HTTP/RPC timeout is used. If the server does not support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Note that this method is on a best-effort basis. It may return the latest state before the specified timeout (including immediately), meaning even an immediate response is no guarantee that the operation is done.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to wait on.
    pub fn locations_operations_wait(&self, request: WaitOperationRequest, name: &str) -> ProjectLocationOperationWaitCall<'a, S> {
        ProjectLocationOperationWaitCall {
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
    /// Translates a large volume of document in asynchronous batch mode. This function provides real-time output as the inputs are being processed. If caller cancels a request, the partial results (for an input file, it's all or nothing) may still be available on the specified output location. This call returns immediately and you can use google.longrunning.Operation.name to poll the status of the call.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Location to make a regional call. Format: `projects/{project-number-or-id}/locations/{location-id}`. The `global` location is not supported for batch translation. Only AutoML Translation models or glossaries within the same region (have the same location-id) can be used, otherwise an INVALID_ARGUMENT (400) error is returned.
    pub fn locations_batch_translate_document(&self, request: BatchTranslateDocumentRequest, parent: &str) -> ProjectLocationBatchTranslateDocumentCall<'a, S> {
        ProjectLocationBatchTranslateDocumentCall {
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
    /// Translates a large volume of text in asynchronous batch mode. This function provides real-time output as the inputs are being processed. If caller cancels a request, the partial results (for an input file, it's all or nothing) may still be available on the specified output location. This call returns immediately and you can use google.longrunning.Operation.name to poll the status of the call.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Location to make a call. Must refer to a caller's project. Format: `projects/{project-number-or-id}/locations/{location-id}`. The `global` location is not supported for batch translation. Only AutoML Translation models or glossaries within the same region (have the same location-id) can be used, otherwise an INVALID_ARGUMENT (400) error is returned.
    pub fn locations_batch_translate_text(&self, request: BatchTranslateTextRequest, parent: &str) -> ProjectLocationBatchTranslateTextCall<'a, S> {
        ProjectLocationBatchTranslateTextCall {
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
    /// Detects the language of text within a request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Project or location to make a call. Must refer to a caller's project. Format: `projects/{project-number-or-id}/locations/{location-id}` or `projects/{project-number-or-id}`. For global calls, use `projects/{project-number-or-id}/locations/global` or `projects/{project-number-or-id}`. Only models within the same region (has same location-id) can be used. Otherwise an INVALID_ARGUMENT (400) error is returned.
    pub fn locations_detect_language(&self, request: DetectLanguageRequest, parent: &str) -> ProjectLocationDetectLanguageCall<'a, S> {
        ProjectLocationDetectLanguageCall {
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
    /// Returns a list of supported languages for translation.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Project or location to make a call. Must refer to a caller's project. Format: `projects/{project-number-or-id}` or `projects/{project-number-or-id}/locations/{location-id}`. For global calls, use `projects/{project-number-or-id}/locations/global` or `projects/{project-number-or-id}`. Non-global location is required for AutoML models. Only models within the same region (have same location-id) can be used, otherwise an INVALID_ARGUMENT (400) error is returned.
    pub fn locations_get_supported_languages(&self, parent: &str) -> ProjectLocationGetSupportedLanguageCall<'a, S> {
        ProjectLocationGetSupportedLanguageCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _model: Default::default(),
            _display_language_code: Default::default(),
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Translates documents in synchronous mode.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Location to make a regional call. Format: `projects/{project-number-or-id}/locations/{location-id}`. For global calls, use `projects/{project-number-or-id}/locations/global` or `projects/{project-number-or-id}`. Non-global location is required for requests using AutoML models or custom glossaries. Models and glossaries must be within the same region (have the same location-id), otherwise an INVALID_ARGUMENT (400) error is returned.
    pub fn locations_translate_document(&self, request: TranslateDocumentRequest, parent: &str) -> ProjectLocationTranslateDocumentCall<'a, S> {
        ProjectLocationTranslateDocumentCall {
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
    /// Translates input text and returns translated text.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Project or location to make a call. Must refer to a caller's project. Format: `projects/{project-number-or-id}` or `projects/{project-number-or-id}/locations/{location-id}`. For global calls, use `projects/{project-number-or-id}/locations/global` or `projects/{project-number-or-id}`. Non-global location is required for requests using AutoML models or custom glossaries. Models and glossaries must be within the same region (have same location-id), otherwise an INVALID_ARGUMENT (400) error is returned.
    pub fn locations_translate_text(&self, request: TranslateTextRequest, parent: &str) -> ProjectLocationTranslateTextCall<'a, S> {
        ProjectLocationTranslateTextCall {
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
    /// Detects the language of text within a request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Project or location to make a call. Must refer to a caller's project. Format: `projects/{project-number-or-id}/locations/{location-id}` or `projects/{project-number-or-id}`. For global calls, use `projects/{project-number-or-id}/locations/global` or `projects/{project-number-or-id}`. Only models within the same region (has same location-id) can be used. Otherwise an INVALID_ARGUMENT (400) error is returned.
    pub fn detect_language(&self, request: DetectLanguageRequest, parent: &str) -> ProjectDetectLanguageCall<'a, S> {
        ProjectDetectLanguageCall {
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
    /// Returns a list of supported languages for translation.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Project or location to make a call. Must refer to a caller's project. Format: `projects/{project-number-or-id}` or `projects/{project-number-or-id}/locations/{location-id}`. For global calls, use `projects/{project-number-or-id}/locations/global` or `projects/{project-number-or-id}`. Non-global location is required for AutoML models. Only models within the same region (have same location-id) can be used, otherwise an INVALID_ARGUMENT (400) error is returned.
    pub fn get_supported_languages(&self, parent: &str) -> ProjectGetSupportedLanguageCall<'a, S> {
        ProjectGetSupportedLanguageCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _model: Default::default(),
            _display_language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Translates input text and returns translated text.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Project or location to make a call. Must refer to a caller's project. Format: `projects/{project-number-or-id}` or `projects/{project-number-or-id}/locations/{location-id}`. For global calls, use `projects/{project-number-or-id}/locations/global` or `projects/{project-number-or-id}`. Non-global location is required for requests using AutoML models or custom glossaries. Models and glossaries must be within the same region (have same location-id), otherwise an INVALID_ARGUMENT (400) error is returned.
    pub fn translate_text(&self, request: TranslateTextRequest, parent: &str) -> ProjectTranslateTextCall<'a, S> {
        ProjectTranslateTextCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



