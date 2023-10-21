use super::*;
/// The analysis resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversations analyses create projects](ProjectLocationConversationAnalysisCreateCall) (request)
/// * [locations conversations analyses get projects](ProjectLocationConversationAnalysisGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1Analysis {
    /// Output only. The result of the analysis, which is populated when the analysis finishes.
    #[serde(rename="analysisResult")]
    
    pub analysis_result: Option<GoogleCloudContactcenterinsightsV1AnalysisResult>,
    /// To select the annotators to run and the phrase matchers to use (if any). If not specified, all annotators will be run.
    #[serde(rename="annotatorSelector")]
    
    pub annotator_selector: Option<GoogleCloudContactcenterinsightsV1AnnotatorSelector>,
    /// Output only. The time at which the analysis was created, which occurs when the long-running operation completes.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. The resource name of the analysis. Format: projects/{project}/locations/{location}/conversations/{conversation}/analyses/{analysis}
    
    pub name: Option<String>,
    /// Output only. The time at which the analysis was requested.
    #[serde(rename="requestTime")]
    
    pub request_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudContactcenterinsightsV1Analysis {}
impl client::ResponseResult for GoogleCloudContactcenterinsightsV1Analysis {}


/// The result of an analysis.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1AnalysisResult {
    /// Call-specific metadata created by the analysis.
    #[serde(rename="callAnalysisMetadata")]
    
    pub call_analysis_metadata: Option<GoogleCloudContactcenterinsightsV1AnalysisResultCallAnalysisMetadata>,
    /// The time at which the analysis ended.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1AnalysisResult {}


/// Call-specific metadata created during analysis.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1AnalysisResultCallAnalysisMetadata {
    /// A list of call annotations that apply to this call.
    
    pub annotations: Option<Vec<GoogleCloudContactcenterinsightsV1CallAnnotation>>,
    /// All the entities in the call.
    
    pub entities: Option<HashMap<String, GoogleCloudContactcenterinsightsV1Entity>>,
    /// All the matched intents in the call.
    
    pub intents: Option<HashMap<String, GoogleCloudContactcenterinsightsV1Intent>>,
    /// Overall conversation-level issue modeling result.
    #[serde(rename="issueModelResult")]
    
    pub issue_model_result: Option<GoogleCloudContactcenterinsightsV1IssueModelResult>,
    /// All the matched phrase matchers in the call.
    #[serde(rename="phraseMatchers")]
    
    pub phrase_matchers: Option<HashMap<String, GoogleCloudContactcenterinsightsV1PhraseMatchData>>,
    /// Overall conversation-level sentiment for each channel of the call.
    
    pub sentiments: Option<Vec<GoogleCloudContactcenterinsightsV1ConversationLevelSentiment>>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1AnalysisResultCallAnalysisMetadata {}


/// A point in a conversation that marks the start or the end of an annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1AnnotationBoundary {
    /// The index in the sequence of transcribed pieces of the conversation where the boundary is located. This index starts at zero.
    #[serde(rename="transcriptIndex")]
    
    pub transcript_index: Option<i32>,
    /// The word index of this boundary with respect to the first word in the transcript piece. This index starts at zero.
    #[serde(rename="wordIndex")]
    
    pub word_index: Option<i32>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1AnnotationBoundary {}


/// Selector of all available annotators and phrase matchers to run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1AnnotatorSelector {
    /// The issue model to run. If not provided, the most recently deployed topic model will be used. The provided issue model will only be used for inference if the issue model is deployed and if run_issue_model_annotator is set to true. If more than one issue model is provided, only the first provided issue model will be used for inference.
    #[serde(rename="issueModels")]
    
    pub issue_models: Option<Vec<String>>,
    /// The list of phrase matchers to run. If not provided, all active phrase matchers will be used. If inactive phrase matchers are provided, they will not be used. Phrase matchers will be run only if run_phrase_matcher_annotator is set to true. Format: projects/{project}/locations/{location}/phraseMatchers/{phrase_matcher}
    #[serde(rename="phraseMatchers")]
    
    pub phrase_matchers: Option<Vec<String>>,
    /// Whether to run the entity annotator.
    #[serde(rename="runEntityAnnotator")]
    
    pub run_entity_annotator: Option<bool>,
    /// Whether to run the intent annotator.
    #[serde(rename="runIntentAnnotator")]
    
    pub run_intent_annotator: Option<bool>,
    /// Whether to run the interruption annotator.
    #[serde(rename="runInterruptionAnnotator")]
    
    pub run_interruption_annotator: Option<bool>,
    /// Whether to run the issue model annotator. A model should have already been deployed for this to take effect.
    #[serde(rename="runIssueModelAnnotator")]
    
    pub run_issue_model_annotator: Option<bool>,
    /// Whether to run the active phrase matcher annotator(s).
    #[serde(rename="runPhraseMatcherAnnotator")]
    
    pub run_phrase_matcher_annotator: Option<bool>,
    /// Whether to run the sentiment annotator.
    #[serde(rename="runSentimentAnnotator")]
    
    pub run_sentiment_annotator: Option<bool>,
    /// Whether to run the silence annotator.
    #[serde(rename="runSilenceAnnotator")]
    
    pub run_silence_annotator: Option<bool>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1AnnotatorSelector {}


/// The feedback that the customer has about a certain answer in the conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1AnswerFeedback {
    /// Indicates whether an answer or item was clicked by the human agent.
    
    pub clicked: Option<bool>,
    /// The correctness level of an answer.
    #[serde(rename="correctnessLevel")]
    
    pub correctness_level: Option<String>,
    /// Indicates whether an answer or item was displayed to the human agent in the agent desktop UI.
    
    pub displayed: Option<bool>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1AnswerFeedback {}


/// Agent Assist Article Suggestion data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ArticleSuggestionData {
    /// The system's confidence score that this article is a good match for this conversation, ranging from 0.0 (completely uncertain) to 1.0 (completely certain).
    #[serde(rename="confidenceScore")]
    
    pub confidence_score: Option<f32>,
    /// Map that contains metadata about the Article Suggestion and the document that it originates from.
    
    pub metadata: Option<HashMap<String, String>>,
    /// The name of the answer record. Format: projects/{project}/locations/{location}/answerRecords/{answer_record}
    #[serde(rename="queryRecord")]
    
    pub query_record: Option<String>,
    /// The knowledge document that this answer was extracted from. Format: projects/{project}/knowledgeBases/{knowledge_base}/documents/{document}
    
    pub source: Option<String>,
    /// Article title.
    
    pub title: Option<String>,
    /// Article URI.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1ArticleSuggestionData {}


/// The request to analyze conversations in bulk.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversations bulk analyze projects](ProjectLocationConversationBulkAnalyzeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1BulkAnalyzeConversationsRequest {
    /// Required. Percentage of selected conversation to analyze, between [0, 100].
    #[serde(rename="analysisPercentage")]
    
    pub analysis_percentage: Option<f32>,
    /// To select the annotators to run and the phrase matchers to use (if any). If not specified, all annotators will be run.
    #[serde(rename="annotatorSelector")]
    
    pub annotator_selector: Option<GoogleCloudContactcenterinsightsV1AnnotatorSelector>,
    /// Required. Filter used to select the subset of conversations to analyze.
    
    pub filter: Option<String>,
    /// Required. The parent resource to create analyses in.
    
    pub parent: Option<String>,
}

impl client::RequestValue for GoogleCloudContactcenterinsightsV1BulkAnalyzeConversationsRequest {}


/// Response of querying an issue model’s statistics.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations issue models calculate issue model stats projects](ProjectLocationIssueModelCalculateIssueModelStatCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1CalculateIssueModelStatsResponse {
    /// The latest label statistics for the queried issue model. Includes results on both training data and data labeled after deployment.
    #[serde(rename="currentStats")]
    
    pub current_stats: Option<GoogleCloudContactcenterinsightsV1IssueModelLabelStats>,
}

impl client::ResponseResult for GoogleCloudContactcenterinsightsV1CalculateIssueModelStatsResponse {}


/// The response for calculating conversation statistics.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversations calculate stats projects](ProjectLocationConversationCalculateStatCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1CalculateStatsResponse {
    /// The average duration of all conversations. The average is calculated using only conversations that have a time duration.
    #[serde(rename="averageDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub average_duration: Option<client::chrono::Duration>,
    /// The average number of turns per conversation.
    #[serde(rename="averageTurnCount")]
    
    pub average_turn_count: Option<i32>,
    /// The total number of conversations.
    #[serde(rename="conversationCount")]
    
    pub conversation_count: Option<i32>,
    /// A time series representing the count of conversations created over time that match that requested filter criteria.
    #[serde(rename="conversationCountTimeSeries")]
    
    pub conversation_count_time_series: Option<GoogleCloudContactcenterinsightsV1CalculateStatsResponseTimeSeries>,
    /// A map associating each custom highlighter resource name with its respective number of matches in the set of conversations.
    #[serde(rename="customHighlighterMatches")]
    
    pub custom_highlighter_matches: Option<HashMap<String, i32>>,
    /// A map associating each issue resource name with its respective number of matches in the set of conversations. Key has the format: `projects//locations//issueModels//issues/` Deprecated, use `issue_matches_stats` field instead.
    #[serde(rename="issueMatches")]
    
    pub issue_matches: Option<HashMap<String, i32>>,
    /// A map associating each issue resource name with its respective number of matches in the set of conversations. Key has the format: `projects//locations//issueModels//issues/`
    #[serde(rename="issueMatchesStats")]
    
    pub issue_matches_stats: Option<HashMap<String, GoogleCloudContactcenterinsightsV1IssueModelLabelStatsIssueStats>>,
    /// A map associating each smart highlighter display name with its respective number of matches in the set of conversations.
    #[serde(rename="smartHighlighterMatches")]
    
    pub smart_highlighter_matches: Option<HashMap<String, i32>>,
}

impl client::ResponseResult for GoogleCloudContactcenterinsightsV1CalculateStatsResponse {}


/// A time series representing conversations over time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1CalculateStatsResponseTimeSeries {
    /// The duration of each interval.
    #[serde(rename="intervalDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub interval_duration: Option<client::chrono::Duration>,
    /// An ordered list of intervals from earliest to latest, where each interval represents the number of conversations that transpired during the time window.
    
    pub points: Option<Vec<GoogleCloudContactcenterinsightsV1CalculateStatsResponseTimeSeriesInterval>>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1CalculateStatsResponseTimeSeries {}


/// A single interval in a time series.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1CalculateStatsResponseTimeSeriesInterval {
    /// The number of conversations created in this interval.
    #[serde(rename="conversationCount")]
    
    pub conversation_count: Option<i32>,
    /// The start time of this interval.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1CalculateStatsResponseTimeSeriesInterval {}


/// A piece of metadata that applies to a window of a call.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1CallAnnotation {
    /// The boundary in the conversation where the annotation ends, inclusive.
    #[serde(rename="annotationEndBoundary")]
    
    pub annotation_end_boundary: Option<GoogleCloudContactcenterinsightsV1AnnotationBoundary>,
    /// The boundary in the conversation where the annotation starts, inclusive.
    #[serde(rename="annotationStartBoundary")]
    
    pub annotation_start_boundary: Option<GoogleCloudContactcenterinsightsV1AnnotationBoundary>,
    /// The channel of the audio where the annotation occurs. For single-channel audio, this field is not populated.
    #[serde(rename="channelTag")]
    
    pub channel_tag: Option<i32>,
    /// Data specifying an entity mention.
    #[serde(rename="entityMentionData")]
    
    pub entity_mention_data: Option<GoogleCloudContactcenterinsightsV1EntityMentionData>,
    /// Data specifying a hold.
    #[serde(rename="holdData")]
    
    pub hold_data: Option<GoogleCloudContactcenterinsightsV1HoldData>,
    /// Data specifying an intent match.
    #[serde(rename="intentMatchData")]
    
    pub intent_match_data: Option<GoogleCloudContactcenterinsightsV1IntentMatchData>,
    /// Data specifying an interruption.
    #[serde(rename="interruptionData")]
    
    pub interruption_data: Option<GoogleCloudContactcenterinsightsV1InterruptionData>,
    /// Data specifying an issue match.
    #[serde(rename="issueMatchData")]
    
    pub issue_match_data: Option<GoogleCloudContactcenterinsightsV1IssueMatchData>,
    /// Data specifying a phrase match.
    #[serde(rename="phraseMatchData")]
    
    pub phrase_match_data: Option<GoogleCloudContactcenterinsightsV1PhraseMatchData>,
    /// Data specifying sentiment.
    #[serde(rename="sentimentData")]
    
    pub sentiment_data: Option<GoogleCloudContactcenterinsightsV1SentimentData>,
    /// Data specifying silence.
    #[serde(rename="silenceData")]
    
    pub silence_data: Option<GoogleCloudContactcenterinsightsV1SilenceData>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1CallAnnotation {}


/// The conversation resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversations create projects](ProjectLocationConversationCreateCall) (request|response)
/// * [locations conversations get projects](ProjectLocationConversationGetCall) (response)
/// * [locations conversations patch projects](ProjectLocationConversationPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1Conversation {
    /// An opaque, user-specified string representing the human agent who handled the conversation.
    #[serde(rename="agentId")]
    
    pub agent_id: Option<String>,
    /// Call-specific metadata.
    #[serde(rename="callMetadata")]
    
    pub call_metadata: Option<GoogleCloudContactcenterinsightsV1ConversationCallMetadata>,
    /// Output only. The time at which the conversation was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The source of the audio and transcription for the conversation.
    #[serde(rename="dataSource")]
    
    pub data_source: Option<GoogleCloudContactcenterinsightsV1ConversationDataSource>,
    /// Output only. All the matched Dialogflow intents in the call. The key corresponds to a Dialogflow intent, format: projects/{project}/agent/{agent}/intents/{intent}
    #[serde(rename="dialogflowIntents")]
    
    pub dialogflow_intents: Option<HashMap<String, GoogleCloudContactcenterinsightsV1DialogflowIntent>>,
    /// Output only. The duration of the conversation.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
    /// The time at which this conversation should expire. After this time, the conversation data and any associated analyses will be deleted.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A map for the user to specify any custom fields. A maximum of 20 labels per conversation is allowed, with a maximum of 256 characters per entry.
    
    pub labels: Option<HashMap<String, String>>,
    /// A user-specified language code for the conversation.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Output only. The conversation's latest analysis, if one exists.
    #[serde(rename="latestAnalysis")]
    
    pub latest_analysis: Option<GoogleCloudContactcenterinsightsV1Analysis>,
    /// Immutable. The conversation medium, if unspecified will default to PHONE_CALL.
    
    pub medium: Option<String>,
    /// Immutable. The resource name of the conversation. Format: projects/{project}/locations/{location}/conversations/{conversation}
    
    pub name: Option<String>,
    /// Obfuscated user ID which the customer sent to us.
    #[serde(rename="obfuscatedUserId")]
    
    pub obfuscated_user_id: Option<String>,
    /// Output only. The annotations that were generated during the customer and agent interaction.
    #[serde(rename="runtimeAnnotations")]
    
    pub runtime_annotations: Option<Vec<GoogleCloudContactcenterinsightsV1RuntimeAnnotation>>,
    /// The time at which the conversation started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The conversation transcript.
    
    pub transcript: Option<GoogleCloudContactcenterinsightsV1ConversationTranscript>,
    /// Input only. The TTL for this resource. If specified, then this TTL will be used to calculate the expire time.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub ttl: Option<client::chrono::Duration>,
    /// Output only. The number of turns in the conversation.
    #[serde(rename="turnCount")]
    
    pub turn_count: Option<i32>,
    /// Output only. The most recent time at which the conversation was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudContactcenterinsightsV1Conversation {}
impl client::ResponseResult for GoogleCloudContactcenterinsightsV1Conversation {}


/// Call-specific metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ConversationCallMetadata {
    /// The audio channel that contains the agent.
    #[serde(rename="agentChannel")]
    
    pub agent_channel: Option<i32>,
    /// The audio channel that contains the customer.
    #[serde(rename="customerChannel")]
    
    pub customer_channel: Option<i32>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1ConversationCallMetadata {}


/// The conversation source, which is a combination of transcript and audio.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ConversationDataSource {
    /// The source when the conversation comes from Dialogflow.
    #[serde(rename="dialogflowSource")]
    
    pub dialogflow_source: Option<GoogleCloudContactcenterinsightsV1DialogflowSource>,
    /// A Cloud Storage location specification for the audio and transcript.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GoogleCloudContactcenterinsightsV1GcsSource>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1ConversationDataSource {}


/// One channel of conversation-level sentiment data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ConversationLevelSentiment {
    /// The channel of the audio that the data applies to.
    #[serde(rename="channelTag")]
    
    pub channel_tag: Option<i32>,
    /// Data specifying sentiment.
    #[serde(rename="sentimentData")]
    
    pub sentiment_data: Option<GoogleCloudContactcenterinsightsV1SentimentData>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1ConversationLevelSentiment {}


/// The call participant speaking for a given utterance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ConversationParticipant {
    /// Deprecated. Use `dialogflow_participant_name` instead. The name of the Dialogflow participant. Format: projects/{project}/locations/{location}/conversations/{conversation}/participants/{participant}
    #[serde(rename="dialogflowParticipant")]
    
    pub dialogflow_participant: Option<String>,
    /// The name of the participant provided by Dialogflow. Format: projects/{project}/locations/{location}/conversations/{conversation}/participants/{participant}
    #[serde(rename="dialogflowParticipantName")]
    
    pub dialogflow_participant_name: Option<String>,
    /// Obfuscated user ID from Dialogflow.
    #[serde(rename="obfuscatedExternalUserId")]
    
    pub obfuscated_external_user_id: Option<String>,
    /// The role of the participant.
    
    pub role: Option<String>,
    /// A user-specified ID representing the participant.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1ConversationParticipant {}


/// A message representing the transcript of a conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ConversationTranscript {
    /// A list of sequential transcript segments that comprise the conversation.
    #[serde(rename="transcriptSegments")]
    
    pub transcript_segments: Option<Vec<GoogleCloudContactcenterinsightsV1ConversationTranscriptTranscriptSegment>>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1ConversationTranscript {}


/// A segment of a full transcript.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ConversationTranscriptTranscriptSegment {
    /// For conversations derived from multi-channel audio, this is the channel number corresponding to the audio from that channel. For audioChannelCount = N, its output values can range from '1' to 'N'. A channel tag of 0 indicates that the audio is mono.
    #[serde(rename="channelTag")]
    
    pub channel_tag: Option<i32>,
    /// A confidence estimate between 0.0 and 1.0 of the fidelity of this segment. A default value of 0.0 indicates that the value is unset.
    
    pub confidence: Option<f32>,
    /// CCAI metadata relating to the current transcript segment.
    #[serde(rename="dialogflowSegmentMetadata")]
    
    pub dialogflow_segment_metadata: Option<GoogleCloudContactcenterinsightsV1ConversationTranscriptTranscriptSegmentDialogflowSegmentMetadata>,
    /// The language code of this segment as a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US".
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// The time that the message occurred, if provided.
    #[serde(rename="messageTime")]
    
    pub message_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The participant of this segment.
    #[serde(rename="segmentParticipant")]
    
    pub segment_participant: Option<GoogleCloudContactcenterinsightsV1ConversationParticipant>,
    /// The sentiment for this transcript segment.
    
    pub sentiment: Option<GoogleCloudContactcenterinsightsV1SentimentData>,
    /// The text of this segment.
    
    pub text: Option<String>,
    /// A list of the word-specific information for each word in the segment.
    
    pub words: Option<Vec<GoogleCloudContactcenterinsightsV1ConversationTranscriptTranscriptSegmentWordInfo>>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1ConversationTranscriptTranscriptSegment {}


/// Metadata from Dialogflow relating to the current transcript segment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ConversationTranscriptTranscriptSegmentDialogflowSegmentMetadata {
    /// Whether the transcript segment was covered under the configured smart reply allowlist in Agent Assist.
    #[serde(rename="smartReplyAllowlistCovered")]
    
    pub smart_reply_allowlist_covered: Option<bool>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1ConversationTranscriptTranscriptSegmentDialogflowSegmentMetadata {}


/// Word-level info for words in a transcript.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ConversationTranscriptTranscriptSegmentWordInfo {
    /// A confidence estimate between 0.0 and 1.0 of the fidelity of this word. A default value of 0.0 indicates that the value is unset.
    
    pub confidence: Option<f32>,
    /// Time offset of the end of this word relative to the beginning of the total conversation.
    #[serde(rename="endOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub end_offset: Option<client::chrono::Duration>,
    /// Time offset of the start of this word relative to the beginning of the total conversation.
    #[serde(rename="startOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub start_offset: Option<client::chrono::Duration>,
    /// The word itself. Includes punctuation marks that surround the word.
    
    pub word: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1ConversationTranscriptTranscriptSegmentWordInfo {}


/// The request to deploy an issue model.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations issue models deploy projects](ProjectLocationIssueModelDeployCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1DeployIssueModelRequest {
    /// Required. The issue model to deploy.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudContactcenterinsightsV1DeployIssueModelRequest {}


/// The data for a Dialogflow intent. Represents a detected intent in the conversation, e.g. MAKES_PROMISE.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1DialogflowIntent {
    /// The human-readable name of the intent.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1DialogflowIntent {}


/// Dialogflow interaction data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1DialogflowInteractionData {
    /// The confidence of the match ranging from 0.0 (completely uncertain) to 1.0 (completely certain).
    
    pub confidence: Option<f32>,
    /// The Dialogflow intent resource path. Format: projects/{project}/agent/{agent}/intents/{intent}
    #[serde(rename="dialogflowIntentId")]
    
    pub dialogflow_intent_id: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1DialogflowInteractionData {}


/// A Dialogflow source of conversation data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1DialogflowSource {
    /// Cloud Storage URI that points to a file that contains the conversation audio.
    #[serde(rename="audioUri")]
    
    pub audio_uri: Option<String>,
    /// Output only. The name of the Dialogflow conversation that this conversation resource is derived from. Format: projects/{project}/locations/{location}/conversations/{conversation}
    #[serde(rename="dialogflowConversation")]
    
    pub dialogflow_conversation: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1DialogflowSource {}


/// The data for an entity annotation. Represents a phrase in the conversation that is a known entity, such as a person, an organization, or location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1Entity {
    /// The representative name for the entity.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Metadata associated with the entity. For most entity types, the metadata is a Wikipedia URL (`wikipedia_url`) and Knowledge Graph MID (`mid`), if they are available. For the metadata associated with other entity types, see the Type table below.
    
    pub metadata: Option<HashMap<String, String>>,
    /// The salience score associated with the entity in the [0, 1.0] range. The salience score for an entity provides information about the importance or centrality of that entity to the entire document text. Scores closer to 0 are less salient, while scores closer to 1.0 are highly salient.
    
    pub salience: Option<f32>,
    /// The aggregate sentiment expressed for this entity in the conversation.
    
    pub sentiment: Option<GoogleCloudContactcenterinsightsV1SentimentData>,
    /// The entity type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1Entity {}


/// The data for an entity mention annotation. This represents a mention of an `Entity` in the conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1EntityMentionData {
    /// The key of this entity in conversation entities. Can be used to retrieve the exact `Entity` this mention is attached to.
    #[serde(rename="entityUniqueId")]
    
    pub entity_unique_id: Option<String>,
    /// Sentiment expressed for this mention of the entity.
    
    pub sentiment: Option<GoogleCloudContactcenterinsightsV1SentimentData>,
    /// The type of the entity mention.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1EntityMentionData {}


/// Exact match configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ExactMatchConfig {
    /// Whether to consider case sensitivity when performing an exact match.
    #[serde(rename="caseSensitive")]
    
    pub case_sensitive: Option<bool>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1ExactMatchConfig {}


/// The request to export insights.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations insightsdata export projects](ProjectLocationInsightsdataExportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ExportInsightsDataRequest {
    /// Specified if sink is a BigQuery table.
    #[serde(rename="bigQueryDestination")]
    
    pub big_query_destination: Option<GoogleCloudContactcenterinsightsV1ExportInsightsDataRequestBigQueryDestination>,
    /// A filter to reduce results to a specific subset. Useful for exporting conversations with specific properties.
    
    pub filter: Option<String>,
    /// A fully qualified KMS key name for BigQuery tables protected by CMEK. Format: projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}/cryptoKeyVersions/{version}
    #[serde(rename="kmsKey")]
    
    pub kms_key: Option<String>,
    /// Required. The parent resource to export data from.
    
    pub parent: Option<String>,
    /// Options for what to do if the destination table already exists.
    #[serde(rename="writeDisposition")]
    
    pub write_disposition: Option<String>,
}

impl client::RequestValue for GoogleCloudContactcenterinsightsV1ExportInsightsDataRequest {}


/// A BigQuery Table Reference.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ExportInsightsDataRequestBigQueryDestination {
    /// Required. The name of the BigQuery dataset that the snapshot result should be exported to. If this dataset does not exist, the export call returns an INVALID_ARGUMENT error.
    
    pub dataset: Option<String>,
    /// A project ID or number. If specified, then export will attempt to write data to this project instead of the resource project. Otherwise, the resource project will be used.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// The BigQuery table name to which the insights data should be written. If this table does not exist, the export call returns an INVALID_ARGUMENT error.
    
    pub table: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1ExportInsightsDataRequestBigQueryDestination {}


/// Agent Assist frequently-asked-question answer data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1FaqAnswerData {
    /// The piece of text from the `source` knowledge base document.
    
    pub answer: Option<String>,
    /// The system's confidence score that this answer is a good match for this conversation, ranging from 0.0 (completely uncertain) to 1.0 (completely certain).
    #[serde(rename="confidenceScore")]
    
    pub confidence_score: Option<f32>,
    /// Map that contains metadata about the FAQ answer and the document that it originates from.
    
    pub metadata: Option<HashMap<String, String>>,
    /// The name of the answer record. Format: projects/{project}/locations/{location}/answerRecords/{answer_record}
    #[serde(rename="queryRecord")]
    
    pub query_record: Option<String>,
    /// The corresponding FAQ question.
    
    pub question: Option<String>,
    /// The knowledge document that this answer was extracted from. Format: projects/{project}/knowledgeBases/{knowledge_base}/documents/{document}.
    
    pub source: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1FaqAnswerData {}


/// A Cloud Storage source of conversation data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1GcsSource {
    /// Cloud Storage URI that points to a file that contains the conversation audio.
    #[serde(rename="audioUri")]
    
    pub audio_uri: Option<String>,
    /// Immutable. Cloud Storage URI that points to a file that contains the conversation transcript.
    #[serde(rename="transcriptUri")]
    
    pub transcript_uri: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1GcsSource {}


/// The data for a hold annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1HoldData { _never_set: Option<bool> }

impl client::Part for GoogleCloudContactcenterinsightsV1HoldData {}


/// The request to ingest conversations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversations ingest projects](ProjectLocationConversationIngestCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1IngestConversationsRequest {
    /// Configuration that applies to all conversations.
    #[serde(rename="conversationConfig")]
    
    pub conversation_config: Option<GoogleCloudContactcenterinsightsV1IngestConversationsRequestConversationConfig>,
    /// A cloud storage bucket source.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GoogleCloudContactcenterinsightsV1IngestConversationsRequestGcsSource>,
    /// Required. The parent resource for new conversations.
    
    pub parent: Option<String>,
    /// Configuration for when `source` contains conversation transcripts.
    #[serde(rename="transcriptObjectConfig")]
    
    pub transcript_object_config: Option<GoogleCloudContactcenterinsightsV1IngestConversationsRequestTranscriptObjectConfig>,
}

impl client::RequestValue for GoogleCloudContactcenterinsightsV1IngestConversationsRequest {}


/// Configuration that applies to all conversations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1IngestConversationsRequestConversationConfig {
    /// An opaque, user-specified string representing the human agent who handled the conversations.
    #[serde(rename="agentId")]
    
    pub agent_id: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1IngestConversationsRequestConversationConfig {}


/// Configuration for Cloud Storage bucket sources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1IngestConversationsRequestGcsSource {
    /// Required. The Cloud Storage bucket containing source objects.
    #[serde(rename="bucketUri")]
    
    pub bucket_uri: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1IngestConversationsRequestGcsSource {}


/// Configuration for processing transcript objects.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1IngestConversationsRequestTranscriptObjectConfig {
    /// Required. The medium transcript objects represent.
    
    pub medium: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1IngestConversationsRequestTranscriptObjectConfig {}


/// The data for an intent. Represents a detected intent in the conversation, for example MAKES_PROMISE.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1Intent {
    /// The human-readable name of the intent.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The unique identifier of the intent.
    
    pub id: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1Intent {}


/// The data for an intent match. Represents an intent match for a text segment in the conversation. A text segment can be part of a sentence, a complete sentence, or an utterance with multiple sentences.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1IntentMatchData {
    /// The id of the matched intent. Can be used to retrieve the corresponding intent information.
    #[serde(rename="intentUniqueId")]
    
    pub intent_unique_id: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1IntentMatchData {}


/// The data for an interruption annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1InterruptionData { _never_set: Option<bool> }

impl client::Part for GoogleCloudContactcenterinsightsV1InterruptionData {}


/// The issue resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations issue models issues get projects](ProjectLocationIssueModelIssueGetCall) (response)
/// * [locations issue models issues patch projects](ProjectLocationIssueModelIssuePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1Issue {
    /// Output only. The time at which this issue was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The representative name for the issue.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Immutable. The resource name of the issue. Format: projects/{project}/locations/{location}/issueModels/{issue_model}/issues/{issue}
    
    pub name: Option<String>,
    /// Output only. Resource names of the sample representative utterances that match to this issue.
    #[serde(rename="sampleUtterances")]
    
    pub sample_utterances: Option<Vec<String>>,
    /// Output only. The most recent time that this issue was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudContactcenterinsightsV1Issue {}
impl client::ResponseResult for GoogleCloudContactcenterinsightsV1Issue {}


/// Information about the issue.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1IssueAssignment {
    /// Immutable. Display name of the assigned issue. This field is set at time of analyis and immutable since then.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Resource name of the assigned issue.
    
    pub issue: Option<String>,
    /// Score indicating the likelihood of the issue assignment. currently bounded on [0,1].
    
    pub score: Option<f64>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1IssueAssignment {}


/// The data for an issue match annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1IssueMatchData {
    /// Information about the issue's assignment.
    #[serde(rename="issueAssignment")]
    
    pub issue_assignment: Option<GoogleCloudContactcenterinsightsV1IssueAssignment>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1IssueMatchData {}


/// The issue model resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations issue models create projects](ProjectLocationIssueModelCreateCall) (request)
/// * [locations issue models get projects](ProjectLocationIssueModelGetCall) (response)
/// * [locations issue models patch projects](ProjectLocationIssueModelPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1IssueModel {
    /// Output only. The time at which this issue model was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The representative name for the issue model.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Configs for the input data that used to create the issue model.
    #[serde(rename="inputDataConfig")]
    
    pub input_data_config: Option<GoogleCloudContactcenterinsightsV1IssueModelInputDataConfig>,
    /// Output only. Number of issues in this issue model.
    #[serde(rename="issueCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub issue_count: Option<i64>,
    /// Immutable. The resource name of the issue model. Format: projects/{project}/locations/{location}/issueModels/{issue_model}
    
    pub name: Option<String>,
    /// Output only. State of the model.
    
    pub state: Option<String>,
    /// Output only. Immutable. The issue model's label statistics on its training data.
    #[serde(rename="trainingStats")]
    
    pub training_stats: Option<GoogleCloudContactcenterinsightsV1IssueModelLabelStats>,
    /// Output only. The most recent time at which the issue model was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudContactcenterinsightsV1IssueModel {}
impl client::ResponseResult for GoogleCloudContactcenterinsightsV1IssueModel {}


/// Configs for the input data used to create the issue model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1IssueModelInputDataConfig {
    /// A filter to reduce the conversations used for training the model to a specific subset.
    
    pub filter: Option<String>,
    /// Medium of conversations used in training data. This field is being deprecated. To specify the medium to be used in training a new issue model, set the `medium` field on `filter`.
    
    pub medium: Option<String>,
    /// Output only. Number of conversations used in training. Output only.
    #[serde(rename="trainingConversationsCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub training_conversations_count: Option<i64>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1IssueModelInputDataConfig {}


/// Aggregated statistics about an issue model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1IssueModelLabelStats {
    /// Number of conversations the issue model has analyzed at this point in time.
    #[serde(rename="analyzedConversationsCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub analyzed_conversations_count: Option<i64>,
    /// Statistics on each issue. Key is the issue's resource name.
    #[serde(rename="issueStats")]
    
    pub issue_stats: Option<HashMap<String, GoogleCloudContactcenterinsightsV1IssueModelLabelStatsIssueStats>>,
    /// Number of analyzed conversations for which no issue was applicable at this point in time.
    #[serde(rename="unclassifiedConversationsCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub unclassified_conversations_count: Option<i64>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1IssueModelLabelStats {}


/// Aggregated statistics about an issue.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1IssueModelLabelStatsIssueStats {
    /// Display name of the issue.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Issue resource. Format: projects/{project}/locations/{location}/issueModels/{issue_model}/issues/{issue}
    
    pub issue: Option<String>,
    /// Number of conversations attached to the issue at this point in time.
    #[serde(rename="labeledConversationsCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub labeled_conversations_count: Option<i64>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1IssueModelLabelStatsIssueStats {}


/// Issue Modeling result on a conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1IssueModelResult {
    /// Issue model that generates the result. Format: projects/{project}/locations/{location}/issueModels/{issue_model}
    #[serde(rename="issueModel")]
    
    pub issue_model: Option<String>,
    /// All the matched issues.
    
    pub issues: Option<Vec<GoogleCloudContactcenterinsightsV1IssueAssignment>>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1IssueModelResult {}


/// The response to list analyses.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversations analyses list projects](ProjectLocationConversationAnalysisListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ListAnalysesResponse {
    /// The analyses that match the request.
    
    pub analyses: Option<Vec<GoogleCloudContactcenterinsightsV1Analysis>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudContactcenterinsightsV1ListAnalysesResponse {}


/// The response of listing conversations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversations list projects](ProjectLocationConversationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ListConversationsResponse {
    /// The conversations that match the request.
    
    pub conversations: Option<Vec<GoogleCloudContactcenterinsightsV1Conversation>>,
    /// A token which can be sent as `page_token` to retrieve the next page. If this field is set, it means there is another page available. If it is not set, it means no other pages are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudContactcenterinsightsV1ListConversationsResponse {}


/// The response of listing issue models.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations issue models list projects](ProjectLocationIssueModelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ListIssueModelsResponse {
    /// The issue models that match the request.
    #[serde(rename="issueModels")]
    
    pub issue_models: Option<Vec<GoogleCloudContactcenterinsightsV1IssueModel>>,
}

impl client::ResponseResult for GoogleCloudContactcenterinsightsV1ListIssueModelsResponse {}


/// The response of listing issues.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations issue models issues list projects](ProjectLocationIssueModelIssueListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ListIssuesResponse {
    /// The issues that match the request.
    
    pub issues: Option<Vec<GoogleCloudContactcenterinsightsV1Issue>>,
}

impl client::ResponseResult for GoogleCloudContactcenterinsightsV1ListIssuesResponse {}


/// The response of listing phrase matchers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations phrase matchers list projects](ProjectLocationPhraseMatcherListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ListPhraseMatchersResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The phrase matchers that match the request.
    #[serde(rename="phraseMatchers")]
    
    pub phrase_matchers: Option<Vec<GoogleCloudContactcenterinsightsV1PhraseMatcher>>,
}

impl client::ResponseResult for GoogleCloudContactcenterinsightsV1ListPhraseMatchersResponse {}


/// The response of listing views.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations views list projects](ProjectLocationViewListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1ListViewsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The views that match the request.
    
    pub views: Option<Vec<GoogleCloudContactcenterinsightsV1View>>,
}

impl client::ResponseResult for GoogleCloudContactcenterinsightsV1ListViewsResponse {}


/// The data for a matched phrase matcher. Represents information identifying a phrase matcher for a given match.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1PhraseMatchData {
    /// The human-readable name of the phrase matcher.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The unique identifier (the resource name) of the phrase matcher.
    #[serde(rename="phraseMatcher")]
    
    pub phrase_matcher: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1PhraseMatchData {}


/// The data for a phrase match rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1PhraseMatchRule {
    /// Provides additional information about the rule that specifies how to apply the rule.
    
    pub config: Option<GoogleCloudContactcenterinsightsV1PhraseMatchRuleConfig>,
    /// Specifies whether the phrase must be missing from the transcript segment or present in the transcript segment.
    
    pub negated: Option<bool>,
    /// Required. The phrase to be matched.
    
    pub query: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1PhraseMatchRule {}


/// Configuration information of a phrase match rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1PhraseMatchRuleConfig {
    /// The configuration for the exact match rule.
    #[serde(rename="exactMatchConfig")]
    
    pub exact_match_config: Option<GoogleCloudContactcenterinsightsV1ExactMatchConfig>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1PhraseMatchRuleConfig {}


/// A message representing a rule in the phrase matcher.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1PhraseMatchRuleGroup {
    /// A list of phase match rules that are included in this group.
    #[serde(rename="phraseMatchRules")]
    
    pub phrase_match_rules: Option<Vec<GoogleCloudContactcenterinsightsV1PhraseMatchRule>>,
    /// Required. The type of this phrase match rule group.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1PhraseMatchRuleGroup {}


/// The phrase matcher resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations phrase matchers create projects](ProjectLocationPhraseMatcherCreateCall) (request|response)
/// * [locations phrase matchers get projects](ProjectLocationPhraseMatcherGetCall) (response)
/// * [locations phrase matchers patch projects](ProjectLocationPhraseMatcherPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1PhraseMatcher {
    /// Output only. The most recent time at which the activation status was updated.
    #[serde(rename="activationUpdateTime")]
    
    pub activation_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Applies the phrase matcher only when it is active.
    
    pub active: Option<bool>,
    /// The human-readable name of the phrase matcher.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The resource name of the phrase matcher. Format: projects/{project}/locations/{location}/phraseMatchers/{phrase_matcher}
    
    pub name: Option<String>,
    /// A list of phase match rule groups that are included in this matcher.
    #[serde(rename="phraseMatchRuleGroups")]
    
    pub phrase_match_rule_groups: Option<Vec<GoogleCloudContactcenterinsightsV1PhraseMatchRuleGroup>>,
    /// Output only. The timestamp of when the revision was created. It is also the create time when a new matcher is added.
    #[serde(rename="revisionCreateTime")]
    
    pub revision_create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Immutable. The revision ID of the phrase matcher. A new revision is committed whenever the matcher is changed, except when it is activated or deactivated. A server generated random ID will be used. Example: locations/global/phraseMatchers/my-first-matcher@1234567
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
    /// The role whose utterances the phrase matcher should be matched against. If the role is ROLE_UNSPECIFIED it will be matched against any utterances in the transcript.
    #[serde(rename="roleMatch")]
    
    pub role_match: Option<String>,
    /// Required. The type of this phrase matcher.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. The most recent time at which the phrase matcher was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The customized version tag to use for the phrase matcher. If not specified, it will default to `revision_id`.
    #[serde(rename="versionTag")]
    
    pub version_tag: Option<String>,
}

impl client::RequestValue for GoogleCloudContactcenterinsightsV1PhraseMatcher {}
impl client::ResponseResult for GoogleCloudContactcenterinsightsV1PhraseMatcher {}


/// An annotation that was generated during the customer and agent interaction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1RuntimeAnnotation {
    /// The unique identifier of the annotation. Format: projects/{project}/locations/{location}/conversationDatasets/{dataset}/conversationDataItems/{data_item}/conversationAnnotations/{annotation}
    #[serde(rename="annotationId")]
    
    pub annotation_id: Option<String>,
    /// The feedback that the customer has about the answer in `data`.
    #[serde(rename="answerFeedback")]
    
    pub answer_feedback: Option<GoogleCloudContactcenterinsightsV1AnswerFeedback>,
    /// Agent Assist Article Suggestion data.
    #[serde(rename="articleSuggestion")]
    
    pub article_suggestion: Option<GoogleCloudContactcenterinsightsV1ArticleSuggestionData>,
    /// The time at which this annotation was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Dialogflow interaction data.
    #[serde(rename="dialogflowInteraction")]
    
    pub dialogflow_interaction: Option<GoogleCloudContactcenterinsightsV1DialogflowInteractionData>,
    /// The boundary in the conversation where the annotation ends, inclusive.
    #[serde(rename="endBoundary")]
    
    pub end_boundary: Option<GoogleCloudContactcenterinsightsV1AnnotationBoundary>,
    /// Agent Assist FAQ answer data.
    #[serde(rename="faqAnswer")]
    
    pub faq_answer: Option<GoogleCloudContactcenterinsightsV1FaqAnswerData>,
    /// Agent Assist Smart Compose suggestion data.
    #[serde(rename="smartComposeSuggestion")]
    
    pub smart_compose_suggestion: Option<GoogleCloudContactcenterinsightsV1SmartComposeSuggestionData>,
    /// Agent Assist Smart Reply data.
    #[serde(rename="smartReply")]
    
    pub smart_reply: Option<GoogleCloudContactcenterinsightsV1SmartReplyData>,
    /// The boundary in the conversation where the annotation starts, inclusive.
    #[serde(rename="startBoundary")]
    
    pub start_boundary: Option<GoogleCloudContactcenterinsightsV1AnnotationBoundary>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1RuntimeAnnotation {}


/// The data for a sentiment annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1SentimentData {
    /// A non-negative number from 0 to infinity which represents the abolute magnitude of sentiment regardless of score.
    
    pub magnitude: Option<f32>,
    /// The sentiment score between -1.0 (negative) and 1.0 (positive).
    
    pub score: Option<f32>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1SentimentData {}


/// The settings resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get settings projects](ProjectLocationGetSettingCall) (response)
/// * [locations update settings projects](ProjectLocationUpdateSettingCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1Settings {
    /// Default analysis settings.
    #[serde(rename="analysisConfig")]
    
    pub analysis_config: Option<GoogleCloudContactcenterinsightsV1SettingsAnalysisConfig>,
    /// The default TTL for newly-created conversations. If a conversation has a specified expiration, that value will be used instead. Changing this value will not change the expiration of existing conversations. Conversations with no expire time persist until they are deleted.
    #[serde(rename="conversationTtl")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub conversation_ttl: Option<client::chrono::Duration>,
    /// Output only. The time at which the settings was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A language code to be applied to each transcript segment unless the segment already specifies a language code. Language code defaults to "en-US" if it is neither specified on the segment nor here.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Immutable. The resource name of the settings resource. Format: projects/{project}/locations/{location}/settings
    
    pub name: Option<String>,
    /// A map that maps a notification trigger to a Pub/Sub topic. Each time a specified trigger occurs, Insights will notify the corresponding Pub/Sub topic. Keys are notification triggers. Supported keys are: * "all-triggers": Notify each time any of the supported triggers occurs. * "create-analysis": Notify each time an analysis is created. * "create-conversation": Notify each time a conversation is created. * "export-insights-data": Notify each time an export is complete. * "update-conversation": Notify each time a conversation is updated via UpdateConversation. Values are Pub/Sub topics. The format of each Pub/Sub topic is: projects/{project}/topics/{topic}
    #[serde(rename="pubsubNotificationSettings")]
    
    pub pubsub_notification_settings: Option<HashMap<String, String>>,
    /// Output only. The time at which the settings were last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudContactcenterinsightsV1Settings {}
impl client::ResponseResult for GoogleCloudContactcenterinsightsV1Settings {}


/// Default configuration when creating Analyses in Insights.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1SettingsAnalysisConfig {
    /// To select the annotators to run and the phrase matchers to use (if any). If not specified, all annotators will be run.
    #[serde(rename="annotatorSelector")]
    
    pub annotator_selector: Option<GoogleCloudContactcenterinsightsV1AnnotatorSelector>,
    /// Percentage of conversations created using Dialogflow runtime integration to analyze automatically, between [0, 100].
    #[serde(rename="runtimeIntegrationAnalysisPercentage")]
    
    pub runtime_integration_analysis_percentage: Option<f64>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1SettingsAnalysisConfig {}


/// The data for a silence annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1SilenceData { _never_set: Option<bool> }

impl client::Part for GoogleCloudContactcenterinsightsV1SilenceData {}


/// Agent Assist Smart Compose suggestion data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1SmartComposeSuggestionData {
    /// The system's confidence score that this suggestion is a good match for this conversation, ranging from 0.0 (completely uncertain) to 1.0 (completely certain).
    #[serde(rename="confidenceScore")]
    
    pub confidence_score: Option<f64>,
    /// Map that contains metadata about the Smart Compose suggestion and the document from which it originates.
    
    pub metadata: Option<HashMap<String, String>>,
    /// The name of the answer record. Format: projects/{project}/locations/{location}/answerRecords/{answer_record}
    #[serde(rename="queryRecord")]
    
    pub query_record: Option<String>,
    /// The content of the suggestion.
    
    pub suggestion: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1SmartComposeSuggestionData {}


/// Agent Assist Smart Reply data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1SmartReplyData {
    /// The system's confidence score that this reply is a good match for this conversation, ranging from 0.0 (completely uncertain) to 1.0 (completely certain).
    #[serde(rename="confidenceScore")]
    
    pub confidence_score: Option<f64>,
    /// Map that contains metadata about the Smart Reply and the document from which it originates.
    
    pub metadata: Option<HashMap<String, String>>,
    /// The name of the answer record. Format: projects/{project}/locations/{location}/answerRecords/{answer_record}
    #[serde(rename="queryRecord")]
    
    pub query_record: Option<String>,
    /// The content of the reply.
    
    pub reply: Option<String>,
}

impl client::Part for GoogleCloudContactcenterinsightsV1SmartReplyData {}


/// The request to undeploy an issue model.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations issue models undeploy projects](ProjectLocationIssueModelUndeployCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1UndeployIssueModelRequest {
    /// Required. The issue model to undeploy.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudContactcenterinsightsV1UndeployIssueModelRequest {}


/// The View resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations views create projects](ProjectLocationViewCreateCall) (request|response)
/// * [locations views get projects](ProjectLocationViewGetCall) (response)
/// * [locations views patch projects](ProjectLocationViewPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudContactcenterinsightsV1View {
    /// Output only. The time at which this view was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The human-readable display name of the view.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Immutable. The resource name of the view. Format: projects/{project}/locations/{location}/views/{view}
    
    pub name: Option<String>,
    /// Output only. The most recent time at which the view was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// String with specific view properties, must be non-empty.
    
    pub value: Option<String>,
}

impl client::RequestValue for GoogleCloudContactcenterinsightsV1View {}
impl client::ResponseResult for GoogleCloudContactcenterinsightsV1View {}


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
/// * [locations conversations analyses create projects](ProjectLocationConversationAnalysisCreateCall) (response)
/// * [locations conversations bulk analyze projects](ProjectLocationConversationBulkAnalyzeCall) (response)
/// * [locations conversations ingest projects](ProjectLocationConversationIngestCall) (response)
/// * [locations insightsdata export projects](ProjectLocationInsightsdataExportCall) (response)
/// * [locations issue models create projects](ProjectLocationIssueModelCreateCall) (response)
/// * [locations issue models delete projects](ProjectLocationIssueModelDeleteCall) (response)
/// * [locations issue models deploy projects](ProjectLocationIssueModelDeployCall) (response)
/// * [locations issue models undeploy projects](ProjectLocationIssueModelUndeployCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
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
/// * [locations conversations analyses delete projects](ProjectLocationConversationAnalysisDeleteCall) (response)
/// * [locations conversations delete projects](ProjectLocationConversationDeleteCall) (response)
/// * [locations issue models issues delete projects](ProjectLocationIssueModelIssueDeleteCall) (response)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations phrase matchers delete projects](ProjectLocationPhraseMatcherDeleteCall) (response)
/// * [locations views delete projects](ProjectLocationViewDeleteCall) (response)
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


