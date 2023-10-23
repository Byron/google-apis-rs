use super::*;
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ABNFGrammar {
    /// All declarations and rules of an ABNF grammar broken up into multiple strings that will end up concatenated.
    #[serde(rename="abnfStrings")]
    
    pub abnf_strings: Option<Vec<String>>,
}

impl client::Part for ABNFGrammar {}


/// An item of the class.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClassItem {
    /// The class item's value.
    
    pub value: Option<String>,
}

impl client::Part for ClassItem {}


/// Message sent by the client for the `CreateCustomClass` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations custom classes create projects](ProjectLocationCustomClassCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateCustomClassRequest {
    /// Required. The custom class to create.
    #[serde(rename="customClass")]
    
    pub custom_class: Option<CustomClass>,
    /// Required. The ID to use for the custom class, which will become the final component of the custom class' resource name. This value should restrict to letters, numbers, and hyphens, with the first character a letter, the last a letter or a number, and be 4-63 characters.
    #[serde(rename="customClassId")]
    
    pub custom_class_id: Option<String>,
}

impl client::RequestValue for CreateCustomClassRequest {}


/// Message sent by the client for the `CreatePhraseSet` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations phrase sets create projects](ProjectLocationPhraseSetCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreatePhraseSetRequest {
    /// Required. The phrase set to create.
    #[serde(rename="phraseSet")]
    
    pub phrase_set: Option<PhraseSet>,
    /// Required. The ID to use for the phrase set, which will become the final component of the phrase set's resource name. This value should restrict to letters, numbers, and hyphens, with the first character a letter, the last a letter or a number, and be 4-63 characters.
    #[serde(rename="phraseSetId")]
    
    pub phrase_set_id: Option<String>,
}

impl client::RequestValue for CreatePhraseSetRequest {}


/// A set of words or phrases that represents a common concept likely to appear in your audio, for example a list of passenger ship names. CustomClass items can be substituted into placeholders that you set in PhraseSet phrases.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations custom classes create projects](ProjectLocationCustomClassCreateCall) (response)
/// * [locations custom classes get projects](ProjectLocationCustomClassGetCall) (response)
/// * [locations custom classes patch projects](ProjectLocationCustomClassPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomClass {
    /// If this custom class is a resource, the custom_class_id is the resource id of the CustomClass. Case sensitive.
    #[serde(rename="customClassId")]
    
    pub custom_class_id: Option<String>,
    /// A collection of class items.
    
    pub items: Option<Vec<ClassItem>>,
    /// The resource name of the custom class.
    
    pub name: Option<String>,
}

impl client::RequestValue for CustomClass {}
impl client::ResponseResult for CustomClass {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations custom classes delete projects](ProjectLocationCustomClassDeleteCall) (response)
/// * [locations phrase sets delete projects](ProjectLocationPhraseSetDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Message returned to the client by the `ListCustomClasses` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations custom classes list projects](ProjectLocationCustomClassListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCustomClassesResponse {
    /// The custom classes.
    #[serde(rename="customClasses")]
    
    pub custom_classes: Option<Vec<CustomClass>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCustomClassesResponse {}


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


/// Message returned to the client by the `ListPhraseSet` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations phrase sets list projects](ProjectLocationPhraseSetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPhraseSetResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The phrase set.
    #[serde(rename="phraseSets")]
    
    pub phrase_sets: Option<Vec<PhraseSet>>,
}

impl client::ResponseResult for ListPhraseSetResponse {}


/// The top-level message sent by the client for the `LongRunningRecognize` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [longrunningrecognize speech](SpeechLongrunningrecognizeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LongRunningRecognizeRequest {
    /// Required. The audio data to be recognized.
    
    pub audio: Option<RecognitionAudio>,
    /// Required. Provides information to the recognizer that specifies how to process the request.
    
    pub config: Option<RecognitionConfig>,
    /// Optional. Specifies an optional destination for the recognition results.
    #[serde(rename="outputConfig")]
    
    pub output_config: Option<TranscriptOutputConfig>,
}

impl client::RequestValue for LongRunningRecognizeRequest {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get operations](OperationGetCall) (response)
/// * [list operations](OperationListCall) (none)
/// * [longrunningrecognize speech](SpeechLongrunningrecognizeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
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

impl client::Resource for Operation {}
impl client::ResponseResult for Operation {}


/// A phrases containing words and phrase "hints" so that the speech recognition is more likely to recognize them. This can be used to improve the accuracy for specific words and phrases, for example, if specific commands are typically spoken by the user. This can also be used to add additional words to the vocabulary of the recognizer. See [usage limits](https://cloud.google.com/speech-to-text/quotas#content). List items can also include pre-built or custom classes containing groups of words that represent common concepts that occur in natural language. For example, rather than providing a phrase hint for every month of the year (e.g. "i was born in january", "i was born in febuary", ...), use the pre-built `$MONTH` class improves the likelihood of correctly transcribing audio that includes months (e.g. "i was born in $month"). To refer to pre-built classes, use the class' symbol prepended with `$` e.g. `$MONTH`. To refer to custom classes that were defined inline in the request, set the class's `custom_class_id` to a string unique to all class resources and inline classes. Then use the class' id wrapped in $`{...}` e.g. "${my-months}". To refer to custom classes resources, use the class' id wrapped in `${}` (e.g. `${my-months}`). Speech-to-Text supports three locations: `global`, `us` (US North America), and `eu` (Europe). If you are calling the `speech.googleapis.com` endpoint, use the `global` location. To specify a region, use a [regional endpoint](https://cloud.google.com/speech-to-text/docs/endpoints) with matching `us` or `eu` location value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Phrase {
    /// Hint Boost. Overrides the boost set at the phrase set level. Positive value will increase the probability that a specific phrase will be recognized over other similar sounding phrases. The higher the boost, the higher the chance of false positive recognition as well. Negative boost will simply be ignored. Though `boost` can accept a wide range of positive values, most use cases are best served with values between 0 and 20. We recommend using a binary search approach to finding the optimal value for your use case as well as adding phrases both with and without boost to your requests.
    
    pub boost: Option<f32>,
    /// The phrase itself.
    
    pub value: Option<String>,
}

impl client::Part for Phrase {}


/// Provides “hints” to the speech recognizer to favor specific words and phrases in the results.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations phrase sets create projects](ProjectLocationPhraseSetCreateCall) (response)
/// * [locations phrase sets get projects](ProjectLocationPhraseSetGetCall) (response)
/// * [locations phrase sets patch projects](ProjectLocationPhraseSetPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PhraseSet {
    /// Hint Boost. Positive value will increase the probability that a specific phrase will be recognized over other similar sounding phrases. The higher the boost, the higher the chance of false positive recognition as well. Negative boost values would correspond to anti-biasing. Anti-biasing is not enabled, so negative boost will simply be ignored. Though `boost` can accept a wide range of positive values, most use cases are best served with values between 0 (exclusive) and 20. We recommend using a binary search approach to finding the optimal value for your use case as well as adding phrases both with and without boost to your requests.
    
    pub boost: Option<f32>,
    /// The resource name of the phrase set.
    
    pub name: Option<String>,
    /// A list of word and phrases.
    
    pub phrases: Option<Vec<Phrase>>,
}

impl client::RequestValue for PhraseSet {}
impl client::ResponseResult for PhraseSet {}


/// Contains audio data in the encoding specified in the `RecognitionConfig`. Either `content` or `uri` must be supplied. Supplying both or neither returns google.rpc.Code.INVALID_ARGUMENT. See [content limits](https://cloud.google.com/speech-to-text/quotas#content).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecognitionAudio {
    /// The audio data bytes encoded as specified in `RecognitionConfig`. Note: as with all bytes fields, proto buffers use a pure binary representation, whereas JSON representations use base64.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub content: Option<Vec<u8>>,
    /// URI that points to a file that contains audio data bytes as specified in `RecognitionConfig`. The file must not be compressed (for example, gzip). Currently, only Google Cloud Storage URIs are supported, which must be specified in the following format: `gs://bucket_name/object_name` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/reference-uris).
    
    pub uri: Option<String>,
}

impl client::Part for RecognitionAudio {}


/// Provides information to the recognizer that specifies how to process the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecognitionConfig {
    /// Speech adaptation configuration improves the accuracy of speech recognition. For more information, see the [speech adaptation](https://cloud.google.com/speech-to-text/docs/adaptation) documentation. When speech adaptation is set it supersedes the `speech_contexts` field.
    
    pub adaptation: Option<SpeechAdaptation>,
    /// A list of up to 3 additional [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tags, listing possible alternative languages of the supplied audio. See [Language Support](https://cloud.google.com/speech-to-text/docs/languages) for a list of the currently supported language codes. If alternative languages are listed, recognition result will contain recognition in the most likely language detected including the main language_code. The recognition result will include the language tag of the language detected in the audio. Note: This feature is only supported for Voice Command and Voice Search use cases and performance may vary for other use cases (e.g., phone call transcription).
    #[serde(rename="alternativeLanguageCodes")]
    
    pub alternative_language_codes: Option<Vec<String>>,
    /// The number of channels in the input audio data. ONLY set this for MULTI-CHANNEL recognition. Valid values for LINEAR16, OGG_OPUS and FLAC are `1`-`8`. Valid value for MULAW, AMR, AMR_WB and SPEEX_WITH_HEADER_BYTE is only `1`. If `0` or omitted, defaults to one channel (mono). Note: We only recognize the first channel by default. To perform independent recognition on each channel set `enable_separate_recognition_per_channel` to 'true'.
    #[serde(rename="audioChannelCount")]
    
    pub audio_channel_count: Option<i32>,
    /// Config to enable speaker diarization and set additional parameters to make diarization better suited for your application. Note: When this is enabled, we send all the words from the beginning of the audio for the top alternative in every consecutive STREAMING responses. This is done in order to improve our speaker tags as our models learn to identify the speakers in the conversation over time. For non-streaming requests, the diarization results will be provided only in the top alternative of the FINAL SpeechRecognitionResult.
    #[serde(rename="diarizationConfig")]
    
    pub diarization_config: Option<SpeakerDiarizationConfig>,
    /// If 'true', adds punctuation to recognition result hypotheses. This feature is only available in select languages. Setting this for requests in other languages has no effect at all. The default 'false' value does not add punctuation to result hypotheses.
    #[serde(rename="enableAutomaticPunctuation")]
    
    pub enable_automatic_punctuation: Option<bool>,
    /// This needs to be set to `true` explicitly and `audio_channel_count` > 1 to get each channel recognized separately. The recognition result will contain a `channel_tag` field to state which channel that result belongs to. If this is not true, we will only recognize the first channel. The request is billed cumulatively for all channels recognized: `audio_channel_count` multiplied by the length of the audio.
    #[serde(rename="enableSeparateRecognitionPerChannel")]
    
    pub enable_separate_recognition_per_channel: Option<bool>,
    /// The spoken emoji behavior for the call If not set, uses default behavior based on model of choice If 'true', adds spoken emoji formatting for the request. This will replace spoken emojis with the corresponding Unicode symbols in the final transcript. If 'false', spoken emojis are not replaced.
    #[serde(rename="enableSpokenEmojis")]
    
    pub enable_spoken_emojis: Option<bool>,
    /// The spoken punctuation behavior for the call If not set, uses default behavior based on model of choice e.g. command_and_search will enable spoken punctuation by default If 'true', replaces spoken punctuation with the corresponding symbols in the request. For example, "how are you question mark" becomes "how are you?". See https://cloud.google.com/speech-to-text/docs/spoken-punctuation for support. If 'false', spoken punctuation is not replaced.
    #[serde(rename="enableSpokenPunctuation")]
    
    pub enable_spoken_punctuation: Option<bool>,
    /// If `true`, the top result includes a list of words and the confidence for those words. If `false`, no word-level confidence information is returned. The default is `false`.
    #[serde(rename="enableWordConfidence")]
    
    pub enable_word_confidence: Option<bool>,
    /// If `true`, the top result includes a list of words and the start and end time offsets (timestamps) for those words. If `false`, no word-level time offset information is returned. The default is `false`.
    #[serde(rename="enableWordTimeOffsets")]
    
    pub enable_word_time_offsets: Option<bool>,
    /// Encoding of audio data sent in all `RecognitionAudio` messages. This field is optional for `FLAC` and `WAV` audio files and required for all other audio formats. For details, see AudioEncoding.
    
    pub encoding: Option<RecognitionConfigEncodingEnum>,
    /// Required. The language of the supplied audio as a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US". See [Language Support](https://cloud.google.com/speech-to-text/docs/languages) for a list of the currently supported language codes.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Maximum number of recognition hypotheses to be returned. Specifically, the maximum number of `SpeechRecognitionAlternative` messages within each `SpeechRecognitionResult`. The server may return fewer than `max_alternatives`. Valid values are `0`-`30`. A value of `0` or `1` will return a maximum of one. If omitted, will return a maximum of one.
    #[serde(rename="maxAlternatives")]
    
    pub max_alternatives: Option<i32>,
    /// Metadata regarding this request.
    
    pub metadata: Option<RecognitionMetadata>,
    /// Which model to select for the given request. Select the model best suited to your domain to get best results. If a model is not explicitly specified, then we auto-select a model based on the parameters in the RecognitionConfig. *Model* *Description* latest_long Best for long form content like media or conversation. latest_short Best for short form content like commands or single shot directed speech. command_and_search Best for short queries such as voice commands or voice search. phone_call Best for audio that originated from a phone call (typically recorded at an 8khz sampling rate). video Best for audio that originated from video or includes multiple speakers. Ideally the audio is recorded at a 16khz or greater sampling rate. This is a premium model that costs more than the standard rate. default Best for audio that is not one of the specific audio models. For example, long-form audio. Ideally the audio is high-fidelity, recorded at a 16khz or greater sampling rate. medical_conversation Best for audio that originated from a conversation between a medical provider and patient. medical_dictation Best for audio that originated from dictation notes by a medical provider. 
    
    pub model: Option<String>,
    /// If set to `true`, the server will attempt to filter out profanities, replacing all but the initial character in each filtered word with asterisks, e.g. "f***". If set to `false` or omitted, profanities won't be filtered out.
    #[serde(rename="profanityFilter")]
    
    pub profanity_filter: Option<bool>,
    /// Sample rate in Hertz of the audio data sent in all `RecognitionAudio` messages. Valid values are: 8000-48000. 16000 is optimal. For best results, set the sampling rate of the audio source to 16000 Hz. If that's not possible, use the native sample rate of the audio source (instead of re-sampling). This field is optional for FLAC and WAV audio files, but is required for all other audio formats. For details, see AudioEncoding.
    #[serde(rename="sampleRateHertz")]
    
    pub sample_rate_hertz: Option<i32>,
    /// Array of SpeechContext. A means to provide context to assist the speech recognition. For more information, see [speech adaptation](https://cloud.google.com/speech-to-text/docs/adaptation).
    #[serde(rename="speechContexts")]
    
    pub speech_contexts: Option<Vec<SpeechContext>>,
    /// Set to true to use an enhanced model for speech recognition. If `use_enhanced` is set to true and the `model` field is not set, then an appropriate enhanced model is chosen if an enhanced model exists for the audio. If `use_enhanced` is true and an enhanced version of the specified model does not exist, then the speech is recognized using the standard version of the specified model.
    #[serde(rename="useEnhanced")]
    
    pub use_enhanced: Option<bool>,
}

impl client::Part for RecognitionConfig {}


/// Description of audio data to be recognized.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecognitionMetadata {
    /// Description of the content. Eg. "Recordings of federal supreme court hearings from 2012".
    #[serde(rename="audioTopic")]
    
    pub audio_topic: Option<String>,
    /// The industry vertical to which this speech recognition request most closely applies. This is most indicative of the topics contained in the audio. Use the 6-digit NAICS code to identify the industry vertical - see https://www.naics.com/search/.
    #[serde(rename="industryNaicsCodeOfAudio")]
    
    pub industry_naics_code_of_audio: Option<u32>,
    /// The use case most closely describing the audio content to be recognized.
    #[serde(rename="interactionType")]
    
    pub interaction_type: Option<RecognitionMetadataInteractionTypeEnum>,
    /// The audio type that most closely describes the audio being recognized.
    #[serde(rename="microphoneDistance")]
    
    pub microphone_distance: Option<RecognitionMetadataMicrophoneDistanceEnum>,
    /// The original media the speech was recorded on.
    #[serde(rename="originalMediaType")]
    
    pub original_media_type: Option<RecognitionMetadataOriginalMediaTypeEnum>,
    /// Mime type of the original audio file. For example `audio/m4a`, `audio/x-alaw-basic`, `audio/mp3`, `audio/3gpp`. A list of possible audio mime types is maintained at http://www.iana.org/assignments/media-types/media-types.xhtml#audio
    #[serde(rename="originalMimeType")]
    
    pub original_mime_type: Option<String>,
    /// The device used to make the recording. Examples 'Nexus 5X' or 'Polycom SoundStation IP 6000' or 'POTS' or 'VoIP' or 'Cardioid Microphone'.
    #[serde(rename="recordingDeviceName")]
    
    pub recording_device_name: Option<String>,
    /// The type of device the speech was recorded with.
    #[serde(rename="recordingDeviceType")]
    
    pub recording_device_type: Option<RecognitionMetadataRecordingDeviceTypeEnum>,
}

impl client::Part for RecognitionMetadata {}


/// The top-level message sent by the client for the `Recognize` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [recognize speech](SpeechRecognizeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecognizeRequest {
    /// Required. The audio data to be recognized.
    
    pub audio: Option<RecognitionAudio>,
    /// Required. Provides information to the recognizer that specifies how to process the request.
    
    pub config: Option<RecognitionConfig>,
}

impl client::RequestValue for RecognizeRequest {}


/// The only message returned to the client by the `Recognize` method. It contains the result as zero or more sequential `SpeechRecognitionResult` messages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [recognize speech](SpeechRecognizeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecognizeResponse {
    /// The ID associated with the request. This is a unique ID specific only to the given request.
    #[serde(rename="requestId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub request_id: Option<i64>,
    /// Sequential list of transcription results corresponding to sequential portions of audio.
    
    pub results: Option<Vec<SpeechRecognitionResult>>,
    /// Provides information on adaptation behavior in response
    #[serde(rename="speechAdaptationInfo")]
    
    pub speech_adaptation_info: Option<SpeechAdaptationInfo>,
    /// When available, billed audio seconds for the corresponding request.
    #[serde(rename="totalBilledTime")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub total_billed_time: Option<client::chrono::Duration>,
}

impl client::ResponseResult for RecognizeResponse {}


/// Config to enable speaker diarization.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpeakerDiarizationConfig {
    /// If 'true', enables speaker detection for each recognized word in the top alternative of the recognition result using a speaker_tag provided in the WordInfo.
    #[serde(rename="enableSpeakerDiarization")]
    
    pub enable_speaker_diarization: Option<bool>,
    /// Maximum number of speakers in the conversation. This range gives you more flexibility by allowing the system to automatically determine the correct number of speakers. If not set, the default value is 6.
    #[serde(rename="maxSpeakerCount")]
    
    pub max_speaker_count: Option<i32>,
    /// Minimum number of speakers in the conversation. This range gives you more flexibility by allowing the system to automatically determine the correct number of speakers. If not set, the default value is 2.
    #[serde(rename="minSpeakerCount")]
    
    pub min_speaker_count: Option<i32>,
    /// Output only. Unused.
    #[serde(rename="speakerTag")]
    
    pub speaker_tag: Option<i32>,
}

impl client::Part for SpeakerDiarizationConfig {}


/// Speech adaptation configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpeechAdaptation {
    /// Augmented Backus-Naur form (ABNF) is a standardized grammar notation comprised by a set of derivation rules. See specifications: https://www.w3.org/TR/speech-grammar
    #[serde(rename="abnfGrammar")]
    
    pub abnf_grammar: Option<ABNFGrammar>,
    /// A collection of custom classes. To specify the classes inline, leave the class' `name` blank and fill in the rest of its fields, giving it a unique `custom_class_id`. Refer to the inline defined class in phrase hints by its `custom_class_id`.
    #[serde(rename="customClasses")]
    
    pub custom_classes: Option<Vec<CustomClass>>,
    /// A collection of phrase set resource names to use.
    #[serde(rename="phraseSetReferences")]
    
    pub phrase_set_references: Option<Vec<String>>,
    /// A collection of phrase sets. To specify the hints inline, leave the phrase set's `name` blank and fill in the rest of its fields. Any phrase set can use any custom class.
    #[serde(rename="phraseSets")]
    
    pub phrase_sets: Option<Vec<PhraseSet>>,
}

impl client::Part for SpeechAdaptation {}


/// Information on speech adaptation use in results
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpeechAdaptationInfo {
    /// Whether there was a timeout when applying speech adaptation. If true, adaptation had no effect in the response transcript.
    #[serde(rename="adaptationTimeout")]
    
    pub adaptation_timeout: Option<bool>,
    /// If set, returns a message specifying which part of the speech adaptation request timed out.
    #[serde(rename="timeoutMessage")]
    
    pub timeout_message: Option<String>,
}

impl client::Part for SpeechAdaptationInfo {}


/// Provides "hints" to the speech recognizer to favor specific words and phrases in the results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpeechContext {
    /// Hint Boost. Positive value will increase the probability that a specific phrase will be recognized over other similar sounding phrases. The higher the boost, the higher the chance of false positive recognition as well. Negative boost values would correspond to anti-biasing. Anti-biasing is not enabled, so negative boost will simply be ignored. Though `boost` can accept a wide range of positive values, most use cases are best served with values between 0 and 20. We recommend using a binary search approach to finding the optimal value for your use case.
    
    pub boost: Option<f32>,
    /// A list of strings containing words and phrases "hints" so that the speech recognition is more likely to recognize them. This can be used to improve the accuracy for specific words and phrases, for example, if specific commands are typically spoken by the user. This can also be used to add additional words to the vocabulary of the recognizer. See [usage limits](https://cloud.google.com/speech-to-text/quotas#content). List items can also be set to classes for groups of words that represent common concepts that occur in natural language. For example, rather than providing phrase hints for every month of the year, using the $MONTH class improves the likelihood of correctly transcribing audio that includes months.
    
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
    /// The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative of a non-streaming result or, of a streaming result where `is_final=true`. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set.
    
    pub confidence: Option<f32>,
    /// Transcript text representing the words that the user spoke. In languages that use spaces to separate words, the transcript might have a leading space if it isn't the first result. You can concatenate each result to obtain the full transcript without using a separator.
    
    pub transcript: Option<String>,
    /// A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is true, you will see all the words from the beginning of the audio.
    
    pub words: Option<Vec<WordInfo>>,
}

impl client::Part for SpeechRecognitionAlternative {}


/// A speech recognition result corresponding to a portion of the audio.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpeechRecognitionResult {
    /// May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer.
    
    pub alternatives: Option<Vec<SpeechRecognitionAlternative>>,
    /// For multi-channel audio, this is the channel number corresponding to the recognized result for the audio from that channel. For audio_channel_count = N, its output values can range from '1' to 'N'.
    #[serde(rename="channelTag")]
    
    pub channel_tag: Option<i32>,
    /// Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Time offset of the end of this result relative to the beginning of the audio.
    #[serde(rename="resultEndTime")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub result_end_time: Option<client::chrono::Duration>,
}

impl client::Part for SpeechRecognitionResult {}


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


/// Specifies an optional destination for the recognition results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TranscriptOutputConfig {
    /// Specifies a Cloud Storage URI for the recognition results. Must be specified in the format: `gs://bucket_name/object_name`, and the bucket must already exist.
    #[serde(rename="gcsUri")]
    
    pub gcs_uri: Option<String>,
}

impl client::Part for TranscriptOutputConfig {}


/// Word-specific information for recognized words.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WordInfo {
    /// The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative of a non-streaming result or, of a streaming result where `is_final=true`. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set.
    
    pub confidence: Option<f32>,
    /// Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary.
    #[serde(rename="endTime")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub end_time: Option<client::chrono::Duration>,
    /// Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from '1' to diarization_speaker_count. speaker_tag is set if enable_speaker_diarization = 'true' and only in the top alternative.
    #[serde(rename="speakerTag")]
    
    pub speaker_tag: Option<i32>,
    /// Time offset relative to the beginning of the audio, and corresponding to the start of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary.
    #[serde(rename="startTime")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub start_time: Option<client::chrono::Duration>,
    /// The word corresponding to this set of information.
    
    pub word: Option<String>,
}

impl client::Part for WordInfo {}


