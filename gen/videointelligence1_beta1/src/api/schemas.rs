use super::*;
/// This resource represents a long-running operation that is the result of a
/// network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotate videos](VideoAnnotateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunning_Operation {
    /// If the value is `false`, it means the operation is still in progress.
    /// If `true`, the operation is completed, and either `error` or `response` is
    /// available.
    
    pub done: Option<bool>,
    /// The normal response of the operation in case of success.  If the original
    /// method returns no data on success, such as `Delete`, the response is
    /// `google.protobuf.Empty`.  If the original method is standard
    /// `Get`/`Create`/`Update`, the response should be the resource.  For other
    /// methods, the response should have the type `XxxResponse`, where `Xxx`
    /// is the original method name.  For example, if the original method name
    /// is `TakeSnapshot()`, the inferred response type is
    /// `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that
    /// originally returns it. If you use the default HTTP mapping, the
    /// `name` should have the format of `operations/some/unique/name`.
    
    pub name: Option<String>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<GoogleRpc_Status>,
    /// Service-specific metadata associated with the operation.  It typically
    /// contains progress information and common metadata such as create time.
    /// Some services might not provide such metadata.  Any method that returns a
    /// long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleLongrunning_Operation {}


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
pub struct GoogleCloudVideointelligenceV1beta1_AnnotateVideoRequest {
    /// Optional location where the output (in JSON format) should be stored.
    /// Currently, only [Google Cloud Storage](https://cloud.google.com/storage/)
    /// URIs are supported, which must be specified in the following format:
    /// `gs://bucket-id/object-id` (other URI formats return
    /// google.rpc.Code.INVALID_ARGUMENT). For more information, see
    /// [Request URIs](https://cloud.google.com/storage/docs/reference-uris).
    #[serde(rename="outputUri")]
    
    pub output_uri: Option<String>,
    /// Requested video annotation features.
    
    pub features: Option<Vec<GoogleCloudVideointelligenceV1beta1AnnotateVideoRequestFeaturesEnum>>,
    /// Additional video context and/or feature-specific parameters.
    #[serde(rename="videoContext")]
    
    pub video_context: Option<GoogleCloudVideointelligenceV1beta1_VideoContext>,
    /// Optional cloud region where annotation should take place. Supported cloud
    /// regions: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region
    /// is specified, a region will be determined based on video file location.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Input video location. Currently, only
    /// [Google Cloud Storage](https://cloud.google.com/storage/) URIs are
    /// supported, which must be specified in the following format:
    /// `gs://bucket-id/object-id` (other URI formats return
    /// google.rpc.Code.INVALID_ARGUMENT). For more information, see
    /// [Request URIs](https://cloud.google.com/storage/docs/reference-uris).
    /// A video URI may include wildcards in `object-id`, and thus identify
    /// multiple videos. Supported wildcards: ‘\*’ to match 0 or more characters;
    /// ‘?’ to match 1 character. If unset, the input video should be embedded
    /// in the request as `input_content`. If set, `input_content` should be unset.
    #[serde(rename="inputUri")]
    
    pub input_uri: Option<String>,
    /// The video data bytes. Encoding: base64. If unset, the input video(s)
    /// should be specified via `input_uri`. If set, `input_uri` should be unset.
    #[serde(rename="inputContent")]
    
    pub input_content: Option<String>,
}

impl client::RequestValue for GoogleCloudVideointelligenceV1beta1_AnnotateVideoRequest {}


/// Video segment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudVideointelligenceV1beta1_VideoSegment {
    /// End offset in microseconds (inclusive). Unset means 0.
    #[serde(rename="endTimeOffset")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_time_offset: Option<i64>,
    /// Start offset in microseconds (inclusive). Unset means 0.
    #[serde(rename="startTimeOffset")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time_offset: Option<i64>,
}

impl client::Part for GoogleCloudVideointelligenceV1beta1_VideoSegment {}


/// Video context and/or feature-specific parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudVideointelligenceV1beta1_VideoContext {
    /// Model to use for safe search detection.
    /// Supported values: "latest" and "stable" (the default).
    #[serde(rename="safeSearchDetectionModel")]
    
    pub safe_search_detection_model: Option<String>,
    /// Video segments to annotate. The segments may overlap and are not required
    /// to be contiguous or span the whole video. If unspecified, each video
    /// is treated as a single segment.
    
    pub segments: Option<Vec<GoogleCloudVideointelligenceV1beta1_VideoSegment>>,
    /// Model to use for label detection.
    /// Supported values: "latest" and "stable" (the default).
    #[serde(rename="labelDetectionModel")]
    
    pub label_detection_model: Option<String>,
    /// Model to use for shot change detection.
    /// Supported values: "latest" and "stable" (the default).
    #[serde(rename="shotChangeDetectionModel")]
    
    pub shot_change_detection_model: Option<String>,
    /// If label detection has been requested, what labels should be detected
    /// in addition to video-level labels or segment-level labels. If unspecified,
    /// defaults to `SHOT_MODE`.
    #[serde(rename="labelDetectionMode")]
    
    pub label_detection_mode: Option<GoogleCloudVideointelligenceV1beta1VideoContextLabelDetectionModeEnum>,
    /// Whether the video has been shot from a stationary (i.e. non-moving) camera.
    /// When set to true, might improve detection accuracy for moving objects.
    #[serde(rename="stationaryCamera")]
    
    pub stationary_camera: Option<bool>,
}

impl client::Part for GoogleCloudVideointelligenceV1beta1_VideoContext {}


/// The `Status` type defines a logical error model that is suitable for different
/// programming environments, including REST APIs and RPC APIs. It is used by
/// [gRPC](https://github.com/grpc). The error model is designed to be:
/// 
/// * Simple to use and understand for most users
/// * Flexible enough to meet unexpected needs
/// 
/// # Overview
/// 
/// The `Status` message contains three pieces of data: error code, error message,
/// and error details. The error code should be an enum value of
/// google.rpc.Code, but it may accept additional error codes if needed.  The
/// error message should be a developer-facing English message that helps
/// developers *understand* and *resolve* the error. If a localized user-facing
/// error message is needed, put the localized message in the error details or
/// localize it in the client. The optional error details may contain arbitrary
/// information about the error. There is a predefined set of error detail types
/// in the package `google.rpc` that can be used for common error conditions.
/// 
/// # Language mapping
/// 
/// The `Status` message is the logical representation of the error model, but it
/// is not necessarily the actual wire format. When the `Status` message is
/// exposed in different client libraries and different wire protocols, it can be
/// mapped differently. For example, it will likely be mapped to some exceptions
/// in Java, but more likely mapped to some error codes in C.
/// 
/// # Other uses
/// 
/// The error model and the `Status` message can be used in a variety of
/// environments, either with or without APIs, to provide a
/// consistent developer experience across different environments.
/// 
/// Example uses of this error model include:
/// 
/// * Partial errors. If a service needs to return partial errors to the client,
///   it may embed the `Status` in the normal response to indicate the partial
///   errors.
/// 
/// * Workflow errors. A typical workflow has multiple steps. Each step may
///   have a `Status` message for error reporting.
/// 
/// * Batch operations. If a client uses batch request and batch response, the
///   `Status` message should be used directly inside batch response, one for
///   each error sub-response.
/// 
/// * Asynchronous operations. If an API call embeds asynchronous operation
///   results in its response, the status of those operations should be
///   represented directly using the `Status` message.
/// 
/// * Logging. If some API errors are stored in logs, the message `Status` could
///   be used directly after any stripping needed for security/privacy reasons.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpc_Status {
    /// A list of messages that carry the error details.  There is a common set of
    /// message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A developer-facing error message, which should be in English. Any
    /// user-facing error message should be localized and sent in the
    /// google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpc_Status {}


