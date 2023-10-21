use super::*;
/// AnnotatedDataset is a set holding annotations for data in a Dataset. Each labeling task will generate an AnnotatedDataset under the Dataset that the task is requested for.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets annotated datasets get projects](ProjectDatasetAnnotatedDatasetGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1AnnotatedDataset {
    /// Output only. Source of the annotation.
    #[serde(rename="annotationSource")]
    
    pub annotation_source: Option<String>,
    /// Output only. Type of the annotation. It is specified when starting labeling task.
    #[serde(rename="annotationType")]
    
    pub annotation_type: Option<String>,
    /// Output only. The names of any related resources that are blocking changes to the annotated dataset.
    #[serde(rename="blockingResources")]
    
    pub blocking_resources: Option<Vec<String>>,
    /// Output only. Number of examples that have annotation in the annotated dataset.
    #[serde(rename="completedExampleCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub completed_example_count: Option<i64>,
    /// Output only. Time the AnnotatedDataset was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The description of the AnnotatedDataset. It is specified in HumanAnnotationConfig when user starts a labeling task. Maximum of 10000 characters.
    
    pub description: Option<String>,
    /// Output only. The display name of the AnnotatedDataset. It is specified in HumanAnnotationConfig when user starts a labeling task. Maximum of 64 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Number of examples in the annotated dataset.
    #[serde(rename="exampleCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub example_count: Option<i64>,
    /// Output only. Per label statistics.
    #[serde(rename="labelStats")]
    
    pub label_stats: Option<GoogleCloudDatalabelingV1beta1LabelStats>,
    /// Output only. Additional information about AnnotatedDataset.
    
    pub metadata: Option<GoogleCloudDatalabelingV1beta1AnnotatedDatasetMetadata>,
    /// Output only. AnnotatedDataset resource name in format of: projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/ {annotated_dataset_id}
    
    pub name: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1AnnotatedDataset {}


/// Metadata on AnnotatedDataset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1AnnotatedDatasetMetadata {
    /// Configuration for image bounding box and bounding poly task.
    #[serde(rename="boundingPolyConfig")]
    
    pub bounding_poly_config: Option<GoogleCloudDatalabelingV1beta1BoundingPolyConfig>,
    /// Configuration for video event labeling task.
    #[serde(rename="eventConfig")]
    
    pub event_config: Option<GoogleCloudDatalabelingV1beta1EventConfig>,
    /// HumanAnnotationConfig used when requesting the human labeling task for this AnnotatedDataset.
    #[serde(rename="humanAnnotationConfig")]
    
    pub human_annotation_config: Option<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
    /// Configuration for image classification task.
    #[serde(rename="imageClassificationConfig")]
    
    pub image_classification_config: Option<GoogleCloudDatalabelingV1beta1ImageClassificationConfig>,
    /// Configuration for video object detection task.
    #[serde(rename="objectDetectionConfig")]
    
    pub object_detection_config: Option<GoogleCloudDatalabelingV1beta1ObjectDetectionConfig>,
    /// Configuration for video object tracking task.
    #[serde(rename="objectTrackingConfig")]
    
    pub object_tracking_config: Option<GoogleCloudDatalabelingV1beta1ObjectTrackingConfig>,
    /// Configuration for image polyline task.
    #[serde(rename="polylineConfig")]
    
    pub polyline_config: Option<GoogleCloudDatalabelingV1beta1PolylineConfig>,
    /// Configuration for image segmentation task.
    #[serde(rename="segmentationConfig")]
    
    pub segmentation_config: Option<GoogleCloudDatalabelingV1beta1SegmentationConfig>,
    /// Configuration for text classification task.
    #[serde(rename="textClassificationConfig")]
    
    pub text_classification_config: Option<GoogleCloudDatalabelingV1beta1TextClassificationConfig>,
    /// Configuration for text entity extraction task.
    #[serde(rename="textEntityExtractionConfig")]
    
    pub text_entity_extraction_config: Option<GoogleCloudDatalabelingV1beta1TextEntityExtractionConfig>,
    /// Configuration for video classification task.
    #[serde(rename="videoClassificationConfig")]
    
    pub video_classification_config: Option<GoogleCloudDatalabelingV1beta1VideoClassificationConfig>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1AnnotatedDatasetMetadata {}


/// Annotation for Example. Each example may have one or more annotations. For example in image classification problem, each image might have one or more labels. We call labels binded with this image an Annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1Annotation {
    /// Output only. Annotation metadata, including information like votes for labels.
    #[serde(rename="annotationMetadata")]
    
    pub annotation_metadata: Option<GoogleCloudDatalabelingV1beta1AnnotationMetadata>,
    /// Output only. Sentiment for this annotation.
    #[serde(rename="annotationSentiment")]
    
    pub annotation_sentiment: Option<String>,
    /// Output only. The source of the annotation.
    #[serde(rename="annotationSource")]
    
    pub annotation_source: Option<String>,
    /// Output only. This is the actual annotation value, e.g classification, bounding box values are stored here.
    #[serde(rename="annotationValue")]
    
    pub annotation_value: Option<GoogleCloudDatalabelingV1beta1AnnotationValue>,
    /// Output only. Unique name of this annotation, format is: projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset}/examples/{example_id}/annotations/{annotation_id}
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1Annotation {}


/// Additional information associated with the annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1AnnotationMetadata {
    /// Metadata related to human labeling.
    #[serde(rename="operatorMetadata")]
    
    pub operator_metadata: Option<GoogleCloudDatalabelingV1beta1OperatorMetadata>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1AnnotationMetadata {}


/// Container of information related to one possible annotation that can be used in a labeling task. For example, an image classification task where images are labeled as `dog` or `cat` must reference an AnnotationSpec for `dog` and an AnnotationSpec for `cat`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1AnnotationSpec {
    /// Optional. User-provided description of the annotation specification. The description can be up to 10,000 characters long.
    
    pub description: Option<String>,
    /// Required. The display name of the AnnotationSpec. Maximum of 64 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. This is the integer index of the AnnotationSpec. The index for the whole AnnotationSpecSet is sequential starting from 0. For example, an AnnotationSpecSet with classes `dog` and `cat`, might contain one AnnotationSpec with `{ display_name: "dog", index: 0 }` and one AnnotationSpec with `{ display_name: "cat", index: 1 }`. This is especially useful for model training as it encodes the string labels into numeric values.
    
    pub index: Option<i32>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1AnnotationSpec {}


/// An AnnotationSpecSet is a collection of label definitions. For example, in image classification tasks, you define a set of possible labels for images as an AnnotationSpecSet. An AnnotationSpecSet is immutable upon creation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotation spec sets create projects](ProjectAnnotationSpecSetCreateCall) (response)
/// * [annotation spec sets get projects](ProjectAnnotationSpecSetGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1AnnotationSpecSet {
    /// Required. The array of AnnotationSpecs that you define when you create the AnnotationSpecSet. These are the possible labels for the labeling task.
    #[serde(rename="annotationSpecs")]
    
    pub annotation_specs: Option<Vec<GoogleCloudDatalabelingV1beta1AnnotationSpec>>,
    /// Output only. The names of any related resources that are blocking changes to the annotation spec set.
    #[serde(rename="blockingResources")]
    
    pub blocking_resources: Option<Vec<String>>,
    /// Optional. User-provided description of the annotation specification set. The description can be up to 10,000 characters long.
    
    pub description: Option<String>,
    /// Required. The display name for AnnotationSpecSet that you define when you create it. Maximum of 64 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The AnnotationSpecSet resource name in the following format: "projects/{project_id}/annotationSpecSets/{annotation_spec_set_id}"
    
    pub name: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1AnnotationSpecSet {}


/// Annotation spec set with the setting of allowing multi labels or not.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1AnnotationSpecSetConfig {
    /// Optional. If allow_multi_label is true, contributors are able to choose multiple labels from one annotation spec set.
    #[serde(rename="allowMultiLabel")]
    
    pub allow_multi_label: Option<bool>,
    /// Required. Annotation spec set resource name.
    #[serde(rename="annotationSpecSet")]
    
    pub annotation_spec_set: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1AnnotationSpecSetConfig {}


/// Annotation value for an example.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1AnnotationValue {
    /// Annotation value for image bounding box, oriented bounding box and polygon cases.
    #[serde(rename="imageBoundingPolyAnnotation")]
    
    pub image_bounding_poly_annotation: Option<GoogleCloudDatalabelingV1beta1ImageBoundingPolyAnnotation>,
    /// Annotation value for image classification case.
    #[serde(rename="imageClassificationAnnotation")]
    
    pub image_classification_annotation: Option<GoogleCloudDatalabelingV1beta1ImageClassificationAnnotation>,
    /// Annotation value for image polyline cases. Polyline here is different from BoundingPoly. It is formed by line segments connected to each other but not closed form(Bounding Poly). The line segments can cross each other.
    #[serde(rename="imagePolylineAnnotation")]
    
    pub image_polyline_annotation: Option<GoogleCloudDatalabelingV1beta1ImagePolylineAnnotation>,
    /// Annotation value for image segmentation.
    #[serde(rename="imageSegmentationAnnotation")]
    
    pub image_segmentation_annotation: Option<GoogleCloudDatalabelingV1beta1ImageSegmentationAnnotation>,
    /// Annotation value for text classification case.
    #[serde(rename="textClassificationAnnotation")]
    
    pub text_classification_annotation: Option<GoogleCloudDatalabelingV1beta1TextClassificationAnnotation>,
    /// Annotation value for text entity extraction case.
    #[serde(rename="textEntityExtractionAnnotation")]
    
    pub text_entity_extraction_annotation: Option<GoogleCloudDatalabelingV1beta1TextEntityExtractionAnnotation>,
    /// Annotation value for video classification case.
    #[serde(rename="videoClassificationAnnotation")]
    
    pub video_classification_annotation: Option<GoogleCloudDatalabelingV1beta1VideoClassificationAnnotation>,
    /// Annotation value for video event case.
    #[serde(rename="videoEventAnnotation")]
    
    pub video_event_annotation: Option<GoogleCloudDatalabelingV1beta1VideoEventAnnotation>,
    /// Annotation value for video object detection and tracking case.
    #[serde(rename="videoObjectTrackingAnnotation")]
    
    pub video_object_tracking_annotation: Option<GoogleCloudDatalabelingV1beta1VideoObjectTrackingAnnotation>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1AnnotationValue {}


/// Records a failed evaluation job run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1Attempt {
    /// no description provided
    #[serde(rename="attemptTime")]
    
    pub attempt_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Details of errors that occurred.
    #[serde(rename="partialFailures")]
    
    pub partial_failures: Option<Vec<GoogleRpcStatus>>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1Attempt {}


/// The BigQuery location for input data. If used in an EvaluationJob, this is where the service saves the prediction input and output sampled from the model version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1BigQuerySource {
    /// Required. BigQuery URI to a table, up to 2,000 characters long. If you specify the URI of a table that does not exist, Data Labeling Service creates a table at the URI with the correct schema when you create your EvaluationJob. If you specify the URI of a table that already exists, it must have the [correct schema](https://cloud.google.com/ml-engine/docs/continuous-evaluation/create-job#table-schema). Provide the table URI in the following format: “bq://{your_project_id}/ {your_dataset_name}/{your_table_name}” [Learn more](https://cloud.google.com/ml-engine/docs/continuous-evaluation/create-job#table-schema).
    #[serde(rename="inputUri")]
    
    pub input_uri: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1BigQuerySource {}


/// Options regarding evaluation between bounding boxes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1BoundingBoxEvaluationOptions {
    /// Minimum [intersection-over-union (IOU)](https://cloud.google.com/vision/automl/object-detection/docs/evaluate#intersection-over-union) required for 2 bounding boxes to be considered a match. This must be a number between 0 and 1.
    #[serde(rename="iouThreshold")]
    
    pub iou_threshold: Option<f32>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1BoundingBoxEvaluationOptions {}


/// A bounding polygon in the image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1BoundingPoly {
    /// The bounding polygon vertices.
    
    pub vertices: Option<Vec<GoogleCloudDatalabelingV1beta1Vertex>>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1BoundingPoly {}


/// Config for image bounding poly (and bounding box) human labeling task.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1BoundingPolyConfig {
    /// Required. Annotation spec set resource name.
    #[serde(rename="annotationSpecSet")]
    
    pub annotation_spec_set: Option<String>,
    /// Optional. Instruction message showed on contributors UI.
    #[serde(rename="instructionMessage")]
    
    pub instruction_message: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1BoundingPolyConfig {}


/// Metadata for classification annotations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ClassificationMetadata {
    /// Whether the classification task is multi-label or not.
    #[serde(rename="isMultiLabel")]
    
    pub is_multi_label: Option<bool>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ClassificationMetadata {}


/// Metrics calculated for a classification model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ClassificationMetrics {
    /// Confusion matrix of predicted labels vs. ground truth labels.
    #[serde(rename="confusionMatrix")]
    
    pub confusion_matrix: Option<GoogleCloudDatalabelingV1beta1ConfusionMatrix>,
    /// Precision-recall curve based on ground truth labels, predicted labels, and scores for the predicted labels.
    #[serde(rename="prCurve")]
    
    pub pr_curve: Option<GoogleCloudDatalabelingV1beta1PrCurve>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ClassificationMetrics {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ConfidenceMetricsEntry {
    /// Threshold used for this entry. For classification tasks, this is a classification threshold: a predicted label is categorized as positive or negative (in the context of this point on the PR curve) based on whether the label’s score meets this threshold. For image object detection (bounding box) tasks, this is the [intersection-over-union (IOU)](https://cloud.google.com/vision/automl/object-detection/docs/evaluate#intersection-over-union) threshold for the context of this point on the PR curve.
    #[serde(rename="confidenceThreshold")]
    
    pub confidence_threshold: Option<f32>,
    /// Harmonic mean of recall and precision.
    #[serde(rename="f1Score")]
    
    pub f1_score: Option<f32>,
    /// The harmonic mean of recall_at1 and precision_at1.
    #[serde(rename="f1ScoreAt1")]
    
    pub f1_score_at1: Option<f32>,
    /// The harmonic mean of recall_at5 and precision_at5.
    #[serde(rename="f1ScoreAt5")]
    
    pub f1_score_at5: Option<f32>,
    /// Precision value.
    
    pub precision: Option<f32>,
    /// Precision value for entries with label that has highest score.
    #[serde(rename="precisionAt1")]
    
    pub precision_at1: Option<f32>,
    /// Precision value for entries with label that has highest 5 scores.
    #[serde(rename="precisionAt5")]
    
    pub precision_at5: Option<f32>,
    /// Recall value.
    
    pub recall: Option<f32>,
    /// Recall value for entries with label that has highest score.
    #[serde(rename="recallAt1")]
    
    pub recall_at1: Option<f32>,
    /// Recall value for entries with label that has highest 5 scores.
    #[serde(rename="recallAt5")]
    
    pub recall_at5: Option<f32>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ConfidenceMetricsEntry {}


/// Confusion matrix of the model running the classification. Only applicable when the metrics entry aggregates multiple labels. Not applicable when the entry is for a single label.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ConfusionMatrix {
    /// no description provided
    
    pub row: Option<Vec<GoogleCloudDatalabelingV1beta1Row>>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ConfusionMatrix {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ConfusionMatrixEntry {
    /// The annotation spec of a predicted label.
    #[serde(rename="annotationSpec")]
    
    pub annotation_spec: Option<GoogleCloudDatalabelingV1beta1AnnotationSpec>,
    /// Number of items predicted to have this label. (The ground truth label for these items is the `Row.annotationSpec` of this entry's parent.)
    #[serde(rename="itemCount")]
    
    pub item_count: Option<i32>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ConfusionMatrixEntry {}


/// Request message for CreateAnnotationSpecSet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotation spec sets create projects](ProjectAnnotationSpecSetCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1CreateAnnotationSpecSetRequest {
    /// Required. Annotation spec set to create. Annotation specs must be included. Only one annotation spec will be accepted for annotation specs with same display_name.
    #[serde(rename="annotationSpecSet")]
    
    pub annotation_spec_set: Option<GoogleCloudDatalabelingV1beta1AnnotationSpecSet>,
}

impl client::RequestValue for GoogleCloudDatalabelingV1beta1CreateAnnotationSpecSetRequest {}


/// Request message for CreateDataset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets create projects](ProjectDatasetCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1CreateDatasetRequest {
    /// Required. The dataset to be created.
    
    pub dataset: Option<GoogleCloudDatalabelingV1beta1Dataset>,
}

impl client::RequestValue for GoogleCloudDatalabelingV1beta1CreateDatasetRequest {}


/// Request message for CreateEvaluationJob.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [evaluation jobs create projects](ProjectEvaluationJobCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1CreateEvaluationJobRequest {
    /// Required. The evaluation job to create.
    
    pub job: Option<GoogleCloudDatalabelingV1beta1EvaluationJob>,
}

impl client::RequestValue for GoogleCloudDatalabelingV1beta1CreateEvaluationJobRequest {}


/// Request message for CreateInstruction.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instructions create projects](ProjectInstructionCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1CreateInstructionRequest {
    /// Required. Instruction of how to perform the labeling task.
    
    pub instruction: Option<GoogleCloudDatalabelingV1beta1Instruction>,
}

impl client::RequestValue for GoogleCloudDatalabelingV1beta1CreateInstructionRequest {}


/// Deprecated: this instruction format is not supported any more. Instruction from a CSV file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1CsvInstruction {
    /// CSV file for the instruction. Only gcs path is allowed.
    #[serde(rename="gcsFileUri")]
    
    pub gcs_file_uri: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1CsvInstruction {}


/// DataItem is a piece of data, without annotation. For example, an image.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets annotated datasets data items get projects](ProjectDatasetAnnotatedDatasetDataItemGetCall) (response)
/// * [datasets data items get projects](ProjectDatasetDataItemGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1DataItem {
    /// The image payload, a container of the image bytes/uri.
    #[serde(rename="imagePayload")]
    
    pub image_payload: Option<GoogleCloudDatalabelingV1beta1ImagePayload>,
    /// Output only. Name of the data item, in format of: projects/{project_id}/datasets/{dataset_id}/dataItems/{data_item_id}
    
    pub name: Option<String>,
    /// The text payload, a container of text content.
    #[serde(rename="textPayload")]
    
    pub text_payload: Option<GoogleCloudDatalabelingV1beta1TextPayload>,
    /// The video payload, a container of the video uri.
    #[serde(rename="videoPayload")]
    
    pub video_payload: Option<GoogleCloudDatalabelingV1beta1VideoPayload>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1DataItem {}


/// Dataset is the resource to hold your data. You can request multiple labeling tasks for a dataset while each one will generate an AnnotatedDataset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets create projects](ProjectDatasetCreateCall) (response)
/// * [datasets get projects](ProjectDatasetGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1Dataset {
    /// Output only. The names of any related resources that are blocking changes to the dataset.
    #[serde(rename="blockingResources")]
    
    pub blocking_resources: Option<Vec<String>>,
    /// Output only. Time the dataset is created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The number of data items in the dataset.
    #[serde(rename="dataItemCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub data_item_count: Option<i64>,
    /// Optional. User-provided description of the annotation specification set. The description can be up to 10000 characters long.
    
    pub description: Option<String>,
    /// Required. The display name of the dataset. Maximum of 64 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. This is populated with the original input configs where ImportData is called. It is available only after the clients import data to this dataset.
    #[serde(rename="inputConfigs")]
    
    pub input_configs: Option<Vec<GoogleCloudDatalabelingV1beta1InputConfig>>,
    /// Last time that the Dataset is migrated to AI Platform V2. If any of the AnnotatedDataset is migrated, the last_migration_time in Dataset is also updated.
    #[serde(rename="lastMigrateTime")]
    
    pub last_migrate_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Dataset resource name, format is: projects/{project_id}/datasets/{dataset_id}
    
    pub name: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1Dataset {}


/// Describes an evaluation between a machine learning model’s predictions and ground truth labels. Created when an EvaluationJob runs successfully.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets evaluations get projects](ProjectDatasetEvaluationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1Evaluation {
    /// Output only. Type of task that the model version being evaluated performs, as defined in the evaluationJobConfig.inputConfig.annotationType field of the evaluation job that created this evaluation.
    #[serde(rename="annotationType")]
    
    pub annotation_type: Option<String>,
    /// Output only. Options used in the evaluation job that created this evaluation.
    
    pub config: Option<GoogleCloudDatalabelingV1beta1EvaluationConfig>,
    /// Output only. Timestamp for when this evaluation was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The number of items in the ground truth dataset that were used for this evaluation. Only populated when the evaulation is for certain AnnotationTypes.
    #[serde(rename="evaluatedItemCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub evaluated_item_count: Option<i64>,
    /// Output only. Timestamp for when the evaluation job that created this evaluation ran.
    #[serde(rename="evaluationJobRunTime")]
    
    pub evaluation_job_run_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Metrics comparing predictions to ground truth labels.
    #[serde(rename="evaluationMetrics")]
    
    pub evaluation_metrics: Option<GoogleCloudDatalabelingV1beta1EvaluationMetrics>,
    /// Output only. Resource name of an evaluation. The name has the following format: "projects/{project_id}/datasets/{dataset_id}/evaluations/ {evaluation_id}'
    
    pub name: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1Evaluation {}


/// Configuration details used for calculating evaluation metrics and creating an Evaluation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1EvaluationConfig {
    /// Only specify this field if the related model performs image object detection (`IMAGE_BOUNDING_BOX_ANNOTATION`). Describes how to evaluate bounding boxes.
    #[serde(rename="boundingBoxEvaluationOptions")]
    
    pub bounding_box_evaluation_options: Option<GoogleCloudDatalabelingV1beta1BoundingBoxEvaluationOptions>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1EvaluationConfig {}


/// Defines an evaluation job that runs periodically to generate Evaluations. [Creating an evaluation job](https://cloud.google.com/ml-engine/docs/continuous-evaluation/create-job) is the starting point for using continuous evaluation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [evaluation jobs create projects](ProjectEvaluationJobCreateCall) (response)
/// * [evaluation jobs get projects](ProjectEvaluationJobGetCall) (response)
/// * [evaluation jobs patch projects](ProjectEvaluationJobPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1EvaluationJob {
    /// Required. Name of the AnnotationSpecSet describing all the labels that your machine learning model outputs. You must create this resource before you create an evaluation job and provide its name in the following format: "projects/{project_id}/annotationSpecSets/{annotation_spec_set_id}"
    #[serde(rename="annotationSpecSet")]
    
    pub annotation_spec_set: Option<String>,
    /// Output only. Every time the evaluation job runs and an error occurs, the failed attempt is appended to this array.
    
    pub attempts: Option<Vec<GoogleCloudDatalabelingV1beta1Attempt>>,
    /// Output only. Timestamp of when this evaluation job was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Description of the job. The description can be up to 25,000 characters long.
    
    pub description: Option<String>,
    /// Required. Configuration details for the evaluation job.
    #[serde(rename="evaluationJobConfig")]
    
    pub evaluation_job_config: Option<GoogleCloudDatalabelingV1beta1EvaluationJobConfig>,
    /// Required. Whether you want Data Labeling Service to provide ground truth labels for prediction input. If you want the service to assign human labelers to annotate your data, set this to `true`. If you want to provide your own ground truth labels in the evaluation job's BigQuery table, set this to `false`.
    #[serde(rename="labelMissingGroundTruth")]
    
    pub label_missing_ground_truth: Option<bool>,
    /// Required. The [AI Platform Prediction model version](https://cloud.google.com/ml-engine/docs/prediction-overview) to be evaluated. Prediction input and output is sampled from this model version. When creating an evaluation job, specify the model version in the following format: “projects/{project_id}/models/{model_name}/versions/{version_name}” There can only be one evaluation job per model version.
    #[serde(rename="modelVersion")]
    
    pub model_version: Option<String>,
    /// Output only. After you create a job, Data Labeling Service assigns a name to the job with the following format: "projects/{project_id}/evaluationJobs/ {evaluation_job_id}"
    
    pub name: Option<String>,
    /// Required. Describes the interval at which the job runs. This interval must be at least 1 day, and it is rounded to the nearest day. For example, if you specify a 50-hour interval, the job runs every 2 days. You can provide the schedule in [crontab format](https://cloud.google.com/scheduler/docs/configuring/cron-job-schedules) or in an [English-like format](https://cloud.google.com/appengine/docs/standard/python/config/cronref#schedule_format). Regardless of what you specify, the job will run at 10:00 AM UTC. Only the interval from this schedule is used, not the specific time of day.
    
    pub schedule: Option<String>,
    /// Output only. Describes the current state of the job.
    
    pub state: Option<String>,
}

impl client::RequestValue for GoogleCloudDatalabelingV1beta1EvaluationJob {}
impl client::ResponseResult for GoogleCloudDatalabelingV1beta1EvaluationJob {}


/// Provides details for how an evaluation job sends email alerts based on the results of a run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1EvaluationJobAlertConfig {
    /// Required. An email address to send alerts to.
    
    pub email: Option<String>,
    /// Required. A number between 0 and 1 that describes a minimum mean average precision threshold. When the evaluation job runs, if it calculates that your model version's predictions from the recent interval have meanAveragePrecision below this threshold, then it sends an alert to your specified email.
    #[serde(rename="minAcceptableMeanAveragePrecision")]
    
    pub min_acceptable_mean_average_precision: Option<f64>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1EvaluationJobAlertConfig {}


/// Configures specific details of how a continuous evaluation job works. Provide this configuration when you create an EvaluationJob.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1EvaluationJobConfig {
    /// Required. Prediction keys that tell Data Labeling Service where to find the data for evaluation in your BigQuery table. When the service samples prediction input and output from your model version and saves it to BigQuery, the data gets stored as JSON strings in the BigQuery table. These keys tell Data Labeling Service how to parse the JSON. You can provide the following entries in this field: * `data_json_key`: the data key for prediction input. You must provide either this key or `reference_json_key`. * `reference_json_key`: the data reference key for prediction input. You must provide either this key or `data_json_key`. * `label_json_key`: the label key for prediction output. Required. * `label_score_json_key`: the score key for prediction output. Required. * `bounding_box_json_key`: the bounding box key for prediction output. Required if your model version perform image object detection. Learn [how to configure prediction keys](https://cloud.google.com/ml-engine/docs/continuous-evaluation/create-job#prediction-keys).
    #[serde(rename="bigqueryImportKeys")]
    
    pub bigquery_import_keys: Option<HashMap<String, String>>,
    /// Specify this field if your model version performs image object detection (bounding box detection). `annotationSpecSet` in this configuration must match EvaluationJob.annotationSpecSet.
    #[serde(rename="boundingPolyConfig")]
    
    pub bounding_poly_config: Option<GoogleCloudDatalabelingV1beta1BoundingPolyConfig>,
    /// Required. Details for calculating evaluation metrics and creating Evaulations. If your model version performs image object detection, you must specify the `boundingBoxEvaluationOptions` field within this configuration. Otherwise, provide an empty object for this configuration.
    #[serde(rename="evaluationConfig")]
    
    pub evaluation_config: Option<GoogleCloudDatalabelingV1beta1EvaluationConfig>,
    /// Optional. Configuration details for evaluation job alerts. Specify this field if you want to receive email alerts if the evaluation job finds that your predictions have low mean average precision during a run.
    #[serde(rename="evaluationJobAlertConfig")]
    
    pub evaluation_job_alert_config: Option<GoogleCloudDatalabelingV1beta1EvaluationJobAlertConfig>,
    /// Required. The maximum number of predictions to sample and save to BigQuery during each evaluation interval. This limit overrides `example_sample_percentage`: even if the service has not sampled enough predictions to fulfill `example_sample_perecentage` during an interval, it stops sampling predictions when it meets this limit.
    #[serde(rename="exampleCount")]
    
    pub example_count: Option<i32>,
    /// Required. Fraction of predictions to sample and save to BigQuery during each evaluation interval. For example, 0.1 means 10% of predictions served by your model version get saved to BigQuery.
    #[serde(rename="exampleSamplePercentage")]
    
    pub example_sample_percentage: Option<f64>,
    /// Optional. Details for human annotation of your data. If you set labelMissingGroundTruth to `true` for this evaluation job, then you must specify this field. If you plan to provide your own ground truth labels, then omit this field. Note that you must create an Instruction resource before you can specify this field. Provide the name of the instruction resource in the `instruction` field within this configuration.
    #[serde(rename="humanAnnotationConfig")]
    
    pub human_annotation_config: Option<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
    /// Specify this field if your model version performs image classification or general classification. `annotationSpecSet` in this configuration must match EvaluationJob.annotationSpecSet. `allowMultiLabel` in this configuration must match `classificationMetadata.isMultiLabel` in input_config.
    #[serde(rename="imageClassificationConfig")]
    
    pub image_classification_config: Option<GoogleCloudDatalabelingV1beta1ImageClassificationConfig>,
    /// Rquired. Details for the sampled prediction input. Within this configuration, there are requirements for several fields: * `dataType` must be one of `IMAGE`, `TEXT`, or `GENERAL_DATA`. * `annotationType` must be one of `IMAGE_CLASSIFICATION_ANNOTATION`, `TEXT_CLASSIFICATION_ANNOTATION`, `GENERAL_CLASSIFICATION_ANNOTATION`, or `IMAGE_BOUNDING_BOX_ANNOTATION` (image object detection). * If your machine learning model performs classification, you must specify `classificationMetadata.isMultiLabel`. * You must specify `bigquerySource` (not `gcsSource`).
    #[serde(rename="inputConfig")]
    
    pub input_config: Option<GoogleCloudDatalabelingV1beta1InputConfig>,
    /// Specify this field if your model version performs text classification. `annotationSpecSet` in this configuration must match EvaluationJob.annotationSpecSet. `allowMultiLabel` in this configuration must match `classificationMetadata.isMultiLabel` in input_config.
    #[serde(rename="textClassificationConfig")]
    
    pub text_classification_config: Option<GoogleCloudDatalabelingV1beta1TextClassificationConfig>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1EvaluationJobConfig {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1EvaluationMetrics {
    /// no description provided
    #[serde(rename="classificationMetrics")]
    
    pub classification_metrics: Option<GoogleCloudDatalabelingV1beta1ClassificationMetrics>,
    /// no description provided
    #[serde(rename="objectDetectionMetrics")]
    
    pub object_detection_metrics: Option<GoogleCloudDatalabelingV1beta1ObjectDetectionMetrics>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1EvaluationMetrics {}


/// Config for video event human labeling task.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1EventConfig {
    /// Required. The list of annotation spec set resource name. Similar to video classification, we support selecting event from multiple AnnotationSpecSet at the same time.
    #[serde(rename="annotationSpecSets")]
    
    pub annotation_spec_sets: Option<Vec<String>>,
    /// Videos will be cut to smaller clips to make it easier for labelers to work on. Users can configure is field in seconds, if not set, default value is 60s.
    #[serde(rename="clipLength")]
    
    pub clip_length: Option<i32>,
    /// The overlap length between different video clips. Users can configure is field in seconds, if not set, default value is 1s.
    #[serde(rename="overlapLength")]
    
    pub overlap_length: Option<i32>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1EventConfig {}


/// An Example is a piece of data and its annotation. For example, an image with label “house”.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets annotated datasets examples get projects](ProjectDatasetAnnotatedDatasetExampleGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1Example {
    /// Output only. Annotations for the piece of data in Example. One piece of data can have multiple annotations.
    
    pub annotations: Option<Vec<GoogleCloudDatalabelingV1beta1Annotation>>,
    /// The image payload, a container of the image bytes/uri.
    #[serde(rename="imagePayload")]
    
    pub image_payload: Option<GoogleCloudDatalabelingV1beta1ImagePayload>,
    /// Output only. Name of the example, in format of: projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/ {annotated_dataset_id}/examples/{example_id}
    
    pub name: Option<String>,
    /// The text payload, a container of the text content.
    #[serde(rename="textPayload")]
    
    pub text_payload: Option<GoogleCloudDatalabelingV1beta1TextPayload>,
    /// The video payload, a container of the video uri.
    #[serde(rename="videoPayload")]
    
    pub video_payload: Option<GoogleCloudDatalabelingV1beta1VideoPayload>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1Example {}


/// Example comparisons comparing ground truth output and predictions for a specific input.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ExampleComparison {
    /// The ground truth output for the input.
    #[serde(rename="groundTruthExample")]
    
    pub ground_truth_example: Option<GoogleCloudDatalabelingV1beta1Example>,
    /// Predictions by the model for the input.
    #[serde(rename="modelCreatedExamples")]
    
    pub model_created_examples: Option<Vec<GoogleCloudDatalabelingV1beta1Example>>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ExampleComparison {}


/// Request message for ExportData API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets export data projects](ProjectDatasetExportDataCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ExportDataRequest {
    /// Required. Annotated dataset resource name. DataItem in Dataset and their annotations in specified annotated dataset will be exported. It's in format of projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/ {annotated_dataset_id}
    #[serde(rename="annotatedDataset")]
    
    pub annotated_dataset: Option<String>,
    /// Optional. Filter is not supported at this moment.
    
    pub filter: Option<String>,
    /// Required. Specify the output destination.
    #[serde(rename="outputConfig")]
    
    pub output_config: Option<GoogleCloudDatalabelingV1beta1OutputConfig>,
    /// Email of the user who started the export task and should be notified by email. If empty no notification will be sent.
    #[serde(rename="userEmailAddress")]
    
    pub user_email_address: Option<String>,
}

impl client::RequestValue for GoogleCloudDatalabelingV1beta1ExportDataRequest {}


/// A feedback message inside a feedback thread.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets annotated datasets feedback threads feedback messages create projects](ProjectDatasetAnnotatedDatasetFeedbackThreadFeedbackMessageCreateCall) (request)
/// * [datasets annotated datasets feedback threads feedback messages get projects](ProjectDatasetAnnotatedDatasetFeedbackThreadFeedbackMessageGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1FeedbackMessage {
    /// String content of the feedback. Maximum of 10000 characters.
    
    pub body: Option<String>,
    /// Create time.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The image storing this feedback if the feedback is an image representing operator's comments.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub image: Option<Vec<u8>>,
    /// Name of the feedback message in a feedback thread. Format: 'project/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}/feedbackThreads/{feedback_thread_id}/feedbackMessage/{feedback_message_id}'
    
    pub name: Option<String>,
    /// no description provided
    #[serde(rename="operatorFeedbackMetadata")]
    
    pub operator_feedback_metadata: Option<GoogleCloudDatalabelingV1beta1OperatorFeedbackMetadata>,
    /// no description provided
    #[serde(rename="requesterFeedbackMetadata")]
    
    pub requester_feedback_metadata: Option<GoogleCloudDatalabelingV1beta1RequesterFeedbackMetadata>,
}

impl client::RequestValue for GoogleCloudDatalabelingV1beta1FeedbackMessage {}
impl client::ResponseResult for GoogleCloudDatalabelingV1beta1FeedbackMessage {}


/// A feedback thread of a certain labeling task on a certain annotated dataset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets annotated datasets feedback threads get projects](ProjectDatasetAnnotatedDatasetFeedbackThreadGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1FeedbackThread {
    /// Metadata regarding the feedback thread.
    #[serde(rename="feedbackThreadMetadata")]
    
    pub feedback_thread_metadata: Option<GoogleCloudDatalabelingV1beta1FeedbackThreadMetadata>,
    /// Name of the feedback thread. Format: 'project/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}/feedbackThreads/{feedback_thread_id}'
    
    pub name: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1FeedbackThread {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1FeedbackThreadMetadata {
    /// When the thread is created
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// When the thread is last updated.
    #[serde(rename="lastUpdateTime")]
    
    pub last_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// no description provided
    
    pub status: Option<String>,
    /// An image thumbnail of this thread.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub thumbnail: Option<Vec<u8>>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1FeedbackThreadMetadata {}


/// Export destination of the data.Only gcs path is allowed in output_uri.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1GcsDestination {
    /// Required. The format of the gcs destination. Only "text/csv" and "application/json" are supported.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Required. The output uri of destination file.
    #[serde(rename="outputUri")]
    
    pub output_uri: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1GcsDestination {}


/// Export folder destination of the data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1GcsFolderDestination {
    /// Required. Cloud Storage directory to export data to.
    #[serde(rename="outputFolderUri")]
    
    pub output_folder_uri: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1GcsFolderDestination {}


/// Source of the Cloud Storage file to be imported.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1GcsSource {
    /// Required. The input URI of source file. This must be a Cloud Storage path (`gs://...`).
    #[serde(rename="inputUri")]
    
    pub input_uri: Option<String>,
    /// Required. The format of the source file. Only "text/csv" is supported.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1GcsSource {}


/// Configuration for how human labeling task should be done.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1HumanAnnotationConfig {
    /// Optional. A human-readable description for AnnotatedDataset. The description can be up to 10000 characters long.
    #[serde(rename="annotatedDatasetDescription")]
    
    pub annotated_dataset_description: Option<String>,
    /// Required. A human-readable name for AnnotatedDataset defined by users. Maximum of 64 characters .
    #[serde(rename="annotatedDatasetDisplayName")]
    
    pub annotated_dataset_display_name: Option<String>,
    /// Optional. If you want your own labeling contributors to manage and work on this labeling request, you can set these contributors here. We will give them access to the question types in crowdcompute. Note that these emails must be registered in crowdcompute worker UI: https://crowd-compute.appspot.com/
    #[serde(rename="contributorEmails")]
    
    pub contributor_emails: Option<Vec<String>>,
    /// Required. Instruction resource name.
    
    pub instruction: Option<String>,
    /// Optional. A human-readable label used to logically group labeling tasks. This string must match the regular expression `[a-zA-Z\\d_-]{0,128}`.
    #[serde(rename="labelGroup")]
    
    pub label_group: Option<String>,
    /// Optional. The Language of this question, as a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). Default value is en-US. Only need to set this when task is language related. For example, French text classification.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Optional. Maximum duration for contributors to answer a question. Maximum is 3600 seconds. Default is 3600 seconds.
    #[serde(rename="questionDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub question_duration: Option<client::chrono::Duration>,
    /// Optional. Replication of questions. Each question will be sent to up to this number of contributors to label. Aggregated answers will be returned. Default is set to 1. For image related labeling, valid values are 1, 3, 5.
    #[serde(rename="replicaCount")]
    
    pub replica_count: Option<i32>,
    /// Email of the user who started the labeling task and should be notified by email. If empty no notification will be sent.
    #[serde(rename="userEmailAddress")]
    
    pub user_email_address: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1HumanAnnotationConfig {}


/// Image bounding poly annotation. It represents a polygon including bounding box in the image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ImageBoundingPolyAnnotation {
    /// Label of object in this bounding polygon.
    #[serde(rename="annotationSpec")]
    
    pub annotation_spec: Option<GoogleCloudDatalabelingV1beta1AnnotationSpec>,
    /// no description provided
    #[serde(rename="boundingPoly")]
    
    pub bounding_poly: Option<GoogleCloudDatalabelingV1beta1BoundingPoly>,
    /// no description provided
    #[serde(rename="normalizedBoundingPoly")]
    
    pub normalized_bounding_poly: Option<GoogleCloudDatalabelingV1beta1NormalizedBoundingPoly>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ImageBoundingPolyAnnotation {}


/// Image classification annotation definition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ImageClassificationAnnotation {
    /// Label of image.
    #[serde(rename="annotationSpec")]
    
    pub annotation_spec: Option<GoogleCloudDatalabelingV1beta1AnnotationSpec>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ImageClassificationAnnotation {}


/// Config for image classification human labeling task.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ImageClassificationConfig {
    /// Optional. If allow_multi_label is true, contributors are able to choose multiple labels for one image.
    #[serde(rename="allowMultiLabel")]
    
    pub allow_multi_label: Option<bool>,
    /// Required. Annotation spec set resource name.
    #[serde(rename="annotationSpecSet")]
    
    pub annotation_spec_set: Option<String>,
    /// Optional. The type of how to aggregate answers.
    #[serde(rename="answerAggregationType")]
    
    pub answer_aggregation_type: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ImageClassificationConfig {}


/// Container of information about an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ImagePayload {
    /// A byte string of a thumbnail image.
    #[serde(rename="imageThumbnail")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub image_thumbnail: Option<Vec<u8>>,
    /// Image uri from the user bucket.
    #[serde(rename="imageUri")]
    
    pub image_uri: Option<String>,
    /// Image format.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Signed uri of the image file in the service bucket.
    #[serde(rename="signedUri")]
    
    pub signed_uri: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ImagePayload {}


/// A polyline for the image annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ImagePolylineAnnotation {
    /// Label of this polyline.
    #[serde(rename="annotationSpec")]
    
    pub annotation_spec: Option<GoogleCloudDatalabelingV1beta1AnnotationSpec>,
    /// no description provided
    #[serde(rename="normalizedPolyline")]
    
    pub normalized_polyline: Option<GoogleCloudDatalabelingV1beta1NormalizedPolyline>,
    /// no description provided
    
    pub polyline: Option<GoogleCloudDatalabelingV1beta1Polyline>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ImagePolylineAnnotation {}


/// Image segmentation annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ImageSegmentationAnnotation {
    /// The mapping between rgb color and annotation spec. The key is the rgb color represented in format of rgb(0, 0, 0). The value is the AnnotationSpec.
    #[serde(rename="annotationColors")]
    
    pub annotation_colors: Option<HashMap<String, GoogleCloudDatalabelingV1beta1AnnotationSpec>>,
    /// A byte string of a full image's color map.
    #[serde(rename="imageBytes")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub image_bytes: Option<Vec<u8>>,
    /// Image format.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ImageSegmentationAnnotation {}


/// Request message for ImportData API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets import data projects](ProjectDatasetImportDataCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ImportDataRequest {
    /// Required. Specify the input source of the data.
    #[serde(rename="inputConfig")]
    
    pub input_config: Option<GoogleCloudDatalabelingV1beta1InputConfig>,
    /// Email of the user who started the import task and should be notified by email. If empty no notification will be sent.
    #[serde(rename="userEmailAddress")]
    
    pub user_email_address: Option<String>,
}

impl client::RequestValue for GoogleCloudDatalabelingV1beta1ImportDataRequest {}


/// The configuration of input data, including data type, location, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1InputConfig {
    /// Optional. The type of annotation to be performed on this data. You must specify this field if you are using this InputConfig in an EvaluationJob.
    #[serde(rename="annotationType")]
    
    pub annotation_type: Option<String>,
    /// Source located in BigQuery. You must specify this field if you are using this InputConfig in an EvaluationJob.
    #[serde(rename="bigquerySource")]
    
    pub bigquery_source: Option<GoogleCloudDatalabelingV1beta1BigQuerySource>,
    /// Optional. Metadata about annotations for the input. You must specify this field if you are using this InputConfig in an EvaluationJob for a model version that performs classification.
    #[serde(rename="classificationMetadata")]
    
    pub classification_metadata: Option<GoogleCloudDatalabelingV1beta1ClassificationMetadata>,
    /// Required. Data type must be specifed when user tries to import data.
    #[serde(rename="dataType")]
    
    pub data_type: Option<String>,
    /// Source located in Cloud Storage.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GoogleCloudDatalabelingV1beta1GcsSource>,
    /// Required for text import, as language code must be specified.
    #[serde(rename="textMetadata")]
    
    pub text_metadata: Option<GoogleCloudDatalabelingV1beta1TextMetadata>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1InputConfig {}


/// Instruction of how to perform the labeling task for human operators. Currently only PDF instruction is supported.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instructions get projects](ProjectInstructionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1Instruction {
    /// Output only. The names of any related resources that are blocking changes to the instruction.
    #[serde(rename="blockingResources")]
    
    pub blocking_resources: Option<Vec<String>>,
    /// Output only. Creation time of instruction.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Deprecated: this instruction format is not supported any more. Instruction from a CSV file, such as for classification task. The CSV file should have exact two columns, in the following format: * The first column is labeled data, such as an image reference, text. * The second column is comma separated labels associated with data.
    #[serde(rename="csvInstruction")]
    
    pub csv_instruction: Option<GoogleCloudDatalabelingV1beta1CsvInstruction>,
    /// Required. The data type of this instruction.
    #[serde(rename="dataType")]
    
    pub data_type: Option<String>,
    /// Optional. User-provided description of the instruction. The description can be up to 10000 characters long.
    
    pub description: Option<String>,
    /// Required. The display name of the instruction. Maximum of 64 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Instruction resource name, format: projects/{project_id}/instructions/{instruction_id}
    
    pub name: Option<String>,
    /// Instruction from a PDF document. The PDF should be in a Cloud Storage bucket.
    #[serde(rename="pdfInstruction")]
    
    pub pdf_instruction: Option<GoogleCloudDatalabelingV1beta1PdfInstruction>,
    /// Output only. Last update time of instruction.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1Instruction {}


/// Request message for starting an image labeling task.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets image label projects](ProjectDatasetImageLabelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1LabelImageRequest {
    /// Required. Basic human annotation config.
    #[serde(rename="basicConfig")]
    
    pub basic_config: Option<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
    /// Configuration for bounding box and bounding poly task. One of image_classification_config, bounding_poly_config, polyline_config and segmentation_config are required.
    #[serde(rename="boundingPolyConfig")]
    
    pub bounding_poly_config: Option<GoogleCloudDatalabelingV1beta1BoundingPolyConfig>,
    /// Required. The type of image labeling task.
    
    pub feature: Option<String>,
    /// Configuration for image classification task. One of image_classification_config, bounding_poly_config, polyline_config and segmentation_config are required.
    #[serde(rename="imageClassificationConfig")]
    
    pub image_classification_config: Option<GoogleCloudDatalabelingV1beta1ImageClassificationConfig>,
    /// Configuration for polyline task. One of image_classification_config, bounding_poly_config, polyline_config and segmentation_config are required.
    #[serde(rename="polylineConfig")]
    
    pub polyline_config: Option<GoogleCloudDatalabelingV1beta1PolylineConfig>,
    /// Configuration for segmentation task. One of image_classification_config, bounding_poly_config, polyline_config and segmentation_config are required.
    #[serde(rename="segmentationConfig")]
    
    pub segmentation_config: Option<GoogleCloudDatalabelingV1beta1SegmentationConfig>,
}

impl client::RequestValue for GoogleCloudDatalabelingV1beta1LabelImageRequest {}


/// Statistics about annotation specs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1LabelStats {
    /// Map of each annotation spec's example count. Key is the annotation spec name and value is the number of examples for that annotation spec. If the annotated dataset does not have annotation spec, the map will return a pair where the key is empty string and value is the total number of annotations.
    #[serde(rename="exampleCount")]
    
    #[serde_as(as = "Option<HashMap<_, ::client::serde_with::DisplayFromStr>>")]
    pub example_count: Option<HashMap<String, i64>>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1LabelStats {}


/// Request message for LabelText.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets text label projects](ProjectDatasetTextLabelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1LabelTextRequest {
    /// Required. Basic human annotation config.
    #[serde(rename="basicConfig")]
    
    pub basic_config: Option<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
    /// Required. The type of text labeling task.
    
    pub feature: Option<String>,
    /// Configuration for text classification task. One of text_classification_config and text_entity_extraction_config is required.
    #[serde(rename="textClassificationConfig")]
    
    pub text_classification_config: Option<GoogleCloudDatalabelingV1beta1TextClassificationConfig>,
    /// Configuration for entity extraction task. One of text_classification_config and text_entity_extraction_config is required.
    #[serde(rename="textEntityExtractionConfig")]
    
    pub text_entity_extraction_config: Option<GoogleCloudDatalabelingV1beta1TextEntityExtractionConfig>,
}

impl client::RequestValue for GoogleCloudDatalabelingV1beta1LabelTextRequest {}


/// Request message for LabelVideo.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets video label projects](ProjectDatasetVideoLabelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1LabelVideoRequest {
    /// Required. Basic human annotation config.
    #[serde(rename="basicConfig")]
    
    pub basic_config: Option<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
    /// Configuration for video event task. One of video_classification_config, object_detection_config, object_tracking_config and event_config is required.
    #[serde(rename="eventConfig")]
    
    pub event_config: Option<GoogleCloudDatalabelingV1beta1EventConfig>,
    /// Required. The type of video labeling task.
    
    pub feature: Option<String>,
    /// Configuration for video object detection task. One of video_classification_config, object_detection_config, object_tracking_config and event_config is required.
    #[serde(rename="objectDetectionConfig")]
    
    pub object_detection_config: Option<GoogleCloudDatalabelingV1beta1ObjectDetectionConfig>,
    /// Configuration for video object tracking task. One of video_classification_config, object_detection_config, object_tracking_config and event_config is required.
    #[serde(rename="objectTrackingConfig")]
    
    pub object_tracking_config: Option<GoogleCloudDatalabelingV1beta1ObjectTrackingConfig>,
    /// Configuration for video classification task. One of video_classification_config, object_detection_config, object_tracking_config and event_config is required.
    #[serde(rename="videoClassificationConfig")]
    
    pub video_classification_config: Option<GoogleCloudDatalabelingV1beta1VideoClassificationConfig>,
}

impl client::RequestValue for GoogleCloudDatalabelingV1beta1LabelVideoRequest {}


/// Results of listing annotated datasets for a dataset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets annotated datasets list projects](ProjectDatasetAnnotatedDatasetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ListAnnotatedDatasetsResponse {
    /// The list of annotated datasets to return.
    #[serde(rename="annotatedDatasets")]
    
    pub annotated_datasets: Option<Vec<GoogleCloudDatalabelingV1beta1AnnotatedDataset>>,
    /// A token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1ListAnnotatedDatasetsResponse {}


/// Results of listing annotation spec set under a project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotation spec sets list projects](ProjectAnnotationSpecSetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ListAnnotationSpecSetsResponse {
    /// The list of annotation spec sets.
    #[serde(rename="annotationSpecSets")]
    
    pub annotation_spec_sets: Option<Vec<GoogleCloudDatalabelingV1beta1AnnotationSpecSet>>,
    /// A token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1ListAnnotationSpecSetsResponse {}


/// Results of listing data items in a dataset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets annotated datasets data items list projects](ProjectDatasetAnnotatedDatasetDataItemListCall) (response)
/// * [datasets data items list projects](ProjectDatasetDataItemListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ListDataItemsResponse {
    /// The list of data items to return.
    #[serde(rename="dataItems")]
    
    pub data_items: Option<Vec<GoogleCloudDatalabelingV1beta1DataItem>>,
    /// A token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1ListDataItemsResponse {}


/// Results of listing datasets within a project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets list projects](ProjectDatasetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ListDatasetsResponse {
    /// The list of datasets to return.
    
    pub datasets: Option<Vec<GoogleCloudDatalabelingV1beta1Dataset>>,
    /// A token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1ListDatasetsResponse {}


/// Results for listing evaluation jobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [evaluation jobs list projects](ProjectEvaluationJobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ListEvaluationJobsResponse {
    /// The list of evaluation jobs to return.
    #[serde(rename="evaluationJobs")]
    
    pub evaluation_jobs: Option<Vec<GoogleCloudDatalabelingV1beta1EvaluationJob>>,
    /// A token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1ListEvaluationJobsResponse {}


/// Results of listing Examples in and annotated dataset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets annotated datasets examples list projects](ProjectDatasetAnnotatedDatasetExampleListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ListExamplesResponse {
    /// The list of examples to return.
    
    pub examples: Option<Vec<GoogleCloudDatalabelingV1beta1Example>>,
    /// A token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1ListExamplesResponse {}


/// Results for listing FeedbackMessages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets annotated datasets feedback threads feedback messages list projects](ProjectDatasetAnnotatedDatasetFeedbackThreadFeedbackMessageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ListFeedbackMessagesResponse {
    /// The list of feedback messages to return.
    #[serde(rename="feedbackMessages")]
    
    pub feedback_messages: Option<Vec<GoogleCloudDatalabelingV1beta1FeedbackMessage>>,
    /// A token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1ListFeedbackMessagesResponse {}


/// Results for listing FeedbackThreads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets annotated datasets feedback threads list projects](ProjectDatasetAnnotatedDatasetFeedbackThreadListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ListFeedbackThreadsResponse {
    /// The list of feedback threads to return.
    #[serde(rename="feedbackThreads")]
    
    pub feedback_threads: Option<Vec<GoogleCloudDatalabelingV1beta1FeedbackThread>>,
    /// A token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1ListFeedbackThreadsResponse {}


/// Results of listing instructions under a project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instructions list projects](ProjectInstructionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ListInstructionsResponse {
    /// The list of Instructions to return.
    
    pub instructions: Option<Vec<GoogleCloudDatalabelingV1beta1Instruction>>,
    /// A token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1ListInstructionsResponse {}


/// Normalized bounding polygon.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1NormalizedBoundingPoly {
    /// The bounding polygon normalized vertices.
    #[serde(rename="normalizedVertices")]
    
    pub normalized_vertices: Option<Vec<GoogleCloudDatalabelingV1beta1NormalizedVertex>>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1NormalizedBoundingPoly {}


/// Normalized polyline.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1NormalizedPolyline {
    /// The normalized polyline vertices.
    #[serde(rename="normalizedVertices")]
    
    pub normalized_vertices: Option<Vec<GoogleCloudDatalabelingV1beta1NormalizedVertex>>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1NormalizedPolyline {}


/// A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1NormalizedVertex {
    /// X coordinate.
    
    pub x: Option<f32>,
    /// Y coordinate.
    
    pub y: Option<f32>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1NormalizedVertex {}


/// Config for video object detection human labeling task. Object detection will be conducted on the images extracted from the video, and those objects will be labeled with bounding boxes. User need to specify the number of images to be extracted per second as the extraction frame rate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ObjectDetectionConfig {
    /// Required. Annotation spec set resource name.
    #[serde(rename="annotationSpecSet")]
    
    pub annotation_spec_set: Option<String>,
    /// Required. Number of frames per second to be extracted from the video.
    #[serde(rename="extractionFrameRate")]
    
    pub extraction_frame_rate: Option<f64>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ObjectDetectionConfig {}


/// Metrics calculated for an image object detection (bounding box) model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ObjectDetectionMetrics {
    /// Precision-recall curve.
    #[serde(rename="prCurve")]
    
    pub pr_curve: Option<GoogleCloudDatalabelingV1beta1PrCurve>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ObjectDetectionMetrics {}


/// Config for video object tracking human labeling task.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ObjectTrackingConfig {
    /// Required. Annotation spec set resource name.
    #[serde(rename="annotationSpecSet")]
    
    pub annotation_spec_set: Option<String>,
    /// Videos will be cut to smaller clips to make it easier for labelers to work on. Users can configure is field in seconds, if not set, default value is 20s.
    #[serde(rename="clipLength")]
    
    pub clip_length: Option<i32>,
    /// The overlap length between different video clips. Users can configure is field in seconds, if not set, default value is 0.3s.
    #[serde(rename="overlapLength")]
    
    pub overlap_length: Option<i32>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ObjectTrackingConfig {}


/// Video frame level annotation for object detection and tracking.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ObjectTrackingFrame {
    /// no description provided
    #[serde(rename="boundingPoly")]
    
    pub bounding_poly: Option<GoogleCloudDatalabelingV1beta1BoundingPoly>,
    /// no description provided
    #[serde(rename="normalizedBoundingPoly")]
    
    pub normalized_bounding_poly: Option<GoogleCloudDatalabelingV1beta1NormalizedBoundingPoly>,
    /// The time offset of this frame relative to the beginning of the video.
    #[serde(rename="timeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub time_offset: Option<client::chrono::Duration>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1ObjectTrackingFrame {}


/// Metadata describing the feedback from the operator.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1OperatorFeedbackMetadata { _never_set: Option<bool> }

impl client::Part for GoogleCloudDatalabelingV1beta1OperatorFeedbackMetadata {}


/// General information useful for labels coming from contributors.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1OperatorMetadata {
    /// Comments from contributors.
    
    pub comments: Option<Vec<String>>,
    /// The total number of contributors that choose this label.
    #[serde(rename="labelVotes")]
    
    pub label_votes: Option<i32>,
    /// Confidence score corresponding to a label. For examle, if 3 contributors have answered the question and 2 of them agree on the final label, the confidence score will be 0.67 (2/3).
    
    pub score: Option<f32>,
    /// The total number of contributors that answer this question.
    #[serde(rename="totalVotes")]
    
    pub total_votes: Option<i32>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1OperatorMetadata {}


/// The configuration of output data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1OutputConfig {
    /// Output to a file in Cloud Storage. Should be used for labeling output other than image segmentation.
    #[serde(rename="gcsDestination")]
    
    pub gcs_destination: Option<GoogleCloudDatalabelingV1beta1GcsDestination>,
    /// Output to a folder in Cloud Storage. Should be used for image segmentation or document de-identification labeling outputs.
    #[serde(rename="gcsFolderDestination")]
    
    pub gcs_folder_destination: Option<GoogleCloudDatalabelingV1beta1GcsFolderDestination>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1OutputConfig {}


/// Request message for PauseEvaluationJob.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [evaluation jobs pause projects](ProjectEvaluationJobPauseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1PauseEvaluationJobRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDatalabelingV1beta1PauseEvaluationJobRequest {}


/// Instruction from a PDF file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1PdfInstruction {
    /// PDF file for the instruction. Only gcs path is allowed.
    #[serde(rename="gcsFileUri")]
    
    pub gcs_file_uri: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1PdfInstruction {}


/// A line with multiple line segments.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1Polyline {
    /// The polyline vertices.
    
    pub vertices: Option<Vec<GoogleCloudDatalabelingV1beta1Vertex>>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1Polyline {}


/// Config for image polyline human labeling task.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1PolylineConfig {
    /// Required. Annotation spec set resource name.
    #[serde(rename="annotationSpecSet")]
    
    pub annotation_spec_set: Option<String>,
    /// Optional. Instruction message showed on contributors UI.
    #[serde(rename="instructionMessage")]
    
    pub instruction_message: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1PolylineConfig {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1PrCurve {
    /// The annotation spec of the label for which the precision-recall curve calculated. If this field is empty, that means the precision-recall curve is an aggregate curve for all labels.
    #[serde(rename="annotationSpec")]
    
    pub annotation_spec: Option<GoogleCloudDatalabelingV1beta1AnnotationSpec>,
    /// Area under the precision-recall curve. Not to be confused with area under a receiver operating characteristic (ROC) curve.
    #[serde(rename="areaUnderCurve")]
    
    pub area_under_curve: Option<f32>,
    /// Entries that make up the precision-recall graph. Each entry is a "point" on the graph drawn for a different `confidence_threshold`.
    #[serde(rename="confidenceMetricsEntries")]
    
    pub confidence_metrics_entries: Option<Vec<GoogleCloudDatalabelingV1beta1ConfidenceMetricsEntry>>,
    /// Mean average prcision of this curve.
    #[serde(rename="meanAveragePrecision")]
    
    pub mean_average_precision: Option<f32>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1PrCurve {}


/// Metadata describing the feedback from the labeling task requester.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1RequesterFeedbackMetadata { _never_set: Option<bool> }

impl client::Part for GoogleCloudDatalabelingV1beta1RequesterFeedbackMetadata {}


/// Request message ResumeEvaluationJob.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [evaluation jobs resume projects](ProjectEvaluationJobResumeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1ResumeEvaluationJobRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDatalabelingV1beta1ResumeEvaluationJobRequest {}


/// A row in the confusion matrix. Each entry in this row has the same ground truth label.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1Row {
    /// The annotation spec of the ground truth label for this row.
    #[serde(rename="annotationSpec")]
    
    pub annotation_spec: Option<GoogleCloudDatalabelingV1beta1AnnotationSpec>,
    /// A list of the confusion matrix entries. One entry for each possible predicted label.
    
    pub entries: Option<Vec<GoogleCloudDatalabelingV1beta1ConfusionMatrixEntry>>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1Row {}


/// Results of searching evaluations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [evaluations search projects](ProjectEvaluationSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1SearchEvaluationsResponse {
    /// The list of evaluations matching the search.
    
    pub evaluations: Option<Vec<GoogleCloudDatalabelingV1beta1Evaluation>>,
    /// A token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1SearchEvaluationsResponse {}


/// Request message of SearchExampleComparisons.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets evaluations example comparisons search projects](ProjectDatasetEvaluationExampleComparisonSearchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1SearchExampleComparisonsRequest {
    /// Optional. Requested page size. Server may return fewer results than requested. Default value is 100.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional. A token identifying a page of results for the server to return. Typically obtained by the nextPageToken of the response to a previous search rquest. If you don't specify this field, the API call requests the first page of the search.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
}

impl client::RequestValue for GoogleCloudDatalabelingV1beta1SearchExampleComparisonsRequest {}


/// Results of searching example comparisons.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datasets evaluations example comparisons search projects](ProjectDatasetEvaluationExampleComparisonSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1SearchExampleComparisonsResponse {
    /// A list of example comparisons matching the search criteria.
    #[serde(rename="exampleComparisons")]
    
    pub example_comparisons: Option<Vec<GoogleCloudDatalabelingV1beta1ExampleComparison>>,
    /// A token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDatalabelingV1beta1SearchExampleComparisonsResponse {}


/// Config for image segmentation
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1SegmentationConfig {
    /// Required. Annotation spec set resource name. format: projects/{project_id}/annotationSpecSets/{annotation_spec_set_id}
    #[serde(rename="annotationSpecSet")]
    
    pub annotation_spec_set: Option<String>,
    /// Instruction message showed on labelers UI.
    #[serde(rename="instructionMessage")]
    
    pub instruction_message: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1SegmentationConfig {}


/// Config for setting up sentiments.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1SentimentConfig {
    /// If set to true, contributors will have the option to select sentiment of the label they selected, to mark it as negative or positive label. Default is false.
    #[serde(rename="enableLabelSentimentSelection")]
    
    pub enable_label_sentiment_selection: Option<bool>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1SentimentConfig {}


/// Start and end position in a sequence (e.g. text segment).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1SequentialSegment {
    /// End position (exclusive).
    
    pub end: Option<i32>,
    /// Start position (inclusive).
    
    pub start: Option<i32>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1SequentialSegment {}


/// Text classification annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1TextClassificationAnnotation {
    /// Label of the text.
    #[serde(rename="annotationSpec")]
    
    pub annotation_spec: Option<GoogleCloudDatalabelingV1beta1AnnotationSpec>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1TextClassificationAnnotation {}


/// Config for text classification human labeling task.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1TextClassificationConfig {
    /// Optional. If allow_multi_label is true, contributors are able to choose multiple labels for one text segment.
    #[serde(rename="allowMultiLabel")]
    
    pub allow_multi_label: Option<bool>,
    /// Required. Annotation spec set resource name.
    #[serde(rename="annotationSpecSet")]
    
    pub annotation_spec_set: Option<String>,
    /// Optional. Configs for sentiment selection. We deprecate sentiment analysis in data labeling side as it is incompatible with uCAIP.
    #[serde(rename="sentimentConfig")]
    
    pub sentiment_config: Option<GoogleCloudDatalabelingV1beta1SentimentConfig>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1TextClassificationConfig {}


/// Text entity extraction annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1TextEntityExtractionAnnotation {
    /// Label of the text entities.
    #[serde(rename="annotationSpec")]
    
    pub annotation_spec: Option<GoogleCloudDatalabelingV1beta1AnnotationSpec>,
    /// Position of the entity.
    #[serde(rename="sequentialSegment")]
    
    pub sequential_segment: Option<GoogleCloudDatalabelingV1beta1SequentialSegment>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1TextEntityExtractionAnnotation {}


/// Config for text entity extraction human labeling task.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1TextEntityExtractionConfig {
    /// Required. Annotation spec set resource name.
    #[serde(rename="annotationSpecSet")]
    
    pub annotation_spec_set: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1TextEntityExtractionConfig {}


/// Metadata for the text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1TextMetadata {
    /// The language of this text, as a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). Default value is en-US.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1TextMetadata {}


/// Container of information about a piece of text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1TextPayload {
    /// Text content.
    #[serde(rename="textContent")]
    
    pub text_content: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1TextPayload {}


/// A time period inside of an example that has a time dimension (e.g. video).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1TimeSegment {
    /// End of the time segment (exclusive), represented as the duration since the example start.
    #[serde(rename="endTimeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub end_time_offset: Option<client::chrono::Duration>,
    /// Start of the time segment (inclusive), represented as the duration since the example start.
    #[serde(rename="startTimeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub start_time_offset: Option<client::chrono::Duration>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1TimeSegment {}


/// A vertex represents a 2D point in the image. NOTE: the vertex coordinates are in the same scale as the original image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1Vertex {
    /// X coordinate.
    
    pub x: Option<i32>,
    /// Y coordinate.
    
    pub y: Option<i32>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1Vertex {}


/// Video classification annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1VideoClassificationAnnotation {
    /// Label of the segment specified by time_segment.
    #[serde(rename="annotationSpec")]
    
    pub annotation_spec: Option<GoogleCloudDatalabelingV1beta1AnnotationSpec>,
    /// The time segment of the video to which the annotation applies.
    #[serde(rename="timeSegment")]
    
    pub time_segment: Option<GoogleCloudDatalabelingV1beta1TimeSegment>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1VideoClassificationAnnotation {}


/// Config for video classification human labeling task. Currently two types of video classification are supported: 1. Assign labels on the entire video. 2. Split the video into multiple video clips based on camera shot, and assign labels on each video clip.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1VideoClassificationConfig {
    /// Required. The list of annotation spec set configs. Since watching a video clip takes much longer time than an image, we support label with multiple AnnotationSpecSet at the same time. Labels in each AnnotationSpecSet will be shown in a group to contributors. Contributors can select one or more (depending on whether to allow multi label) from each group.
    #[serde(rename="annotationSpecSetConfigs")]
    
    pub annotation_spec_set_configs: Option<Vec<GoogleCloudDatalabelingV1beta1AnnotationSpecSetConfig>>,
    /// Optional. Option to apply shot detection on the video.
    #[serde(rename="applyShotDetection")]
    
    pub apply_shot_detection: Option<bool>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1VideoClassificationConfig {}


/// Video event annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1VideoEventAnnotation {
    /// Label of the event in this annotation.
    #[serde(rename="annotationSpec")]
    
    pub annotation_spec: Option<GoogleCloudDatalabelingV1beta1AnnotationSpec>,
    /// The time segment of the video to which the annotation applies.
    #[serde(rename="timeSegment")]
    
    pub time_segment: Option<GoogleCloudDatalabelingV1beta1TimeSegment>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1VideoEventAnnotation {}


/// Video object tracking annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1VideoObjectTrackingAnnotation {
    /// Label of the object tracked in this annotation.
    #[serde(rename="annotationSpec")]
    
    pub annotation_spec: Option<GoogleCloudDatalabelingV1beta1AnnotationSpec>,
    /// The list of frames where this object track appears.
    #[serde(rename="objectTrackingFrames")]
    
    pub object_tracking_frames: Option<Vec<GoogleCloudDatalabelingV1beta1ObjectTrackingFrame>>,
    /// The time segment of the video to which object tracking applies.
    #[serde(rename="timeSegment")]
    
    pub time_segment: Option<GoogleCloudDatalabelingV1beta1TimeSegment>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1VideoObjectTrackingAnnotation {}


/// Container of information of a video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1VideoPayload {
    /// FPS of the video.
    #[serde(rename="frameRate")]
    
    pub frame_rate: Option<f32>,
    /// Video format.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Signed uri of the video file in the service bucket.
    #[serde(rename="signedUri")]
    
    pub signed_uri: Option<String>,
    /// The list of video thumbnails.
    #[serde(rename="videoThumbnails")]
    
    pub video_thumbnails: Option<Vec<GoogleCloudDatalabelingV1beta1VideoThumbnail>>,
    /// Video uri from the user bucket.
    #[serde(rename="videoUri")]
    
    pub video_uri: Option<String>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1VideoPayload {}


/// Container of information of a video thumbnail.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDatalabelingV1beta1VideoThumbnail {
    /// A byte string of the video frame.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub thumbnail: Option<Vec<u8>>,
    /// Time offset relative to the beginning of the video, corresponding to the video frame where the thumbnail has been extracted from.
    #[serde(rename="timeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub time_offset: Option<client::chrono::Duration>,
}

impl client::Part for GoogleCloudDatalabelingV1beta1VideoThumbnail {}


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
/// * [datasets annotated datasets feedback threads feedback messages create projects](ProjectDatasetAnnotatedDatasetFeedbackThreadFeedbackMessageCreateCall) (response)
/// * [datasets image label projects](ProjectDatasetImageLabelCall) (response)
/// * [datasets text label projects](ProjectDatasetTextLabelCall) (response)
/// * [datasets video label projects](ProjectDatasetVideoLabelCall) (response)
/// * [datasets export data projects](ProjectDatasetExportDataCall) (response)
/// * [datasets import data projects](ProjectDatasetImportDataCall) (response)
/// * [instructions create projects](ProjectInstructionCreateCall) (response)
/// * [operations get projects](ProjectOperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<GoogleRpcStatus>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleLongrunningOperation {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotation spec sets delete projects](ProjectAnnotationSpecSetDeleteCall) (response)
/// * [datasets annotated datasets feedback threads feedback messages delete projects](ProjectDatasetAnnotatedDatasetFeedbackThreadFeedbackMessageDeleteCall) (response)
/// * [datasets annotated datasets feedback threads delete projects](ProjectDatasetAnnotatedDatasetFeedbackThreadDeleteCall) (response)
/// * [datasets annotated datasets delete projects](ProjectDatasetAnnotatedDatasetDeleteCall) (response)
/// * [datasets delete projects](ProjectDatasetDeleteCall) (response)
/// * [evaluation jobs delete projects](ProjectEvaluationJobDeleteCall) (response)
/// * [evaluation jobs pause projects](ProjectEvaluationJobPauseCall) (response)
/// * [evaluation jobs resume projects](ProjectEvaluationJobResumeCall) (response)
/// * [instructions delete projects](ProjectInstructionDeleteCall) (response)
/// * [operations cancel projects](ProjectOperationCancelCall) (response)
/// * [operations delete projects](ProjectOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpcStatus {}


