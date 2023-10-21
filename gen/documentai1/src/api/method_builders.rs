use super::*;
/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`Document`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_documentai1 as documentai1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use documentai1::{Document, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Document::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Document<S>,
}

impl<'a, S> client::MethodsBuilder for OperationMethods<'a, S> {}

impl<'a, S> OperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn delete(&self, name: &str) -> OperationDeleteCall<'a, S> {
        OperationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Document`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_documentai1 as documentai1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use documentai1::{Document, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Document::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_fetch_processor_types(...)`, `locations_get(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_processor_types_get(...)`, `locations_processor_types_list(...)`, `locations_processors_batch_process(...)`, `locations_processors_create(...)`, `locations_processors_delete(...)`, `locations_processors_disable(...)`, `locations_processors_enable(...)`, `locations_processors_get(...)`, `locations_processors_human_review_config_review_document(...)`, `locations_processors_list(...)`, `locations_processors_process(...)`, `locations_processors_processor_versions_batch_process(...)`, `locations_processors_processor_versions_delete(...)`, `locations_processors_processor_versions_deploy(...)`, `locations_processors_processor_versions_evaluate_processor_version(...)`, `locations_processors_processor_versions_evaluations_get(...)`, `locations_processors_processor_versions_evaluations_list(...)`, `locations_processors_processor_versions_get(...)`, `locations_processors_processor_versions_list(...)`, `locations_processors_processor_versions_process(...)`, `locations_processors_processor_versions_train(...)`, `locations_processors_processor_versions_undeploy(...)`, `locations_processors_set_default_processor_version(...)` and `operations_get(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Document<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn locations_operations_cancel(&self, name: &str) -> ProjectLocationOperationCancelCall<'a, S> {
        ProjectLocationOperationCancelCall {
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
    /// Gets a processor type detail.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The processor type resource name.
    pub fn locations_processor_types_get(&self, name: &str) -> ProjectLocationProcessorTypeGetCall<'a, S> {
        ProjectLocationProcessorTypeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the processor types that exist.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The location of processor type to list. The available processor types may depend on the allow-listing on projects. Format: `projects/{project}/locations/{location}`
    pub fn locations_processor_types_list(&self, parent: &str) -> ProjectLocationProcessorTypeListCall<'a, S> {
        ProjectLocationProcessorTypeListCall {
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
    /// Send a document for Human Review. The input document should be processed by the specified processor.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `humanReviewConfig` - Required. The resource name of the HumanReviewConfig that the document will be reviewed with.
    pub fn locations_processors_human_review_config_review_document(&self, request: GoogleCloudDocumentaiV1ReviewDocumentRequest, human_review_config: &str) -> ProjectLocationProcessorHumanReviewConfigReviewDocumentCall<'a, S> {
        ProjectLocationProcessorHumanReviewConfigReviewDocumentCall {
            hub: self.hub,
            _request: request,
            _human_review_config: human_review_config.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a specific evaluation.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Evaluation to get. `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}/evaluations/{evaluation}`
    pub fn locations_processors_processor_versions_evaluations_get(&self, name: &str) -> ProjectLocationProcessorProcessorVersionEvaluationGetCall<'a, S> {
        ProjectLocationProcessorProcessorVersionEvaluationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a set of evaluations for a given processor version.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the ProcessorVersion to list evaluations for. `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`
    pub fn locations_processors_processor_versions_evaluations_list(&self, parent: &str) -> ProjectLocationProcessorProcessorVersionEvaluationListCall<'a, S> {
        ProjectLocationProcessorProcessorVersionEvaluationListCall {
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
    /// LRO endpoint to batch process many documents. The output is written to Cloud Storage as JSON in the [Document] format.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of Processor or ProcessorVersion. Format: `projects/{project}/locations/{location}/processors/{processor}`, or `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`
    pub fn locations_processors_processor_versions_batch_process(&self, request: GoogleCloudDocumentaiV1BatchProcessRequest, name: &str) -> ProjectLocationProcessorProcessorVersionBatchProcesCall<'a, S> {
        ProjectLocationProcessorProcessorVersionBatchProcesCall {
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
    /// Deletes the processor version, all artifacts under the processor version will be deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The processor version resource name to be deleted.
    pub fn locations_processors_processor_versions_delete(&self, name: &str) -> ProjectLocationProcessorProcessorVersionDeleteCall<'a, S> {
        ProjectLocationProcessorProcessorVersionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deploys the processor version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The processor version resource name to be deployed.
    pub fn locations_processors_processor_versions_deploy(&self, request: GoogleCloudDocumentaiV1DeployProcessorVersionRequest, name: &str) -> ProjectLocationProcessorProcessorVersionDeployCall<'a, S> {
        ProjectLocationProcessorProcessorVersionDeployCall {
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
    /// Evaluates a ProcessorVersion against annotated documents, producing an Evaluation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `processorVersion` - Required. The resource name of the ProcessorVersion to evaluate. `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`
    pub fn locations_processors_processor_versions_evaluate_processor_version(&self, request: GoogleCloudDocumentaiV1EvaluateProcessorVersionRequest, processor_version: &str) -> ProjectLocationProcessorProcessorVersionEvaluateProcessorVersionCall<'a, S> {
        ProjectLocationProcessorProcessorVersionEvaluateProcessorVersionCall {
            hub: self.hub,
            _request: request,
            _processor_version: processor_version.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a processor version detail.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The processor resource name.
    pub fn locations_processors_processor_versions_get(&self, name: &str) -> ProjectLocationProcessorProcessorVersionGetCall<'a, S> {
        ProjectLocationProcessorProcessorVersionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all versions of a processor.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent (project, location and processor) to list all versions. Format: `projects/{project}/locations/{location}/processors/{processor}`
    pub fn locations_processors_processor_versions_list(&self, parent: &str) -> ProjectLocationProcessorProcessorVersionListCall<'a, S> {
        ProjectLocationProcessorProcessorVersionListCall {
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
    /// Processes a single document.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the Processor or ProcessorVersion to use for processing. If a Processor is specified, the server will use its default version. Format: `projects/{project}/locations/{location}/processors/{processor}`, or `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`
    pub fn locations_processors_processor_versions_process(&self, request: GoogleCloudDocumentaiV1ProcessRequest, name: &str) -> ProjectLocationProcessorProcessorVersionProcesCall<'a, S> {
        ProjectLocationProcessorProcessorVersionProcesCall {
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
    /// Trains a new processor version. Operation metadata is returned as cloud_documentai_core.TrainProcessorVersionMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent (project, location and processor) to create the new version for. Format: `projects/{project}/locations/{location}/processors/{processor}`.
    pub fn locations_processors_processor_versions_train(&self, request: GoogleCloudDocumentaiV1TrainProcessorVersionRequest, parent: &str) -> ProjectLocationProcessorProcessorVersionTrainCall<'a, S> {
        ProjectLocationProcessorProcessorVersionTrainCall {
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
    /// Undeploys the processor version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The processor version resource name to be undeployed.
    pub fn locations_processors_processor_versions_undeploy(&self, request: GoogleCloudDocumentaiV1UndeployProcessorVersionRequest, name: &str) -> ProjectLocationProcessorProcessorVersionUndeployCall<'a, S> {
        ProjectLocationProcessorProcessorVersionUndeployCall {
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
    /// LRO endpoint to batch process many documents. The output is written to Cloud Storage as JSON in the [Document] format.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of Processor or ProcessorVersion. Format: `projects/{project}/locations/{location}/processors/{processor}`, or `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`
    pub fn locations_processors_batch_process(&self, request: GoogleCloudDocumentaiV1BatchProcessRequest, name: &str) -> ProjectLocationProcessorBatchProcesCall<'a, S> {
        ProjectLocationProcessorBatchProcesCall {
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
    /// Creates a processor from the type processor that the user chose. The processor will be at "ENABLED" state by default after its creation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent (project and location) under which to create the processor. Format: `projects/{project}/locations/{location}`
    pub fn locations_processors_create(&self, request: GoogleCloudDocumentaiV1Processor, parent: &str) -> ProjectLocationProcessorCreateCall<'a, S> {
        ProjectLocationProcessorCreateCall {
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
    /// Deletes the processor, unloads all deployed model artifacts if it was enabled and then deletes all artifacts associated with this processor.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The processor resource name to be deleted.
    pub fn locations_processors_delete(&self, name: &str) -> ProjectLocationProcessorDeleteCall<'a, S> {
        ProjectLocationProcessorDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Disables a processor
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The processor resource name to be disabled.
    pub fn locations_processors_disable(&self, request: GoogleCloudDocumentaiV1DisableProcessorRequest, name: &str) -> ProjectLocationProcessorDisableCall<'a, S> {
        ProjectLocationProcessorDisableCall {
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
    /// Enables a processor
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The processor resource name to be enabled.
    pub fn locations_processors_enable(&self, request: GoogleCloudDocumentaiV1EnableProcessorRequest, name: &str) -> ProjectLocationProcessorEnableCall<'a, S> {
        ProjectLocationProcessorEnableCall {
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
    /// Gets a processor detail.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The processor resource name.
    pub fn locations_processors_get(&self, name: &str) -> ProjectLocationProcessorGetCall<'a, S> {
        ProjectLocationProcessorGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all processors which belong to this project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent (project and location) which owns this collection of Processors. Format: `projects/{project}/locations/{location}`
    pub fn locations_processors_list(&self, parent: &str) -> ProjectLocationProcessorListCall<'a, S> {
        ProjectLocationProcessorListCall {
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
    /// Processes a single document.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the Processor or ProcessorVersion to use for processing. If a Processor is specified, the server will use its default version. Format: `projects/{project}/locations/{location}/processors/{processor}`, or `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`
    pub fn locations_processors_process(&self, request: GoogleCloudDocumentaiV1ProcessRequest, name: &str) -> ProjectLocationProcessorProcesCall<'a, S> {
        ProjectLocationProcessorProcesCall {
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
    /// Set the default (active) version of a Processor that will be used in ProcessDocument and BatchProcessDocuments.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `processor` - Required. The resource name of the Processor to change default version.
    pub fn locations_processors_set_default_processor_version(&self, request: GoogleCloudDocumentaiV1SetDefaultProcessorVersionRequest, processor: &str) -> ProjectLocationProcessorSetDefaultProcessorVersionCall<'a, S> {
        ProjectLocationProcessorSetDefaultProcessorVersionCall {
            hub: self.hub,
            _request: request,
            _processor: processor.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches processor types. Note that we do not use ListProcessorTypes here because it is not paginated.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project of processor type to list. The available processor types may depend on the allow-listing on projects. Format: `projects/{project}/locations/{location}`
    pub fn locations_fetch_processor_types(&self, parent: &str) -> ProjectLocationFetchProcessorTypeCall<'a, S> {
        ProjectLocationFetchProcessorTypeCall {
            hub: self.hub,
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
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn operations_get(&self, name: &str) -> ProjectOperationGetCall<'a, S> {
        ProjectOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



