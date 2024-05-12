use super::*;
/// This resource represents a long-running operation that is the result of a
/// network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list operations](OperationListCall) (none)
/// * [get operations](OperationGetCall) (response)
/// * [asyncrecognize speech](SpeechAsyncrecognizeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
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
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation.  It typically
    /// contains progress information and common metadata such as create time.
    /// Some services might not provide such metadata.  Any method that returns a
    /// long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
}

impl client::Resource for Operation {}
impl client::ResponseResult for Operation {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list operations](OperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListOperationsResponse {}


/// Provides "hints" to the speech recognizer to favor specific words and phrases
/// in the results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpeechContext {
    /// *Optional* A list of strings containing words and phrases "hints" so that
    /// the speech recognition is more likely to recognize them. This can be used
    /// to improve the accuracy for specific words and phrases, for example, if
    /// specific commands are typically spoken by the user. This can also be used
    /// to add additional words to the vocabulary of the recognizer. See
    /// [usage limits](https://cloud.google.com/speech/limits#content).
    
    pub phrases: Option<Vec<String>>,
}

impl client::Part for SpeechContext {}


/// Alternative hypotheses (a.k.a. n-best list).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpeechRecognitionAlternative {
    /// *Output-only* The confidence estimate between 0.0 and 1.0. A higher number
    /// indicates an estimated greater likelihood that the recognized words are
    /// correct. This field is typically provided only for the top hypothesis, and
    /// only for `is_final=true` results. Clients should not rely on the
    /// `confidence` field as it is not guaranteed to be accurate, or even set, in
    /// any of the results.
    /// The default of 0.0 is a sentinel value indicating `confidence` was not set.
    
    pub confidence: Option<f32>,
    /// *Output-only* Transcript text representing the words that the user spoke.
    
    pub transcript: Option<String>,
}

impl client::Part for SpeechRecognitionAlternative {}


/// A speech recognition result corresponding to a portion of the audio.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpeechRecognitionResult {
    /// *Output-only* May contain one or more recognition hypotheses (up to the
    /// maximum specified in `max_alternatives`).
    
    pub alternatives: Option<Vec<SpeechRecognitionAlternative>>,
}

impl client::Part for SpeechRecognitionResult {}


/// Provides information to the recognizer that specifies how to process the
/// request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecognitionConfig {
    /// *Optional* Maximum number of recognition hypotheses to be returned.
    /// Specifically, the maximum number of `SpeechRecognitionAlternative` messages
    /// within each `SpeechRecognitionResult`.
    /// The server may return fewer than `max_alternatives`.
    /// Valid values are `0`-`30`. A value of `0` or `1` will return a maximum of
    /// one. If omitted, will return a maximum of one.
    #[serde(rename="maxAlternatives")]
    
    pub max_alternatives: Option<i32>,
    /// *Required* Sample rate in Hertz of the audio data sent in all
    /// `RecognitionAudio` messages. Valid values are: 8000-48000.
    /// 16000 is optimal. For best results, set the sampling rate of the audio
    /// source to 16000 Hz. If that's not possible, use the native sample rate of
    /// the audio source (instead of re-sampling).
    #[serde(rename="sampleRate")]
    
    pub sample_rate: Option<i32>,
    /// *Optional* The language of the supplied audio as a BCP-47 language tag.
    /// Example: "en-GB"  https://www.rfc-editor.org/rfc/bcp/bcp47.txt
    /// If omitted, defaults to "en-US". See
    /// [Language Support](https://cloud.google.com/speech/docs/languages)
    /// for a list of the currently supported language codes.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// *Optional* If set to `true`, the server will attempt to filter out
    /// profanities, replacing all but the initial character in each filtered word
    /// with asterisks, e.g. "f***". If set to `false` or omitted, profanities
    /// won't be filtered out.
    #[serde(rename="profanityFilter")]
    
    pub profanity_filter: Option<bool>,
    /// *Optional* A means to provide context to assist the speech recognition.
    #[serde(rename="speechContext")]
    
    pub speech_context: Option<SpeechContext>,
    /// *Required* Encoding of audio data sent in all `RecognitionAudio` messages.
    
    pub encoding: Option<RecognitionConfigEncodingEnum>,
}

impl client::Part for RecognitionConfig {}


/// The top-level message sent by the client for the `AsyncRecognize` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [asyncrecognize speech](SpeechAsyncrecognizeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AsyncRecognizeRequest {
    /// *Required* Provides information to the recognizer that specifies how to
    /// process the request.
    
    pub config: Option<RecognitionConfig>,
    /// *Required* The audio data to be recognized.
    
    pub audio: Option<RecognitionAudio>,
}

impl client::RequestValue for AsyncRecognizeRequest {}


/// Contains audio data in the encoding specified in the `RecognitionConfig`.
/// Either `content` or `uri` must be supplied. Supplying both or neither
/// returns google.rpc.Code.INVALID_ARGUMENT. See
/// [audio limits](https://cloud.google.com/speech/limits#content).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecognitionAudio {
    /// URI that points to a file that contains audio data bytes as specified in
    /// `RecognitionConfig`. Currently, only Google Cloud Storage URIs are
    /// supported, which must be specified in the following format:
    /// `gs://bucket_name/object_name` (other URI formats return
    /// google.rpc.Code.INVALID_ARGUMENT). For more information, see
    /// [Request URIs](https://cloud.google.com/storage/docs/reference-uris).
    
    pub uri: Option<String>,
    /// The audio data bytes encoded as specified in
    /// `RecognitionConfig`. Note: as with all bytes fields, protobuffers use a
    /// pure binary representation, whereas JSON representations use base64.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub content: Option<Vec<u8>>,
}

impl client::Part for RecognitionAudio {}


/// The top-level message sent by the client for the `SyncRecognize` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [syncrecognize speech](SpeechSyncrecognizeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SyncRecognizeRequest {
    /// *Required* Provides information to the recognizer that specifies how to
    /// process the request.
    
    pub config: Option<RecognitionConfig>,
    /// *Required* The audio data to be recognized.
    
    pub audio: Option<RecognitionAudio>,
}

impl client::RequestValue for SyncRecognizeRequest {}


/// The only message returned to the client by `SyncRecognize`. method. It
/// contains the result as zero or more sequential `SpeechRecognitionResult`
/// messages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [syncrecognize speech](SpeechSyncrecognizeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SyncRecognizeResponse {
    /// *Output-only* Sequential list of transcription results corresponding to
    /// sequential portions of audio.
    
    pub results: Option<Vec<SpeechRecognitionResult>>,
}

impl client::ResponseResult for SyncRecognizeResponse {}


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
pub struct Status {
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

impl client::Part for Status {}


