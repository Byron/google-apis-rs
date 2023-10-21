use super::*;
/// Video annotation request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotate videos](VideoAnnotateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudVideointelligenceV1_AnnotateVideoRequest {
    /// Required. Requested video annotation features.
    
    pub features: Option<Vec<String>>,
    /// The video data bytes. If unset, the input video(s) should be specified via the `input_uri`. If set, `input_uri` must be unset.
    #[serde(rename="inputContent")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub input_content: Option<Vec<u8>>,
    /// Input video location. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. URIs must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints). To identify multiple videos, a video URI may include wildcards in the `object-id`. Supported wildcards: '*' to match 0 or more characters; '?' to match 1 character. If unset, the input video should be embedded in the request as `input_content`. If set, `input_content` must be unset.
    #[serde(rename="inputUri")]
    
    pub input_uri: Option<String>,
    /// Optional. Cloud region where annotation should take place. Supported cloud regions are: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region is specified, the region will be determined based on video file location.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Optional. Location where the output (in JSON format) should be stored. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. These must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints).
    #[serde(rename="outputUri")]
    
    pub output_uri: Option<String>,
    /// Additional video context and/or feature-specific parameters.
    #[serde(rename="videoContext")]
    
    pub video_context: Option<GoogleCloudVideointelligenceV1_VideoContext>,
}

impl client::RequestValue for GoogleCloudVideointelligenceV1_AnnotateVideoRequest {}


/// Config for EXPLICIT_CONTENT_DETECTION.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudVideointelligenceV1_ExplicitContentDetectionConfig {
    /// Model to use for explicit content detection. Supported values: "builtin/stable" (the default if unset) and "builtin/latest".
    
    pub model: Option<String>,
}

impl client::Part for GoogleCloudVideointelligenceV1_ExplicitContentDetectionConfig {}


/// Config for FACE_DETECTION.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudVideointelligenceV1_FaceDetectionConfig {
    /// Whether to enable face attributes detection, such as glasses, dark_glasses, mouth_open etc. Ignored if 'include_bounding_boxes' is set to false.
    #[serde(rename="includeAttributes")]
    
    pub include_attributes: Option<bool>,
    /// Whether bounding boxes are included in the face annotation output.
    #[serde(rename="includeBoundingBoxes")]
    
    pub include_bounding_boxes: Option<bool>,
    /// Model to use for face detection. Supported values: "builtin/stable" (the default if unset) and "builtin/latest".
    
    pub model: Option<String>,
}

impl client::Part for GoogleCloudVideointelligenceV1_FaceDetectionConfig {}


/// Config for LABEL_DETECTION.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudVideointelligenceV1_LabelDetectionConfig {
    /// The confidence threshold we perform filtering on the labels from frame-level detection. If not set, it is set to 0.4 by default. The valid range for this threshold is [0.1, 0.9]. Any value set outside of this range will be clipped. Note: For best results, follow the default threshold. We will update the default threshold everytime when we release a new model.
    #[serde(rename="frameConfidenceThreshold")]
    
    pub frame_confidence_threshold: Option<f32>,
    /// What labels should be detected with LABEL_DETECTION, in addition to video-level labels or segment-level labels. If unspecified, defaults to `SHOT_MODE`.
    #[serde(rename="labelDetectionMode")]
    
    pub label_detection_mode: Option<String>,
    /// Model to use for label detection. Supported values: "builtin/stable" (the default if unset) and "builtin/latest".
    
    pub model: Option<String>,
    /// Whether the video has been shot from a stationary (i.e., non-moving) camera. When set to true, might improve detection accuracy for moving objects. Should be used with `SHOT_AND_FRAME_MODE` enabled.
    #[serde(rename="stationaryCamera")]
    
    pub stationary_camera: Option<bool>,
    /// The confidence threshold we perform filtering on the labels from video-level and shot-level detections. If not set, it's set to 0.3 by default. The valid range for this threshold is [0.1, 0.9]. Any value set outside of this range will be clipped. Note: For best results, follow the default threshold. We will update the default threshold everytime when we release a new model.
    #[serde(rename="videoConfidenceThreshold")]
    
    pub video_confidence_threshold: Option<f32>,
}

impl client::Part for GoogleCloudVideointelligenceV1_LabelDetectionConfig {}


/// Config for OBJECT_TRACKING.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudVideointelligenceV1_ObjectTrackingConfig {
    /// Model to use for object tracking. Supported values: "builtin/stable" (the default if unset) and "builtin/latest".
    
    pub model: Option<String>,
}

impl client::Part for GoogleCloudVideointelligenceV1_ObjectTrackingConfig {}


/// Config for PERSON_DETECTION.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudVideointelligenceV1_PersonDetectionConfig {
    /// Whether to enable person attributes detection, such as cloth color (black, blue, etc), type (coat, dress, etc), pattern (plain, floral, etc), hair, etc. Ignored if 'include_bounding_boxes' is set to false.
    #[serde(rename="includeAttributes")]
    
    pub include_attributes: Option<bool>,
    /// Whether bounding boxes are included in the person detection annotation output.
    #[serde(rename="includeBoundingBoxes")]
    
    pub include_bounding_boxes: Option<bool>,
    /// Whether to enable pose landmarks detection. Ignored if 'include_bounding_boxes' is set to false.
    #[serde(rename="includePoseLandmarks")]
    
    pub include_pose_landmarks: Option<bool>,
}

impl client::Part for GoogleCloudVideointelligenceV1_PersonDetectionConfig {}


/// Config for SHOT_CHANGE_DETECTION.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudVideointelligenceV1_ShotChangeDetectionConfig {
    /// Model to use for shot change detection. Supported values: "builtin/stable" (the default if unset), "builtin/latest", and "builtin/legacy".
    
    pub model: Option<String>,
}

impl client::Part for GoogleCloudVideointelligenceV1_ShotChangeDetectionConfig {}


/// Provides "hints" to the speech recognizer to favor specific words and phrases in the results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudVideointelligenceV1_SpeechContext {
    /// Optional. A list of strings containing words and phrases "hints" so that the speech recognition is more likely to recognize them. This can be used to improve the accuracy for specific words and phrases, for example, if specific commands are typically spoken by the user. This can also be used to add additional words to the vocabulary of the recognizer. See [usage limits](https://cloud.google.com/speech/limits#content).
    
    pub phrases: Option<Vec<String>>,
}

impl client::Part for GoogleCloudVideointelligenceV1_SpeechContext {}


/// Config for SPEECH_TRANSCRIPTION.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudVideointelligenceV1_SpeechTranscriptionConfig {
    /// Optional. For file formats, such as MXF or MKV, supporting multiple audio tracks, specify up to two tracks. Default: track 0.
    #[serde(rename="audioTracks")]
    
    pub audio_tracks: Option<Vec<i32>>,
    /// Optional. If set, specifies the estimated number of speakers in the conversation. If not set, defaults to '2'. Ignored unless enable_speaker_diarization is set to true.
    #[serde(rename="diarizationSpeakerCount")]
    
    pub diarization_speaker_count: Option<i32>,
    /// Optional. If 'true', adds punctuation to recognition result hypotheses. This feature is only available in select languages. Setting this for requests in other languages has no effect at all. The default 'false' value does not add punctuation to result hypotheses. NOTE: "This is currently offered as an experimental service, complimentary to all users. In the future this may be exclusively available as a premium feature."
    #[serde(rename="enableAutomaticPunctuation")]
    
    pub enable_automatic_punctuation: Option<bool>,
    /// Optional. If 'true', enables speaker detection for each recognized word in the top alternative of the recognition result using a speaker_tag provided in the WordInfo. Note: When this is true, we send all the words from the beginning of the audio for the top alternative in every consecutive response. This is done in order to improve our speaker tags as our models learn to identify the speakers in the conversation over time.
    #[serde(rename="enableSpeakerDiarization")]
    
    pub enable_speaker_diarization: Option<bool>,
    /// Optional. If `true`, the top result includes a list of words and the confidence for those words. If `false`, no word-level confidence information is returned. The default is `false`.
    #[serde(rename="enableWordConfidence")]
    
    pub enable_word_confidence: Option<bool>,
    /// Optional. If set to `true`, the server will attempt to filter out profanities, replacing all but the initial character in each filtered word with asterisks, e.g. "f***". If set to `false` or omitted, profanities won't be filtered out.
    #[serde(rename="filterProfanity")]
    
    pub filter_profanity: Option<bool>,
    /// Required. *Required* The language of the supplied audio as a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US". See [Language Support](https://cloud.google.com/speech/docs/languages) for a list of the currently supported language codes.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Optional. Maximum number of recognition hypotheses to be returned. Specifically, the maximum number of `SpeechRecognitionAlternative` messages within each `SpeechTranscription`. The server may return fewer than `max_alternatives`. Valid values are `0`-`30`. A value of `0` or `1` will return a maximum of one. If omitted, will return a maximum of one.
    #[serde(rename="maxAlternatives")]
    
    pub max_alternatives: Option<i32>,
    /// Optional. A means to provide context to assist the speech recognition.
    #[serde(rename="speechContexts")]
    
    pub speech_contexts: Option<Vec<GoogleCloudVideointelligenceV1_SpeechContext>>,
}

impl client::Part for GoogleCloudVideointelligenceV1_SpeechTranscriptionConfig {}


/// Config for TEXT_DETECTION.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudVideointelligenceV1_TextDetectionConfig {
    /// Language hint can be specified if the language to be detected is known a priori. It can increase the accuracy of the detection. Language hint must be language code in BCP-47 format. Automatic language detection is performed if no hint is provided.
    #[serde(rename="languageHints")]
    
    pub language_hints: Option<Vec<String>>,
    /// Model to use for text detection. Supported values: "builtin/stable" (the default if unset) and "builtin/latest".
    
    pub model: Option<String>,
}

impl client::Part for GoogleCloudVideointelligenceV1_TextDetectionConfig {}


/// Video context and/or feature-specific parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudVideointelligenceV1_VideoContext {
    /// Config for EXPLICIT_CONTENT_DETECTION.
    #[serde(rename="explicitContentDetectionConfig")]
    
    pub explicit_content_detection_config: Option<GoogleCloudVideointelligenceV1_ExplicitContentDetectionConfig>,
    /// Config for FACE_DETECTION.
    #[serde(rename="faceDetectionConfig")]
    
    pub face_detection_config: Option<GoogleCloudVideointelligenceV1_FaceDetectionConfig>,
    /// Config for LABEL_DETECTION.
    #[serde(rename="labelDetectionConfig")]
    
    pub label_detection_config: Option<GoogleCloudVideointelligenceV1_LabelDetectionConfig>,
    /// Config for OBJECT_TRACKING.
    #[serde(rename="objectTrackingConfig")]
    
    pub object_tracking_config: Option<GoogleCloudVideointelligenceV1_ObjectTrackingConfig>,
    /// Config for PERSON_DETECTION.
    #[serde(rename="personDetectionConfig")]
    
    pub person_detection_config: Option<GoogleCloudVideointelligenceV1_PersonDetectionConfig>,
    /// Video segments to annotate. The segments may overlap and are not required to be contiguous or span the whole video. If unspecified, each video is treated as a single segment.
    
    pub segments: Option<Vec<GoogleCloudVideointelligenceV1_VideoSegment>>,
    /// Config for SHOT_CHANGE_DETECTION.
    #[serde(rename="shotChangeDetectionConfig")]
    
    pub shot_change_detection_config: Option<GoogleCloudVideointelligenceV1_ShotChangeDetectionConfig>,
    /// Config for SPEECH_TRANSCRIPTION.
    #[serde(rename="speechTranscriptionConfig")]
    
    pub speech_transcription_config: Option<GoogleCloudVideointelligenceV1_SpeechTranscriptionConfig>,
    /// Config for TEXT_DETECTION.
    #[serde(rename="textDetectionConfig")]
    
    pub text_detection_config: Option<GoogleCloudVideointelligenceV1_TextDetectionConfig>,
}

impl client::Part for GoogleCloudVideointelligenceV1_VideoContext {}


/// Video segment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudVideointelligenceV1_VideoSegment {
    /// Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive).
    #[serde(rename="endTimeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub end_time_offset: Option<client::chrono::Duration>,
    /// Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive).
    #[serde(rename="startTimeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub start_time_offset: Option<client::chrono::Duration>,
}

impl client::Part for GoogleCloudVideointelligenceV1_VideoSegment {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunning_CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleLongrunning_CancelOperationRequest {}


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
pub struct GoogleLongrunning_ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<GoogleLongrunning_Operation>>,
}

impl client::ResponseResult for GoogleLongrunning_ListOperationsResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations operations get operations](OperationProjectLocationOperationGetCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [annotate videos](VideoAnnotateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunning_Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<GoogleRpc_Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleLongrunning_Operation {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations operations cancel operations](OperationProjectLocationOperationCancelCall) (response)
/// * [projects locations operations delete operations](OperationProjectLocationOperationDeleteCall) (response)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobuf_Empty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobuf_Empty {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpc_Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpc_Status {}


