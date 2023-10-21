use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`DataLabeling`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_datalabeling1_beta1 as datalabeling1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use datalabeling1_beta1::{DataLabeling, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DataLabeling::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `annotation_spec_sets_create(...)`, `annotation_spec_sets_delete(...)`, `annotation_spec_sets_get(...)`, `annotation_spec_sets_list(...)`, `datasets_annotated_datasets_data_items_get(...)`, `datasets_annotated_datasets_data_items_list(...)`, `datasets_annotated_datasets_delete(...)`, `datasets_annotated_datasets_examples_get(...)`, `datasets_annotated_datasets_examples_list(...)`, `datasets_annotated_datasets_feedback_threads_delete(...)`, `datasets_annotated_datasets_feedback_threads_feedback_messages_create(...)`, `datasets_annotated_datasets_feedback_threads_feedback_messages_delete(...)`, `datasets_annotated_datasets_feedback_threads_feedback_messages_get(...)`, `datasets_annotated_datasets_feedback_threads_feedback_messages_list(...)`, `datasets_annotated_datasets_feedback_threads_get(...)`, `datasets_annotated_datasets_feedback_threads_list(...)`, `datasets_annotated_datasets_get(...)`, `datasets_annotated_datasets_list(...)`, `datasets_create(...)`, `datasets_data_items_get(...)`, `datasets_data_items_list(...)`, `datasets_delete(...)`, `datasets_evaluations_example_comparisons_search(...)`, `datasets_evaluations_get(...)`, `datasets_export_data(...)`, `datasets_get(...)`, `datasets_image_label(...)`, `datasets_import_data(...)`, `datasets_list(...)`, `datasets_text_label(...)`, `datasets_video_label(...)`, `evaluation_jobs_create(...)`, `evaluation_jobs_delete(...)`, `evaluation_jobs_get(...)`, `evaluation_jobs_list(...)`, `evaluation_jobs_patch(...)`, `evaluation_jobs_pause(...)`, `evaluation_jobs_resume(...)`, `evaluations_search(...)`, `instructions_create(...)`, `instructions_delete(...)`, `instructions_get(...)`, `instructions_list(...)`, `operations_cancel(...)`, `operations_delete(...)`, `operations_get(...)` and `operations_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DataLabeling<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an annotation spec set by providing a set of labels.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. AnnotationSpecSet resource parent, format: projects/{project_id}
    pub fn annotation_spec_sets_create(&self, request: GoogleCloudDatalabelingV1beta1CreateAnnotationSpecSetRequest, parent: &str) -> ProjectAnnotationSpecSetCreateCall<'a, S> {
        ProjectAnnotationSpecSetCreateCall {
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
    /// Deletes an annotation spec set by resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. AnnotationSpec resource name, format: `projects/{project_id}/annotationSpecSets/{annotation_spec_set_id}`.
    pub fn annotation_spec_sets_delete(&self, name: &str) -> ProjectAnnotationSpecSetDeleteCall<'a, S> {
        ProjectAnnotationSpecSetDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an annotation spec set by resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. AnnotationSpecSet resource name, format: projects/{project_id}/annotationSpecSets/{annotation_spec_set_id}
    pub fn annotation_spec_sets_get(&self, name: &str) -> ProjectAnnotationSpecSetGetCall<'a, S> {
        ProjectAnnotationSpecSetGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists annotation spec sets for a project. Pagination is supported.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent of AnnotationSpecSet resource, format: projects/{project_id}
    pub fn annotation_spec_sets_list(&self, parent: &str) -> ProjectAnnotationSpecSetListCall<'a, S> {
        ProjectAnnotationSpecSetListCall {
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
    /// Gets a data item in a dataset by resource name. This API can be called after data are imported into dataset.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the data item to get, format: projects/{project_id}/datasets/{dataset_id}/dataItems/{data_item_id}
    pub fn datasets_annotated_datasets_data_items_get(&self, name: &str) -> ProjectDatasetAnnotatedDatasetDataItemGetCall<'a, S> {
        ProjectDatasetAnnotatedDatasetDataItemGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists data items in a dataset. This API can be called after data are imported into dataset. Pagination is supported.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the dataset to list data items, format: projects/{project_id}/datasets/{dataset_id}
    pub fn datasets_annotated_datasets_data_items_list(&self, parent: &str) -> ProjectDatasetAnnotatedDatasetDataItemListCall<'a, S> {
        ProjectDatasetAnnotatedDatasetDataItemListCall {
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
    /// Gets an example by resource name, including both data and annotation.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of example, format: projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/ {annotated_dataset_id}/examples/{example_id}
    pub fn datasets_annotated_datasets_examples_get(&self, name: &str) -> ProjectDatasetAnnotatedDatasetExampleGetCall<'a, S> {
        ProjectDatasetAnnotatedDatasetExampleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists examples in an annotated dataset. Pagination is supported.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Example resource parent.
    pub fn datasets_annotated_datasets_examples_list(&self, parent: &str) -> ProjectDatasetAnnotatedDatasetExampleListCall<'a, S> {
        ProjectDatasetAnnotatedDatasetExampleListCall {
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
    /// Create a FeedbackMessage object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. FeedbackMessage resource parent, format: projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}/feedbackThreads/{feedback_thread_id}.
    pub fn datasets_annotated_datasets_feedback_threads_feedback_messages_create(&self, request: GoogleCloudDatalabelingV1beta1FeedbackMessage, parent: &str) -> ProjectDatasetAnnotatedDatasetFeedbackThreadFeedbackMessageCreateCall<'a, S> {
        ProjectDatasetAnnotatedDatasetFeedbackThreadFeedbackMessageCreateCall {
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
    /// Delete a FeedbackMessage.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the FeedbackMessage that is going to be deleted. Format: 'projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}/feedbackThreads/{feedback_thread_id}/feedbackMessages/{feedback_message_id}'.
    pub fn datasets_annotated_datasets_feedback_threads_feedback_messages_delete(&self, name: &str) -> ProjectDatasetAnnotatedDatasetFeedbackThreadFeedbackMessageDeleteCall<'a, S> {
        ProjectDatasetAnnotatedDatasetFeedbackThreadFeedbackMessageDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a FeedbackMessage object.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the feedback. Format: 'projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}/feedbackThreads/{feedback_thread_id}/feedbackMessages/{feedback_message_id}'.
    pub fn datasets_annotated_datasets_feedback_threads_feedback_messages_get(&self, name: &str) -> ProjectDatasetAnnotatedDatasetFeedbackThreadFeedbackMessageGetCall<'a, S> {
        ProjectDatasetAnnotatedDatasetFeedbackThreadFeedbackMessageGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List FeedbackMessages with pagination.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. FeedbackMessage resource parent. Format: "projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}/feedbackThreads/{feedback_thread_id}"
    pub fn datasets_annotated_datasets_feedback_threads_feedback_messages_list(&self, parent: &str) -> ProjectDatasetAnnotatedDatasetFeedbackThreadFeedbackMessageListCall<'a, S> {
        ProjectDatasetAnnotatedDatasetFeedbackThreadFeedbackMessageListCall {
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
    /// Delete a FeedbackThread.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the FeedbackThread that is going to be deleted. Format: 'projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}/feedbackThreads/{feedback_thread_id}'.
    pub fn datasets_annotated_datasets_feedback_threads_delete(&self, name: &str) -> ProjectDatasetAnnotatedDatasetFeedbackThreadDeleteCall<'a, S> {
        ProjectDatasetAnnotatedDatasetFeedbackThreadDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a FeedbackThread object.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the feedback. Format: 'projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}/feedbackThreads/{feedback_thread_id}'.
    pub fn datasets_annotated_datasets_feedback_threads_get(&self, name: &str) -> ProjectDatasetAnnotatedDatasetFeedbackThreadGetCall<'a, S> {
        ProjectDatasetAnnotatedDatasetFeedbackThreadGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List FeedbackThreads with pagination.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. FeedbackThread resource parent. Format: "projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}"
    pub fn datasets_annotated_datasets_feedback_threads_list(&self, parent: &str) -> ProjectDatasetAnnotatedDatasetFeedbackThreadListCall<'a, S> {
        ProjectDatasetAnnotatedDatasetFeedbackThreadListCall {
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
    /// Deletes an annotated dataset by resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the annotated dataset to delete, format: projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/ {annotated_dataset_id}
    pub fn datasets_annotated_datasets_delete(&self, name: &str) -> ProjectDatasetAnnotatedDatasetDeleteCall<'a, S> {
        ProjectDatasetAnnotatedDatasetDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an annotated dataset by resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the annotated dataset to get, format: projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/ {annotated_dataset_id}
    pub fn datasets_annotated_datasets_get(&self, name: &str) -> ProjectDatasetAnnotatedDatasetGetCall<'a, S> {
        ProjectDatasetAnnotatedDatasetGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists annotated datasets for a dataset. Pagination is supported.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the dataset to list annotated datasets, format: projects/{project_id}/datasets/{dataset_id}
    pub fn datasets_annotated_datasets_list(&self, parent: &str) -> ProjectDatasetAnnotatedDatasetListCall<'a, S> {
        ProjectDatasetAnnotatedDatasetListCall {
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
    /// Gets a data item in a dataset by resource name. This API can be called after data are imported into dataset.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the data item to get, format: projects/{project_id}/datasets/{dataset_id}/dataItems/{data_item_id}
    pub fn datasets_data_items_get(&self, name: &str) -> ProjectDatasetDataItemGetCall<'a, S> {
        ProjectDatasetDataItemGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists data items in a dataset. This API can be called after data are imported into dataset. Pagination is supported.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the dataset to list data items, format: projects/{project_id}/datasets/{dataset_id}
    pub fn datasets_data_items_list(&self, parent: &str) -> ProjectDatasetDataItemListCall<'a, S> {
        ProjectDatasetDataItemListCall {
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
    /// Searches example comparisons from an evaluation. The return format is a list of example comparisons that show ground truth and prediction(s) for a single input. Search by providing an evaluation ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the Evaluation resource to search for example comparisons from. Format: "projects/{project_id}/datasets/{dataset_id}/evaluations/ {evaluation_id}"
    pub fn datasets_evaluations_example_comparisons_search(&self, request: GoogleCloudDatalabelingV1beta1SearchExampleComparisonsRequest, parent: &str) -> ProjectDatasetEvaluationExampleComparisonSearchCall<'a, S> {
        ProjectDatasetEvaluationExampleComparisonSearchCall {
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
    /// Gets an evaluation by resource name (to search, use projects.evaluations.search).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the evaluation. Format: "projects/{project_id}/datasets/ {dataset_id}/evaluations/{evaluation_id}'
    pub fn datasets_evaluations_get(&self, name: &str) -> ProjectDatasetEvaluationGetCall<'a, S> {
        ProjectDatasetEvaluationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts a labeling task for image. The type of image labeling task is configured by feature in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the dataset to request labeling task, format: projects/{project_id}/datasets/{dataset_id}
    pub fn datasets_image_label(&self, request: GoogleCloudDatalabelingV1beta1LabelImageRequest, parent: &str) -> ProjectDatasetImageLabelCall<'a, S> {
        ProjectDatasetImageLabelCall {
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
    /// Starts a labeling task for text. The type of text labeling task is configured by feature in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the data set to request labeling task, format: projects/{project_id}/datasets/{dataset_id}
    pub fn datasets_text_label(&self, request: GoogleCloudDatalabelingV1beta1LabelTextRequest, parent: &str) -> ProjectDatasetTextLabelCall<'a, S> {
        ProjectDatasetTextLabelCall {
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
    /// Starts a labeling task for video. The type of video labeling task is configured by feature in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the dataset to request labeling task, format: projects/{project_id}/datasets/{dataset_id}
    pub fn datasets_video_label(&self, request: GoogleCloudDatalabelingV1beta1LabelVideoRequest, parent: &str) -> ProjectDatasetVideoLabelCall<'a, S> {
        ProjectDatasetVideoLabelCall {
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
    /// Creates dataset. If success return a Dataset resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Dataset resource parent, format: projects/{project_id}
    pub fn datasets_create(&self, request: GoogleCloudDatalabelingV1beta1CreateDatasetRequest, parent: &str) -> ProjectDatasetCreateCall<'a, S> {
        ProjectDatasetCreateCall {
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
    /// Deletes a dataset by resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Dataset resource name, format: projects/{project_id}/datasets/{dataset_id}
    pub fn datasets_delete(&self, name: &str) -> ProjectDatasetDeleteCall<'a, S> {
        ProjectDatasetDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports data and annotations from dataset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Dataset resource name, format: projects/{project_id}/datasets/{dataset_id}
    pub fn datasets_export_data(&self, request: GoogleCloudDatalabelingV1beta1ExportDataRequest, name: &str) -> ProjectDatasetExportDataCall<'a, S> {
        ProjectDatasetExportDataCall {
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
    /// Gets dataset by resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Dataset resource name, format: projects/{project_id}/datasets/{dataset_id}
    pub fn datasets_get(&self, name: &str) -> ProjectDatasetGetCall<'a, S> {
        ProjectDatasetGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Imports data into dataset based on source locations defined in request. It can be called multiple times for the same dataset. Each dataset can only have one long running operation running on it. For example, no labeling task (also long running operation) can be started while importing is still ongoing. Vice versa.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Dataset resource name, format: projects/{project_id}/datasets/{dataset_id}
    pub fn datasets_import_data(&self, request: GoogleCloudDatalabelingV1beta1ImportDataRequest, name: &str) -> ProjectDatasetImportDataCall<'a, S> {
        ProjectDatasetImportDataCall {
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
    /// Lists datasets under a project. Pagination is supported.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Dataset resource parent, format: projects/{project_id}
    pub fn datasets_list(&self, parent: &str) -> ProjectDatasetListCall<'a, S> {
        ProjectDatasetListCall {
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
    /// Creates an evaluation job.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Evaluation job resource parent. Format: "projects/{project_id}"
    pub fn evaluation_jobs_create(&self, request: GoogleCloudDatalabelingV1beta1CreateEvaluationJobRequest, parent: &str) -> ProjectEvaluationJobCreateCall<'a, S> {
        ProjectEvaluationJobCreateCall {
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
    /// Stops and deletes an evaluation job.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the evaluation job that is going to be deleted. Format: "projects/{project_id}/evaluationJobs/{evaluation_job_id}"
    pub fn evaluation_jobs_delete(&self, name: &str) -> ProjectEvaluationJobDeleteCall<'a, S> {
        ProjectEvaluationJobDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an evaluation job by resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the evaluation job. Format: "projects/{project_id} /evaluationJobs/{evaluation_job_id}"
    pub fn evaluation_jobs_get(&self, name: &str) -> ProjectEvaluationJobGetCall<'a, S> {
        ProjectEvaluationJobGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all evaluation jobs within a project with possible filters. Pagination is supported.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Evaluation job resource parent. Format: "projects/{project_id}"
    pub fn evaluation_jobs_list(&self, parent: &str) -> ProjectEvaluationJobListCall<'a, S> {
        ProjectEvaluationJobListCall {
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
    /// Updates an evaluation job. You can only update certain fields of the job's EvaluationJobConfig: `humanAnnotationConfig.instruction`, `exampleCount`, and `exampleSamplePercentage`. If you want to change any other aspect of the evaluation job, you must delete the job and create a new one.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. After you create a job, Data Labeling Service assigns a name to the job with the following format: "projects/{project_id}/evaluationJobs/ {evaluation_job_id}"
    pub fn evaluation_jobs_patch(&self, request: GoogleCloudDatalabelingV1beta1EvaluationJob, name: &str) -> ProjectEvaluationJobPatchCall<'a, S> {
        ProjectEvaluationJobPatchCall {
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
    /// Pauses an evaluation job. Pausing an evaluation job that is already in a `PAUSED` state is a no-op.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the evaluation job that is going to be paused. Format: "projects/{project_id}/evaluationJobs/{evaluation_job_id}"
    pub fn evaluation_jobs_pause(&self, request: GoogleCloudDatalabelingV1beta1PauseEvaluationJobRequest, name: &str) -> ProjectEvaluationJobPauseCall<'a, S> {
        ProjectEvaluationJobPauseCall {
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
    /// Resumes a paused evaluation job. A deleted evaluation job can't be resumed. Resuming a running or scheduled evaluation job is a no-op.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the evaluation job that is going to be resumed. Format: "projects/{project_id}/evaluationJobs/{evaluation_job_id}"
    pub fn evaluation_jobs_resume(&self, request: GoogleCloudDatalabelingV1beta1ResumeEvaluationJobRequest, name: &str) -> ProjectEvaluationJobResumeCall<'a, S> {
        ProjectEvaluationJobResumeCall {
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
    /// Searches evaluations within a project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Evaluation search parent (project ID). Format: "projects/ {project_id}"
    pub fn evaluations_search(&self, parent: &str) -> ProjectEvaluationSearchCall<'a, S> {
        ProjectEvaluationSearchCall {
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
    /// Creates an instruction for how data should be labeled.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Instruction resource parent, format: projects/{project_id}
    pub fn instructions_create(&self, request: GoogleCloudDatalabelingV1beta1CreateInstructionRequest, parent: &str) -> ProjectInstructionCreateCall<'a, S> {
        ProjectInstructionCreateCall {
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
    /// Deletes an instruction object by resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Instruction resource name, format: projects/{project_id}/instructions/{instruction_id}
    pub fn instructions_delete(&self, name: &str) -> ProjectInstructionDeleteCall<'a, S> {
        ProjectInstructionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an instruction by resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Instruction resource name, format: projects/{project_id}/instructions/{instruction_id}
    pub fn instructions_get(&self, name: &str) -> ProjectInstructionGetCall<'a, S> {
        ProjectInstructionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists instructions for a project. Pagination is supported.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Instruction resource parent, format: projects/{project_id}
    pub fn instructions_list(&self, parent: &str) -> ProjectInstructionListCall<'a, S> {
        ProjectInstructionListCall {
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
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn operations_cancel(&self, name: &str) -> ProjectOperationCancelCall<'a, S> {
        ProjectOperationCancelCall {
            hub: self.hub,
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
    pub fn operations_delete(&self, name: &str) -> ProjectOperationDeleteCall<'a, S> {
        ProjectOperationDeleteCall {
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
    pub fn operations_get(&self, name: &str) -> ProjectOperationGetCall<'a, S> {
        ProjectOperationGetCall {
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
    pub fn operations_list(&self, name: &str) -> ProjectOperationListCall<'a, S> {
        ProjectOperationListCall {
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



