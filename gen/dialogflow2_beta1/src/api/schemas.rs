use super::*;
/// A Dialogflow agent is a virtual agent that handles conversations with your end-users. It is a natural language understanding module that understands the nuances of human language. Dialogflow translates end-user text or audio during a conversation to structured data that your apps and services can understand. You design and build a Dialogflow agent to handle the types of conversations required for your system. For more information about agents, see the [Agent guide](https://cloud.google.com/dialogflow/docs/agents-overview). 
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get agent projects](ProjectLocationGetAgentCall) (response)
/// * [locations set agent projects](ProjectLocationSetAgentCall) (request|response)
/// * [get agent projects](ProjectGetAgentCall) (response)
/// * [set agent projects](ProjectSetAgentCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1Agent {
    /// Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version.
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<GoogleCloudDialogflowV2beta1AgentApiVersionEnum>,
    /// Optional. The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration.
    #[serde(rename="avatarUri")]
    
    pub avatar_uri: Option<String>,
    /// Optional. To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used.
    #[serde(rename="classificationThreshold")]
    
    pub classification_threshold: Option<f32>,
    /// Required. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the `Update` method.
    #[serde(rename="defaultLanguageCode")]
    
    pub default_language_code: Option<String>,
    /// Optional. The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected.
    
    pub description: Option<String>,
    /// Required. The name of this agent.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. Determines whether this agent should log conversation queries.
    #[serde(rename="enableLogging")]
    
    pub enable_logging: Option<bool>,
    /// Optional. Determines how intents are detected from user queries.
    #[serde(rename="matchMode")]
    
    pub match_mode: Option<GoogleCloudDialogflowV2beta1AgentMatchModeEnum>,
    /// Required. The project of this agent. Format: `projects/` or `projects//locations/`
    
    pub parent: Option<String>,
    /// Optional. The list of all languages supported by this agent (except for the `default_language_code`).
    #[serde(rename="supportedLanguageCodes")]
    
    pub supported_language_codes: Option<Vec<String>>,
    /// Optional. The agent tier. If not specified, TIER_STANDARD is assumed.
    
    pub tier: Option<GoogleCloudDialogflowV2beta1AgentTierEnum>,
    /// Required. The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1Agent {}
impl client::ResponseResult for GoogleCloudDialogflowV2beta1Agent {}


/// Detail feedback of Agent Assistant result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1AgentAssistantFeedback {
    /// Optional. Whether or not the suggested answer is relevant. For example: * Query: "Can I change my mailing address?" * Suggested document says: "Items must be returned/exchanged within 60 days of the purchase date." * answer_relevance: AnswerRelevance.IRRELEVANT
    #[serde(rename="answerRelevance")]
    
    pub answer_relevance: Option<GoogleCloudDialogflowV2beta1AgentAssistantFeedbackAnswerRelevanceEnum>,
    /// Optional. Whether or not the information in the document is correct. For example: * Query: "Can I return the package in 2 days once received?" * Suggested document says: "Items must be returned/exchanged within 60 days of the purchase date." * Ground truth: "No return or exchange is allowed." * [document_correctness]: INCORRECT
    #[serde(rename="documentCorrectness")]
    
    pub document_correctness: Option<GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentCorrectnessEnum>,
    /// Optional. Whether or not the suggested document is efficient. For example, if the document is poorly written, hard to understand, hard to use or too long to find useful information, document_efficiency is DocumentEfficiency.INEFFICIENT.
    #[serde(rename="documentEfficiency")]
    
    pub document_efficiency: Option<GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentEfficiencyEnum>,
    /// Feedback for conversation summarization.
    #[serde(rename="summarizationFeedback")]
    
    pub summarization_feedback: Option<GoogleCloudDialogflowV2beta1AgentAssistantFeedbackSummarizationFeedback>,
}

impl client::Part for GoogleCloudDialogflowV2beta1AgentAssistantFeedback {}


/// Feedback for conversation summarization.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1AgentAssistantFeedbackSummarizationFeedback {
    /// Timestamp when composing of the summary starts.
    #[serde(rename="startTimestamp")]
    
    pub start_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Timestamp when the summary was submitted.
    #[serde(rename="submitTimestamp")]
    
    pub submit_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Text of actual submitted summary.
    #[serde(rename="summaryText")]
    
    pub summary_text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1AgentAssistantFeedbackSummarizationFeedback {}


/// Represents a record of a human agent assistant answer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1AgentAssistantRecord {
    /// Output only. The article suggestion answer.
    #[serde(rename="articleSuggestionAnswer")]
    
    pub article_suggestion_answer: Option<GoogleCloudDialogflowV2beta1ArticleAnswer>,
    /// Output only. The FAQ answer.
    #[serde(rename="faqAnswer")]
    
    pub faq_answer: Option<GoogleCloudDialogflowV2beta1FaqAnswer>,
}

impl client::Part for GoogleCloudDialogflowV2beta1AgentAssistantRecord {}


/// The request message for Participants.AnalyzeContent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations participants analyze content projects](ProjectConversationParticipantAnalyzeContentCall) (request)
/// * [locations conversations participants analyze content projects](ProjectLocationConversationParticipantAnalyzeContentCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1AnalyzeContentRequest {
    /// Parameters for a human assist query.
    #[serde(rename="assistQueryParams")]
    
    pub assist_query_params: Option<GoogleCloudDialogflowV2beta1AssistQueryParameters>,
    /// The natural language speech audio to be processed.
    #[serde(rename="audioInput")]
    
    pub audio_input: Option<GoogleCloudDialogflowV2beta1AudioInput>,
    /// The unique identifier of the CX page to override the `current_page` in the session. Format: `projects//locations//agents//flows//pages/`. If `cx_current_page` is specified, the previous state of the session will be ignored by Dialogflow CX, including the previous page and the previous session parameters. In most cases, `cx_current_page` and `cx_parameters` should be configured together to direct a session to a specific state. Note: this field should only be used if you are connecting to a Dialogflow CX agent.
    #[serde(rename="cxCurrentPage")]
    
    pub cx_current_page: Option<String>,
    /// Additional parameters to be put into Dialogflow CX session parameters. To remove a parameter from the session, clients should explicitly set the parameter value to null. Note: this field should only be used if you are connecting to a Dialogflow CX agent.
    #[serde(rename="cxParameters")]
    
    pub cx_parameters: Option<HashMap<String, json::Value>>,
    /// An input event to send to Dialogflow.
    #[serde(rename="eventInput")]
    
    pub event_input: Option<GoogleCloudDialogflowV2beta1EventInput>,
    /// Optional. The send time of the message from end user or human agent's perspective. It is used for identifying the same message under one participant. Given two messages under the same participant: - If send time are different regardless of whether the content of the messages are exactly the same, the conversation will regard them as two distinct messages sent by the participant. - If send time is the same regardless of whether the content of the messages are exactly the same, the conversation will regard them as same message, and ignore the message received later. If the value is not provided, a new request will always be regarded as a new message without any de-duplication.
    #[serde(rename="messageSendTime")]
    
    pub message_send_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Parameters for a Dialogflow virtual-agent query.
    #[serde(rename="queryParams")]
    
    pub query_params: Option<GoogleCloudDialogflowV2beta1QueryParameters>,
    /// Speech synthesis configuration. The speech synthesis settings for a virtual agent that may be configured for the associated conversation profile are not used when calling AnalyzeContent. If this configuration is not supplied, speech synthesis is disabled.
    #[serde(rename="replyAudioConfig")]
    
    pub reply_audio_config: Option<GoogleCloudDialogflowV2beta1OutputAudioConfig>,
    /// A unique identifier for this request. Restricted to 36 ASCII characters. A random UUID is recommended. This request is only idempotent if a `request_id` is provided.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// The natural language text to be processed.
    #[serde(rename="textInput")]
    
    pub text_input: Option<GoogleCloudDialogflowV2beta1TextInput>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1AnalyzeContentRequest {}


/// The response message for Participants.AnalyzeContent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations participants analyze content projects](ProjectConversationParticipantAnalyzeContentCall) (response)
/// * [locations conversations participants analyze content projects](ProjectLocationConversationParticipantAnalyzeContentCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1AnalyzeContentResponse {
    /// Optional. Only set if a Dialogflow automated agent has responded. Note that: AutomatedAgentReply.detect_intent_response.output_audio and AutomatedAgentReply.detect_intent_response.output_audio_config are always empty, use reply_audio instead.
    #[serde(rename="automatedAgentReply")]
    
    pub automated_agent_reply: Option<GoogleCloudDialogflowV2beta1AutomatedAgentReply>,
    /// Indicates the parameters of DTMF.
    #[serde(rename="dtmfParameters")]
    
    pub dtmf_parameters: Option<GoogleCloudDialogflowV2beta1DtmfParameters>,
    /// The suggestions for end user. The order is the same as HumanAgentAssistantConfig.SuggestionConfig.feature_configs of HumanAgentAssistantConfig.end_user_suggestion_config. Same as human_agent_suggestion_results, any failure of Agent Assist features will not lead to the overall failure of an AnalyzeContent API call. Instead, the features will fail silently with the error field set in the corresponding SuggestionResult.
    #[serde(rename="endUserSuggestionResults")]
    
    pub end_user_suggestion_results: Option<Vec<GoogleCloudDialogflowV2beta1SuggestionResult>>,
    /// The suggestions for most recent human agent. The order is the same as HumanAgentAssistantConfig.SuggestionConfig.feature_configs of HumanAgentAssistantConfig.human_agent_suggestion_config. Note that any failure of Agent Assist features will not lead to the overall failure of an AnalyzeContent API call. Instead, the features will fail silently with the error field set in the corresponding SuggestionResult.
    #[serde(rename="humanAgentSuggestionResults")]
    
    pub human_agent_suggestion_results: Option<Vec<GoogleCloudDialogflowV2beta1SuggestionResult>>,
    /// Output only. Message analyzed by CCAI.
    
    pub message: Option<GoogleCloudDialogflowV2beta1Message>,
    /// Optional. The audio data bytes encoded as specified in the request. This field is set if: - `reply_audio_config` was specified in the request, or - The automated agent responded with audio to play to the user. In such case, `reply_audio.config` contains settings used to synthesize the speech. In some scenarios, multiple output audio fields may be present in the response structure. In these cases, only the top-most-level audio output has content.
    #[serde(rename="replyAudio")]
    
    pub reply_audio: Option<GoogleCloudDialogflowV2beta1OutputAudio>,
    /// Output only. The output text content. This field is set if the automated agent responded with text to show to the user.
    #[serde(rename="replyText")]
    
    pub reply_text: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1AnalyzeContentResponse {}


/// Represents a part of a message possibly annotated with an entity. The part can be an entity or purely a part of the message between two entities or message start/end.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1AnnotatedMessagePart {
    /// Optional. The [Dialogflow system entity type](https://cloud.google.com/dialogflow/docs/reference/system-entities) of this message part. If this is empty, Dialogflow could not annotate the phrase part with a system entity.
    #[serde(rename="entityType")]
    
    pub entity_type: Option<String>,
    /// Optional. The [Dialogflow system entity formatted value ](https://cloud.google.com/dialogflow/docs/reference/system-entities) of this message part. For example for a system entity of type `@sys.unit-currency`, this may contain: { "amount": 5, "currency": "USD" } 
    #[serde(rename="formattedValue")]
    
    pub formatted_value: Option<json::Value>,
    /// Required. A part of a message possibly annotated with an entity.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1AnnotatedMessagePart {}


/// Represents feedback the customer has about the quality & correctness of a certain answer in a conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1AnswerFeedback {
    /// Optional. Detail feedback of agent assistant suggestions.
    #[serde(rename="agentAssistantDetailFeedback")]
    
    pub agent_assistant_detail_feedback: Option<GoogleCloudDialogflowV2beta1AgentAssistantFeedback>,
    /// Time when the answer/item was clicked.
    #[serde(rename="clickTime")]
    
    pub click_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Indicates whether the answer/item was clicked by the human agent or not. Default to false.
    
    pub clicked: Option<bool>,
    /// The correctness level of the specific answer.
    #[serde(rename="correctnessLevel")]
    
    pub correctness_level: Option<GoogleCloudDialogflowV2beta1AnswerFeedbackCorrectnessLevelEnum>,
    /// Time when the answer/item was displayed.
    #[serde(rename="displayTime")]
    
    pub display_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Indicates whether the answer/item was displayed to the human agent in the agent desktop UI. Default to false.
    
    pub displayed: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2beta1AnswerFeedback {}


/// Answer records are records to manage answer history and feedbacks for Dialogflow. Currently, answer record includes: - human agent assistant article suggestion - human agent assistant faq article It doesn’t include: - `DetectIntent` intent matching - `DetectIntent` knowledge Answer records are not related to the conversation history in the Dialogflow Console. A Record is generated even when the end-user disables conversation history in the console. Records are created when there’s a human agent assistant suggestion generated. A typical workflow for customers provide feedback to an answer is: 1. For human agent assistant, customers get suggestion via ListSuggestions API. Together with the answers, AnswerRecord.name are returned to the customers. 2. The customer uses the AnswerRecord.name to call the UpdateAnswerRecord method to send feedback about a specific answer that they believe is wrong.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [answer records get projects](ProjectAnswerRecordGetCall) (response)
/// * [answer records patch projects](ProjectAnswerRecordPatchCall) (request|response)
/// * [locations answer records get projects](ProjectLocationAnswerRecordGetCall) (response)
/// * [locations answer records patch projects](ProjectLocationAnswerRecordPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1AnswerRecord {
    /// Output only. The record for human agent assistant.
    #[serde(rename="agentAssistantRecord")]
    
    pub agent_assistant_record: Option<GoogleCloudDialogflowV2beta1AgentAssistantRecord>,
    /// Optional. The AnswerFeedback for this record. You can set this with AnswerRecords.UpdateAnswerRecord in order to give us feedback about this answer.
    #[serde(rename="answerFeedback")]
    
    pub answer_feedback: Option<GoogleCloudDialogflowV2beta1AnswerFeedback>,
    /// The unique identifier of this answer record. Required for AnswerRecords.UpdateAnswerRecord method. Format: `projects//locations//answerRecords/`.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1AnswerRecord {}
impl client::ResponseResult for GoogleCloudDialogflowV2beta1AnswerRecord {}


/// Represents article answer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ArticleAnswer {
    /// The name of answer record, in the format of "projects//locations//answerRecords/"
    #[serde(rename="answerRecord")]
    
    pub answer_record: Option<String>,
    /// A map that contains metadata about the answer and the document from which it originates.
    
    pub metadata: Option<HashMap<String, String>>,
    /// Output only. Article snippets.
    
    pub snippets: Option<Vec<String>>,
    /// The article title.
    
    pub title: Option<String>,
    /// The article URI.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1ArticleAnswer {}


/// Represents the parameters of human assist query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1AssistQueryParameters {
    /// Key-value filters on the metadata of documents returned by article suggestion. If specified, article suggestion only returns suggested documents that match all filters in their Document.metadata. Multiple values for a metadata key should be concatenated by comma. For example, filters to match all documents that have 'US' or 'CA' in their market metadata values and 'agent' in their user metadata values will be ``` documents_metadata_filters { key: "market" value: "US,CA" } documents_metadata_filters { key: "user" value: "agent" } ```
    #[serde(rename="documentsMetadataFilters")]
    
    pub documents_metadata_filters: Option<HashMap<String, String>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1AssistQueryParameters {}


/// Represents the natural language speech audio to be processed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1AudioInput {
    /// Required. The natural language speech audio to be processed. A single request can contain up to 1 minute of speech audio data. The transcribed text cannot contain more than 256 bytes for virtual agent interactions.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub audio: Option<Vec<u8>>,
    /// Required. Instructs the speech recognizer how to process the speech audio.
    
    pub config: Option<GoogleCloudDialogflowV2beta1InputAudioConfig>,
}

impl client::Part for GoogleCloudDialogflowV2beta1AudioInput {}


/// Defines the Automated Agent to connect to a conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1AutomatedAgentConfig {
    /// Required. ID of the Dialogflow agent environment to use. This project needs to either be the same project as the conversation or you need to grant `service-@gcp-sa-dialogflow.iam.gserviceaccount.com` the `Dialogflow API Service Agent` role in this project. - For ES agents, use format: `projects//locations//agent/environments/`. If environment is not specified, the default `draft` environment is used. Refer to [DetectIntentRequest](https://cloud.google.com/dialogflow/docs/reference/rpc/google.cloud.dialogflow.v2beta1#google.cloud.dialogflow.v2beta1.DetectIntentRequest) for more details. - For CX agents, use format `projects//locations//agents//environments/`. If environment is not specified, the default `draft` environment is used.
    
    pub agent: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1AutomatedAgentConfig {}


/// Represents a response from an automated agent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1AutomatedAgentReply {
    /// Indicates whether the partial automated agent reply is interruptible when a later reply message arrives. e.g. if the agent specified some music as partial response, it can be cancelled.
    #[serde(rename="allowCancellation")]
    
    pub allow_cancellation: Option<bool>,
    /// AutomatedAgentReply type.
    #[serde(rename="automatedAgentReplyType")]
    
    pub automated_agent_reply_type: Option<GoogleCloudDialogflowV2beta1AutomatedAgentReplyAutomatedAgentReplyTypeEnum>,
    /// The unique identifier of the current Dialogflow CX conversation page. Format: `projects//locations//agents//flows//pages/`.
    #[serde(rename="cxCurrentPage")]
    
    pub cx_current_page: Option<String>,
    /// The collection of current Dialogflow CX agent session parameters at the time of this response. Deprecated: Use `parameters` instead.
    #[serde(rename="cxSessionParameters")]
    
    pub cx_session_parameters: Option<HashMap<String, json::Value>>,
    /// Response of the Dialogflow Sessions.DetectIntent call.
    #[serde(rename="detectIntentResponse")]
    
    pub detect_intent_response: Option<GoogleCloudDialogflowV2beta1DetectIntentResponse>,
    /// Event name if an event is triggered for the query.
    
    pub event: Option<String>,
    /// Name of the intent if an intent is matched for the query. For a V2 query, the value format is `projects//locations/ /agent/intents/`. For a V3 query, the value format is `projects//locations/ /agents//intents/`.
    
    pub intent: Option<String>,
    /// The confidence of the match. Values range from 0.0 (completely uncertain) to 1.0 (completely certain). This value is for informational purpose only and is only used to help match the best intent within the classification threshold. This value may change for the same end-user expression at any time due to a model retraining or change in implementation.
    #[serde(rename="matchConfidence")]
    
    pub match_confidence: Option<f32>,
    /// The collection of current parameters at the time of this response.
    
    pub parameters: Option<HashMap<String, json::Value>>,
    /// Response messages from the automated agent.
    #[serde(rename="responseMessages")]
    
    pub response_messages: Option<Vec<GoogleCloudDialogflowV2beta1ResponseMessage>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1AutomatedAgentReply {}


/// The request message for EntityTypes.BatchCreateEntities.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent entity types entities batch create projects](ProjectAgentEntityTypeEntityBatchCreateCall) (request)
/// * [locations agent entity types entities batch create projects](ProjectLocationAgentEntityTypeEntityBatchCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1BatchCreateEntitiesRequest {
    /// Required. The entities to create.
    
    pub entities: Option<Vec<GoogleCloudDialogflowV2beta1EntityTypeEntity>>,
    /// Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1BatchCreateEntitiesRequest {}


/// The request message for Conversations.BatchCreateMessagesRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations messages batch create projects](ProjectConversationMessageBatchCreateCall) (request)
/// * [locations conversations messages batch create projects](ProjectLocationConversationMessageBatchCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1BatchCreateMessagesRequest {
    /// Required. A maximum of 1000 Messages can be created in a batch. CreateMessageRequest.message.send_time is required. All created messages will have identical Message.create_time.
    
    pub requests: Option<Vec<GoogleCloudDialogflowV2beta1CreateMessageRequest>>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1BatchCreateMessagesRequest {}


/// The request message for Conversations.BatchCreateMessagesResponse.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations messages batch create projects](ProjectConversationMessageBatchCreateCall) (response)
/// * [locations conversations messages batch create projects](ProjectLocationConversationMessageBatchCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1BatchCreateMessagesResponse {
    /// Messages created.
    
    pub messages: Option<Vec<GoogleCloudDialogflowV2beta1Message>>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1BatchCreateMessagesResponse {}


/// The request message for EntityTypes.BatchDeleteEntities.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent entity types entities batch delete projects](ProjectAgentEntityTypeEntityBatchDeleteCall) (request)
/// * [locations agent entity types entities batch delete projects](ProjectLocationAgentEntityTypeEntityBatchDeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1BatchDeleteEntitiesRequest {
    /// Required. The reference `values` of the entities to delete. Note that these are not fully-qualified names, i.e. they don't start with `projects/`.
    #[serde(rename="entityValues")]
    
    pub entity_values: Option<Vec<String>>,
    /// Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1BatchDeleteEntitiesRequest {}


/// The request message for EntityTypes.BatchDeleteEntityTypes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent entity types batch delete projects](ProjectAgentEntityTypeBatchDeleteCall) (request)
/// * [locations agent entity types batch delete projects](ProjectLocationAgentEntityTypeBatchDeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1BatchDeleteEntityTypesRequest {
    /// Required. The names entity types to delete. All names must point to the same agent as `parent`.
    #[serde(rename="entityTypeNames")]
    
    pub entity_type_names: Option<Vec<String>>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1BatchDeleteEntityTypesRequest {}


/// The request message for Intents.BatchDeleteIntents.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent intents batch delete projects](ProjectAgentIntentBatchDeleteCall) (request)
/// * [locations agent intents batch delete projects](ProjectLocationAgentIntentBatchDeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1BatchDeleteIntentsRequest {
    /// Required. The collection of intents to delete. Only intent `name` must be filled in.
    
    pub intents: Option<Vec<GoogleCloudDialogflowV2beta1Intent>>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1BatchDeleteIntentsRequest {}


/// The request message for EntityTypes.BatchUpdateEntities.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent entity types entities batch update projects](ProjectAgentEntityTypeEntityBatchUpdateCall) (request)
/// * [locations agent entity types entities batch update projects](ProjectLocationAgentEntityTypeEntityBatchUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1BatchUpdateEntitiesRequest {
    /// Required. The entities to update or create.
    
    pub entities: Option<Vec<GoogleCloudDialogflowV2beta1EntityTypeEntity>>,
    /// Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Optional. The mask to control which fields get updated.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1BatchUpdateEntitiesRequest {}


/// The request message for EntityTypes.BatchUpdateEntityTypes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent entity types batch update projects](ProjectAgentEntityTypeBatchUpdateCall) (request)
/// * [locations agent entity types batch update projects](ProjectLocationAgentEntityTypeBatchUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1BatchUpdateEntityTypesRequest {
    /// The collection of entity types to update or create.
    #[serde(rename="entityTypeBatchInline")]
    
    pub entity_type_batch_inline: Option<GoogleCloudDialogflowV2beta1EntityTypeBatch>,
    /// The URI to a Google Cloud Storage file containing entity types to update or create. The file format can either be a serialized proto (of EntityBatch type) or a JSON object. Note: The URI must start with "gs://".
    #[serde(rename="entityTypeBatchUri")]
    
    pub entity_type_batch_uri: Option<String>,
    /// Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Optional. The mask to control which fields get updated.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1BatchUpdateEntityTypesRequest {}


/// The request message for Intents.BatchUpdateIntents.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent intents batch update projects](ProjectAgentIntentBatchUpdateCall) (request)
/// * [locations agent intents batch update projects](ProjectLocationAgentIntentBatchUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequest {
    /// The collection of intents to update or create.
    #[serde(rename="intentBatchInline")]
    
    pub intent_batch_inline: Option<GoogleCloudDialogflowV2beta1IntentBatch>,
    /// The URI to a Google Cloud Storage file containing intents to update or create. The file format can either be a serialized proto (of IntentBatch type) or JSON object. Note: The URI must start with "gs://".
    #[serde(rename="intentBatchUri")]
    
    pub intent_batch_uri: Option<String>,
    /// Optional. The resource view to apply to the returned intent.
    #[serde(rename="intentView")]
    
    pub intent_view: Option<GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequestIntentViewEnum>,
    /// Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Optional. The mask to control which fields get updated.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequest {}


/// The request message for ConversationProfiles.ClearFeature.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversation profiles clear suggestion feature config projects](ProjectConversationProfileClearSuggestionFeatureConfigCall) (request)
/// * [locations conversation profiles clear suggestion feature config projects](ProjectLocationConversationProfileClearSuggestionFeatureConfigCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequest {
    /// Required. The participant role to remove the suggestion feature config. Only HUMAN_AGENT or END_USER can be used.
    #[serde(rename="participantRole")]
    
    pub participant_role: Option<GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestParticipantRoleEnum>,
    /// Required. The type of the suggestion feature to remove.
    #[serde(rename="suggestionFeatureType")]
    
    pub suggestion_feature_type: Option<GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequest {}


/// The request message for Participants.CompileSuggestion.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations participants suggestions compile projects](ProjectConversationParticipantSuggestionCompileCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1CompileSuggestionRequest {
    /// Optional. Max number of messages prior to and including [latest_message] to use as context when compiling the suggestion. If zero or less than zero, 20 is used.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// Optional. The name of the latest conversation message to compile suggestion for. If empty, it will be the latest message of the conversation. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1CompileSuggestionRequest {}


/// The response message for Participants.CompileSuggestion.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations participants suggestions compile projects](ProjectConversationParticipantSuggestionCompileCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1CompileSuggestionResponse {
    /// Number of messages prior to and including latest_message to compile the suggestion. It may be smaller than the CompileSuggestionRequest.context_size field in the request if there aren't that many messages in the conversation.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// The name of the latest conversation message used to compile suggestion for. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
    /// The compiled suggestion.
    
    pub suggestion: Option<GoogleCloudDialogflowV2beta1Suggestion>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1CompileSuggestionResponse {}


/// The request message for Conversations.CompleteConversation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations complete projects](ProjectConversationCompleteCall) (request)
/// * [locations conversations complete projects](ProjectLocationConversationCompleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1CompleteConversationRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDialogflowV2beta1CompleteConversationRequest {}


/// Dialogflow contexts are similar to natural language context. If a person says to you “they are orange”, you need context in order to understand what “they” is referring to. Similarly, for Dialogflow to handle an end-user expression like that, it needs to be provided with context in order to correctly match an intent. Using contexts, you can control the flow of a conversation. You can configure contexts for an intent by setting input and output contexts, which are identified by string names. When an intent is matched, any configured output contexts for that intent become active. While any contexts are active, Dialogflow is more likely to match intents that are configured with input contexts that correspond to the currently active contexts. For more information about context, see the [Contexts guide](https://cloud.google.com/dialogflow/docs/contexts-overview).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent environments users sessions contexts create projects](ProjectAgentEnvironmentUserSessionContextCreateCall) (request|response)
/// * [agent environments users sessions contexts get projects](ProjectAgentEnvironmentUserSessionContextGetCall) (response)
/// * [agent environments users sessions contexts patch projects](ProjectAgentEnvironmentUserSessionContextPatchCall) (request|response)
/// * [agent sessions contexts create projects](ProjectAgentSessionContextCreateCall) (request|response)
/// * [agent sessions contexts get projects](ProjectAgentSessionContextGetCall) (response)
/// * [agent sessions contexts patch projects](ProjectAgentSessionContextPatchCall) (request|response)
/// * [locations agent environments users sessions contexts create projects](ProjectLocationAgentEnvironmentUserSessionContextCreateCall) (request|response)
/// * [locations agent environments users sessions contexts get projects](ProjectLocationAgentEnvironmentUserSessionContextGetCall) (response)
/// * [locations agent environments users sessions contexts patch projects](ProjectLocationAgentEnvironmentUserSessionContextPatchCall) (request|response)
/// * [locations agent sessions contexts create projects](ProjectLocationAgentSessionContextCreateCall) (request|response)
/// * [locations agent sessions contexts get projects](ProjectLocationAgentSessionContextGetCall) (response)
/// * [locations agent sessions contexts patch projects](ProjectLocationAgentSessionContextPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1Context {
    /// Optional. The number of conversational query requests after which the context expires. The default is `0`. If set to `0`, the context expires immediately. Contexts expire automatically after 20 minutes if there are no matching queries.
    #[serde(rename="lifespanCount")]
    
    pub lifespan_count: Option<i32>,
    /// Required. The unique identifier of the context. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, The `Context ID` is always converted to lowercase, may only contain characters in a-zA-Z0-9_-% and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size`
    
    pub name: Option<String>,
    /// Optional. The collection of parameters associated with this context. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value
    
    pub parameters: Option<HashMap<String, json::Value>>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1Context {}
impl client::ResponseResult for GoogleCloudDialogflowV2beta1Context {}


/// Represents a conversation. A conversation is an interaction between an agent, including live agents and Dialogflow agents, and a support customer. Conversations can include phone calls and text-based chat sessions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations complete projects](ProjectConversationCompleteCall) (response)
/// * [conversations create projects](ProjectConversationCreateCall) (request|response)
/// * [conversations get projects](ProjectConversationGetCall) (response)
/// * [locations conversations complete projects](ProjectLocationConversationCompleteCall) (response)
/// * [locations conversations create projects](ProjectLocationConversationCreateCall) (request|response)
/// * [locations conversations get projects](ProjectLocationConversationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1Conversation {
    /// Required. The Conversation Profile to be used to configure this Conversation. This field cannot be updated. Format: `projects//locations//conversationProfiles/`.
    #[serde(rename="conversationProfile")]
    
    pub conversation_profile: Option<String>,
    /// The stage of a conversation. It indicates whether the virtual agent or a human agent is handling the conversation. If the conversation is created with the conversation profile that has Dialogflow config set, defaults to ConversationStage.VIRTUAL_AGENT_STAGE; Otherwise, defaults to ConversationStage.HUMAN_ASSIST_STAGE. If the conversation is created with the conversation profile that has Dialogflow config set but explicitly sets conversation_stage to ConversationStage.HUMAN_ASSIST_STAGE, it skips ConversationStage.VIRTUAL_AGENT_STAGE stage and directly goes to ConversationStage.HUMAN_ASSIST_STAGE.
    #[serde(rename="conversationStage")]
    
    pub conversation_stage: Option<GoogleCloudDialogflowV2beta1ConversationConversationStageEnum>,
    /// Output only. The time the conversation was finished.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The current state of the Conversation.
    #[serde(rename="lifecycleState")]
    
    pub lifecycle_state: Option<GoogleCloudDialogflowV2beta1ConversationLifecycleStateEnum>,
    /// Output only. The unique identifier of this conversation. Format: `projects//locations//conversations/`.
    
    pub name: Option<String>,
    /// Output only. Required if the conversation is to be connected over telephony.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<GoogleCloudDialogflowV2beta1ConversationPhoneNumber>,
    /// Output only. The time the conversation was started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1Conversation {}
impl client::ResponseResult for GoogleCloudDialogflowV2beta1Conversation {}


/// Represents a phone number for telephony integration. It allows for connecting a particular conversation over telephony.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ConversationPhoneNumber {
    /// Output only. The phone number to connect to this conversation.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1ConversationPhoneNumber {}


/// Defines the services to connect to incoming Dialogflow conversations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversation profiles create projects](ProjectConversationProfileCreateCall) (request|response)
/// * [conversation profiles get projects](ProjectConversationProfileGetCall) (response)
/// * [conversation profiles patch projects](ProjectConversationProfilePatchCall) (request|response)
/// * [locations conversation profiles create projects](ProjectLocationConversationProfileCreateCall) (request|response)
/// * [locations conversation profiles get projects](ProjectLocationConversationProfileGetCall) (response)
/// * [locations conversation profiles patch projects](ProjectLocationConversationProfilePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ConversationProfile {
    /// Configuration for an automated agent to use with this profile.
    #[serde(rename="automatedAgentConfig")]
    
    pub automated_agent_config: Option<GoogleCloudDialogflowV2beta1AutomatedAgentConfig>,
    /// Output only. Create time of the conversation profile.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Human readable name for this profile. Max length 1024 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Configuration for agent assistance to use with this profile.
    #[serde(rename="humanAgentAssistantConfig")]
    
    pub human_agent_assistant_config: Option<GoogleCloudDialogflowV2beta1HumanAgentAssistantConfig>,
    /// Configuration for connecting to a live agent. Currently, this feature is not general available, please contact Google to get access.
    #[serde(rename="humanAgentHandoffConfig")]
    
    pub human_agent_handoff_config: Option<GoogleCloudDialogflowV2beta1HumanAgentHandoffConfig>,
    /// Language code for the conversation profile. If not specified, the language is en-US. Language at ConversationProfile should be set for all non en-us languages. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US".
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Configuration for logging conversation lifecycle events.
    #[serde(rename="loggingConfig")]
    
    pub logging_config: Option<GoogleCloudDialogflowV2beta1LoggingConfig>,
    /// The unique identifier of this conversation profile. Format: `projects//locations//conversationProfiles/`.
    
    pub name: Option<String>,
    /// Configuration for publishing new message events. Event will be sent in format of ConversationEvent
    #[serde(rename="newMessageEventNotificationConfig")]
    
    pub new_message_event_notification_config: Option<GoogleCloudDialogflowV2beta1NotificationConfig>,
    /// Configuration for publishing conversation lifecycle events.
    #[serde(rename="notificationConfig")]
    
    pub notification_config: Option<GoogleCloudDialogflowV2beta1NotificationConfig>,
    /// Name of the CX SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`.
    #[serde(rename="securitySettings")]
    
    pub security_settings: Option<String>,
    /// Settings for speech transcription.
    #[serde(rename="sttConfig")]
    
    pub stt_config: Option<GoogleCloudDialogflowV2beta1SpeechToTextConfig>,
    /// The time zone of this conversational profile from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. Defaults to America/New_York.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// Output only. Update time of the conversation profile.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1ConversationProfile {}
impl client::ResponseResult for GoogleCloudDialogflowV2beta1ConversationProfile {}


/// The request message to create one Message. Currently it is only used in BatchCreateMessagesRequest.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1CreateMessageRequest {
    /// Required. The message to create. Message.participant is required.
    
    pub message: Option<GoogleCloudDialogflowV2beta1Message>,
    /// Required. Resource identifier of the conversation to create message. Format: `projects//locations//conversations/`.
    
    pub parent: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1CreateMessageRequest {}


/// The request to detect user’s intent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent environments users sessions detect intent projects](ProjectAgentEnvironmentUserSessionDetectIntentCall) (request)
/// * [agent sessions detect intent projects](ProjectAgentSessionDetectIntentCall) (request)
/// * [locations agent environments users sessions detect intent projects](ProjectLocationAgentEnvironmentUserSessionDetectIntentCall) (request)
/// * [locations agent sessions detect intent projects](ProjectLocationAgentSessionDetectIntentCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1DetectIntentRequest {
    /// The natural language speech audio to be processed. This field should be populated iff `query_input` is set to an input audio config. A single request can contain up to 1 minute of speech audio data.
    #[serde(rename="inputAudio")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub input_audio: Option<Vec<u8>>,
    /// Instructs the speech synthesizer how to generate the output audio. If this field is not set and agent-level speech synthesizer is not configured, no output audio is generated.
    #[serde(rename="outputAudioConfig")]
    
    pub output_audio_config: Option<GoogleCloudDialogflowV2beta1OutputAudioConfig>,
    /// Mask for output_audio_config indicating which settings in this request-level config should override speech synthesizer settings defined at agent-level. If unspecified or empty, output_audio_config replaces the agent-level config in its entirety.
    #[serde(rename="outputAudioConfigMask")]
    
    pub output_audio_config_mask: Option<client::FieldMask>,
    /// Required. The input specification. It can be set to: 1. an audio config which instructs the speech recognizer how to process the speech audio, 2. a conversational query in the form of text, or 3. an event that specifies which intent to trigger.
    #[serde(rename="queryInput")]
    
    pub query_input: Option<GoogleCloudDialogflowV2beta1QueryInput>,
    /// The parameters of this query.
    #[serde(rename="queryParams")]
    
    pub query_params: Option<GoogleCloudDialogflowV2beta1QueryParameters>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1DetectIntentRequest {}


/// The message returned from the DetectIntent method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent environments users sessions detect intent projects](ProjectAgentEnvironmentUserSessionDetectIntentCall) (response)
/// * [agent sessions detect intent projects](ProjectAgentSessionDetectIntentCall) (response)
/// * [locations agent environments users sessions detect intent projects](ProjectLocationAgentEnvironmentUserSessionDetectIntentCall) (response)
/// * [locations agent sessions detect intent projects](ProjectLocationAgentSessionDetectIntentCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1DetectIntentResponse {
    /// If Knowledge Connectors are enabled, there could be more than one result returned for a given query or event, and this field will contain all results except for the top one, which is captured in query_result. The alternative results are ordered by decreasing `QueryResult.intent_detection_confidence`. If Knowledge Connectors are disabled, this field will be empty until multiple responses for regular intents are supported, at which point those additional results will be surfaced here.
    #[serde(rename="alternativeQueryResults")]
    
    pub alternative_query_results: Option<Vec<GoogleCloudDialogflowV2beta1QueryResult>>,
    /// The audio data bytes encoded as specified in the request. Note: The output audio is generated based on the values of default platform text responses found in the `query_result.fulfillment_messages` field. If multiple default text responses exist, they will be concatenated when generating audio. If no default platform text responses exist, the generated audio content will be empty. In some scenarios, multiple output audio fields may be present in the response structure. In these cases, only the top-most-level audio output has content.
    #[serde(rename="outputAudio")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub output_audio: Option<Vec<u8>>,
    /// The config used by the speech synthesizer to generate the output audio.
    #[serde(rename="outputAudioConfig")]
    
    pub output_audio_config: Option<GoogleCloudDialogflowV2beta1OutputAudioConfig>,
    /// The selected results of the conversational query or event processing. See `alternative_query_results` for additional potential results.
    #[serde(rename="queryResult")]
    
    pub query_result: Option<GoogleCloudDialogflowV2beta1QueryResult>,
    /// The unique identifier of the response. It can be used to locate a response in the training example set or for reporting issues.
    #[serde(rename="responseId")]
    
    pub response_id: Option<String>,
    /// Specifies the status of the webhook request.
    #[serde(rename="webhookStatus")]
    
    pub webhook_status: Option<GoogleRpcStatus>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1DetectIntentResponse {}


/// A knowledge document to be used by a KnowledgeBase. For more information, see the [knowledge base guide](https://cloud.google.com/dialogflow/docs/how/knowledge-bases). Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent knowledge bases documents create projects](ProjectAgentKnowledgeBaseDocumentCreateCall) (request)
/// * [agent knowledge bases documents get projects](ProjectAgentKnowledgeBaseDocumentGetCall) (response)
/// * [agent knowledge bases documents patch projects](ProjectAgentKnowledgeBaseDocumentPatchCall) (request)
/// * [knowledge bases documents create projects](ProjectKnowledgeBaseDocumentCreateCall) (request)
/// * [knowledge bases documents get projects](ProjectKnowledgeBaseDocumentGetCall) (response)
/// * [knowledge bases documents patch projects](ProjectKnowledgeBaseDocumentPatchCall) (request)
/// * [locations knowledge bases documents create projects](ProjectLocationKnowledgeBaseDocumentCreateCall) (request)
/// * [locations knowledge bases documents get projects](ProjectLocationKnowledgeBaseDocumentGetCall) (response)
/// * [locations knowledge bases documents patch projects](ProjectLocationKnowledgeBaseDocumentPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1Document {
    /// The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types. Note: This field is in the process of being deprecated, please use raw_content instead.
    
    pub content: Option<String>,
    /// The URI where the file content is located. For documents stored in Google Cloud Storage, these URIs must have the form `gs:///`. NOTE: External URLs must correspond to public webpages, i.e., they must be indexed by Google Search. In particular, URLs for showing documents in Google Cloud Storage (i.e. the URL in your browser) are not supported. Instead use the `gs://` format URI described above.
    #[serde(rename="contentUri")]
    
    pub content_uri: Option<String>,
    /// Required. The display name of the document. The name must be 1024 bytes or less; otherwise, the creation request fails.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. If true, we try to automatically reload the document every day (at a time picked by the system). If false or unspecified, we don't try to automatically reload the document. Currently you can only enable automatic reload for documents sourced from a public url, see `source` field for the source types. Reload status can be tracked in `latest_reload_status`. If a reload fails, we will keep the document unchanged. If a reload fails with internal errors, the system will try to reload the document on the next day. If a reload fails with non-retriable errors (e.g. PERMISSION_DENIED), the system will not try to reload the document anymore. You need to manually reload the document successfully by calling `ReloadDocument` and clear the errors.
    #[serde(rename="enableAutoReload")]
    
    pub enable_auto_reload: Option<bool>,
    /// Required. The knowledge type of document content.
    #[serde(rename="knowledgeTypes")]
    
    pub knowledge_types: Option<Vec<GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum>>,
    /// Output only. The time and status of the latest reload. This reload may have been triggered automatically or manually and may not have succeeded.
    #[serde(rename="latestReloadStatus")]
    
    pub latest_reload_status: Option<GoogleCloudDialogflowV2beta1DocumentReloadStatus>,
    /// Optional. Metadata for the document. The metadata supports arbitrary key-value pairs. Suggested use cases include storing a document's title, an external URL distinct from the document's content_uri, etc. The max size of a `key` or a `value` of the metadata is 1024 bytes.
    
    pub metadata: Option<HashMap<String, String>>,
    /// Required. The MIME type of this document.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Optional. The document resource name. The name must be empty when creating a document. Format: `projects//locations//knowledgeBases//documents/`.
    
    pub name: Option<String>,
    /// The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types.
    #[serde(rename="rawContent")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub raw_content: Option<Vec<u8>>,
    /// Output only. The current state of the document.
    
    pub state: Option<GoogleCloudDialogflowV2beta1DocumentStateEnum>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1Document {}
impl client::ResponseResult for GoogleCloudDialogflowV2beta1Document {}


/// The status of a reload attempt.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1DocumentReloadStatus {
    /// Output only. The status of a reload attempt or the initial load.
    
    pub status: Option<GoogleRpcStatus>,
    /// Output only. The time of a reload attempt. This reload may have been triggered automatically or manually and may not have succeeded.
    
    pub time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1DocumentReloadStatus {}


/// The message in the response that indicates the parameters of DTMF.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1DtmfParameters {
    /// Indicates whether DTMF input can be handled in the next request.
    #[serde(rename="acceptsDtmfInput")]
    
    pub accepts_dtmf_input: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2beta1DtmfParameters {}


/// Each intent parameter has a type, called the entity type, which dictates exactly how data from an end-user expression is extracted. Dialogflow provides predefined system entities that can match many common types of data. For example, there are system entities for matching dates, times, colors, email addresses, and so on. You can also create your own custom entities for matching custom data. For example, you could define a vegetable entity that can match the types of vegetables available for purchase with a grocery store agent. For more information, see the [Entity guide](https://cloud.google.com/dialogflow/docs/entities-overview).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent entity types create projects](ProjectAgentEntityTypeCreateCall) (request|response)
/// * [agent entity types get projects](ProjectAgentEntityTypeGetCall) (response)
/// * [agent entity types patch projects](ProjectAgentEntityTypePatchCall) (request|response)
/// * [locations agent entity types create projects](ProjectLocationAgentEntityTypeCreateCall) (request|response)
/// * [locations agent entity types get projects](ProjectLocationAgentEntityTypeGetCall) (response)
/// * [locations agent entity types patch projects](ProjectLocationAgentEntityTypePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1EntityType {
    /// Optional. Indicates whether the entity type can be automatically expanded.
    #[serde(rename="autoExpansionMode")]
    
    pub auto_expansion_mode: Option<GoogleCloudDialogflowV2beta1EntityTypeAutoExpansionModeEnum>,
    /// Required. The name of the entity type.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. Enables fuzzy entity extraction during classification.
    #[serde(rename="enableFuzzyExtraction")]
    
    pub enable_fuzzy_extraction: Option<bool>,
    /// Optional. The collection of entity entries associated with the entity type.
    
    pub entities: Option<Vec<GoogleCloudDialogflowV2beta1EntityTypeEntity>>,
    /// Required. Indicates the kind of entity type.
    
    pub kind: Option<GoogleCloudDialogflowV2beta1EntityTypeKindEnum>,
    /// The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType and EntityTypes.BatchUpdateEntityTypes methods. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1EntityType {}
impl client::ResponseResult for GoogleCloudDialogflowV2beta1EntityType {}


/// This message is a wrapper around a collection of entity types.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1EntityTypeBatch {
    /// A collection of entity types.
    #[serde(rename="entityTypes")]
    
    pub entity_types: Option<Vec<GoogleCloudDialogflowV2beta1EntityType>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1EntityTypeBatch {}


/// An **entity entry** for an associated entity type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1EntityTypeEntity {
    /// Required. A collection of value synonyms. For example, if the entity type is *vegetable*, and `value` is *scallions*, a synonym could be *green onions*. For `KIND_LIST` entity types: * This collection must contain exactly one synonym equal to `value`.
    
    pub synonyms: Option<Vec<String>>,
    /// Required. The primary value associated with this entity entry. For example, if the entity type is *vegetable*, the value could be *scallions*. For `KIND_MAP` entity types: * A reference value to be used in place of synonyms. For `KIND_LIST` entity types: * A string that can contain references to other entity types (with or without aliases).
    
    pub value: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1EntityTypeEntity {}


/// You can create multiple versions of your agent and publish them to separate environments. When you edit an agent, you are editing the draft agent. At any point, you can save the draft agent as an agent version, which is an immutable snapshot of your agent. When you save the draft agent, it is published to the default environment. When you create agent versions, you can publish them to custom environments. You can create a variety of custom environments for: - testing - development - production - etc. For more information, see the [versions and environments guide](https://cloud.google.com/dialogflow/docs/agents-versions).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent environments create projects](ProjectAgentEnvironmentCreateCall) (request|response)
/// * [agent environments get projects](ProjectAgentEnvironmentGetCall) (response)
/// * [agent environments patch projects](ProjectAgentEnvironmentPatchCall) (request|response)
/// * [locations agent environments create projects](ProjectLocationAgentEnvironmentCreateCall) (request|response)
/// * [locations agent environments get projects](ProjectLocationAgentEnvironmentGetCall) (response)
/// * [locations agent environments patch projects](ProjectLocationAgentEnvironmentPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1Environment {
    /// Optional. The agent version loaded into this environment. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/`
    #[serde(rename="agentVersion")]
    
    pub agent_version: Option<String>,
    /// Optional. The developer-provided description for this environment. The maximum length is 500 characters. If exceeded, the request is rejected.
    
    pub description: Option<String>,
    /// Optional. The fulfillment settings to use for this environment.
    
    pub fulfillment: Option<GoogleCloudDialogflowV2beta1Fulfillment>,
    /// Output only. The unique identifier of this agent environment. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/`
    
    pub name: Option<String>,
    /// Output only. The state of this environment. This field is read-only, i.e., it cannot be set by create and update methods.
    
    pub state: Option<GoogleCloudDialogflowV2beta1EnvironmentStateEnum>,
    /// Optional. Text to speech settings for this environment.
    #[serde(rename="textToSpeechSettings")]
    
    pub text_to_speech_settings: Option<GoogleCloudDialogflowV2beta1TextToSpeechSettings>,
    /// Output only. The last update time of this environment. This field is read-only, i.e., it cannot be set by create and update methods.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1Environment {}
impl client::ResponseResult for GoogleCloudDialogflowV2beta1Environment {}


/// The response message for Environments.GetEnvironmentHistory.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent environments get history projects](ProjectAgentEnvironmentGetHistoryCall) (response)
/// * [locations agent environments get history projects](ProjectLocationAgentEnvironmentGetHistoryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1EnvironmentHistory {
    /// Output only. The list of agent environments. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub entries: Option<Vec<GoogleCloudDialogflowV2beta1EnvironmentHistoryEntry>>,
    /// Output only. Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Output only. The name of the environment this history is for. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/`
    
    pub parent: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1EnvironmentHistory {}


/// Represents an environment history entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1EnvironmentHistoryEntry {
    /// The agent version loaded into this environment history entry.
    #[serde(rename="agentVersion")]
    
    pub agent_version: Option<String>,
    /// The creation time of this environment history entry.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The developer-provided description for this environment history entry.
    
    pub description: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1EnvironmentHistoryEntry {}


/// Events allow for matching intents by event name instead of the natural language input. For instance, input `` can trigger a personalized welcome response. The parameter `name` may be used by the agent in the response: `"Hello #welcome_event.name! What can I do for you today?"`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1EventInput {
    /// Required. The language of this query. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language. This field is ignored when used in the context of a WebhookResponse.followup_event_input field, because the language was already defined in the originating detect intent request.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Required. The unique identifier of the event.
    
    pub name: Option<String>,
    /// The collection of parameters associated with the event. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value
    
    pub parameters: Option<HashMap<String, json::Value>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1EventInput {}


/// The request message for Agents.ExportAgent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent export projects](ProjectAgentExportCall) (request)
/// * [locations agent export projects](ProjectLocationAgentExportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ExportAgentRequest {
    /// Optional. The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to export the agent to. The format of this URI must be `gs:///`. If left unspecified, the serialized agent is returned inline. Dialogflow performs a write operation for the Cloud Storage object on the caller's behalf, so your request authentication must have write permissions for the object. For more information, see [Dialogflow access control](https://cloud.google.com/dialogflow/cx/docs/concept/access-control#storage).
    #[serde(rename="agentUri")]
    
    pub agent_uri: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1ExportAgentRequest {}


/// Represents answer from "frequently asked questions".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1FaqAnswer {
    /// The piece of text from the `source` knowledge base document.
    
    pub answer: Option<String>,
    /// The name of answer record, in the format of "projects//locations//answerRecords/"
    #[serde(rename="answerRecord")]
    
    pub answer_record: Option<String>,
    /// The system's confidence score that this Knowledge answer is a good match for this conversational query, range from 0.0 (completely uncertain) to 1.0 (completely certain).
    
    pub confidence: Option<f32>,
    /// A map that contains metadata about the answer and the document from which it originates.
    
    pub metadata: Option<HashMap<String, String>>,
    /// The corresponding FAQ question.
    
    pub question: Option<String>,
    /// Indicates which Knowledge Document this answer was extracted from. Format: `projects//locations//agent/knowledgeBases//documents/`.
    
    pub source: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1FaqAnswer {}


/// By default, your agent responds to a matched intent with a static response. As an alternative, you can provide a more dynamic response by using fulfillment. When you enable fulfillment for an intent, Dialogflow responds to that intent by calling a service that you define. For example, if an end-user wants to schedule a haircut on Friday, your service can check your database and respond to the end-user with availability information for Friday. For more information, see the [fulfillment guide](https://cloud.google.com/dialogflow/docs/fulfillment-overview).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent get fulfillment projects](ProjectAgentGetFulfillmentCall) (response)
/// * [agent update fulfillment projects](ProjectAgentUpdateFulfillmentCall) (request|response)
/// * [locations agent get fulfillment projects](ProjectLocationAgentGetFulfillmentCall) (response)
/// * [locations agent update fulfillment projects](ProjectLocationAgentUpdateFulfillmentCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1Fulfillment {
    /// The human-readable name of the fulfillment, unique within the agent. This field is not used for Fulfillment in an Environment.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Whether fulfillment is enabled.
    
    pub enabled: Option<bool>,
    /// The field defines whether the fulfillment is enabled for certain features.
    
    pub features: Option<Vec<GoogleCloudDialogflowV2beta1FulfillmentFeature>>,
    /// Configuration for a generic web service.
    #[serde(rename="genericWebService")]
    
    pub generic_web_service: Option<GoogleCloudDialogflowV2beta1FulfillmentGenericWebService>,
    /// Required. The unique identifier of the fulfillment. Supported formats: - `projects//agent/fulfillment` - `projects//locations//agent/fulfillment` This field is not used for Fulfillment in an Environment.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1Fulfillment {}
impl client::ResponseResult for GoogleCloudDialogflowV2beta1Fulfillment {}


/// Whether fulfillment is enabled for the specific feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1FulfillmentFeature {
    /// The type of the feature that enabled for fulfillment.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudDialogflowV2beta1FulfillmentFeatureTypeEnum>,
}

impl client::Part for GoogleCloudDialogflowV2beta1FulfillmentFeature {}


/// Represents configuration for a generic web service. Dialogflow supports two mechanisms for authentications: - Basic authentication with username and password. - Authentication with additional authentication headers. More information could be found at: https://cloud.google.com/dialogflow/docs/fulfillment-configure.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1FulfillmentGenericWebService {
    /// Optional. Indicates if generic web service is created through Cloud Functions integration. Defaults to false. is_cloud_function is deprecated. Cloud functions can be configured by its uri as a regular web service now.
    #[serde(rename="isCloudFunction")]
    
    pub is_cloud_function: Option<bool>,
    /// The password for HTTP Basic authentication.
    
    pub password: Option<String>,
    /// The HTTP request headers to send together with fulfillment requests.
    #[serde(rename="requestHeaders")]
    
    pub request_headers: Option<HashMap<String, String>>,
    /// Required. The fulfillment URI for receiving POST requests. It must use https protocol.
    
    pub uri: Option<String>,
    /// The user name for HTTP Basic authentication.
    
    pub username: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1FulfillmentGenericWebService {}


/// Google Cloud Storage location for single input.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1GcsSource {
    /// Required. The Google Cloud Storage URIs for the inputs. A URI is of the form: gs://bucket/object-prefix-or-name Whether a prefix or name is used depends on the use case.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1GcsSource {}


/// Google Cloud Storage locations for the inputs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1GcsSources {
    /// Required. Google Cloud Storage URIs for the inputs. A URI is of the form: gs://bucket/object-prefix-or-name Whether a prefix or name is used depends on the use case.
    
    pub uris: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1GcsSources {}


/// Defines the Human Agent Assistant to connect to a conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1HumanAgentAssistantConfig {
    /// Configuration for agent assistance of end user participant. Currently, this feature is not general available, please contact Google to get access.
    #[serde(rename="endUserSuggestionConfig")]
    
    pub end_user_suggestion_config: Option<GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionConfig>,
    /// Configuration for agent assistance of human agent participant.
    #[serde(rename="humanAgentSuggestionConfig")]
    
    pub human_agent_suggestion_config: Option<GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionConfig>,
    /// Configuration for message analysis.
    #[serde(rename="messageAnalysisConfig")]
    
    pub message_analysis_config: Option<GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigMessageAnalysisConfig>,
    /// Pub/Sub topic on which to publish new agent assistant events.
    #[serde(rename="notificationConfig")]
    
    pub notification_config: Option<GoogleCloudDialogflowV2beta1NotificationConfig>,
}

impl client::Part for GoogleCloudDialogflowV2beta1HumanAgentAssistantConfig {}


/// Custom conversation models used in agent assist feature. Supported feature: ARTICLE_SUGGESTION, SMART_COMPOSE, SMART_REPLY, CONVERSATION_SUMMARIZATION.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigConversationModelConfig {
    /// Conversation model resource name. Format: `projects//conversationModels/`.
    
    pub model: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigConversationModelConfig {}


/// Config to process conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigConversationProcessConfig {
    /// Number of recent non-small-talk sentences to use as context for article and FAQ suggestion
    #[serde(rename="recentSentencesCount")]
    
    pub recent_sentences_count: Option<i32>,
}

impl client::Part for GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigConversationProcessConfig {}


/// Configuration for analyses to run on each conversation message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigMessageAnalysisConfig {
    /// Enable entity extraction in conversation messages on [agent assist stage](https://cloud.google.com/dialogflow/priv/docs/contact-center/basics#stages). If unspecified, defaults to false. Currently, this feature is not general available, please contact Google to get access.
    #[serde(rename="enableEntityExtraction")]
    
    pub enable_entity_extraction: Option<bool>,
    /// Enable sentiment analysis in conversation messages on [agent assist stage](https://cloud.google.com/dialogflow/priv/docs/contact-center/basics#stages). If unspecified, defaults to false. Sentiment analysis inspects user input and identifies the prevailing subjective opinion, especially to determine a user's attitude as positive, negative, or neutral: https://cloud.google.com/natural-language/docs/basics#sentiment_analysis For Participants.StreamingAnalyzeContent method, result will be in StreamingAnalyzeContentResponse.message.SentimentAnalysisResult. For Participants.AnalyzeContent method, result will be in AnalyzeContentResponse.message.SentimentAnalysisResult For Conversations.ListMessages method, result will be in ListMessagesResponse.messages.SentimentAnalysisResult If Pub/Sub notification is configured, result will be in ConversationEvent.new_message_payload.SentimentAnalysisResult.
    #[serde(rename="enableSentimentAnalysis")]
    
    pub enable_sentiment_analysis: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigMessageAnalysisConfig {}


/// Detail human agent assistant config.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionConfig {
    /// Configuration of different suggestion features. One feature can have only one config.
    #[serde(rename="featureConfigs")]
    
    pub feature_configs: Option<Vec<GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionFeatureConfig>>,
    /// If `group_suggestion_responses` is false, and there are multiple `feature_configs` in `event based suggestion` or StreamingAnalyzeContent, we will try to deliver suggestions to customers as soon as we get new suggestion. Different type of suggestions based on the same context will be in separate Pub/Sub event or `StreamingAnalyzeContentResponse`. If `group_suggestion_responses` set to true. All the suggestions to the same participant based on the same context will be grouped into a single Pub/Sub event or StreamingAnalyzeContentResponse.
    #[serde(rename="groupSuggestionResponses")]
    
    pub group_suggestion_responses: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionConfig {}


/// Config for suggestion features.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionFeatureConfig {
    /// Configs of custom conversation model.
    #[serde(rename="conversationModelConfig")]
    
    pub conversation_model_config: Option<GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigConversationModelConfig>,
    /// Configs for processing conversation.
    #[serde(rename="conversationProcessConfig")]
    
    pub conversation_process_config: Option<GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigConversationProcessConfig>,
    /// Automatically iterates all participants and tries to compile suggestions. Supported features: ARTICLE_SUGGESTION, FAQ, DIALOGFLOW_ASSIST.
    #[serde(rename="enableEventBasedSuggestion")]
    
    pub enable_event_based_suggestion: Option<bool>,
    /// Configs of query.
    #[serde(rename="queryConfig")]
    
    pub query_config: Option<GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfig>,
    /// The suggestion feature.
    #[serde(rename="suggestionFeature")]
    
    pub suggestion_feature: Option<GoogleCloudDialogflowV2beta1SuggestionFeature>,
    /// Settings of suggestion trigger. Currently, only ARTICLE_SUGGESTION, FAQ, and DIALOGFLOW_ASSIST will use this field.
    #[serde(rename="suggestionTriggerSettings")]
    
    pub suggestion_trigger_settings: Option<GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionTriggerSettings>,
}

impl client::Part for GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionFeatureConfig {}


/// Config for suggestion query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfig {
    /// Confidence threshold of query result. Agent Assist gives each suggestion a score in the range [0.0, 1.0], based on the relevance between the suggestion and the current conversation context. A score of 0.0 has no relevance, while a score of 1.0 has high relevance. Only suggestions with a score greater than or equal to the value of this field are included in the results. For a baseline model (the default), the recommended value is in the range [0.05, 0.1]. For a custom model, there is no recommended value. Tune this value by starting from a very low value and slowly increasing until you have desired results. If this field is not set, it is default to 0.0, which means that all suggestions are returned. Supported features: ARTICLE_SUGGESTION, FAQ, SMART_REPLY, SMART_COMPOSE.
    #[serde(rename="confidenceThreshold")]
    
    pub confidence_threshold: Option<f32>,
    /// Determines how recent conversation context is filtered when generating suggestions. If unspecified, no messages will be dropped.
    #[serde(rename="contextFilterSettings")]
    
    pub context_filter_settings: Option<GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigContextFilterSettings>,
    /// Query from Dialogflow agent. It is used by DIALOGFLOW_ASSIST.
    #[serde(rename="dialogflowQuerySource")]
    
    pub dialogflow_query_source: Option<GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigDialogflowQuerySource>,
    /// Query from knowledge base document. It is used by: SMART_REPLY, SMART_COMPOSE.
    #[serde(rename="documentQuerySource")]
    
    pub document_query_source: Option<GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigDocumentQuerySource>,
    /// Query from knowledgebase. It is used by: ARTICLE_SUGGESTION, FAQ.
    #[serde(rename="knowledgeBaseQuerySource")]
    
    pub knowledge_base_query_source: Option<GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigKnowledgeBaseQuerySource>,
    /// Maximum number of results to return. Currently, if unset, defaults to 10. And the max number is 20.
    #[serde(rename="maxResults")]
    
    pub max_results: Option<i32>,
}

impl client::Part for GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfig {}


/// Settings that determine how to filter recent conversation context when generating suggestions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigContextFilterSettings {
    /// If set to true, the last message from virtual agent (hand off message) and the message before it (trigger message of hand off) are dropped.
    #[serde(rename="dropHandoffMessages")]
    
    pub drop_handoff_messages: Option<bool>,
    /// If set to true, all messages from ivr stage are dropped.
    #[serde(rename="dropIvrMessages")]
    
    pub drop_ivr_messages: Option<bool>,
    /// If set to true, all messages from virtual agent are dropped.
    #[serde(rename="dropVirtualAgentMessages")]
    
    pub drop_virtual_agent_messages: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigContextFilterSettings {}


/// Dialogflow source setting. Supported feature: DIALOGFLOW_ASSIST.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigDialogflowQuerySource {
    /// Required. The name of a dialogflow virtual agent used for end user side intent detection and suggestion. Format: `projects//locations//agent`. When multiple agents are allowed in the same Dialogflow project.
    
    pub agent: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigDialogflowQuerySource {}


/// Document source settings. Supported features: SMART_REPLY, SMART_COMPOSE.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigDocumentQuerySource {
    /// Required. Knowledge documents to query from. Format: `projects//locations//knowledgeBases//documents/`. Currently, only one document is supported.
    
    pub documents: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigDocumentQuerySource {}


/// Knowledge base source settings. Supported features: ARTICLE_SUGGESTION, FAQ.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigKnowledgeBaseQuerySource {
    /// Required. Knowledge bases to query. Format: `projects//locations//knowledgeBases/`. Currently, only one knowledge base is supported.
    #[serde(rename="knowledgeBases")]
    
    pub knowledge_bases: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigKnowledgeBaseQuerySource {}


/// Settings of suggestion trigger.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionTriggerSettings {
    /// Do not trigger if last utterance is small talk.
    #[serde(rename="noSmallTalk")]
    
    pub no_small_talk: Option<bool>,
    /// Only trigger suggestion if participant role of last utterance is END_USER.
    #[serde(rename="onlyEndUser")]
    
    pub only_end_user: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionTriggerSettings {}


/// Defines the hand off to a live agent, typically on which external agent service provider to connect to a conversation. Currently, this feature is not general available, please contact Google to get access.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1HumanAgentHandoffConfig {
    /// Uses LivePerson (https://www.liveperson.com).
    #[serde(rename="livePersonConfig")]
    
    pub live_person_config: Option<GoogleCloudDialogflowV2beta1HumanAgentHandoffConfigLivePersonConfig>,
    /// Uses Salesforce Live Agent.
    #[serde(rename="salesforceLiveAgentConfig")]
    
    pub salesforce_live_agent_config: Option<GoogleCloudDialogflowV2beta1HumanAgentHandoffConfigSalesforceLiveAgentConfig>,
}

impl client::Part for GoogleCloudDialogflowV2beta1HumanAgentHandoffConfig {}


/// Configuration specific to LivePerson (https://www.liveperson.com).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1HumanAgentHandoffConfigLivePersonConfig {
    /// Required. Account number of the LivePerson account to connect. This is the account number you input at the login page.
    #[serde(rename="accountNumber")]
    
    pub account_number: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1HumanAgentHandoffConfigLivePersonConfig {}


/// Configuration specific to Salesforce Live Agent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1HumanAgentHandoffConfigSalesforceLiveAgentConfig {
    /// Required. Live Agent chat button ID.
    #[serde(rename="buttonId")]
    
    pub button_id: Option<String>,
    /// Required. Live Agent deployment ID.
    #[serde(rename="deploymentId")]
    
    pub deployment_id: Option<String>,
    /// Required. Domain of the Live Agent endpoint for this agent. You can find the endpoint URL in the `Live Agent settings` page. For example if URL has the form https://d.la4-c2-phx.salesforceliveagent.com/..., you should fill in d.la4-c2-phx.salesforceliveagent.com.
    #[serde(rename="endpointDomain")]
    
    pub endpoint_domain: Option<String>,
    /// Required. The organization ID of the Salesforce account.
    #[serde(rename="organizationId")]
    
    pub organization_id: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1HumanAgentHandoffConfigSalesforceLiveAgentConfig {}


/// The request message for Agents.ImportAgent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent import projects](ProjectAgentImportCall) (request)
/// * [locations agent import projects](ProjectLocationAgentImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ImportAgentRequest {
    /// Zip compressed raw byte content for agent.
    #[serde(rename="agentContent")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub agent_content: Option<Vec<u8>>,
    /// The URI to a Google Cloud Storage file containing the agent to import. Note: The URI must start with "gs://". Dialogflow performs a read operation for the Cloud Storage object on the caller's behalf, so your request authentication must have read permissions for the object. For more information, see [Dialogflow access control](https://cloud.google.com/dialogflow/cx/docs/concept/access-control#storage).
    #[serde(rename="agentUri")]
    
    pub agent_uri: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1ImportAgentRequest {}


/// The template used for importing documents.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ImportDocumentTemplate {
    /// Required. The knowledge type of document content.
    #[serde(rename="knowledgeTypes")]
    
    pub knowledge_types: Option<Vec<GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum>>,
    /// Metadata for the document. The metadata supports arbitrary key-value pairs. Suggested use cases include storing a document's title, an external URL distinct from the document's content_uri, etc. The max size of a `key` or a `value` of the metadata is 1024 bytes.
    
    pub metadata: Option<HashMap<String, String>>,
    /// Required. The MIME type of the document.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1ImportDocumentTemplate {}


/// Request message for Documents.ImportDocuments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [knowledge bases documents import projects](ProjectKnowledgeBaseDocumentImportCall) (request)
/// * [locations knowledge bases documents import projects](ProjectLocationKnowledgeBaseDocumentImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ImportDocumentsRequest {
    /// Required. Document template used for importing all the documents.
    #[serde(rename="documentTemplate")]
    
    pub document_template: Option<GoogleCloudDialogflowV2beta1ImportDocumentTemplate>,
    /// The Google Cloud Storage location for the documents. The path can include a wildcard. These URIs may have the forms `gs:///`. `gs:////*.`.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GoogleCloudDialogflowV2beta1GcsSources>,
    /// Whether to import custom metadata from Google Cloud Storage. Only valid when the document source is Google Cloud Storage URI.
    #[serde(rename="importGcsCustomMetadata")]
    
    pub import_gcs_custom_metadata: Option<bool>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1ImportDocumentsRequest {}


/// Instructs the speech recognizer on how to process the audio content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1InputAudioConfig {
    /// Required. Audio encoding of the audio content to process.
    #[serde(rename="audioEncoding")]
    
    pub audio_encoding: Option<GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum>,
    /// Only used in Participants.AnalyzeContent and Participants.StreamingAnalyzeContent. If `false` and recognition doesn't return any result, trigger `NO_SPEECH_RECOGNIZED` event to Dialogflow agent.
    #[serde(rename="disableNoSpeechRecognizedEvent")]
    
    pub disable_no_speech_recognized_event: Option<bool>,
    /// If `true`, Dialogflow returns SpeechWordInfo in StreamingRecognitionResult with information about the recognized speech words, e.g. start and end time offsets. If false or unspecified, Speech doesn't return any word-level information.
    #[serde(rename="enableWordInfo")]
    
    pub enable_word_info: Option<bool>,
    /// Required. The language of the supplied audio. Dialogflow does not do translations. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Which Speech model to select for the given request. Select the model best suited to your domain to get best results. If a model is not explicitly specified, then we auto-select a model based on the parameters in the InputAudioConfig. If enhanced speech model is enabled for the agent and an enhanced version of the specified model for the language does not exist, then the speech is recognized using the standard version of the specified model. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics#select-model) for more details.
    
    pub model: Option<String>,
    /// Which variant of the Speech model to use.
    #[serde(rename="modelVariant")]
    
    pub model_variant: Option<GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum>,
    /// A list of strings containing words and phrases that the speech recognizer should recognize with higher likelihood. See [the Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints) for more details. This field is deprecated. Please use [speech_contexts]() instead. If you specify both [phrase_hints]() and [speech_contexts](), Dialogflow will treat the [phrase_hints]() as a single additional [SpeechContext]().
    #[serde(rename="phraseHints")]
    
    pub phrase_hints: Option<Vec<String>>,
    /// Required. Sample rate (in Hertz) of the audio content sent in the query. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics) for more details.
    #[serde(rename="sampleRateHertz")]
    
    pub sample_rate_hertz: Option<i32>,
    /// If `false` (default), recognition does not cease until the client closes the stream. If `true`, the recognizer will detect a single spoken utterance in input audio. Recognition ceases when it detects the audio's voice has stopped or paused. In this case, once a detected intent is received, the client should close the stream and start a new request with a new stream as needed. Note: This setting is relevant only for streaming methods. Note: When specified, InputAudioConfig.single_utterance takes precedence over StreamingDetectIntentRequest.single_utterance.
    #[serde(rename="singleUtterance")]
    
    pub single_utterance: Option<bool>,
    /// Context information to assist speech recognition. See [the Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints) for more details.
    #[serde(rename="speechContexts")]
    
    pub speech_contexts: Option<Vec<GoogleCloudDialogflowV2beta1SpeechContext>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1InputAudioConfig {}


/// An intent categorizes an end-user’s intention for one conversation turn. For each agent, you define many intents, where your combined intents can handle a complete conversation. When an end-user writes or says something, referred to as an end-user expression or end-user input, Dialogflow matches the end-user input to the best intent in your agent. Matching an intent is also known as intent classification. For more information, see the [intent guide](https://cloud.google.com/dialogflow/docs/intents-overview).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent intents create projects](ProjectAgentIntentCreateCall) (request|response)
/// * [agent intents get projects](ProjectAgentIntentGetCall) (response)
/// * [agent intents patch projects](ProjectAgentIntentPatchCall) (request|response)
/// * [locations agent intents create projects](ProjectLocationAgentIntentCreateCall) (request|response)
/// * [locations agent intents get projects](ProjectLocationAgentIntentGetCall) (response)
/// * [locations agent intents patch projects](ProjectLocationAgentIntentPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1Intent {
    /// Optional. The name of the action associated with the intent. Note: The action name must not contain whitespaces.
    
    pub action: Option<String>,
    /// Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform).
    #[serde(rename="defaultResponsePlatforms")]
    
    pub default_response_platforms: Option<Vec<GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum>>,
    /// Required. The name of this intent.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. Indicates that this intent ends an interaction. Some integrations (e.g., Actions on Google or Dialogflow phone gateway) use this information to close interaction with an end user. Default is false.
    #[serde(rename="endInteraction")]
    
    pub end_interaction: Option<bool>,
    /// Optional. The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of the contexts must be present in the active user session for an event to trigger this intent. Event names are limited to 150 characters.
    
    pub events: Option<Vec<String>>,
    /// Output only. Information about all followup intents that have this intent as a direct or indirect parent. We populate this field only in the output.
    #[serde(rename="followupIntentInfo")]
    
    pub followup_intent_info: Option<Vec<GoogleCloudDialogflowV2beta1IntentFollowupIntentInfo>>,
    /// Optional. The list of context names required for this intent to be triggered. Formats: - `projects//agent/sessions/-/contexts/` - `projects//locations//agent/sessions/-/contexts/`
    #[serde(rename="inputContextNames")]
    
    pub input_context_names: Option<Vec<String>>,
    /// Optional. Indicates whether this is a fallback intent.
    #[serde(rename="isFallback")]
    
    pub is_fallback: Option<bool>,
    /// Optional. Indicates that a live agent should be brought in to handle the interaction with the user. In most cases, when you set this flag to true, you would also want to set end_interaction to true as well. Default is false.
    #[serde(rename="liveAgentHandoff")]
    
    pub live_agent_handoff: Option<bool>,
    /// Optional. The collection of rich messages corresponding to the `Response` field in the Dialogflow console.
    
    pub messages: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessage>>,
    /// Optional. Indicates whether Machine Learning is disabled for the intent. Note: If `ml_disabled` setting is set to true, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off.
    #[serde(rename="mlDisabled")]
    
    pub ml_disabled: Option<bool>,
    /// Optional. Indicates whether Machine Learning is enabled for the intent. Note: If `ml_enabled` setting is set to false, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off. DEPRECATED! Please use `ml_disabled` field instead. NOTE: If both `ml_enabled` and `ml_disabled` are either not set or false, then the default value is determined as follows: - Before April 15th, 2018 the default is: ml_enabled = false / ml_disabled = true. - After April 15th, 2018 the default is: ml_enabled = true / ml_disabled = false.
    #[serde(rename="mlEnabled")]
    
    pub ml_enabled: Option<bool>,
    /// Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Supported formats: - `projects//agent/intents/` - `projects//locations//agent/intents/`
    
    pub name: Option<String>,
    /// Optional. The collection of contexts that are activated when the intent is matched. Context messages in this collection should not set the parameters field. Setting the `lifespan_count` to 0 will reset the context when the intent is matched. Format: `projects//agent/sessions/-/contexts/`.
    #[serde(rename="outputContexts")]
    
    pub output_contexts: Option<Vec<GoogleCloudDialogflowV2beta1Context>>,
    /// Optional. The collection of parameters associated with the intent.
    
    pub parameters: Option<Vec<GoogleCloudDialogflowV2beta1IntentParameter>>,
    /// Optional. The unique identifier of the parent intent in the chain of followup intents. You can set this field when creating an intent, for example with CreateIntent or BatchUpdateIntents, in order to make this intent a followup intent. It identifies the parent followup intent. Format: `projects//agent/intents/`.
    #[serde(rename="parentFollowupIntentName")]
    
    pub parent_followup_intent_name: Option<String>,
    /// Optional. The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests.
    
    pub priority: Option<i32>,
    /// Optional. Indicates whether to delete all contexts in the current session when this intent is matched.
    #[serde(rename="resetContexts")]
    
    pub reset_contexts: Option<bool>,
    /// Output only. The unique identifier of the root intent in the chain of followup intents. It identifies the correct followup intents chain for this intent. Format: `projects//agent/intents/`.
    #[serde(rename="rootFollowupIntentName")]
    
    pub root_followup_intent_name: Option<String>,
    /// Optional. The collection of examples that the agent is trained on.
    #[serde(rename="trainingPhrases")]
    
    pub training_phrases: Option<Vec<GoogleCloudDialogflowV2beta1IntentTrainingPhrase>>,
    /// Optional. Indicates whether webhooks are enabled for the intent.
    #[serde(rename="webhookState")]
    
    pub webhook_state: Option<GoogleCloudDialogflowV2beta1IntentWebhookStateEnum>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1Intent {}
impl client::ResponseResult for GoogleCloudDialogflowV2beta1Intent {}


/// This message is a wrapper around a collection of intents.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentBatch {
    /// A collection of intents.
    
    pub intents: Option<Vec<GoogleCloudDialogflowV2beta1Intent>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentBatch {}


/// Represents a single followup intent in the chain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentFollowupIntentInfo {
    /// The unique identifier of the followup intent. Format: `projects//agent/intents/`.
    #[serde(rename="followupIntentName")]
    
    pub followup_intent_name: Option<String>,
    /// The unique identifier of the followup intent's parent. Format: `projects//agent/intents/`.
    #[serde(rename="parentFollowupIntentName")]
    
    pub parent_followup_intent_name: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentFollowupIntentInfo {}


/// Corresponds to the `Response` field in the Dialogflow console.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessage {
    /// Displays a basic card for Actions on Google.
    #[serde(rename="basicCard")]
    
    pub basic_card: Option<GoogleCloudDialogflowV2beta1IntentMessageBasicCard>,
    /// Browse carousel card for Actions on Google.
    #[serde(rename="browseCarouselCard")]
    
    pub browse_carousel_card: Option<GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCard>,
    /// Displays a card.
    
    pub card: Option<GoogleCloudDialogflowV2beta1IntentMessageCard>,
    /// Displays a carousel card for Actions on Google.
    #[serde(rename="carouselSelect")]
    
    pub carousel_select: Option<GoogleCloudDialogflowV2beta1IntentMessageCarouselSelect>,
    /// Displays an image.
    
    pub image: Option<GoogleCloudDialogflowV2beta1IntentMessageImage>,
    /// Displays a link out suggestion chip for Actions on Google.
    #[serde(rename="linkOutSuggestion")]
    
    pub link_out_suggestion: Option<GoogleCloudDialogflowV2beta1IntentMessageLinkOutSuggestion>,
    /// Displays a list card for Actions on Google.
    #[serde(rename="listSelect")]
    
    pub list_select: Option<GoogleCloudDialogflowV2beta1IntentMessageListSelect>,
    /// The media content card for Actions on Google.
    #[serde(rename="mediaContent")]
    
    pub media_content: Option<GoogleCloudDialogflowV2beta1IntentMessageMediaContent>,
    /// A custom platform-specific response.
    
    pub payload: Option<HashMap<String, json::Value>>,
    /// Optional. The platform that this message is intended for.
    
    pub platform: Option<GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum>,
    /// Displays quick replies.
    #[serde(rename="quickReplies")]
    
    pub quick_replies: Option<GoogleCloudDialogflowV2beta1IntentMessageQuickReplies>,
    /// Rich Business Messaging (RBM) carousel rich card response.
    #[serde(rename="rbmCarouselRichCard")]
    
    pub rbm_carousel_rich_card: Option<GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCard>,
    /// Standalone Rich Business Messaging (RBM) rich card response.
    #[serde(rename="rbmStandaloneRichCard")]
    
    pub rbm_standalone_rich_card: Option<GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCard>,
    /// Rich Business Messaging (RBM) text response. RBM allows businesses to send enriched and branded versions of SMS. See https://jibe.google.com/business-messaging.
    #[serde(rename="rbmText")]
    
    pub rbm_text: Option<GoogleCloudDialogflowV2beta1IntentMessageRbmText>,
    /// Returns a voice or text-only response for Actions on Google.
    #[serde(rename="simpleResponses")]
    
    pub simple_responses: Option<GoogleCloudDialogflowV2beta1IntentMessageSimpleResponses>,
    /// Displays suggestion chips for Actions on Google.
    
    pub suggestions: Option<GoogleCloudDialogflowV2beta1IntentMessageSuggestions>,
    /// Table card for Actions on Google.
    #[serde(rename="tableCard")]
    
    pub table_card: Option<GoogleCloudDialogflowV2beta1IntentMessageTableCard>,
    /// Plays audio from a file in Telephony Gateway.
    #[serde(rename="telephonyPlayAudio")]
    
    pub telephony_play_audio: Option<GoogleCloudDialogflowV2beta1IntentMessageTelephonyPlayAudio>,
    /// Synthesizes speech in Telephony Gateway.
    #[serde(rename="telephonySynthesizeSpeech")]
    
    pub telephony_synthesize_speech: Option<GoogleCloudDialogflowV2beta1IntentMessageTelephonySynthesizeSpeech>,
    /// Transfers the call in Telephony Gateway.
    #[serde(rename="telephonyTransferCall")]
    
    pub telephony_transfer_call: Option<GoogleCloudDialogflowV2beta1IntentMessageTelephonyTransferCall>,
    /// Returns a text response.
    
    pub text: Option<GoogleCloudDialogflowV2beta1IntentMessageText>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessage {}


/// The basic card message. Useful for displaying information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageBasicCard {
    /// Optional. The collection of card buttons.
    
    pub buttons: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessageBasicCardButton>>,
    /// Required, unless image is present. The body text of the card.
    #[serde(rename="formattedText")]
    
    pub formatted_text: Option<String>,
    /// Optional. The image for the card.
    
    pub image: Option<GoogleCloudDialogflowV2beta1IntentMessageImage>,
    /// Optional. The subtitle of the card.
    
    pub subtitle: Option<String>,
    /// Optional. The title of the card.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageBasicCard {}


/// The button object that appears at the bottom of a card.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageBasicCardButton {
    /// Required. Action to take when a user taps on the button.
    #[serde(rename="openUriAction")]
    
    pub open_uri_action: Option<GoogleCloudDialogflowV2beta1IntentMessageBasicCardButtonOpenUriAction>,
    /// Required. The title of the button.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageBasicCardButton {}


/// Opens the given URI.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageBasicCardButtonOpenUriAction {
    /// Required. The HTTP or HTTPS scheme URI.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageBasicCardButtonOpenUriAction {}


/// Browse Carousel Card for Actions on Google. https://developers.google.com/actions/assistant/responses#browsing_carousel
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCard {
    /// Optional. Settings for displaying the image. Applies to every image in items.
    #[serde(rename="imageDisplayOptions")]
    
    pub image_display_options: Option<GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum>,
    /// Required. List of items in the Browse Carousel Card. Minimum of two items, maximum of ten.
    
    pub items: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItem>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCard {}


/// Browsing carousel tile
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItem {
    /// Optional. Description of the carousel item. Maximum of four lines of text.
    
    pub description: Option<String>,
    /// Optional. Text that appears at the bottom of the Browse Carousel Card. Maximum of one line of text.
    
    pub footer: Option<String>,
    /// Optional. Hero image for the carousel item.
    
    pub image: Option<GoogleCloudDialogflowV2beta1IntentMessageImage>,
    /// Required. Action to present to the user.
    #[serde(rename="openUriAction")]
    
    pub open_uri_action: Option<GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction>,
    /// Required. Title of the carousel item. Maximum of two lines of text.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItem {}


/// Actions on Google action to open a given url.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction {
    /// Required. URL
    
    pub url: Option<String>,
    /// Optional. Specifies the type of viewer that is used when opening the URL. Defaults to opening via web browser.
    #[serde(rename="urlTypeHint")]
    
    pub url_type_hint: Option<GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction {}


/// The card response message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageCard {
    /// Optional. The collection of card buttons.
    
    pub buttons: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessageCardButton>>,
    /// Optional. The public URI to an image file for the card.
    #[serde(rename="imageUri")]
    
    pub image_uri: Option<String>,
    /// Optional. The subtitle of the card.
    
    pub subtitle: Option<String>,
    /// Optional. The title of the card.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageCard {}


/// Optional. Contains information about a button.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageCardButton {
    /// Optional. The text to send back to the Dialogflow API or a URI to open.
    
    pub postback: Option<String>,
    /// Optional. The text to show on the button.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageCardButton {}


/// The card for presenting a carousel of options to select from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageCarouselSelect {
    /// Required. Carousel items.
    
    pub items: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessageCarouselSelectItem>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageCarouselSelect {}


/// An item in the carousel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageCarouselSelectItem {
    /// Optional. The body text of the card.
    
    pub description: Option<String>,
    /// Optional. The image to display.
    
    pub image: Option<GoogleCloudDialogflowV2beta1IntentMessageImage>,
    /// Required. Additional info about the option item.
    
    pub info: Option<GoogleCloudDialogflowV2beta1IntentMessageSelectItemInfo>,
    /// Required. Title of the carousel item.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageCarouselSelectItem {}


/// Column properties for TableCard.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageColumnProperties {
    /// Required. Column heading.
    
    pub header: Option<String>,
    /// Optional. Defines text alignment for all cells in this column.
    #[serde(rename="horizontalAlignment")]
    
    pub horizontal_alignment: Option<GoogleCloudDialogflowV2beta1IntentMessageColumnPropertyHorizontalAlignmentEnum>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageColumnProperties {}


/// The image response message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageImage {
    /// A text description of the image to be used for accessibility, e.g., screen readers. Required if image_uri is set for CarouselSelect.
    #[serde(rename="accessibilityText")]
    
    pub accessibility_text: Option<String>,
    /// Optional. The public URI to an image file.
    #[serde(rename="imageUri")]
    
    pub image_uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageImage {}


/// The suggestion chip message that allows the user to jump out to the app or website associated with this agent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageLinkOutSuggestion {
    /// Required. The name of the app or site this chip is linking to.
    #[serde(rename="destinationName")]
    
    pub destination_name: Option<String>,
    /// Required. The URI of the app or site to open when the user taps the suggestion chip.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageLinkOutSuggestion {}


/// The card for presenting a list of options to select from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageListSelect {
    /// Required. List items.
    
    pub items: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessageListSelectItem>>,
    /// Optional. Subtitle of the list.
    
    pub subtitle: Option<String>,
    /// Optional. The overall title of the list.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageListSelect {}


/// An item in the list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageListSelectItem {
    /// Optional. The main text describing the item.
    
    pub description: Option<String>,
    /// Optional. The image to display.
    
    pub image: Option<GoogleCloudDialogflowV2beta1IntentMessageImage>,
    /// Required. Additional information about this option.
    
    pub info: Option<GoogleCloudDialogflowV2beta1IntentMessageSelectItemInfo>,
    /// Required. The title of the list item.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageListSelectItem {}


/// The media content card for Actions on Google.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageMediaContent {
    /// Required. List of media objects.
    #[serde(rename="mediaObjects")]
    
    pub media_objects: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessageMediaContentResponseMediaObject>>,
    /// Optional. What type of media is the content (ie "audio").
    #[serde(rename="mediaType")]
    
    pub media_type: Option<GoogleCloudDialogflowV2beta1IntentMessageMediaContentMediaTypeEnum>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageMediaContent {}


/// Response media object for media content card.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageMediaContentResponseMediaObject {
    /// Required. Url where the media is stored.
    #[serde(rename="contentUrl")]
    
    pub content_url: Option<String>,
    /// Optional. Description of media card.
    
    pub description: Option<String>,
    /// Optional. Icon to display above media content.
    
    pub icon: Option<GoogleCloudDialogflowV2beta1IntentMessageImage>,
    /// Optional. Image to display above media content.
    #[serde(rename="largeImage")]
    
    pub large_image: Option<GoogleCloudDialogflowV2beta1IntentMessageImage>,
    /// Required. Name of media card.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageMediaContentResponseMediaObject {}


/// The quick replies response message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageQuickReplies {
    /// Optional. The collection of quick replies.
    #[serde(rename="quickReplies")]
    
    pub quick_replies: Option<Vec<String>>,
    /// Optional. The title of the collection of quick replies.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageQuickReplies {}


/// Rich Business Messaging (RBM) Card content
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmCardContent {
    /// Optional. Description of the card (at most 2000 bytes). At least one of the title, description or media must be set.
    
    pub description: Option<String>,
    /// Optional. However at least one of the title, description or media must be set. Media (image, GIF or a video) to include in the card.
    
    pub media: Option<GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMedia>,
    /// Optional. List of suggestions to include in the card.
    
    pub suggestions: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestion>>,
    /// Optional. Title of the card (at most 200 bytes). At least one of the title, description or media must be set.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageRbmCardContent {}


/// Rich Business Messaging (RBM) Media displayed in Cards The following media-types are currently supported: Image Types * image/jpeg * image/jpg' * image/gif * image/png Video Types * video/h263 * video/m4v * video/mp4 * video/mpeg * video/mpeg4 * video/webm
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMedia {
    /// Required. Publicly reachable URI of the file. The RBM platform determines the MIME type of the file from the content-type field in the HTTP headers when the platform fetches the file. The content-type field must be present and accurate in the HTTP response from the URL.
    #[serde(rename="fileUri")]
    
    pub file_uri: Option<String>,
    /// Required for cards with vertical orientation. The height of the media within a rich card with a vertical layout. For a standalone card with horizontal layout, height is not customizable, and this field is ignored.
    
    pub height: Option<GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum>,
    /// Optional. Publicly reachable URI of the thumbnail.If you don't provide a thumbnail URI, the RBM platform displays a blank placeholder thumbnail until the user's device downloads the file. Depending on the user's setting, the file may not download automatically and may require the user to tap a download button.
    #[serde(rename="thumbnailUri")]
    
    pub thumbnail_uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMedia {}


/// Carousel Rich Business Messaging (RBM) rich card. Rich cards allow you to respond to users with more vivid content, e.g. with media and suggestions. If you want to show a single card with more control over the layout, please use RbmStandaloneCard instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCard {
    /// Required. The cards in the carousel. A carousel must have at least 2 cards and at most 10.
    #[serde(rename="cardContents")]
    
    pub card_contents: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessageRbmCardContent>>,
    /// Required. The width of the cards in the carousel.
    #[serde(rename="cardWidth")]
    
    pub card_width: Option<GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCard {}


/// Standalone Rich Business Messaging (RBM) rich card. Rich cards allow you to respond to users with more vivid content, e.g. with media and suggestions. You can group multiple rich cards into one using RbmCarouselCard but carousel cards will give you less control over the card layout.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCard {
    /// Required. Card content.
    #[serde(rename="cardContent")]
    
    pub card_content: Option<GoogleCloudDialogflowV2beta1IntentMessageRbmCardContent>,
    /// Required. Orientation of the card.
    #[serde(rename="cardOrientation")]
    
    pub card_orientation: Option<GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum>,
    /// Required if orientation is horizontal. Image preview alignment for standalone cards with horizontal layout.
    #[serde(rename="thumbnailImageAlignment")]
    
    pub thumbnail_image_alignment: Option<GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCard {}


/// Rich Business Messaging (RBM) suggested client-side action that the user can choose from the card.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedAction {
    /// Suggested client side action: Dial a phone number
    
    pub dial: Option<GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionDial>,
    /// Suggested client side action: Open a URI on device
    #[serde(rename="openUrl")]
    
    pub open_url: Option<GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionOpenUri>,
    /// Opaque payload that the Dialogflow receives in a user event when the user taps the suggested action. This data will be also forwarded to webhook to allow performing custom business logic.
    #[serde(rename="postbackData")]
    
    pub postback_data: Option<String>,
    /// Suggested client side action: Share user location
    #[serde(rename="shareLocation")]
    
    pub share_location: Option<GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionShareLocation>,
    /// Text to display alongside the action.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedAction {}


/// Opens the user's default dialer app with the specified phone number but does not dial automatically.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionDial {
    /// Required. The phone number to fill in the default dialer app. This field should be in [E.164](https://en.wikipedia.org/wiki/E.164) format. An example of a correctly formatted phone number: +15556767888.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionDial {}


/// Opens the user's default web browser app to the specified uri If the user has an app installed that is registered as the default handler for the URL, then this app will be opened instead, and its icon will be used in the suggested action UI.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionOpenUri {
    /// Required. The uri to open on the user device
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionOpenUri {}


/// Opens the device's location chooser so the user can pick a location to send back to the agent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionShareLocation { _never_set: Option<bool> }

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionShareLocation {}


/// Rich Business Messaging (RBM) suggested reply that the user can click instead of typing in their own response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedReply {
    /// Opaque payload that the Dialogflow receives in a user event when the user taps the suggested reply. This data will be also forwarded to webhook to allow performing custom business logic.
    #[serde(rename="postbackData")]
    
    pub postback_data: Option<String>,
    /// Suggested reply text.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedReply {}


/// Rich Business Messaging (RBM) suggestion. Suggestions allow user to easily select/click a predefined response or perform an action (like opening a web uri).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestion {
    /// Predefined client side actions that user can choose
    
    pub action: Option<GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedAction>,
    /// Predefined replies for user to select instead of typing
    
    pub reply: Option<GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedReply>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestion {}


/// Rich Business Messaging (RBM) text response with suggestions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmText {
    /// Optional. One or more suggestions to show to the user.
    #[serde(rename="rbmSuggestion")]
    
    pub rbm_suggestion: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestion>>,
    /// Required. Text sent and displayed to the user.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageRbmText {}


/// Additional info about the select item for when it is triggered in a dialog.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageSelectItemInfo {
    /// Required. A unique key that will be sent back to the agent if this response is given.
    
    pub key: Option<String>,
    /// Optional. A list of synonyms that can also be used to trigger this item in dialog.
    
    pub synonyms: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageSelectItemInfo {}


/// The simple response message containing speech or text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageSimpleResponse {
    /// Optional. The text to display.
    #[serde(rename="displayText")]
    
    pub display_text: Option<String>,
    /// One of text_to_speech or ssml must be provided. Structured spoken response to the user in the SSML format. Mutually exclusive with text_to_speech.
    
    pub ssml: Option<String>,
    /// One of text_to_speech or ssml must be provided. The plain text of the speech output. Mutually exclusive with ssml.
    #[serde(rename="textToSpeech")]
    
    pub text_to_speech: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageSimpleResponse {}


/// The collection of simple response candidates. This message in `QueryResult.fulfillment_messages` and `WebhookResponse.fulfillment_messages` should contain only one `SimpleResponse`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageSimpleResponses {
    /// Required. The list of simple responses.
    #[serde(rename="simpleResponses")]
    
    pub simple_responses: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessageSimpleResponse>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageSimpleResponses {}


/// The suggestion chip message that the user can tap to quickly post a reply to the conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageSuggestion {
    /// Required. The text shown the in the suggestion chip.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageSuggestion {}


/// The collection of suggestions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageSuggestions {
    /// Required. The list of suggested replies.
    
    pub suggestions: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessageSuggestion>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageSuggestions {}


/// Table card for Actions on Google.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageTableCard {
    /// Optional. List of buttons for the card.
    
    pub buttons: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessageBasicCardButton>>,
    /// Optional. Display properties for the columns in this table.
    #[serde(rename="columnProperties")]
    
    pub column_properties: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessageColumnProperties>>,
    /// Optional. Image which should be displayed on the card.
    
    pub image: Option<GoogleCloudDialogflowV2beta1IntentMessageImage>,
    /// Optional. Rows in this table of data.
    
    pub rows: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessageTableCardRow>>,
    /// Optional. Subtitle to the title.
    
    pub subtitle: Option<String>,
    /// Required. Title of the card.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageTableCard {}


/// Cell of TableCardRow.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageTableCardCell {
    /// Required. Text in this cell.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageTableCardCell {}


/// Row of TableCard.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageTableCardRow {
    /// Optional. List of cells that make up this row.
    
    pub cells: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessageTableCardCell>>,
    /// Optional. Whether to add a visual divider after this row.
    #[serde(rename="dividerAfter")]
    
    pub divider_after: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageTableCardRow {}


/// Plays audio from a file in Telephony Gateway.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageTelephonyPlayAudio {
    /// Required. URI to a Google Cloud Storage object containing the audio to play, e.g., "gs://bucket/object". The object must contain a single channel (mono) of linear PCM audio (2 bytes / sample) at 8kHz. This object must be readable by the `service-@gcp-sa-dialogflow.iam.gserviceaccount.com` service account where is the number of the Telephony Gateway project (usually the same as the Dialogflow agent project). If the Google Cloud Storage bucket is in the Telephony Gateway project, this permission is added by default when enabling the Dialogflow V2 API. For audio from other sources, consider using the `TelephonySynthesizeSpeech` message with SSML.
    #[serde(rename="audioUri")]
    
    pub audio_uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageTelephonyPlayAudio {}


/// Synthesizes speech and plays back the synthesized audio to the caller in Telephony Gateway. Telephony Gateway takes the synthesizer settings from `DetectIntentResponse.output_audio_config` which can either be set at request-level or can come from the agent-level synthesizer config.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageTelephonySynthesizeSpeech {
    /// The SSML to be synthesized. For more information, see [SSML](https://developers.google.com/actions/reference/ssml).
    
    pub ssml: Option<String>,
    /// The raw text to be synthesized.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageTelephonySynthesizeSpeech {}


/// Transfers the call in Telephony Gateway.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageTelephonyTransferCall {
    /// Required. The phone number to transfer the call to in [E.164 format](https://en.wikipedia.org/wiki/E.164). We currently only allow transferring to US numbers (+1xxxyyyzzzz).
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageTelephonyTransferCall {}


/// The text response message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentMessageText {
    /// Optional. The collection of the agent's responses.
    
    pub text: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentMessageText {}


/// Represents intent parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentParameter {
    /// Optional. The default value to use when the `value` yields an empty result. Default values can be extracted from contexts by using the following syntax: `#context_name.parameter_name`.
    #[serde(rename="defaultValue")]
    
    pub default_value: Option<String>,
    /// Required. The name of the parameter.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. The name of the entity type, prefixed with `@`, that describes values of the parameter. If the parameter is required, this must be provided.
    #[serde(rename="entityTypeDisplayName")]
    
    pub entity_type_display_name: Option<String>,
    /// Optional. Indicates whether the parameter represents a list of values.
    #[serde(rename="isList")]
    
    pub is_list: Option<bool>,
    /// Optional. Indicates whether the parameter is required. That is, whether the intent cannot be completed without collecting the parameter value.
    
    pub mandatory: Option<bool>,
    /// The unique identifier of this parameter.
    
    pub name: Option<String>,
    /// Optional. The collection of prompts that the agent can present to the user in order to collect a value for the parameter.
    
    pub prompts: Option<Vec<String>>,
    /// Optional. The definition of the parameter value. It can be: - a constant string, - a parameter value defined as `$parameter_name`, - an original parameter value defined as `$parameter_name.original`, - a parameter value from some context defined as `#context_name.parameter_name`.
    
    pub value: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentParameter {}


/// Represents an example that the agent is trained on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentTrainingPhrase {
    /// Output only. The unique identifier of this training phrase.
    
    pub name: Option<String>,
    /// Required. The ordered list of training phrase parts. The parts are concatenated in order to form the training phrase. Note: The API does not automatically annotate training phrases like the Dialogflow Console does. Note: Do not forget to include whitespace at part boundaries, so the training phrase is well formatted when the parts are concatenated. If the training phrase does not need to be annotated with parameters, you just need a single part with only the Part.text field set. If you want to annotate the training phrase, you must create multiple parts, where the fields of each part are populated in one of two ways: - `Part.text` is set to a part of the phrase that has no parameters. - `Part.text` is set to a part of the phrase that you want to annotate, and the `entity_type`, `alias`, and `user_defined` fields are all set.
    
    pub parts: Option<Vec<GoogleCloudDialogflowV2beta1IntentTrainingPhrasePart>>,
    /// Optional. Indicates how many times this example was added to the intent. Each time a developer adds an existing sample by editing an intent or training, this counter is increased.
    #[serde(rename="timesAddedCount")]
    
    pub times_added_count: Option<i32>,
    /// Required. The type of the training phrase.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentTrainingPhrase {}


/// Represents a part of a training phrase.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1IntentTrainingPhrasePart {
    /// Optional. The parameter name for the value extracted from the annotated part of the example. This field is required for annotated parts of the training phrase.
    
    pub alias: Option<String>,
    /// Optional. The entity type name prefixed with `@`. This field is required for annotated parts of the training phrase.
    #[serde(rename="entityType")]
    
    pub entity_type: Option<String>,
    /// Required. The text for this part.
    
    pub text: Option<String>,
    /// Optional. Indicates whether the text was manually annotated. This field is set to true when the Dialogflow Console is used to manually annotate the part. When creating an annotated part with the API, you must set this to true.
    #[serde(rename="userDefined")]
    
    pub user_defined: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2beta1IntentTrainingPhrasePart {}


/// Represents the result of querying a Knowledge base.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1KnowledgeAnswers {
    /// A list of answers from Knowledge Connector.
    
    pub answers: Option<Vec<GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswer>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1KnowledgeAnswers {}


/// An answer from Knowledge Connector.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswer {
    /// The piece of text from the `source` knowledge base document that answers this conversational query.
    
    pub answer: Option<String>,
    /// The corresponding FAQ question if the answer was extracted from a FAQ Document, empty otherwise.
    #[serde(rename="faqQuestion")]
    
    pub faq_question: Option<String>,
    /// The system's confidence score that this Knowledge answer is a good match for this conversational query. The range is from 0.0 (completely uncertain) to 1.0 (completely certain). Note: The confidence score is likely to vary somewhat (possibly even for identical requests), as the underlying model is under constant improvement. It may be deprecated in the future. We recommend using `match_confidence_level` which should be generally more stable.
    #[serde(rename="matchConfidence")]
    
    pub match_confidence: Option<f32>,
    /// The system's confidence level that this knowledge answer is a good match for this conversational query. NOTE: The confidence level for a given `` pair may change without notice, as it depends on models that are constantly being improved. However, it will change less frequently than the confidence score below, and should be preferred for referencing the quality of an answer.
    #[serde(rename="matchConfidenceLevel")]
    
    pub match_confidence_level: Option<GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum>,
    /// Indicates which Knowledge Document this answer was extracted from. Format: `projects//knowledgeBases//documents/`.
    
    pub source: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswer {}


/// A knowledge base represents a collection of knowledge documents that you provide to Dialogflow. Your knowledge documents contain information that may be useful during conversations with end-users. Some Dialogflow features use knowledge bases when looking for a response to an end-user input. For more information, see the [knowledge base guide](https://cloud.google.com/dialogflow/docs/how/knowledge-bases). Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent knowledge bases create projects](ProjectAgentKnowledgeBaseCreateCall) (request|response)
/// * [agent knowledge bases get projects](ProjectAgentKnowledgeBaseGetCall) (response)
/// * [agent knowledge bases patch projects](ProjectAgentKnowledgeBasePatchCall) (request|response)
/// * [knowledge bases create projects](ProjectKnowledgeBaseCreateCall) (request|response)
/// * [knowledge bases get projects](ProjectKnowledgeBaseGetCall) (response)
/// * [knowledge bases patch projects](ProjectKnowledgeBasePatchCall) (request|response)
/// * [locations knowledge bases create projects](ProjectLocationKnowledgeBaseCreateCall) (request|response)
/// * [locations knowledge bases get projects](ProjectLocationKnowledgeBaseGetCall) (response)
/// * [locations knowledge bases patch projects](ProjectLocationKnowledgeBasePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1KnowledgeBase {
    /// Required. The display name of the knowledge base. The name must be 1024 bytes or less; otherwise, the creation request fails.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Language which represents the KnowledgeBase. When the KnowledgeBase is created/updated, this is populated for all non en-us languages. If not populated, the default language en-us applies.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// The knowledge base resource name. The name must be empty when creating a knowledge base. Format: `projects//locations//knowledgeBases/`.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1KnowledgeBase {}
impl client::ResponseResult for GoogleCloudDialogflowV2beta1KnowledgeBase {}


/// Response message for AnswerRecords.ListAnswerRecords.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [answer records list projects](ProjectAnswerRecordListCall) (response)
/// * [locations answer records list projects](ProjectLocationAnswerRecordListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ListAnswerRecordsResponse {
    /// The list of answer records.
    #[serde(rename="answerRecords")]
    
    pub answer_records: Option<Vec<GoogleCloudDialogflowV2beta1AnswerRecord>>,
    /// A token to retrieve next page of results. Or empty if there are no more results. Pass this value in the ListAnswerRecordsRequest.page_token field in the subsequent call to `ListAnswerRecords` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1ListAnswerRecordsResponse {}


/// The response message for Contexts.ListContexts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent environments users sessions contexts list projects](ProjectAgentEnvironmentUserSessionContextListCall) (response)
/// * [agent sessions contexts list projects](ProjectAgentSessionContextListCall) (response)
/// * [locations agent environments users sessions contexts list projects](ProjectLocationAgentEnvironmentUserSessionContextListCall) (response)
/// * [locations agent sessions contexts list projects](ProjectLocationAgentSessionContextListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ListContextsResponse {
    /// The list of contexts. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub contexts: Option<Vec<GoogleCloudDialogflowV2beta1Context>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1ListContextsResponse {}


/// The response message for ConversationProfiles.ListConversationProfiles.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversation profiles list projects](ProjectConversationProfileListCall) (response)
/// * [locations conversation profiles list projects](ProjectLocationConversationProfileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ListConversationProfilesResponse {
    /// The list of project conversation profiles. There is a maximum number of items returned based on the page_size field in the request.
    #[serde(rename="conversationProfiles")]
    
    pub conversation_profiles: Option<Vec<GoogleCloudDialogflowV2beta1ConversationProfile>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1ListConversationProfilesResponse {}


/// The response message for Conversations.ListConversations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations list projects](ProjectConversationListCall) (response)
/// * [locations conversations list projects](ProjectLocationConversationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ListConversationsResponse {
    /// The list of conversations. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub conversations: Option<Vec<GoogleCloudDialogflowV2beta1Conversation>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1ListConversationsResponse {}


/// Response message for Documents.ListDocuments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent knowledge bases documents list projects](ProjectAgentKnowledgeBaseDocumentListCall) (response)
/// * [knowledge bases documents list projects](ProjectKnowledgeBaseDocumentListCall) (response)
/// * [locations knowledge bases documents list projects](ProjectLocationKnowledgeBaseDocumentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ListDocumentsResponse {
    /// The list of documents.
    
    pub documents: Option<Vec<GoogleCloudDialogflowV2beta1Document>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1ListDocumentsResponse {}


/// The response message for EntityTypes.ListEntityTypes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent entity types list projects](ProjectAgentEntityTypeListCall) (response)
/// * [locations agent entity types list projects](ProjectLocationAgentEntityTypeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ListEntityTypesResponse {
    /// The list of agent entity types. There will be a maximum number of items returned based on the page_size field in the request.
    #[serde(rename="entityTypes")]
    
    pub entity_types: Option<Vec<GoogleCloudDialogflowV2beta1EntityType>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1ListEntityTypesResponse {}


/// The response message for Environments.ListEnvironments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent environments list projects](ProjectAgentEnvironmentListCall) (response)
/// * [locations agent environments list projects](ProjectLocationAgentEnvironmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ListEnvironmentsResponse {
    /// The list of agent environments. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub environments: Option<Vec<GoogleCloudDialogflowV2beta1Environment>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1ListEnvironmentsResponse {}


/// The response message for Intents.ListIntents.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent environments intents list projects](ProjectAgentEnvironmentIntentListCall) (response)
/// * [agent intents list projects](ProjectAgentIntentListCall) (response)
/// * [locations agent environments intents list projects](ProjectLocationAgentEnvironmentIntentListCall) (response)
/// * [locations agent intents list projects](ProjectLocationAgentIntentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ListIntentsResponse {
    /// The list of agent intents. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub intents: Option<Vec<GoogleCloudDialogflowV2beta1Intent>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1ListIntentsResponse {}


/// Response message for KnowledgeBases.ListKnowledgeBases.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent knowledge bases list projects](ProjectAgentKnowledgeBaseListCall) (response)
/// * [knowledge bases list projects](ProjectKnowledgeBaseListCall) (response)
/// * [locations knowledge bases list projects](ProjectLocationKnowledgeBaseListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ListKnowledgeBasesResponse {
    /// The list of knowledge bases.
    #[serde(rename="knowledgeBases")]
    
    pub knowledge_bases: Option<Vec<GoogleCloudDialogflowV2beta1KnowledgeBase>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1ListKnowledgeBasesResponse {}


/// The response message for Conversations.ListMessages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations messages list projects](ProjectConversationMessageListCall) (response)
/// * [locations conversations messages list projects](ProjectLocationConversationMessageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ListMessagesResponse {
    /// Required. The list of messages. There will be a maximum number of items returned based on the page_size field in the request. `messages` is sorted by `create_time` in descending order.
    
    pub messages: Option<Vec<GoogleCloudDialogflowV2beta1Message>>,
    /// Optional. Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1ListMessagesResponse {}


/// The response message for Participants.ListParticipants.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations participants list projects](ProjectConversationParticipantListCall) (response)
/// * [locations conversations participants list projects](ProjectLocationConversationParticipantListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ListParticipantsResponse {
    /// Token to retrieve the next page of results or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of participants. There is a maximum number of items returned based on the page_size field in the request.
    
    pub participants: Option<Vec<GoogleCloudDialogflowV2beta1Participant>>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1ListParticipantsResponse {}


/// The response message for SessionEntityTypes.ListSessionEntityTypes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent environments users sessions entity types list projects](ProjectAgentEnvironmentUserSessionEntityTypeListCall) (response)
/// * [agent sessions entity types list projects](ProjectAgentSessionEntityTypeListCall) (response)
/// * [locations agent environments users sessions entity types list projects](ProjectLocationAgentEnvironmentUserSessionEntityTypeListCall) (response)
/// * [locations agent sessions entity types list projects](ProjectLocationAgentSessionEntityTypeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ListSessionEntityTypesResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of session entity types. There will be a maximum number of items returned based on the page_size field in the request.
    #[serde(rename="sessionEntityTypes")]
    
    pub session_entity_types: Option<Vec<GoogleCloudDialogflowV2beta1SessionEntityType>>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1ListSessionEntityTypesResponse {}


/// The response message for Participants.ListSuggestions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations participants suggestions list projects](ProjectConversationParticipantSuggestionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ListSuggestionsResponse {
    /// Optional. Token to retrieve the next page of results or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Required. The list of suggestions. There will be a maximum number of items returned based on the page_size field in the request. `suggestions` is sorted by `create_time` in descending order.
    
    pub suggestions: Option<Vec<GoogleCloudDialogflowV2beta1Suggestion>>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1ListSuggestionsResponse {}


/// The response message for Versions.ListVersions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent versions list projects](ProjectAgentVersionListCall) (response)
/// * [locations agent versions list projects](ProjectLocationAgentVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ListVersionsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of agent versions. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub versions: Option<Vec<GoogleCloudDialogflowV2beta1Version>>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1ListVersionsResponse {}


/// Defines logging behavior for conversation lifecycle events.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1LoggingConfig {
    /// Whether to log conversation events like CONVERSATION_STARTED to Stackdriver in the conversation project as JSON format ConversationEvent protos.
    #[serde(rename="enableStackdriverLogging")]
    
    pub enable_stackdriver_logging: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2beta1LoggingConfig {}


/// Represents a message posted into a conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1Message {
    /// Required. The message content.
    
    pub content: Option<String>,
    /// Output only. The time when the message was created in Contact Center AI.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The message language. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US".
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Output only. The annotation for the message.
    #[serde(rename="messageAnnotation")]
    
    pub message_annotation: Option<GoogleCloudDialogflowV2beta1MessageAnnotation>,
    /// Optional. The unique identifier of the message. Format: `projects//locations//conversations//messages/`.
    
    pub name: Option<String>,
    /// Output only. The participant that sends this message.
    
    pub participant: Option<String>,
    /// Output only. The role of the participant.
    #[serde(rename="participantRole")]
    
    pub participant_role: Option<GoogleCloudDialogflowV2beta1MessageParticipantRoleEnum>,
    /// Optional. The time when the message was sent.
    #[serde(rename="sendTime")]
    
    pub send_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The sentiment analysis result for the message.
    #[serde(rename="sentimentAnalysis")]
    
    pub sentiment_analysis: Option<GoogleCloudDialogflowV2beta1SentimentAnalysisResult>,
}

impl client::Part for GoogleCloudDialogflowV2beta1Message {}


/// Represents the result of annotation for the message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1MessageAnnotation {
    /// Required. Indicates whether the text message contains entities.
    #[serde(rename="containEntities")]
    
    pub contain_entities: Option<bool>,
    /// Optional. The collection of annotated message parts ordered by their position in the message. You can recover the annotated message by concatenating [AnnotatedMessagePart.text].
    
    pub parts: Option<Vec<GoogleCloudDialogflowV2beta1AnnotatedMessagePart>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1MessageAnnotation {}


/// Defines notification behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1NotificationConfig {
    /// Format of message.
    #[serde(rename="messageFormat")]
    
    pub message_format: Option<GoogleCloudDialogflowV2beta1NotificationConfigMessageFormatEnum>,
    /// Name of the Pub/Sub topic to publish conversation events like CONVERSATION_STARTED as serialized ConversationEvent protos. For telephony integration to receive notification, make sure either this topic is in the same project as the conversation or you grant `service-@gcp-sa-dialogflow.iam.gserviceaccount.com` the `Dialogflow Service Agent` role in the topic project. For chat integration to receive notification, make sure API caller has been granted the `Dialogflow Service Agent` role for the topic. Format: `projects//locations//topics/`.
    
    pub topic: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1NotificationConfig {}


/// Represents the natural language speech audio to be played to the end user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1OutputAudio {
    /// Required. The natural language speech audio.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub audio: Option<Vec<u8>>,
    /// Required. Instructs the speech synthesizer how to generate the speech audio.
    
    pub config: Option<GoogleCloudDialogflowV2beta1OutputAudioConfig>,
}

impl client::Part for GoogleCloudDialogflowV2beta1OutputAudio {}


/// Instructs the speech synthesizer how to generate the output audio content. If this audio config is supplied in a request, it overrides all existing text-to-speech settings applied to the agent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1OutputAudioConfig {
    /// Required. Audio encoding of the synthesized audio content.
    #[serde(rename="audioEncoding")]
    
    pub audio_encoding: Option<GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum>,
    /// The synthesis sample rate (in hertz) for this audio. If not provided, then the synthesizer will use the default sample rate based on the audio encoding. If this is different from the voice's natural sample rate, then the synthesizer will honor this request by converting to the desired sample rate (which might result in worse audio quality).
    #[serde(rename="sampleRateHertz")]
    
    pub sample_rate_hertz: Option<i32>,
    /// Configuration of how speech should be synthesized.
    #[serde(rename="synthesizeSpeechConfig")]
    
    pub synthesize_speech_config: Option<GoogleCloudDialogflowV2beta1SynthesizeSpeechConfig>,
}

impl client::Part for GoogleCloudDialogflowV2beta1OutputAudioConfig {}


/// Represents a conversation participant (human agent, virtual agent, end-user).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations participants create projects](ProjectConversationParticipantCreateCall) (request|response)
/// * [conversations participants get projects](ProjectConversationParticipantGetCall) (response)
/// * [conversations participants patch projects](ProjectConversationParticipantPatchCall) (request|response)
/// * [locations conversations participants create projects](ProjectLocationConversationParticipantCreateCall) (request|response)
/// * [locations conversations participants get projects](ProjectLocationConversationParticipantGetCall) (response)
/// * [locations conversations participants patch projects](ProjectLocationConversationParticipantPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1Participant {
    /// Optional. Key-value filters on the metadata of documents returned by article suggestion. If specified, article suggestion only returns suggested documents that match all filters in their Document.metadata. Multiple values for a metadata key should be concatenated by comma. For example, filters to match all documents that have 'US' or 'CA' in their market metadata values and 'agent' in their user metadata values will be ``` documents_metadata_filters { key: "market" value: "US,CA" } documents_metadata_filters { key: "user" value: "agent" } ```
    #[serde(rename="documentsMetadataFilters")]
    
    pub documents_metadata_filters: Option<HashMap<String, String>>,
    /// Optional. The unique identifier of this participant. Format: `projects//locations//conversations//participants/`.
    
    pub name: Option<String>,
    /// Optional. Obfuscated user id that should be associated with the created participant. You can specify a user id as follows: 1. If you set this field in CreateParticipantRequest or UpdateParticipantRequest, Dialogflow adds the obfuscated user id with the participant. 2. If you set this field in AnalyzeContent or StreamingAnalyzeContent, Dialogflow will update Participant.obfuscated_external_user_id. Dialogflow uses this user id for following purposes: 1) Billing and measurement. If user with the same obfuscated_external_user_id is created in a later conversation, dialogflow will know it's the same user. 2) Agent assist suggestion personalization. For example, Dialogflow can use it to provide personalized smart reply suggestions for this user. Note: * Please never pass raw user ids to Dialogflow. Always obfuscate your user id first. * Dialogflow only accepts a UTF-8 encoded string, e.g., a hex digest of a hash function like SHA-512. * The length of the user id must be <= 256 characters.
    #[serde(rename="obfuscatedExternalUserId")]
    
    pub obfuscated_external_user_id: Option<String>,
    /// Immutable. The role this participant plays in the conversation. This field must be set during participant creation and is then immutable.
    
    pub role: Option<GoogleCloudDialogflowV2beta1ParticipantRoleEnum>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1Participant {}
impl client::ResponseResult for GoogleCloudDialogflowV2beta1Participant {}


/// Represents the query input. It can contain either: 1. An audio config which instructs the speech recognizer how to process the speech audio. 2. A conversational query in the form of text. 3. An event that specifies which intent to trigger.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1QueryInput {
    /// Instructs the speech recognizer how to process the speech audio.
    #[serde(rename="audioConfig")]
    
    pub audio_config: Option<GoogleCloudDialogflowV2beta1InputAudioConfig>,
    /// The DTMF digits used to invoke intent and fill in parameter value.
    
    pub dtmf: Option<GoogleCloudDialogflowV2beta1TelephonyDtmfEvents>,
    /// The event to be processed.
    
    pub event: Option<GoogleCloudDialogflowV2beta1EventInput>,
    /// The natural language text to be processed.
    
    pub text: Option<GoogleCloudDialogflowV2beta1TextInput>,
}

impl client::Part for GoogleCloudDialogflowV2beta1QueryInput {}


/// Represents the parameters of the conversational query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1QueryParameters {
    /// The collection of contexts to be activated before this query is executed.
    
    pub contexts: Option<Vec<GoogleCloudDialogflowV2beta1Context>>,
    /// The geo location of this conversational query.
    #[serde(rename="geoLocation")]
    
    pub geo_location: Option<GoogleTypeLatLng>,
    /// KnowledgeBases to get alternative results from. If not set, the KnowledgeBases enabled in the agent (through UI) will be used. Format: `projects//knowledgeBases/`.
    #[serde(rename="knowledgeBaseNames")]
    
    pub knowledge_base_names: Option<Vec<String>>,
    /// This field can be used to pass custom data to your webhook. Arbitrary JSON objects are supported. If supplied, the value is used to populate the `WebhookRequest.original_detect_intent_request.payload` field sent to your webhook.
    
    pub payload: Option<HashMap<String, json::Value>>,
    /// Specifies whether to delete all contexts in the current session before the new ones are activated.
    #[serde(rename="resetContexts")]
    
    pub reset_contexts: Option<bool>,
    /// Configures the type of sentiment analysis to perform. If not provided, sentiment analysis is not performed. Note: Sentiment Analysis is only currently available for Essentials Edition agents.
    #[serde(rename="sentimentAnalysisRequestConfig")]
    
    pub sentiment_analysis_request_config: Option<GoogleCloudDialogflowV2beta1SentimentAnalysisRequestConfig>,
    /// Additional session entity types to replace or extend developer entity types with. The entity synonyms apply to all languages and persist for the session of this query.
    #[serde(rename="sessionEntityTypes")]
    
    pub session_entity_types: Option<Vec<GoogleCloudDialogflowV2beta1SessionEntityType>>,
    /// For mega agent query, directly specify which sub agents to query. If any specified sub agent is not linked to the mega agent, an error will be returned. If empty, Dialogflow will decide which sub agents to query. If specified for a non-mega-agent query, will be silently ignored.
    #[serde(rename="subAgents")]
    
    pub sub_agents: Option<Vec<GoogleCloudDialogflowV2beta1SubAgent>>,
    /// The time zone of this conversational query from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. If not provided, the time zone specified in agent settings is used.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// This field can be used to pass HTTP headers for a webhook call. These headers will be sent to webhook along with the headers that have been configured through Dialogflow web console. The headers defined within this field will overwrite the headers configured through Dialogflow console if there is a conflict. Header names are case-insensitive. Google's specified headers are not allowed. Including: "Host", "Content-Length", "Connection", "From", "User-Agent", "Accept-Encoding", "If-Modified-Since", "If-None-Match", "X-Forwarded-For", etc.
    #[serde(rename="webhookHeaders")]
    
    pub webhook_headers: Option<HashMap<String, String>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1QueryParameters {}


/// Represents the result of conversational query or event processing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1QueryResult {
    /// The action name from the matched intent.
    
    pub action: Option<String>,
    /// This field is set to: - `false` if the matched intent has required parameters and not all of the required parameter values have been collected. - `true` if all required parameter values have been collected, or if the matched intent doesn't contain any required parameters.
    #[serde(rename="allRequiredParamsPresent")]
    
    pub all_required_params_present: Option<bool>,
    /// Indicates whether the conversational query triggers a cancellation for slot filling. For more information, see the [cancel slot filling documentation](https://cloud.google.com/dialogflow/es/docs/intents-actions-parameters#cancel).
    #[serde(rename="cancelsSlotFilling")]
    
    pub cancels_slot_filling: Option<bool>,
    /// Free-form diagnostic information for the associated detect intent request. The fields of this data can change without notice, so you should not write code that depends on its structure. The data may contain: - webhook call latency - webhook errors
    #[serde(rename="diagnosticInfo")]
    
    pub diagnostic_info: Option<HashMap<String, json::Value>>,
    /// The collection of rich messages to present to the user.
    #[serde(rename="fulfillmentMessages")]
    
    pub fulfillment_messages: Option<Vec<GoogleCloudDialogflowV2beta1IntentMessage>>,
    /// The text to be pronounced to the user or shown on the screen. Note: This is a legacy field, `fulfillment_messages` should be preferred.
    #[serde(rename="fulfillmentText")]
    
    pub fulfillment_text: Option<String>,
    /// The intent that matched the conversational query. Some, not all fields are filled in this message, including but not limited to: `name`, `display_name`, `end_interaction` and `is_fallback`.
    
    pub intent: Option<GoogleCloudDialogflowV2beta1Intent>,
    /// The intent detection confidence. Values range from 0.0 (completely uncertain) to 1.0 (completely certain). This value is for informational purpose only and is only used to help match the best intent within the classification threshold. This value may change for the same end-user expression at any time due to a model retraining or change in implementation. If there are `multiple knowledge_answers` messages, this value is set to the greatest `knowledgeAnswers.match_confidence` value in the list.
    #[serde(rename="intentDetectionConfidence")]
    
    pub intent_detection_confidence: Option<f32>,
    /// The result from Knowledge Connector (if any), ordered by decreasing `KnowledgeAnswers.match_confidence`.
    #[serde(rename="knowledgeAnswers")]
    
    pub knowledge_answers: Option<GoogleCloudDialogflowV2beta1KnowledgeAnswers>,
    /// The language that was triggered during intent detection. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// The collection of output contexts. If applicable, `output_contexts.parameters` contains entries with name `.original` containing the original parameter values before the query.
    #[serde(rename="outputContexts")]
    
    pub output_contexts: Option<Vec<GoogleCloudDialogflowV2beta1Context>>,
    /// The collection of extracted parameters. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value
    
    pub parameters: Option<HashMap<String, json::Value>>,
    /// The original conversational query text: - If natural language text was provided as input, `query_text` contains a copy of the input. - If natural language speech audio was provided as input, `query_text` contains the speech recognition result. If speech recognizer produced multiple alternatives, a particular one is picked. - If automatic spell correction is enabled, `query_text` will contain the corrected user input.
    #[serde(rename="queryText")]
    
    pub query_text: Option<String>,
    /// The sentiment analysis result, which depends on the `sentiment_analysis_request_config` specified in the request.
    #[serde(rename="sentimentAnalysisResult")]
    
    pub sentiment_analysis_result: Option<GoogleCloudDialogflowV2beta1SentimentAnalysisResult>,
    /// The Speech recognition confidence between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. The default of 0.0 is a sentinel value indicating that confidence was not set. This field is not guaranteed to be accurate or set. In particular this field isn't set for StreamingDetectIntent since the streaming endpoint has separate confidence estimates per portion of the audio in StreamingRecognitionResult.
    #[serde(rename="speechRecognitionConfidence")]
    
    pub speech_recognition_confidence: Option<f32>,
    /// If the query was fulfilled by a webhook call, this field is set to the value of the `payload` field returned in the webhook response.
    #[serde(rename="webhookPayload")]
    
    pub webhook_payload: Option<HashMap<String, json::Value>>,
    /// If the query was fulfilled by a webhook call, this field is set to the value of the `source` field returned in the webhook response.
    #[serde(rename="webhookSource")]
    
    pub webhook_source: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1QueryResult {}


/// Request message for Documents.ReloadDocument.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent knowledge bases documents reload projects](ProjectAgentKnowledgeBaseDocumentReloadCall) (request)
/// * [knowledge bases documents reload projects](ProjectKnowledgeBaseDocumentReloadCall) (request)
/// * [locations knowledge bases documents reload projects](ProjectLocationKnowledgeBaseDocumentReloadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ReloadDocumentRequest {
    /// The path for a Cloud Storage source file for reloading document content. If not provided, the Document's existing source will be reloaded.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GoogleCloudDialogflowV2beta1GcsSource>,
    /// Whether to import custom metadata from Google Cloud Storage. Only valid when the document source is Google Cloud Storage URI.
    #[serde(rename="importGcsCustomMetadata")]
    
    pub import_gcs_custom_metadata: Option<bool>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1ReloadDocumentRequest {}


/// Response messages from an automated agent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ResponseMessage {
    /// A signal that indicates the interaction with the Dialogflow agent has ended.
    #[serde(rename="endInteraction")]
    
    pub end_interaction: Option<GoogleCloudDialogflowV2beta1ResponseMessageEndInteraction>,
    /// Hands off conversation to a live agent.
    #[serde(rename="liveAgentHandoff")]
    
    pub live_agent_handoff: Option<GoogleCloudDialogflowV2beta1ResponseMessageLiveAgentHandoff>,
    /// An audio response message composed of both the synthesized Dialogflow agent responses and the audios hosted in places known to the client.
    #[serde(rename="mixedAudio")]
    
    pub mixed_audio: Option<GoogleCloudDialogflowV2beta1ResponseMessageMixedAudio>,
    /// Returns a response containing a custom, platform-specific payload.
    
    pub payload: Option<HashMap<String, json::Value>>,
    /// A signal that the client should transfer the phone call connected to this agent to a third-party endpoint.
    #[serde(rename="telephonyTransferCall")]
    
    pub telephony_transfer_call: Option<GoogleCloudDialogflowV2beta1ResponseMessageTelephonyTransferCall>,
    /// Returns a text response.
    
    pub text: Option<GoogleCloudDialogflowV2beta1ResponseMessageText>,
}

impl client::Part for GoogleCloudDialogflowV2beta1ResponseMessage {}


/// Indicates that interaction with the Dialogflow agent has ended.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ResponseMessageEndInteraction { _never_set: Option<bool> }

impl client::Part for GoogleCloudDialogflowV2beta1ResponseMessageEndInteraction {}


/// Indicates that the conversation should be handed off to a human agent. Dialogflow only uses this to determine which conversations were handed off to a human agent for measurement purposes. What else to do with this signal is up to you and your handoff procedures. You may set this, for example: * In the entry fulfillment of a CX Page if entering the page indicates something went extremely wrong in the conversation. * In a webhook response when you determine that the customer issue can only be handled by a human.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ResponseMessageLiveAgentHandoff {
    /// Custom metadata for your handoff procedure. Dialogflow doesn't impose any structure on this.
    
    pub metadata: Option<HashMap<String, json::Value>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1ResponseMessageLiveAgentHandoff {}


/// Represents an audio message that is composed of both segments synthesized from the Dialogflow agent prompts and ones hosted externally at the specified URIs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ResponseMessageMixedAudio {
    /// Segments this audio response is composed of.
    
    pub segments: Option<Vec<GoogleCloudDialogflowV2beta1ResponseMessageMixedAudioSegment>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1ResponseMessageMixedAudio {}


/// Represents one segment of audio.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ResponseMessageMixedAudioSegment {
    /// Whether the playback of this segment can be interrupted by the end user's speech and the client should then start the next Dialogflow request.
    #[serde(rename="allowPlaybackInterruption")]
    
    pub allow_playback_interruption: Option<bool>,
    /// Raw audio synthesized from the Dialogflow agent's response using the output config specified in the request.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub audio: Option<Vec<u8>>,
    /// Client-specific URI that points to an audio clip accessible to the client.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1ResponseMessageMixedAudioSegment {}


/// Represents the signal that telles the client to transfer the phone call connected to the agent to a third-party endpoint.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ResponseMessageTelephonyTransferCall {
    /// Transfer the call to a phone number in [E.164 format](https://en.wikipedia.org/wiki/E.164).
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// Transfer the call to a SIP endpoint.
    #[serde(rename="sipUri")]
    
    pub sip_uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1ResponseMessageTelephonyTransferCall {}


/// The text response message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ResponseMessageText {
    /// A collection of text responses.
    
    pub text: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1ResponseMessageText {}


/// The request message for Agents.RestoreAgent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent restore projects](ProjectAgentRestoreCall) (request)
/// * [locations agent restore projects](ProjectLocationAgentRestoreCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1RestoreAgentRequest {
    /// Zip compressed raw byte content for agent.
    #[serde(rename="agentContent")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub agent_content: Option<Vec<u8>>,
    /// The URI to a Google Cloud Storage file containing the agent to restore. Note: The URI must start with "gs://". Dialogflow performs a read operation for the Cloud Storage object on the caller's behalf, so your request authentication must have read permissions for the object. For more information, see [Dialogflow access control](https://cloud.google.com/dialogflow/cx/docs/concept/access-control#storage).
    #[serde(rename="agentUri")]
    
    pub agent_uri: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1RestoreAgentRequest {}


/// The response message for Agents.SearchAgents.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent search projects](ProjectAgentSearchCall) (response)
/// * [locations agent search projects](ProjectLocationAgentSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SearchAgentsResponse {
    /// The list of agents. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub agents: Option<Vec<GoogleCloudDialogflowV2beta1Agent>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1SearchAgentsResponse {}


/// The sentiment, such as positive/negative feeling or association, for a unit of analysis, such as the query text. See: https://cloud.google.com/natural-language/docs/basics#interpreting_sentiment_analysis_values for how to interpret the result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1Sentiment {
    /// A non-negative number in the [0, +inf) range, which represents the absolute magnitude of sentiment, regardless of score (positive or negative).
    
    pub magnitude: Option<f32>,
    /// Sentiment score between -1.0 (negative sentiment) and 1.0 (positive sentiment).
    
    pub score: Option<f32>,
}

impl client::Part for GoogleCloudDialogflowV2beta1Sentiment {}


/// Configures the types of sentiment analysis to perform.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SentimentAnalysisRequestConfig {
    /// Instructs the service to perform sentiment analysis on `query_text`. If not provided, sentiment analysis is not performed on `query_text`.
    #[serde(rename="analyzeQueryTextSentiment")]
    
    pub analyze_query_text_sentiment: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2beta1SentimentAnalysisRequestConfig {}


/// The result of sentiment analysis. Sentiment analysis inspects user input and identifies the prevailing subjective opinion, especially to determine a user's attitude as positive, negative, or neutral. For Participants.DetectIntent, it needs to be configured in DetectIntentRequest.query_params. For Participants.StreamingDetectIntent, it needs to be configured in StreamingDetectIntentRequest.query_params. And for Participants.AnalyzeContent and Participants.StreamingAnalyzeContent, it needs to be configured in ConversationProfile.human_agent_assistant_config
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SentimentAnalysisResult {
    /// The sentiment analysis result for `query_text`.
    #[serde(rename="queryTextSentiment")]
    
    pub query_text_sentiment: Option<GoogleCloudDialogflowV2beta1Sentiment>,
}

impl client::Part for GoogleCloudDialogflowV2beta1SentimentAnalysisResult {}


/// A session represents a conversation between a Dialogflow agent and an end-user. You can create special entities, called session entities, during a session. Session entities can extend or replace custom entity types and only exist during the session that they were created for. All session data, including session entities, is stored by Dialogflow for 20 minutes. For more information, see the [session entity guide](https://cloud.google.com/dialogflow/docs/entities-session).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent environments users sessions entity types create projects](ProjectAgentEnvironmentUserSessionEntityTypeCreateCall) (request|response)
/// * [agent environments users sessions entity types get projects](ProjectAgentEnvironmentUserSessionEntityTypeGetCall) (response)
/// * [agent environments users sessions entity types patch projects](ProjectAgentEnvironmentUserSessionEntityTypePatchCall) (request|response)
/// * [agent sessions entity types create projects](ProjectAgentSessionEntityTypeCreateCall) (request|response)
/// * [agent sessions entity types get projects](ProjectAgentSessionEntityTypeGetCall) (response)
/// * [agent sessions entity types patch projects](ProjectAgentSessionEntityTypePatchCall) (request|response)
/// * [locations agent environments users sessions entity types create projects](ProjectLocationAgentEnvironmentUserSessionEntityTypeCreateCall) (request|response)
/// * [locations agent environments users sessions entity types get projects](ProjectLocationAgentEnvironmentUserSessionEntityTypeGetCall) (response)
/// * [locations agent environments users sessions entity types patch projects](ProjectLocationAgentEnvironmentUserSessionEntityTypePatchCall) (request|response)
/// * [locations agent sessions entity types create projects](ProjectLocationAgentSessionEntityTypeCreateCall) (request|response)
/// * [locations agent sessions entity types get projects](ProjectLocationAgentSessionEntityTypeGetCall) (response)
/// * [locations agent sessions entity types patch projects](ProjectLocationAgentSessionEntityTypePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SessionEntityType {
    /// Required. The collection of entities associated with this session entity type.
    
    pub entities: Option<Vec<GoogleCloudDialogflowV2beta1EntityTypeEntity>>,
    /// Required. Indicates whether the additional data should override or supplement the custom entity type definition.
    #[serde(rename="entityOverrideMode")]
    
    pub entity_override_mode: Option<GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum>,
    /// Required. The unique identifier of this session entity type. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. `` must be the display name of an existing entity type in the same agent that will be overridden or supplemented.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1SessionEntityType {}
impl client::ResponseResult for GoogleCloudDialogflowV2beta1SessionEntityType {}


/// The request message for ConversationProfiles.SetSuggestionFeature.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversation profiles set suggestion feature config projects](ProjectConversationProfileSetSuggestionFeatureConfigCall) (request)
/// * [locations conversation profiles set suggestion feature config projects](ProjectLocationConversationProfileSetSuggestionFeatureConfigCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequest {
    /// Required. The participant role to add or update the suggestion feature config. Only HUMAN_AGENT or END_USER can be used.
    #[serde(rename="participantRole")]
    
    pub participant_role: Option<GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequestParticipantRoleEnum>,
    /// Required. The suggestion feature config to add or update.
    #[serde(rename="suggestionFeatureConfig")]
    
    pub suggestion_feature_config: Option<GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionFeatureConfig>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequest {}


/// Represents a smart reply answer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SmartReplyAnswer {
    /// The name of answer record, in the format of "projects//locations//answerRecords/"
    #[serde(rename="answerRecord")]
    
    pub answer_record: Option<String>,
    /// Smart reply confidence. The system's confidence score that this reply is a good match for this conversation, as a value from 0.0 (completely uncertain) to 1.0 (completely certain).
    
    pub confidence: Option<f32>,
    /// The content of the reply.
    
    pub reply: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1SmartReplyAnswer {}


/// Hints for the speech recognizer to help with recognition in a specific conversation state.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SpeechContext {
    /// Optional. Boost for this context compared to other contexts: * If the boost is positive, Dialogflow will increase the probability that the phrases in this context are recognized over similar sounding phrases. * If the boost is unspecified or non-positive, Dialogflow will not apply any boost. Dialogflow recommends that you use boosts in the range (0, 20] and that you find a value that fits your use case with binary search.
    
    pub boost: Option<f32>,
    /// Optional. A list of strings containing words and phrases that the speech recognizer should recognize with higher likelihood. This list can be used to: * improve accuracy for words and phrases you expect the user to say, e.g. typical commands for your Dialogflow agent * add additional words to the speech recognizer vocabulary * ... See the [Cloud Speech documentation](https://cloud.google.com/speech-to-text/quotas) for usage limits.
    
    pub phrases: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1SpeechContext {}


/// Configures speech transcription for ConversationProfile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SpeechToTextConfig {
    /// Which Speech model to select. Select the model best suited to your domain to get best results. If a model is not explicitly specified, then a default model is used. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics#select-model) for more details.
    
    pub model: Option<String>,
    /// The speech model used in speech to text. `SPEECH_MODEL_VARIANT_UNSPECIFIED`, `USE_BEST_AVAILABLE` will be treated as `USE_ENHANCED`. It can be overridden in AnalyzeContentRequest and StreamingAnalyzeContentRequest request. If enhanced model variant is specified and an enhanced version of the specified model for the language does not exist, then it would emit an error.
    #[serde(rename="speechModelVariant")]
    
    pub speech_model_variant: Option<GoogleCloudDialogflowV2beta1SpeechToTextConfigSpeechModelVariantEnum>,
}

impl client::Part for GoogleCloudDialogflowV2beta1SpeechToTextConfig {}


/// Contains basic configuration for a sub-agent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SubAgent {
    /// Optional. The unique identifier (`environment name` in dialogflow console) of this sub-agent environment. Assumes draft environment if `environment` is not set.
    
    pub environment: Option<String>,
    /// Required. The project of this agent. Format: `projects/` or `projects//locations/`.
    
    pub project: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1SubAgent {}


/// The request message for Participants.SuggestArticles.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations participants suggestions suggest articles projects](ProjectConversationParticipantSuggestionSuggestArticleCall) (request)
/// * [locations conversations participants suggestions suggest articles projects](ProjectLocationConversationParticipantSuggestionSuggestArticleCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SuggestArticlesRequest {
    /// Optional. Parameters for a human assist query.
    #[serde(rename="assistQueryParams")]
    
    pub assist_query_params: Option<GoogleCloudDialogflowV2beta1AssistQueryParameters>,
    /// Optional. Max number of messages prior to and including latest_message to use as context when compiling the suggestion. By default 20 and at most 50.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// Optional. The name of the latest conversation message to compile suggestion for. If empty, it will be the latest message of the conversation. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1SuggestArticlesRequest {}


/// The response message for Participants.SuggestArticles.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations participants suggestions suggest articles projects](ProjectConversationParticipantSuggestionSuggestArticleCall) (response)
/// * [locations conversations participants suggestions suggest articles projects](ProjectLocationConversationParticipantSuggestionSuggestArticleCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SuggestArticlesResponse {
    /// Output only. Articles ordered by score in descending order.
    #[serde(rename="articleAnswers")]
    
    pub article_answers: Option<Vec<GoogleCloudDialogflowV2beta1ArticleAnswer>>,
    /// Number of messages prior to and including latest_message to compile the suggestion. It may be smaller than the SuggestArticlesResponse.context_size field in the request if there aren't that many messages in the conversation.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// The name of the latest conversation message used to compile suggestion for. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1SuggestArticlesResponse {}


/// The request message for Conversations.SuggestConversationSummary.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations suggestions suggest conversation summary projects](ProjectConversationSuggestionSuggestConversationSummaryCall) (request)
/// * [locations conversations suggestions suggest conversation summary projects](ProjectLocationConversationSuggestionSuggestConversationSummaryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SuggestConversationSummaryRequest {
    /// Max number of messages prior to and including [latest_message] to use as context when compiling the suggestion. By default 500 and at most 1000.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// The name of the latest conversation message used as context for compiling suggestion. If empty, the latest message of the conversation will be used. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1SuggestConversationSummaryRequest {}


/// The response message for Conversations.SuggestConversationSummary.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations suggestions suggest conversation summary projects](ProjectConversationSuggestionSuggestConversationSummaryCall) (response)
/// * [locations conversations suggestions suggest conversation summary projects](ProjectLocationConversationSuggestionSuggestConversationSummaryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SuggestConversationSummaryResponse {
    /// Number of messages prior to and including last_conversation_message used to compile the suggestion. It may be smaller than the SuggestSummaryRequest.context_size field in the request if there weren't that many messages in the conversation.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// The name of the latest conversation message used as context for compiling suggestion. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
    /// Generated summary.
    
    pub summary: Option<GoogleCloudDialogflowV2beta1SuggestConversationSummaryResponseSummary>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1SuggestConversationSummaryResponse {}


/// Generated summary for a conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SuggestConversationSummaryResponseSummary {
    /// The name of the answer record. Format: "projects//answerRecords/"
    #[serde(rename="answerRecord")]
    
    pub answer_record: Option<String>,
    /// The summary content that is concatenated into one string.
    
    pub text: Option<String>,
    /// The summary content that is divided into sections. The key is the section's name and the value is the section's content. There is no specific format for the key or value.
    #[serde(rename="textSections")]
    
    pub text_sections: Option<HashMap<String, String>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1SuggestConversationSummaryResponseSummary {}


/// The request message for Participants.SuggestFaqAnswers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations participants suggestions suggest faq answers projects](ProjectConversationParticipantSuggestionSuggestFaqAnswerCall) (request)
/// * [locations conversations participants suggestions suggest faq answers projects](ProjectLocationConversationParticipantSuggestionSuggestFaqAnswerCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SuggestFaqAnswersRequest {
    /// Optional. Parameters for a human assist query.
    #[serde(rename="assistQueryParams")]
    
    pub assist_query_params: Option<GoogleCloudDialogflowV2beta1AssistQueryParameters>,
    /// Optional. Max number of messages prior to and including [latest_message] to use as context when compiling the suggestion. By default 20 and at most 50.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// Optional. The name of the latest conversation message to compile suggestion for. If empty, it will be the latest message of the conversation. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1SuggestFaqAnswersRequest {}


/// The request message for Participants.SuggestFaqAnswers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations participants suggestions suggest faq answers projects](ProjectConversationParticipantSuggestionSuggestFaqAnswerCall) (response)
/// * [locations conversations participants suggestions suggest faq answers projects](ProjectLocationConversationParticipantSuggestionSuggestFaqAnswerCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SuggestFaqAnswersResponse {
    /// Number of messages prior to and including latest_message to compile the suggestion. It may be smaller than the SuggestFaqAnswersRequest.context_size field in the request if there aren't that many messages in the conversation.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// Output only. Answers extracted from FAQ documents.
    #[serde(rename="faqAnswers")]
    
    pub faq_answers: Option<Vec<GoogleCloudDialogflowV2beta1FaqAnswer>>,
    /// The name of the latest conversation message used to compile suggestion for. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1SuggestFaqAnswersResponse {}


/// The request message for Participants.SuggestSmartReplies.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations participants suggestions suggest smart replies projects](ProjectConversationParticipantSuggestionSuggestSmartReplyCall) (request)
/// * [locations conversations participants suggestions suggest smart replies projects](ProjectLocationConversationParticipantSuggestionSuggestSmartReplyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SuggestSmartRepliesRequest {
    /// Optional. Max number of messages prior to and including [latest_message] to use as context when compiling the suggestion. By default 20 and at most 50.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// The current natural language text segment to compile suggestion for. This provides a way for user to get follow up smart reply suggestion after a smart reply selection, without sending a text message.
    #[serde(rename="currentTextInput")]
    
    pub current_text_input: Option<GoogleCloudDialogflowV2beta1TextInput>,
    /// The name of the latest conversation message to compile suggestion for. If empty, it will be the latest message of the conversation. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1SuggestSmartRepliesRequest {}


/// The response message for Participants.SuggestSmartReplies.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversations participants suggestions suggest smart replies projects](ProjectConversationParticipantSuggestionSuggestSmartReplyCall) (response)
/// * [locations conversations participants suggestions suggest smart replies projects](ProjectLocationConversationParticipantSuggestionSuggestSmartReplyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SuggestSmartRepliesResponse {
    /// Number of messages prior to and including latest_message to compile the suggestion. It may be smaller than the SuggestSmartRepliesRequest.context_size field in the request if there aren't that many messages in the conversation.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// The name of the latest conversation message used to compile suggestion for. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
    /// Output only. Multiple reply options provided by smart reply service. The order is based on the rank of the model prediction. The maximum number of the returned replies is set in SmartReplyConfig.
    #[serde(rename="smartReplyAnswers")]
    
    pub smart_reply_answers: Option<Vec<GoogleCloudDialogflowV2beta1SmartReplyAnswer>>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1SuggestSmartRepliesResponse {}


/// Represents a suggestion for a human agent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1Suggestion {
    /// Output only. Articles ordered by score in descending order.
    
    pub articles: Option<Vec<GoogleCloudDialogflowV2beta1SuggestionArticle>>,
    /// Output only. The time the suggestion was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Answers extracted from FAQ documents.
    #[serde(rename="faqAnswers")]
    
    pub faq_answers: Option<Vec<GoogleCloudDialogflowV2beta1SuggestionFaqAnswer>>,
    /// Output only. Latest message used as context to compile this suggestion. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
    /// Output only. The name of this suggestion. Format: `projects//locations//conversations//participants/*/suggestions/`.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1Suggestion {}


/// Represents suggested article.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SuggestionArticle {
    /// Output only. The name of answer record, in the format of "projects//locations//answerRecords/"
    #[serde(rename="answerRecord")]
    
    pub answer_record: Option<String>,
    /// Output only. A map that contains metadata about the answer and the document from which it originates.
    
    pub metadata: Option<HashMap<String, String>>,
    /// Output only. Article snippets.
    
    pub snippets: Option<Vec<String>>,
    /// Output only. The article title.
    
    pub title: Option<String>,
    /// Output only. The article URI.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1SuggestionArticle {}


/// Represents suggested answer from "frequently asked questions".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SuggestionFaqAnswer {
    /// Output only. The piece of text from the `source` knowledge base document.
    
    pub answer: Option<String>,
    /// Output only. The name of answer record, in the format of "projects//locations//answerRecords/"
    #[serde(rename="answerRecord")]
    
    pub answer_record: Option<String>,
    /// The system's confidence score that this Knowledge answer is a good match for this conversational query, range from 0.0 (completely uncertain) to 1.0 (completely certain).
    
    pub confidence: Option<f32>,
    /// Output only. A map that contains metadata about the answer and the document from which it originates.
    
    pub metadata: Option<HashMap<String, String>>,
    /// Output only. The corresponding FAQ question.
    
    pub question: Option<String>,
    /// Output only. Indicates which Knowledge Document this answer was extracted from. Format: `projects//locations//agent/knowledgeBases//documents/`.
    
    pub source: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1SuggestionFaqAnswer {}


/// The type of Human Agent Assistant API suggestion to perform, and the maximum number of results to return for that type. Multiple `Feature` objects can be specified in the `features` list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SuggestionFeature {
    /// Type of Human Agent Assistant API feature to request.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum>,
}

impl client::Part for GoogleCloudDialogflowV2beta1SuggestionFeature {}


/// One response of different type of suggestion response which is used in the response of Participants.AnalyzeContent and Participants.AnalyzeContent, as well as HumanAgentAssistantEvent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SuggestionResult {
    /// Error status if the request failed.
    
    pub error: Option<GoogleRpcStatus>,
    /// SuggestArticlesResponse if request is for ARTICLE_SUGGESTION.
    #[serde(rename="suggestArticlesResponse")]
    
    pub suggest_articles_response: Option<GoogleCloudDialogflowV2beta1SuggestArticlesResponse>,
    /// SuggestFaqAnswersResponse if request is for FAQ_ANSWER.
    #[serde(rename="suggestFaqAnswersResponse")]
    
    pub suggest_faq_answers_response: Option<GoogleCloudDialogflowV2beta1SuggestFaqAnswersResponse>,
    /// SuggestSmartRepliesResponse if request is for SMART_REPLY.
    #[serde(rename="suggestSmartRepliesResponse")]
    
    pub suggest_smart_replies_response: Option<GoogleCloudDialogflowV2beta1SuggestSmartRepliesResponse>,
}

impl client::Part for GoogleCloudDialogflowV2beta1SuggestionResult {}


/// Configuration of how speech should be synthesized.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1SynthesizeSpeechConfig {
    /// Optional. An identifier which selects 'audio effects' profiles that are applied on (post synthesized) text to speech. Effects are applied on top of each other in the order they are given.
    #[serde(rename="effectsProfileId")]
    
    pub effects_profile_id: Option<Vec<String>>,
    /// Optional. Speaking pitch, in the range [-20.0, 20.0]. 20 means increase 20 semitones from the original pitch. -20 means decrease 20 semitones from the original pitch.
    
    pub pitch: Option<f64>,
    /// Optional. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is the normal native speed supported by the specific voice. 2.0 is twice as fast, and 0.5 is half as fast. If unset(0.0), defaults to the native 1.0 speed. Any other values < 0.25 or > 4.0 will return an error.
    #[serde(rename="speakingRate")]
    
    pub speaking_rate: Option<f64>,
    /// Optional. The desired voice of the synthesized audio.
    
    pub voice: Option<GoogleCloudDialogflowV2beta1VoiceSelectionParams>,
    /// Optional. Volume gain (in dB) of the normal native volume supported by the specific voice, in the range [-96.0, 16.0]. If unset, or set to a value of 0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB) will play at approximately half the amplitude of the normal native signal amplitude. A value of +6.0 (dB) will play at approximately twice the amplitude of the normal native signal amplitude. We strongly recommend not to exceed +10 (dB) as there's usually no effective increase in loudness for any value greater than that.
    #[serde(rename="volumeGainDb")]
    
    pub volume_gain_db: Option<f64>,
}

impl client::Part for GoogleCloudDialogflowV2beta1SynthesizeSpeechConfig {}


/// A wrapper of repeated TelephonyDtmf digits.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1TelephonyDtmfEvents {
    /// A sequence of TelephonyDtmf digits.
    #[serde(rename="dtmfEvents")]
    
    pub dtmf_events: Option<Vec<GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1TelephonyDtmfEvents {}


/// Represents the natural language text to be processed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1TextInput {
    /// Required. The language of this conversational query. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Required. The UTF-8 encoded natural language text to be processed. Text length must not exceed 256 characters for virtual agent interactions.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2beta1TextInput {}


/// Instructs the speech synthesizer on how to generate the output audio content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1TextToSpeechSettings {
    /// Optional. Indicates whether text to speech is enabled. Even when this field is false, other settings in this proto are still retained.
    #[serde(rename="enableTextToSpeech")]
    
    pub enable_text_to_speech: Option<bool>,
    /// Required. Audio encoding of the synthesized audio content.
    #[serde(rename="outputAudioEncoding")]
    
    pub output_audio_encoding: Option<GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum>,
    /// Optional. The synthesis sample rate (in hertz) for this audio. If not provided, then the synthesizer will use the default sample rate based on the audio encoding. If this is different from the voice's natural sample rate, then the synthesizer will honor this request by converting to the desired sample rate (which might result in worse audio quality).
    #[serde(rename="sampleRateHertz")]
    
    pub sample_rate_hertz: Option<i32>,
    /// Optional. Configuration of how speech should be synthesized, mapping from language (https://cloud.google.com/dialogflow/docs/reference/language) to SynthesizeSpeechConfig.
    #[serde(rename="synthesizeSpeechConfigs")]
    
    pub synthesize_speech_configs: Option<HashMap<String, GoogleCloudDialogflowV2beta1SynthesizeSpeechConfig>>,
}

impl client::Part for GoogleCloudDialogflowV2beta1TextToSpeechSettings {}


/// The request message for Agents.TrainAgent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent train projects](ProjectAgentTrainCall) (request)
/// * [locations agent train projects](ProjectLocationAgentTrainCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1TrainAgentRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDialogflowV2beta1TrainAgentRequest {}


/// Represents a single validation error.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ValidationError {
    /// The names of the entries that the error is associated with. Format: - "projects//agent", if the error is associated with the entire agent. - "projects//agent/intents/", if the error is associated with certain intents. - "projects//agent/intents//trainingPhrases/", if the error is associated with certain intent training phrases. - "projects//agent/intents//parameters/", if the error is associated with certain intent parameters. - "projects//agent/entities/", if the error is associated with certain entities.
    
    pub entries: Option<Vec<String>>,
    /// The detailed error message.
    #[serde(rename="errorMessage")]
    
    pub error_message: Option<String>,
    /// The severity of the error.
    
    pub severity: Option<GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum>,
}

impl client::Part for GoogleCloudDialogflowV2beta1ValidationError {}


/// Represents the output of agent validation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent get validation result projects](ProjectAgentGetValidationResultCall) (response)
/// * [locations agent get validation result projects](ProjectLocationAgentGetValidationResultCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1ValidationResult {
    /// Contains all validation errors.
    #[serde(rename="validationErrors")]
    
    pub validation_errors: Option<Vec<GoogleCloudDialogflowV2beta1ValidationError>>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2beta1ValidationResult {}


/// You can create multiple versions of your agent and publish them to separate environments. When you edit an agent, you are editing the draft agent. At any point, you can save the draft agent as an agent version, which is an immutable snapshot of your agent. When you save the draft agent, it is published to the default environment. When you create agent versions, you can publish them to custom environments. You can create a variety of custom environments for: - testing - development - production - etc. For more information, see the [versions and environments guide](https://cloud.google.com/dialogflow/docs/agents-versions).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [agent versions create projects](ProjectAgentVersionCreateCall) (request|response)
/// * [agent versions get projects](ProjectAgentVersionGetCall) (response)
/// * [agent versions patch projects](ProjectAgentVersionPatchCall) (request|response)
/// * [locations agent versions create projects](ProjectLocationAgentVersionCreateCall) (request|response)
/// * [locations agent versions get projects](ProjectLocationAgentVersionGetCall) (response)
/// * [locations agent versions patch projects](ProjectLocationAgentVersionPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1Version {
    /// Output only. The creation time of this version. This field is read-only, i.e., it cannot be set by create and update methods.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The developer-provided description of this version.
    
    pub description: Option<String>,
    /// Output only. The unique identifier of this agent version. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/`
    
    pub name: Option<String>,
    /// Output only. The status of this version. This field is read-only and cannot be set by create and update methods.
    
    pub status: Option<GoogleCloudDialogflowV2beta1VersionStatusEnum>,
    /// Output only. The sequential number of this version. This field is read-only which means it cannot be set by create and update methods.
    #[serde(rename="versionNumber")]
    
    pub version_number: Option<i32>,
}

impl client::RequestValue for GoogleCloudDialogflowV2beta1Version {}
impl client::ResponseResult for GoogleCloudDialogflowV2beta1Version {}


/// Description of which voice to use for speech synthesis.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2beta1VoiceSelectionParams {
    /// Optional. The name of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and ssml_gender. For the list of available voices, please refer to [Supported voices and languages](https://cloud.google.com/text-to-speech/docs/voices).
    
    pub name: Option<String>,
    /// Optional. The preferred gender of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and name. Note that this is only a preference, not requirement. If a voice of the appropriate gender is not available, the synthesizer should substitute a voice with a different gender rather than failing the request.
    #[serde(rename="ssmlGender")]
    
    pub ssml_gender: Option<GoogleCloudDialogflowV2beta1VoiceSelectionParamSsmlGenderEnum>,
}

impl client::Part for GoogleCloudDialogflowV2beta1VoiceSelectionParams {}


/// The response message for Locations.ListLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations list projects](ProjectLocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudLocationListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    
    pub locations: Option<Vec<GoogleCloudLocationLocation>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudLocationListLocationsResponse {}


/// A resource that represents Google Cloud Platform location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get projects](ProjectLocationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudLocationLocation {
    /// The friendly name for this location, typically a nearby city name. For example, "Tokyo".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}
    
    pub labels: Option<HashMap<String, String>>,
    /// The canonical id for this location. For example: `"us-east1"`.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given location.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"`
    
    pub name: Option<String>,
}

impl client::ResponseResult for GoogleCloudLocationLocation {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations list projects](ProjectLocationOperationListCall) (response)
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
/// * [agent entity types entities batch create projects](ProjectAgentEntityTypeEntityBatchCreateCall) (response)
/// * [agent entity types entities batch delete projects](ProjectAgentEntityTypeEntityBatchDeleteCall) (response)
/// * [agent entity types entities batch update projects](ProjectAgentEntityTypeEntityBatchUpdateCall) (response)
/// * [agent entity types batch delete projects](ProjectAgentEntityTypeBatchDeleteCall) (response)
/// * [agent entity types batch update projects](ProjectAgentEntityTypeBatchUpdateCall) (response)
/// * [agent intents batch delete projects](ProjectAgentIntentBatchDeleteCall) (response)
/// * [agent intents batch update projects](ProjectAgentIntentBatchUpdateCall) (response)
/// * [agent knowledge bases documents create projects](ProjectAgentKnowledgeBaseDocumentCreateCall) (response)
/// * [agent knowledge bases documents delete projects](ProjectAgentKnowledgeBaseDocumentDeleteCall) (response)
/// * [agent knowledge bases documents patch projects](ProjectAgentKnowledgeBaseDocumentPatchCall) (response)
/// * [agent knowledge bases documents reload projects](ProjectAgentKnowledgeBaseDocumentReloadCall) (response)
/// * [agent export projects](ProjectAgentExportCall) (response)
/// * [agent import projects](ProjectAgentImportCall) (response)
/// * [agent restore projects](ProjectAgentRestoreCall) (response)
/// * [agent train projects](ProjectAgentTrainCall) (response)
/// * [conversation profiles clear suggestion feature config projects](ProjectConversationProfileClearSuggestionFeatureConfigCall) (response)
/// * [conversation profiles set suggestion feature config projects](ProjectConversationProfileSetSuggestionFeatureConfigCall) (response)
/// * [knowledge bases documents create projects](ProjectKnowledgeBaseDocumentCreateCall) (response)
/// * [knowledge bases documents delete projects](ProjectKnowledgeBaseDocumentDeleteCall) (response)
/// * [knowledge bases documents import projects](ProjectKnowledgeBaseDocumentImportCall) (response)
/// * [knowledge bases documents patch projects](ProjectKnowledgeBaseDocumentPatchCall) (response)
/// * [knowledge bases documents reload projects](ProjectKnowledgeBaseDocumentReloadCall) (response)
/// * [locations agent entity types entities batch create projects](ProjectLocationAgentEntityTypeEntityBatchCreateCall) (response)
/// * [locations agent entity types entities batch delete projects](ProjectLocationAgentEntityTypeEntityBatchDeleteCall) (response)
/// * [locations agent entity types entities batch update projects](ProjectLocationAgentEntityTypeEntityBatchUpdateCall) (response)
/// * [locations agent entity types batch delete projects](ProjectLocationAgentEntityTypeBatchDeleteCall) (response)
/// * [locations agent entity types batch update projects](ProjectLocationAgentEntityTypeBatchUpdateCall) (response)
/// * [locations agent intents batch delete projects](ProjectLocationAgentIntentBatchDeleteCall) (response)
/// * [locations agent intents batch update projects](ProjectLocationAgentIntentBatchUpdateCall) (response)
/// * [locations agent export projects](ProjectLocationAgentExportCall) (response)
/// * [locations agent import projects](ProjectLocationAgentImportCall) (response)
/// * [locations agent restore projects](ProjectLocationAgentRestoreCall) (response)
/// * [locations agent train projects](ProjectLocationAgentTrainCall) (response)
/// * [locations conversation profiles clear suggestion feature config projects](ProjectLocationConversationProfileClearSuggestionFeatureConfigCall) (response)
/// * [locations conversation profiles set suggestion feature config projects](ProjectLocationConversationProfileSetSuggestionFeatureConfigCall) (response)
/// * [locations knowledge bases documents create projects](ProjectLocationKnowledgeBaseDocumentCreateCall) (response)
/// * [locations knowledge bases documents delete projects](ProjectLocationKnowledgeBaseDocumentDeleteCall) (response)
/// * [locations knowledge bases documents import projects](ProjectLocationKnowledgeBaseDocumentImportCall) (response)
/// * [locations knowledge bases documents patch projects](ProjectLocationKnowledgeBaseDocumentPatchCall) (response)
/// * [locations knowledge bases documents reload projects](ProjectLocationKnowledgeBaseDocumentReloadCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
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
/// * [agent entity types delete projects](ProjectAgentEntityTypeDeleteCall) (response)
/// * [agent environments users sessions contexts delete projects](ProjectAgentEnvironmentUserSessionContextDeleteCall) (response)
/// * [agent environments users sessions entity types delete projects](ProjectAgentEnvironmentUserSessionEntityTypeDeleteCall) (response)
/// * [agent environments users sessions delete contexts projects](ProjectAgentEnvironmentUserSessionDeleteContextCall) (response)
/// * [agent environments delete projects](ProjectAgentEnvironmentDeleteCall) (response)
/// * [agent intents delete projects](ProjectAgentIntentDeleteCall) (response)
/// * [agent knowledge bases delete projects](ProjectAgentKnowledgeBaseDeleteCall) (response)
/// * [agent sessions contexts delete projects](ProjectAgentSessionContextDeleteCall) (response)
/// * [agent sessions entity types delete projects](ProjectAgentSessionEntityTypeDeleteCall) (response)
/// * [agent sessions delete contexts projects](ProjectAgentSessionDeleteContextCall) (response)
/// * [agent versions delete projects](ProjectAgentVersionDeleteCall) (response)
/// * [conversation profiles delete projects](ProjectConversationProfileDeleteCall) (response)
/// * [knowledge bases delete projects](ProjectKnowledgeBaseDeleteCall) (response)
/// * [locations agent entity types delete projects](ProjectLocationAgentEntityTypeDeleteCall) (response)
/// * [locations agent environments users sessions contexts delete projects](ProjectLocationAgentEnvironmentUserSessionContextDeleteCall) (response)
/// * [locations agent environments users sessions entity types delete projects](ProjectLocationAgentEnvironmentUserSessionEntityTypeDeleteCall) (response)
/// * [locations agent environments users sessions delete contexts projects](ProjectLocationAgentEnvironmentUserSessionDeleteContextCall) (response)
/// * [locations agent environments delete projects](ProjectLocationAgentEnvironmentDeleteCall) (response)
/// * [locations agent intents delete projects](ProjectLocationAgentIntentDeleteCall) (response)
/// * [locations agent sessions contexts delete projects](ProjectLocationAgentSessionContextDeleteCall) (response)
/// * [locations agent sessions entity types delete projects](ProjectLocationAgentSessionEntityTypeDeleteCall) (response)
/// * [locations agent sessions delete contexts projects](ProjectLocationAgentSessionDeleteContextCall) (response)
/// * [locations agent versions delete projects](ProjectLocationAgentVersionDeleteCall) (response)
/// * [locations conversation profiles delete projects](ProjectLocationConversationProfileDeleteCall) (response)
/// * [locations knowledge bases delete projects](ProjectLocationKnowledgeBaseDeleteCall) (response)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations delete agent projects](ProjectLocationDeleteAgentCall) (response)
/// * [operations cancel projects](ProjectOperationCancelCall) (response)
/// * [delete agent projects](ProjectDeleteAgentCall) (response)
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


/// An object that represents a latitude/longitude pair. This is expressed as a pair of doubles to represent degrees latitude and degrees longitude. Unless specified otherwise, this object must conform to the WGS84 standard. Values must be within normalized ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeLatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    
    pub latitude: Option<f64>,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    
    pub longitude: Option<f64>,
}

impl client::Part for GoogleTypeLatLng {}


