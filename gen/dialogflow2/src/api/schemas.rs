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
pub struct GoogleCloudDialogflowV2Agent {
    /// Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version.
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
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
    
    pub match_mode: Option<String>,
    /// Required. The project of this agent. Format: `projects/`.
    
    pub parent: Option<String>,
    /// Optional. The list of all languages supported by this agent (except for the `default_language_code`).
    #[serde(rename="supportedLanguageCodes")]
    
    pub supported_language_codes: Option<Vec<String>>,
    /// Optional. The agent tier. If not specified, TIER_STANDARD is assumed.
    
    pub tier: Option<String>,
    /// Required. The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2Agent {}
impl client::ResponseResult for GoogleCloudDialogflowV2Agent {}


/// Detail feedback of Agent Assist result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2AgentAssistantFeedback {
    /// Optional. Whether or not the suggested answer is relevant. For example: * Query: "Can I change my mailing address?" * Suggested document says: "Items must be returned/exchanged within 60 days of the purchase date." * answer_relevance: AnswerRelevance.IRRELEVANT
    #[serde(rename="answerRelevance")]
    
    pub answer_relevance: Option<String>,
    /// Optional. Whether or not the information in the document is correct. For example: * Query: "Can I return the package in 2 days once received?" * Suggested document says: "Items must be returned/exchanged within 60 days of the purchase date." * Ground truth: "No return or exchange is allowed." * [document_correctness]: INCORRECT
    #[serde(rename="documentCorrectness")]
    
    pub document_correctness: Option<String>,
    /// Optional. Whether or not the suggested document is efficient. For example, if the document is poorly written, hard to understand, hard to use or too long to find useful information, document_efficiency is DocumentEfficiency.INEFFICIENT.
    #[serde(rename="documentEfficiency")]
    
    pub document_efficiency: Option<String>,
    /// Optional. Feedback for conversation summarization.
    #[serde(rename="summarizationFeedback")]
    
    pub summarization_feedback: Option<GoogleCloudDialogflowV2AgentAssistantFeedbackSummarizationFeedback>,
}

impl client::Part for GoogleCloudDialogflowV2AgentAssistantFeedback {}


/// Feedback for conversation summarization.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2AgentAssistantFeedbackSummarizationFeedback {
    /// Timestamp when composing of the summary starts.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Timestamp when the summary was submitted.
    #[serde(rename="submitTime")]
    
    pub submit_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Text of actual submitted summary.
    #[serde(rename="summaryText")]
    
    pub summary_text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2AgentAssistantFeedbackSummarizationFeedback {}


/// Represents a record of a human agent assist answer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2AgentAssistantRecord {
    /// Output only. The article suggestion answer.
    #[serde(rename="articleSuggestionAnswer")]
    
    pub article_suggestion_answer: Option<GoogleCloudDialogflowV2ArticleAnswer>,
    /// Output only. The FAQ answer.
    #[serde(rename="faqAnswer")]
    
    pub faq_answer: Option<GoogleCloudDialogflowV2FaqAnswer>,
}

impl client::Part for GoogleCloudDialogflowV2AgentAssistantRecord {}


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
pub struct GoogleCloudDialogflowV2AnalyzeContentRequest {
    /// Parameters for a human assist query.
    #[serde(rename="assistQueryParams")]
    
    pub assist_query_params: Option<GoogleCloudDialogflowV2AssistQueryParameters>,
    /// Additional parameters to be put into Dialogflow CX session parameters. To remove a parameter from the session, clients should explicitly set the parameter value to null. Note: this field should only be used if you are connecting to a Dialogflow CX agent.
    #[serde(rename="cxParameters")]
    
    pub cx_parameters: Option<HashMap<String, json::Value>>,
    /// An input event to send to Dialogflow.
    #[serde(rename="eventInput")]
    
    pub event_input: Option<GoogleCloudDialogflowV2EventInput>,
    /// Parameters for a Dialogflow virtual-agent query.
    #[serde(rename="queryParams")]
    
    pub query_params: Option<GoogleCloudDialogflowV2QueryParameters>,
    /// Speech synthesis configuration. The speech synthesis settings for a virtual agent that may be configured for the associated conversation profile are not used when calling AnalyzeContent. If this configuration is not supplied, speech synthesis is disabled.
    #[serde(rename="replyAudioConfig")]
    
    pub reply_audio_config: Option<GoogleCloudDialogflowV2OutputAudioConfig>,
    /// A unique identifier for this request. Restricted to 36 ASCII characters. A random UUID is recommended. This request is only idempotent if a `request_id` is provided.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// The natural language text to be processed.
    #[serde(rename="textInput")]
    
    pub text_input: Option<GoogleCloudDialogflowV2TextInput>,
}

impl client::RequestValue for GoogleCloudDialogflowV2AnalyzeContentRequest {}


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
pub struct GoogleCloudDialogflowV2AnalyzeContentResponse {
    /// Only set if a Dialogflow automated agent has responded. Note that: AutomatedAgentReply.detect_intent_response.output_audio and AutomatedAgentReply.detect_intent_response.output_audio_config are always empty, use reply_audio instead.
    #[serde(rename="automatedAgentReply")]
    
    pub automated_agent_reply: Option<GoogleCloudDialogflowV2AutomatedAgentReply>,
    /// Indicates the parameters of DTMF.
    #[serde(rename="dtmfParameters")]
    
    pub dtmf_parameters: Option<GoogleCloudDialogflowV2DtmfParameters>,
    /// The suggestions for end user. The order is the same as HumanAgentAssistantConfig.SuggestionConfig.feature_configs of HumanAgentAssistantConfig.end_user_suggestion_config. Same as human_agent_suggestion_results, any failure of Agent Assist features will not lead to the overall failure of an AnalyzeContent API call. Instead, the features will fail silently with the error field set in the corresponding SuggestionResult.
    #[serde(rename="endUserSuggestionResults")]
    
    pub end_user_suggestion_results: Option<Vec<GoogleCloudDialogflowV2SuggestionResult>>,
    /// The suggestions for most recent human agent. The order is the same as HumanAgentAssistantConfig.SuggestionConfig.feature_configs of HumanAgentAssistantConfig.human_agent_suggestion_config. Note that any failure of Agent Assist features will not lead to the overall failure of an AnalyzeContent API call. Instead, the features will fail silently with the error field set in the corresponding SuggestionResult.
    #[serde(rename="humanAgentSuggestionResults")]
    
    pub human_agent_suggestion_results: Option<Vec<GoogleCloudDialogflowV2SuggestionResult>>,
    /// Message analyzed by CCAI.
    
    pub message: Option<GoogleCloudDialogflowV2Message>,
    /// The audio data bytes encoded as specified in the request. This field is set if: - `reply_audio_config` was specified in the request, or - The automated agent responded with audio to play to the user. In such case, `reply_audio.config` contains settings used to synthesize the speech. In some scenarios, multiple output audio fields may be present in the response structure. In these cases, only the top-most-level audio output has content.
    #[serde(rename="replyAudio")]
    
    pub reply_audio: Option<GoogleCloudDialogflowV2OutputAudio>,
    /// The output text content. This field is set if the automated agent responded with text to show to the user.
    #[serde(rename="replyText")]
    
    pub reply_text: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2AnalyzeContentResponse {}


/// Represents a part of a message possibly annotated with an entity. The part can be an entity or purely a part of the message between two entities or message start/end.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2AnnotatedMessagePart {
    /// The [Dialogflow system entity type](https://cloud.google.com/dialogflow/docs/reference/system-entities) of this message part. If this is empty, Dialogflow could not annotate the phrase part with a system entity.
    #[serde(rename="entityType")]
    
    pub entity_type: Option<String>,
    /// The [Dialogflow system entity formatted value ](https://cloud.google.com/dialogflow/docs/reference/system-entities) of this message part. For example for a system entity of type `@sys.unit-currency`, this may contain: { "amount": 5, "currency": "USD" } 
    #[serde(rename="formattedValue")]
    
    pub formatted_value: Option<json::Value>,
    /// A part of a message possibly annotated with an entity.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2AnnotatedMessagePart {}


/// Represents feedback the customer has about the quality & correctness of a certain answer in a conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2AnswerFeedback {
    /// Detail feedback of agent assist suggestions.
    #[serde(rename="agentAssistantDetailFeedback")]
    
    pub agent_assistant_detail_feedback: Option<GoogleCloudDialogflowV2AgentAssistantFeedback>,
    /// Time when the answer/item was clicked.
    #[serde(rename="clickTime")]
    
    pub click_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Indicates whether the answer/item was clicked by the human agent or not. Default to false.
    
    pub clicked: Option<bool>,
    /// The correctness level of the specific answer.
    #[serde(rename="correctnessLevel")]
    
    pub correctness_level: Option<String>,
    /// Time when the answer/item was displayed.
    #[serde(rename="displayTime")]
    
    pub display_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Indicates whether the answer/item was displayed to the human agent in the agent desktop UI. Default to false.
    
    pub displayed: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2AnswerFeedback {}


/// Answer records are records to manage answer history and feedbacks for Dialogflow. Currently, answer record includes: - human agent assistant article suggestion - human agent assistant faq article It doesn’t include: - `DetectIntent` intent matching - `DetectIntent` knowledge Answer records are not related to the conversation history in the Dialogflow Console. A Record is generated even when the end-user disables conversation history in the console. Records are created when there’s a human agent assistant suggestion generated. A typical workflow for customers provide feedback to an answer is: 1. For human agent assistant, customers get suggestion via ListSuggestions API. Together with the answers, AnswerRecord.name are returned to the customers. 2. The customer uses the AnswerRecord.name to call the UpdateAnswerRecord method to send feedback about a specific answer that they believe is wrong.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [answer records patch projects](ProjectAnswerRecordPatchCall) (request|response)
/// * [locations answer records patch projects](ProjectLocationAnswerRecordPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2AnswerRecord {
    /// Output only. The record for human agent assistant.
    #[serde(rename="agentAssistantRecord")]
    
    pub agent_assistant_record: Option<GoogleCloudDialogflowV2AgentAssistantRecord>,
    /// Required. The AnswerFeedback for this record. You can set this with AnswerRecords.UpdateAnswerRecord in order to give us feedback about this answer.
    #[serde(rename="answerFeedback")]
    
    pub answer_feedback: Option<GoogleCloudDialogflowV2AnswerFeedback>,
    /// The unique identifier of this answer record. Format: `projects//locations//answerRecords/`.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2AnswerRecord {}
impl client::ResponseResult for GoogleCloudDialogflowV2AnswerRecord {}


/// Represents article answer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2ArticleAnswer {
    /// The name of answer record, in the format of "projects//locations//answerRecords/"
    #[serde(rename="answerRecord")]
    
    pub answer_record: Option<String>,
    /// Article match confidence. The system's confidence score that this article is a good match for this conversation, as a value from 0.0 (completely uncertain) to 1.0 (completely certain).
    
    pub confidence: Option<f32>,
    /// A map that contains metadata about the answer and the document from which it originates.
    
    pub metadata: Option<HashMap<String, String>>,
    /// Article snippets.
    
    pub snippets: Option<Vec<String>>,
    /// The article title.
    
    pub title: Option<String>,
    /// The article URI.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2ArticleAnswer {}


/// Metadata for article suggestion models.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2ArticleSuggestionModelMetadata {
    /// Optional. Type of the article suggestion model. If not provided, model_type is used.
    #[serde(rename="trainingModelType")]
    
    pub training_model_type: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2ArticleSuggestionModelMetadata {}


/// Represents the parameters of human assist query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2AssistQueryParameters {
    /// Key-value filters on the metadata of documents returned by article suggestion. If specified, article suggestion only returns suggested documents that match all filters in their Document.metadata. Multiple values for a metadata key should be concatenated by comma. For example, filters to match all documents that have 'US' or 'CA' in their market metadata values and 'agent' in their user metadata values will be ``` documents_metadata_filters { key: "market" value: "US,CA" } documents_metadata_filters { key: "user" value: "agent" } ```
    #[serde(rename="documentsMetadataFilters")]
    
    pub documents_metadata_filters: Option<HashMap<String, String>>,
}

impl client::Part for GoogleCloudDialogflowV2AssistQueryParameters {}


/// Defines the Automated Agent to connect to a conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2AutomatedAgentConfig {
    /// Required. ID of the Dialogflow agent environment to use. This project needs to either be the same project as the conversation or you need to grant `service-@gcp-sa-dialogflow.iam.gserviceaccount.com` the `Dialogflow API Service Agent` role in this project. - For ES agents, use format: `projects//locations//agent/environments/`. If environment is not specified, the default `draft` environment is used. Refer to [DetectIntentRequest](https://cloud.google.com/dialogflow/docs/reference/rpc/google.cloud.dialogflow.v2#google.cloud.dialogflow.v2.DetectIntentRequest) for more details. - For CX agents, use format `projects//locations//agents//environments/`. If environment is not specified, the default `draft` environment is used.
    
    pub agent: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2AutomatedAgentConfig {}


/// Represents a response from an automated agent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2AutomatedAgentReply {
    /// Indicates whether the partial automated agent reply is interruptible when a later reply message arrives. e.g. if the agent specified some music as partial response, it can be cancelled.
    #[serde(rename="allowCancellation")]
    
    pub allow_cancellation: Option<bool>,
    /// AutomatedAgentReply type.
    #[serde(rename="automatedAgentReplyType")]
    
    pub automated_agent_reply_type: Option<String>,
    /// The unique identifier of the current Dialogflow CX conversation page. Format: `projects//locations//agents//flows//pages/`.
    #[serde(rename="cxCurrentPage")]
    
    pub cx_current_page: Option<String>,
    /// Response of the Dialogflow Sessions.DetectIntent call.
    #[serde(rename="detectIntentResponse")]
    
    pub detect_intent_response: Option<GoogleCloudDialogflowV2DetectIntentResponse>,
}

impl client::Part for GoogleCloudDialogflowV2AutomatedAgentReply {}


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
pub struct GoogleCloudDialogflowV2BatchCreateEntitiesRequest {
    /// Required. The entities to create.
    
    pub entities: Option<Vec<GoogleCloudDialogflowV2EntityTypeEntity>>,
    /// Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2BatchCreateEntitiesRequest {}


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
pub struct GoogleCloudDialogflowV2BatchDeleteEntitiesRequest {
    /// Required. The reference `values` of the entities to delete. Note that these are not fully-qualified names, i.e. they don't start with `projects/`.
    #[serde(rename="entityValues")]
    
    pub entity_values: Option<Vec<String>>,
    /// Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2BatchDeleteEntitiesRequest {}


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
pub struct GoogleCloudDialogflowV2BatchDeleteEntityTypesRequest {
    /// Required. The names entity types to delete. All names must point to the same agent as `parent`.
    #[serde(rename="entityTypeNames")]
    
    pub entity_type_names: Option<Vec<String>>,
}

impl client::RequestValue for GoogleCloudDialogflowV2BatchDeleteEntityTypesRequest {}


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
pub struct GoogleCloudDialogflowV2BatchDeleteIntentsRequest {
    /// Required. The collection of intents to delete. Only intent `name` must be filled in.
    
    pub intents: Option<Vec<GoogleCloudDialogflowV2Intent>>,
}

impl client::RequestValue for GoogleCloudDialogflowV2BatchDeleteIntentsRequest {}


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
pub struct GoogleCloudDialogflowV2BatchUpdateEntitiesRequest {
    /// Required. The entities to update or create.
    
    pub entities: Option<Vec<GoogleCloudDialogflowV2EntityTypeEntity>>,
    /// Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Optional. The mask to control which fields get updated.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GoogleCloudDialogflowV2BatchUpdateEntitiesRequest {}


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
pub struct GoogleCloudDialogflowV2BatchUpdateEntityTypesRequest {
    /// The collection of entity types to update or create.
    #[serde(rename="entityTypeBatchInline")]
    
    pub entity_type_batch_inline: Option<GoogleCloudDialogflowV2EntityTypeBatch>,
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

impl client::RequestValue for GoogleCloudDialogflowV2BatchUpdateEntityTypesRequest {}


/// There is no detailed description.
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
pub struct GoogleCloudDialogflowV2BatchUpdateIntentsRequest {
    /// The collection of intents to update or create.
    #[serde(rename="intentBatchInline")]
    
    pub intent_batch_inline: Option<GoogleCloudDialogflowV2IntentBatch>,
    /// The URI to a Google Cloud Storage file containing intents to update or create. The file format can either be a serialized proto (of IntentBatch type) or JSON object. Note: The URI must start with "gs://".
    #[serde(rename="intentBatchUri")]
    
    pub intent_batch_uri: Option<String>,
    /// Optional. The resource view to apply to the returned intent.
    #[serde(rename="intentView")]
    
    pub intent_view: Option<String>,
    /// Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Optional. The mask to control which fields get updated.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GoogleCloudDialogflowV2BatchUpdateIntentsRequest {}


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
pub struct GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequest {
    /// Required. The participant role to remove the suggestion feature config. Only HUMAN_AGENT or END_USER can be used.
    #[serde(rename="participantRole")]
    
    pub participant_role: Option<String>,
    /// Required. The type of the suggestion feature to remove.
    #[serde(rename="suggestionFeatureType")]
    
    pub suggestion_feature_type: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequest {}


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
pub struct GoogleCloudDialogflowV2CompleteConversationRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDialogflowV2CompleteConversationRequest {}


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
pub struct GoogleCloudDialogflowV2Context {
    /// Optional. The number of conversational query requests after which the context expires. The default is `0`. If set to `0`, the context expires immediately. Contexts expire automatically after 20 minutes if there are no matching queries.
    #[serde(rename="lifespanCount")]
    
    pub lifespan_count: Option<i32>,
    /// Required. The unique identifier of the context. Format: `projects//agent/sessions//contexts/`, or `projects//agent/environments//users//sessions//contexts/`. The `Context ID` is always converted to lowercase, may only contain characters in a-zA-Z0-9_-% and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size`
    
    pub name: Option<String>,
    /// Optional. The collection of parameters associated with this context. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value
    
    pub parameters: Option<HashMap<String, json::Value>>,
}

impl client::RequestValue for GoogleCloudDialogflowV2Context {}
impl client::ResponseResult for GoogleCloudDialogflowV2Context {}


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
pub struct GoogleCloudDialogflowV2Conversation {
    /// Required. The Conversation Profile to be used to configure this Conversation. This field cannot be updated. Format: `projects//locations//conversationProfiles/`.
    #[serde(rename="conversationProfile")]
    
    pub conversation_profile: Option<String>,
    /// The stage of a conversation. It indicates whether the virtual agent or a human agent is handling the conversation. If the conversation is created with the conversation profile that has Dialogflow config set, defaults to ConversationStage.VIRTUAL_AGENT_STAGE; Otherwise, defaults to ConversationStage.HUMAN_ASSIST_STAGE. If the conversation is created with the conversation profile that has Dialogflow config set but explicitly sets conversation_stage to ConversationStage.HUMAN_ASSIST_STAGE, it skips ConversationStage.VIRTUAL_AGENT_STAGE stage and directly goes to ConversationStage.HUMAN_ASSIST_STAGE.
    #[serde(rename="conversationStage")]
    
    pub conversation_stage: Option<String>,
    /// Output only. The time the conversation was finished.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The current state of the Conversation.
    #[serde(rename="lifecycleState")]
    
    pub lifecycle_state: Option<String>,
    /// Output only. The unique identifier of this conversation. Format: `projects//locations//conversations/`.
    
    pub name: Option<String>,
    /// Output only. It will not be empty if the conversation is to be connected over telephony.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<GoogleCloudDialogflowV2ConversationPhoneNumber>,
    /// Output only. The time the conversation was started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDialogflowV2Conversation {}
impl client::ResponseResult for GoogleCloudDialogflowV2Conversation {}


/// Represents a conversation dataset that a user imports raw data into. The data inside ConversationDataset can not be changed after ImportConversationData finishes (and calling ImportConversationData on a dataset that already has data is not allowed).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversation datasets get projects](ProjectConversationDatasetGetCall) (response)
/// * [locations conversation datasets create projects](ProjectLocationConversationDatasetCreateCall) (request)
/// * [locations conversation datasets get projects](ProjectLocationConversationDatasetGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2ConversationDataset {
    /// Output only. The number of conversations this conversation dataset contains.
    #[serde(rename="conversationCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub conversation_count: Option<i64>,
    /// Output only. Metadata set during conversation data import.
    #[serde(rename="conversationInfo")]
    
    pub conversation_info: Option<GoogleCloudDialogflowV2ConversationInfo>,
    /// Output only. Creation time of this dataset.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The description of the dataset. Maximum of 10000 bytes.
    
    pub description: Option<String>,
    /// Required. The display name of the dataset. Maximum of 64 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Input configurations set during conversation data import.
    #[serde(rename="inputConfig")]
    
    pub input_config: Option<GoogleCloudDialogflowV2InputConfig>,
    /// Output only. ConversationDataset resource name. Format: `projects//locations//conversationDatasets/`
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2ConversationDataset {}
impl client::ResponseResult for GoogleCloudDialogflowV2ConversationDataset {}


/// Represents metadata of a conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2ConversationInfo {
    /// Optional. The language code of the conversation data within this dataset. See https://cloud.google.com/apis/design/standard_fields for more information. Supports all UTF-8 languages.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2ConversationInfo {}


/// Represents a conversation model.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversation models create projects](ProjectConversationModelCreateCall) (request)
/// * [conversation models get projects](ProjectConversationModelGetCall) (response)
/// * [locations conversation models create projects](ProjectLocationConversationModelCreateCall) (request)
/// * [locations conversation models get projects](ProjectLocationConversationModelGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2ConversationModel {
    /// Metadata for article suggestion models.
    #[serde(rename="articleSuggestionModelMetadata")]
    
    pub article_suggestion_model_metadata: Option<GoogleCloudDialogflowV2ArticleSuggestionModelMetadata>,
    /// Output only. Creation time of this model.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Datasets used to create model.
    
    pub datasets: Option<Vec<GoogleCloudDialogflowV2InputDataset>>,
    /// Required. The display name of the model. At most 64 bytes long.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Language code for the conversation model. If not specified, the language is en-US. Language at ConversationModel should be set for all non en-us languages. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US".
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// ConversationModel resource name. Format: `projects//conversationModels/`
    
    pub name: Option<String>,
    /// Metadata for smart reply models.
    #[serde(rename="smartReplyModelMetadata")]
    
    pub smart_reply_model_metadata: Option<GoogleCloudDialogflowV2SmartReplyModelMetadata>,
    /// Output only. State of the model. A model can only serve prediction requests after it gets deployed.
    
    pub state: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2ConversationModel {}
impl client::ResponseResult for GoogleCloudDialogflowV2ConversationModel {}


/// Represents evaluation result of a conversation model.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversation models evaluations get projects](ProjectConversationModelEvaluationGetCall) (response)
/// * [locations conversation models evaluations get projects](ProjectLocationConversationModelEvaluationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2ConversationModelEvaluation {
    /// Output only. Creation time of this model.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The display name of the model evaluation. At most 64 bytes long.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. The configuration of the evaluation task.
    #[serde(rename="evaluationConfig")]
    
    pub evaluation_config: Option<GoogleCloudDialogflowV2EvaluationConfig>,
    /// The resource name of the evaluation. Format: `projects//conversationModels//evaluations/`
    
    pub name: Option<String>,
    /// Output only. Human eval template in csv format. It tooks real-world conversations provided through input dataset, generates example suggestions for customer to verify quality of the model. For Smart Reply, the generated csv file contains columns of Context, (Suggestions,Q1,Q2)*3, Actual reply. Context contains at most 10 latest messages in the conversation prior to the current suggestion. Q1: "Would you send it as the next message of agent?" Evaluated based on whether the suggest is appropriate to be sent by agent in current context. Q2: "Does the suggestion move the conversation closer to resolution?" Evaluated based on whether the suggestion provide solutions, or answers customer's question or collect information from customer to resolve the customer's issue. Actual reply column contains the actual agent reply sent in the context.
    #[serde(rename="rawHumanEvalTemplateCsv")]
    
    pub raw_human_eval_template_csv: Option<String>,
    /// Output only. Only available when model is for smart reply.
    #[serde(rename="smartReplyMetrics")]
    
    pub smart_reply_metrics: Option<GoogleCloudDialogflowV2SmartReplyMetrics>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ConversationModelEvaluation {}


/// Represents a phone number for telephony integration. It allows for connecting a particular conversation over telephony.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2ConversationPhoneNumber {
    /// Output only. The phone number to connect to this conversation.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2ConversationPhoneNumber {}


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
pub struct GoogleCloudDialogflowV2ConversationProfile {
    /// Configuration for an automated agent to use with this profile.
    #[serde(rename="automatedAgentConfig")]
    
    pub automated_agent_config: Option<GoogleCloudDialogflowV2AutomatedAgentConfig>,
    /// Output only. Create time of the conversation profile.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Human readable name for this profile. Max length 1024 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Configuration for agent assistance to use with this profile.
    #[serde(rename="humanAgentAssistantConfig")]
    
    pub human_agent_assistant_config: Option<GoogleCloudDialogflowV2HumanAgentAssistantConfig>,
    /// Configuration for connecting to a live agent. Currently, this feature is not general available, please contact Google to get access.
    #[serde(rename="humanAgentHandoffConfig")]
    
    pub human_agent_handoff_config: Option<GoogleCloudDialogflowV2HumanAgentHandoffConfig>,
    /// Language code for the conversation profile. If not specified, the language is en-US. Language at ConversationProfile should be set for all non en-US languages. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US".
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Configuration for logging conversation lifecycle events.
    #[serde(rename="loggingConfig")]
    
    pub logging_config: Option<GoogleCloudDialogflowV2LoggingConfig>,
    /// The unique identifier of this conversation profile. Format: `projects//locations//conversationProfiles/`.
    
    pub name: Option<String>,
    /// Configuration for publishing new message events. Event will be sent in format of ConversationEvent
    #[serde(rename="newMessageEventNotificationConfig")]
    
    pub new_message_event_notification_config: Option<GoogleCloudDialogflowV2NotificationConfig>,
    /// Configuration for publishing conversation lifecycle events.
    #[serde(rename="notificationConfig")]
    
    pub notification_config: Option<GoogleCloudDialogflowV2NotificationConfig>,
    /// Name of the CX SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`.
    #[serde(rename="securitySettings")]
    
    pub security_settings: Option<String>,
    /// Settings for speech transcription.
    #[serde(rename="sttConfig")]
    
    pub stt_config: Option<GoogleCloudDialogflowV2SpeechToTextConfig>,
    /// The time zone of this conversational profile from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. Defaults to America/New_York.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// Output only. Update time of the conversation profile.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDialogflowV2ConversationProfile {}
impl client::ResponseResult for GoogleCloudDialogflowV2ConversationProfile {}


/// The request message for ConversationModels.CreateConversationModelEvaluation
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversation models evaluations create projects](ProjectLocationConversationModelEvaluationCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2CreateConversationModelEvaluationRequest {
    /// Required. The conversation model evaluation to be created.
    #[serde(rename="conversationModelEvaluation")]
    
    pub conversation_model_evaluation: Option<GoogleCloudDialogflowV2ConversationModelEvaluation>,
}

impl client::RequestValue for GoogleCloudDialogflowV2CreateConversationModelEvaluationRequest {}


/// The request message for ConversationModels.DeployConversationModel
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversation models deploy projects](ProjectConversationModelDeployCall) (request)
/// * [locations conversation models deploy projects](ProjectLocationConversationModelDeployCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2DeployConversationModelRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDialogflowV2DeployConversationModelRequest {}


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
pub struct GoogleCloudDialogflowV2DetectIntentRequest {
    /// The natural language speech audio to be processed. This field should be populated iff `query_input` is set to an input audio config. A single request can contain up to 1 minute of speech audio data.
    #[serde(rename="inputAudio")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub input_audio: Option<Vec<u8>>,
    /// Instructs the speech synthesizer how to generate the output audio. If this field is not set and agent-level speech synthesizer is not configured, no output audio is generated.
    #[serde(rename="outputAudioConfig")]
    
    pub output_audio_config: Option<GoogleCloudDialogflowV2OutputAudioConfig>,
    /// Mask for output_audio_config indicating which settings in this request-level config should override speech synthesizer settings defined at agent-level. If unspecified or empty, output_audio_config replaces the agent-level config in its entirety.
    #[serde(rename="outputAudioConfigMask")]
    
    pub output_audio_config_mask: Option<client::FieldMask>,
    /// Required. The input specification. It can be set to: 1. an audio config which instructs the speech recognizer how to process the speech audio, 2. a conversational query in the form of text, or 3. an event that specifies which intent to trigger.
    #[serde(rename="queryInput")]
    
    pub query_input: Option<GoogleCloudDialogflowV2QueryInput>,
    /// The parameters of this query.
    #[serde(rename="queryParams")]
    
    pub query_params: Option<GoogleCloudDialogflowV2QueryParameters>,
}

impl client::RequestValue for GoogleCloudDialogflowV2DetectIntentRequest {}


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
pub struct GoogleCloudDialogflowV2DetectIntentResponse {
    /// The audio data bytes encoded as specified in the request. Note: The output audio is generated based on the values of default platform text responses found in the `query_result.fulfillment_messages` field. If multiple default text responses exist, they will be concatenated when generating audio. If no default platform text responses exist, the generated audio content will be empty. In some scenarios, multiple output audio fields may be present in the response structure. In these cases, only the top-most-level audio output has content.
    #[serde(rename="outputAudio")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub output_audio: Option<Vec<u8>>,
    /// The config used by the speech synthesizer to generate the output audio.
    #[serde(rename="outputAudioConfig")]
    
    pub output_audio_config: Option<GoogleCloudDialogflowV2OutputAudioConfig>,
    /// The selected results of the conversational query or event processing. See `alternative_query_results` for additional potential results.
    #[serde(rename="queryResult")]
    
    pub query_result: Option<GoogleCloudDialogflowV2QueryResult>,
    /// The unique identifier of the response. It can be used to locate a response in the training example set or for reporting issues.
    #[serde(rename="responseId")]
    
    pub response_id: Option<String>,
    /// Specifies the status of the webhook request.
    #[serde(rename="webhookStatus")]
    
    pub webhook_status: Option<GoogleRpcStatus>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2DetectIntentResponse {}


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
pub struct GoogleCloudDialogflowV2Document {
    /// The URI where the file content is located. For documents stored in Google Cloud Storage, these URIs must have the form `gs:///`. NOTE: External URLs must correspond to public webpages, i.e., they must be indexed by Google Search. In particular, URLs for showing documents in Google Cloud Storage (i.e. the URL in your browser) are not supported. Instead use the `gs://` format URI described above.
    #[serde(rename="contentUri")]
    
    pub content_uri: Option<String>,
    /// Required. The display name of the document. The name must be 1024 bytes or less; otherwise, the creation request fails.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. If true, we try to automatically reload the document every day (at a time picked by the system). If false or unspecified, we don't try to automatically reload the document. Currently you can only enable automatic reload for documents sourced from a public url, see `source` field for the source types. Reload status can be tracked in `latest_reload_status`. If a reload fails, we will keep the document unchanged. If a reload fails with internal errors, the system will try to reload the document on the next day. If a reload fails with non-retriable errors (e.g. PERMISION_DENIED), the system will not try to reload the document anymore. You need to manually reload the document successfully by calling `ReloadDocument` and clear the errors.
    #[serde(rename="enableAutoReload")]
    
    pub enable_auto_reload: Option<bool>,
    /// Required. The knowledge type of document content.
    #[serde(rename="knowledgeTypes")]
    
    pub knowledge_types: Option<Vec<String>>,
    /// Output only. The time and status of the latest reload. This reload may have been triggered automatically or manually and may not have succeeded.
    #[serde(rename="latestReloadStatus")]
    
    pub latest_reload_status: Option<GoogleCloudDialogflowV2DocumentReloadStatus>,
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
    
    pub state: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2Document {}
impl client::ResponseResult for GoogleCloudDialogflowV2Document {}


/// The status of a reload attempt.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2DocumentReloadStatus {
    /// The status of a reload attempt or the initial load.
    
    pub status: Option<GoogleRpcStatus>,
    /// The time of a reload attempt. This reload may have been triggered automatically or manually and may not have succeeded.
    
    pub time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudDialogflowV2DocumentReloadStatus {}


/// The message in the response that indicates the parameters of DTMF.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2DtmfParameters {
    /// Indicates whether DTMF input can be handled in the next request.
    #[serde(rename="acceptsDtmfInput")]
    
    pub accepts_dtmf_input: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2DtmfParameters {}


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
pub struct GoogleCloudDialogflowV2EntityType {
    /// Optional. Indicates whether the entity type can be automatically expanded.
    #[serde(rename="autoExpansionMode")]
    
    pub auto_expansion_mode: Option<String>,
    /// Required. The name of the entity type.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. Enables fuzzy entity extraction during classification.
    #[serde(rename="enableFuzzyExtraction")]
    
    pub enable_fuzzy_extraction: Option<bool>,
    /// Optional. The collection of entity entries associated with the entity type.
    
    pub entities: Option<Vec<GoogleCloudDialogflowV2EntityTypeEntity>>,
    /// Required. Indicates the kind of entity type.
    
    pub kind: Option<String>,
    /// The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType and EntityTypes.BatchUpdateEntityTypes methods. Format: `projects//agent/entityTypes/`.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2EntityType {}
impl client::ResponseResult for GoogleCloudDialogflowV2EntityType {}


/// This message is a wrapper around a collection of entity types.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2EntityTypeBatch {
    /// A collection of entity types.
    #[serde(rename="entityTypes")]
    
    pub entity_types: Option<Vec<GoogleCloudDialogflowV2EntityType>>,
}

impl client::Part for GoogleCloudDialogflowV2EntityTypeBatch {}


/// An **entity entry** for an associated entity type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2EntityTypeEntity {
    /// Required. A collection of value synonyms. For example, if the entity type is *vegetable*, and `value` is *scallions*, a synonym could be *green onions*. For `KIND_LIST` entity types: * This collection must contain exactly one synonym equal to `value`.
    
    pub synonyms: Option<Vec<String>>,
    /// Required. The primary value associated with this entity entry. For example, if the entity type is *vegetable*, the value could be *scallions*. For `KIND_MAP` entity types: * A reference value to be used in place of synonyms. For `KIND_LIST` entity types: * A string that can contain references to other entity types (with or without aliases).
    
    pub value: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2EntityTypeEntity {}


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
pub struct GoogleCloudDialogflowV2Environment {
    /// Optional. The agent version loaded into this environment. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/`
    #[serde(rename="agentVersion")]
    
    pub agent_version: Option<String>,
    /// Optional. The developer-provided description for this environment. The maximum length is 500 characters. If exceeded, the request is rejected.
    
    pub description: Option<String>,
    /// Optional. The fulfillment settings to use for this environment.
    
    pub fulfillment: Option<GoogleCloudDialogflowV2Fulfillment>,
    /// Output only. The unique identifier of this agent environment. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/` The environment ID for the default environment is `-`.
    
    pub name: Option<String>,
    /// Output only. The state of this environment. This field is read-only, i.e., it cannot be set by create and update methods.
    
    pub state: Option<String>,
    /// Optional. Text to speech settings for this environment.
    #[serde(rename="textToSpeechSettings")]
    
    pub text_to_speech_settings: Option<GoogleCloudDialogflowV2TextToSpeechSettings>,
    /// Output only. The last update time of this environment. This field is read-only, i.e., it cannot be set by create and update methods.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudDialogflowV2Environment {}
impl client::ResponseResult for GoogleCloudDialogflowV2Environment {}


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
pub struct GoogleCloudDialogflowV2EnvironmentHistory {
    /// Output only. The list of agent environments. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub entries: Option<Vec<GoogleCloudDialogflowV2EnvironmentHistoryEntry>>,
    /// Output only. Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Output only. The name of the environment this history is for. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/` The environment ID for the default environment is `-`.
    
    pub parent: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2EnvironmentHistory {}


/// Represents an environment history entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2EnvironmentHistoryEntry {
    /// The agent version loaded into this environment history entry.
    #[serde(rename="agentVersion")]
    
    pub agent_version: Option<String>,
    /// The creation time of this environment history entry.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The developer-provided description for this environment history entry.
    
    pub description: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2EnvironmentHistoryEntry {}


/// The configuration for model evaluation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2EvaluationConfig {
    /// Required. Datasets used for evaluation.
    
    pub datasets: Option<Vec<GoogleCloudDialogflowV2InputDataset>>,
    /// Configuration for smart compose model evalution.
    #[serde(rename="smartComposeConfig")]
    
    pub smart_compose_config: Option<GoogleCloudDialogflowV2EvaluationConfigSmartComposeConfig>,
    /// Configuration for smart reply model evalution.
    #[serde(rename="smartReplyConfig")]
    
    pub smart_reply_config: Option<GoogleCloudDialogflowV2EvaluationConfigSmartReplyConfig>,
}

impl client::Part for GoogleCloudDialogflowV2EvaluationConfig {}


/// Smart compose specific configuration for evaluation job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2EvaluationConfigSmartComposeConfig {
    /// The allowlist document resource name. Format: `projects//knowledgeBases//documents/`. Only used for smart compose model.
    #[serde(rename="allowlistDocument")]
    
    pub allowlist_document: Option<String>,
    /// Required. The model to be evaluated can return multiple results with confidence score on each query. These results will be sorted by the descending order of the scores and we only keep the first max_result_count results as the final results to evaluate.
    #[serde(rename="maxResultCount")]
    
    pub max_result_count: Option<i32>,
}

impl client::Part for GoogleCloudDialogflowV2EvaluationConfigSmartComposeConfig {}


/// Smart reply specific configuration for evaluation job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2EvaluationConfigSmartReplyConfig {
    /// The allowlist document resource name. Format: `projects//knowledgeBases//documents/`. Only used for smart reply model.
    #[serde(rename="allowlistDocument")]
    
    pub allowlist_document: Option<String>,
    /// Required. The model to be evaluated can return multiple results with confidence score on each query. These results will be sorted by the descending order of the scores and we only keep the first max_result_count results as the final results to evaluate.
    #[serde(rename="maxResultCount")]
    
    pub max_result_count: Option<i32>,
}

impl client::Part for GoogleCloudDialogflowV2EvaluationConfigSmartReplyConfig {}


/// Events allow for matching intents by event name instead of the natural language input. For instance, input `` can trigger a personalized welcome response. The parameter `name` may be used by the agent in the response: `"Hello #welcome_event.name! What can I do for you today?"`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2EventInput {
    /// Required. The language of this query. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language. This field is ignored when used in the context of a WebhookResponse.followup_event_input field, because the language was already defined in the originating detect intent request.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Required. The unique identifier of the event.
    
    pub name: Option<String>,
    /// The collection of parameters associated with the event. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value
    
    pub parameters: Option<HashMap<String, json::Value>>,
}

impl client::Part for GoogleCloudDialogflowV2EventInput {}


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
pub struct GoogleCloudDialogflowV2ExportAgentRequest {
    /// Required. The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to export the agent to. The format of this URI must be `gs:///`. If left unspecified, the serialized agent is returned inline. Dialogflow performs a write operation for the Cloud Storage object on the caller's behalf, so your request authentication must have write permissions for the object. For more information, see [Dialogflow access control](https://cloud.google.com/dialogflow/cx/docs/concept/access-control#storage).
    #[serde(rename="agentUri")]
    
    pub agent_uri: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2ExportAgentRequest {}


/// Request message for Documents.ExportDocument.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [knowledge bases documents export projects](ProjectKnowledgeBaseDocumentExportCall) (request)
/// * [locations knowledge bases documents export projects](ProjectLocationKnowledgeBaseDocumentExportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2ExportDocumentRequest {
    /// When enabled, export the full content of the document including empirical probability.
    #[serde(rename="exportFullContent")]
    
    pub export_full_content: Option<bool>,
    /// Cloud Storage file path to export the document.
    #[serde(rename="gcsDestination")]
    
    pub gcs_destination: Option<GoogleCloudDialogflowV2GcsDestination>,
    /// When enabled, export the smart messaging allowlist document for partial update.
    #[serde(rename="smartMessagingPartialUpdate")]
    
    pub smart_messaging_partial_update: Option<bool>,
}

impl client::RequestValue for GoogleCloudDialogflowV2ExportDocumentRequest {}


/// Represents answer from "frequently asked questions".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2FaqAnswer {
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

impl client::Part for GoogleCloudDialogflowV2FaqAnswer {}


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
pub struct GoogleCloudDialogflowV2Fulfillment {
    /// Optional. The human-readable name of the fulfillment, unique within the agent. This field is not used for Fulfillment in an Environment.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. Whether fulfillment is enabled.
    
    pub enabled: Option<bool>,
    /// Optional. The field defines whether the fulfillment is enabled for certain features.
    
    pub features: Option<Vec<GoogleCloudDialogflowV2FulfillmentFeature>>,
    /// Configuration for a generic web service.
    #[serde(rename="genericWebService")]
    
    pub generic_web_service: Option<GoogleCloudDialogflowV2FulfillmentGenericWebService>,
    /// Required. The unique identifier of the fulfillment. Supported formats: - `projects//agent/fulfillment` - `projects//locations//agent/fulfillment` This field is not used for Fulfillment in an Environment.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2Fulfillment {}
impl client::ResponseResult for GoogleCloudDialogflowV2Fulfillment {}


/// Whether fulfillment is enabled for the specific feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2FulfillmentFeature {
    /// The type of the feature that enabled for fulfillment.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2FulfillmentFeature {}


/// Represents configuration for a generic web service. Dialogflow supports two mechanisms for authentications: - Basic authentication with username and password. - Authentication with additional authentication headers. More information could be found at: https://cloud.google.com/dialogflow/docs/fulfillment-configure.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2FulfillmentGenericWebService {
    /// Optional. Indicates if generic web service is created through Cloud Functions integration. Defaults to false. is_cloud_function is deprecated. Cloud functions can be configured by its uri as a regular web service now.
    #[serde(rename="isCloudFunction")]
    
    pub is_cloud_function: Option<bool>,
    /// Optional. The password for HTTP Basic authentication.
    
    pub password: Option<String>,
    /// Optional. The HTTP request headers to send together with fulfillment requests.
    #[serde(rename="requestHeaders")]
    
    pub request_headers: Option<HashMap<String, String>>,
    /// Required. The fulfillment URI for receiving POST requests. It must use https protocol.
    
    pub uri: Option<String>,
    /// Optional. The user name for HTTP Basic authentication.
    
    pub username: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2FulfillmentGenericWebService {}


/// Google Cloud Storage location for the output.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2GcsDestination {
    /// The Google Cloud Storage URIs for the output. A URI is of the form: gs://bucket/object-prefix-or-name Whether a prefix or name is used depends on the use case. The requesting user must have "write-permission" to the bucket.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2GcsDestination {}


/// Google Cloud Storage location for the inputs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2GcsSources {
    /// Required. Google Cloud Storage URIs for the inputs. A URI is of the form: gs://bucket/object-prefix-or-name Whether a prefix or name is used depends on the use case.
    
    pub uris: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowV2GcsSources {}


/// Defines the Human Agent Assist to connect to a conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2HumanAgentAssistantConfig {
    /// Configuration for agent assistance of end user participant. Currently, this feature is not general available, please contact Google to get access.
    #[serde(rename="endUserSuggestionConfig")]
    
    pub end_user_suggestion_config: Option<GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionConfig>,
    /// Configuration for agent assistance of human agent participant.
    #[serde(rename="humanAgentSuggestionConfig")]
    
    pub human_agent_suggestion_config: Option<GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionConfig>,
    /// Configuration for message analysis.
    #[serde(rename="messageAnalysisConfig")]
    
    pub message_analysis_config: Option<GoogleCloudDialogflowV2HumanAgentAssistantConfigMessageAnalysisConfig>,
    /// Pub/Sub topic on which to publish new agent assistant events.
    #[serde(rename="notificationConfig")]
    
    pub notification_config: Option<GoogleCloudDialogflowV2NotificationConfig>,
}

impl client::Part for GoogleCloudDialogflowV2HumanAgentAssistantConfig {}


/// Custom conversation models used in agent assist feature. Supported feature: ARTICLE_SUGGESTION, SMART_COMPOSE, SMART_REPLY, CONVERSATION_SUMMARIZATION.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2HumanAgentAssistantConfigConversationModelConfig {
    /// Conversation model resource name. Format: `projects//conversationModels/`.
    
    pub model: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2HumanAgentAssistantConfigConversationModelConfig {}


/// Config to process conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2HumanAgentAssistantConfigConversationProcessConfig {
    /// Number of recent non-small-talk sentences to use as context for article and FAQ suggestion
    #[serde(rename="recentSentencesCount")]
    
    pub recent_sentences_count: Option<i32>,
}

impl client::Part for GoogleCloudDialogflowV2HumanAgentAssistantConfigConversationProcessConfig {}


/// Configuration for analyses to run on each conversation message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2HumanAgentAssistantConfigMessageAnalysisConfig {
    /// Enable entity extraction in conversation messages on [agent assist stage](https://cloud.google.com/dialogflow/priv/docs/contact-center/basics#stages). If unspecified, defaults to false. Currently, this feature is not general available, please contact Google to get access.
    #[serde(rename="enableEntityExtraction")]
    
    pub enable_entity_extraction: Option<bool>,
    /// Enable sentiment analysis in conversation messages on [agent assist stage](https://cloud.google.com/dialogflow/priv/docs/contact-center/basics#stages). If unspecified, defaults to false. Sentiment analysis inspects user input and identifies the prevailing subjective opinion, especially to determine a user's attitude as positive, negative, or neutral: https://cloud.google.com/natural-language/docs/basics#sentiment_analysis For Participants.StreamingAnalyzeContent method, result will be in StreamingAnalyzeContentResponse.message.SentimentAnalysisResult. For Participants.AnalyzeContent method, result will be in AnalyzeContentResponse.message.SentimentAnalysisResult For Conversations.ListMessages method, result will be in ListMessagesResponse.messages.SentimentAnalysisResult If Pub/Sub notification is configured, result will be in ConversationEvent.new_message_payload.SentimentAnalysisResult.
    #[serde(rename="enableSentimentAnalysis")]
    
    pub enable_sentiment_analysis: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2HumanAgentAssistantConfigMessageAnalysisConfig {}


/// Detail human agent assistant config.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionConfig {
    /// Configuration of different suggestion features. One feature can have only one config.
    #[serde(rename="featureConfigs")]
    
    pub feature_configs: Option<Vec<GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionFeatureConfig>>,
    /// If `group_suggestion_responses` is false, and there are multiple `feature_configs` in `event based suggestion` or StreamingAnalyzeContent, we will try to deliver suggestions to customers as soon as we get new suggestion. Different type of suggestions based on the same context will be in separate Pub/Sub event or `StreamingAnalyzeContentResponse`. If `group_suggestion_responses` set to true. All the suggestions to the same participant based on the same context will be grouped into a single Pub/Sub event or StreamingAnalyzeContentResponse.
    #[serde(rename="groupSuggestionResponses")]
    
    pub group_suggestion_responses: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionConfig {}


/// Config for suggestion features.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionFeatureConfig {
    /// Configs of custom conversation model.
    #[serde(rename="conversationModelConfig")]
    
    pub conversation_model_config: Option<GoogleCloudDialogflowV2HumanAgentAssistantConfigConversationModelConfig>,
    /// Configs for processing conversation.
    #[serde(rename="conversationProcessConfig")]
    
    pub conversation_process_config: Option<GoogleCloudDialogflowV2HumanAgentAssistantConfigConversationProcessConfig>,
    /// Automatically iterates all participants and tries to compile suggestions. Supported features: ARTICLE_SUGGESTION, FAQ, DIALOGFLOW_ASSIST.
    #[serde(rename="enableEventBasedSuggestion")]
    
    pub enable_event_based_suggestion: Option<bool>,
    /// Configs of query.
    #[serde(rename="queryConfig")]
    
    pub query_config: Option<GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfig>,
    /// The suggestion feature.
    #[serde(rename="suggestionFeature")]
    
    pub suggestion_feature: Option<GoogleCloudDialogflowV2SuggestionFeature>,
    /// Settings of suggestion trigger. Currently, only ARTICLE_SUGGESTION and FAQ will use this field.
    #[serde(rename="suggestionTriggerSettings")]
    
    pub suggestion_trigger_settings: Option<GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionTriggerSettings>,
}

impl client::Part for GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionFeatureConfig {}


/// Config for suggestion query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfig {
    /// Confidence threshold of query result. Agent Assist gives each suggestion a score in the range [0.0, 1.0], based on the relevance between the suggestion and the current conversation context. A score of 0.0 has no relevance, while a score of 1.0 has high relevance. Only suggestions with a score greater than or equal to the value of this field are included in the results. For a baseline model (the default), the recommended value is in the range [0.05, 0.1]. For a custom model, there is no recommended value. Tune this value by starting from a very low value and slowly increasing until you have desired results. If this field is not set, it defaults to 0.0, which means that all suggestions are returned. Supported features: ARTICLE_SUGGESTION, FAQ, SMART_REPLY, SMART_COMPOSE.
    #[serde(rename="confidenceThreshold")]
    
    pub confidence_threshold: Option<f32>,
    /// Determines how recent conversation context is filtered when generating suggestions. If unspecified, no messages will be dropped.
    #[serde(rename="contextFilterSettings")]
    
    pub context_filter_settings: Option<GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigContextFilterSettings>,
    /// Query from Dialogflow agent. It is used by DIALOGFLOW_ASSIST.
    #[serde(rename="dialogflowQuerySource")]
    
    pub dialogflow_query_source: Option<GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigDialogflowQuerySource>,
    /// Query from knowledge base document. It is used by: SMART_REPLY, SMART_COMPOSE.
    #[serde(rename="documentQuerySource")]
    
    pub document_query_source: Option<GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigDocumentQuerySource>,
    /// Query from knowledgebase. It is used by: ARTICLE_SUGGESTION, FAQ.
    #[serde(rename="knowledgeBaseQuerySource")]
    
    pub knowledge_base_query_source: Option<GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigKnowledgeBaseQuerySource>,
    /// Maximum number of results to return. Currently, if unset, defaults to 10. And the max number is 20.
    #[serde(rename="maxResults")]
    
    pub max_results: Option<i32>,
}

impl client::Part for GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfig {}


/// Settings that determine how to filter recent conversation context when generating suggestions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigContextFilterSettings {
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

impl client::Part for GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigContextFilterSettings {}


/// Dialogflow source setting. Supported feature: DIALOGFLOW_ASSIST.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigDialogflowQuerySource {
    /// Required. The name of a Dialogflow virtual agent used for end user side intent detection and suggestion. Format: `projects//locations//agent`. When multiple agents are allowed in the same Dialogflow project.
    
    pub agent: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigDialogflowQuerySource {}


/// Document source settings. Supported features: SMART_REPLY, SMART_COMPOSE.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigDocumentQuerySource {
    /// Required. Knowledge documents to query from. Format: `projects//locations//knowledgeBases//documents/`. Currently, at most 5 documents are supported.
    
    pub documents: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigDocumentQuerySource {}


/// Knowledge base source settings. Supported features: ARTICLE_SUGGESTION, FAQ.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigKnowledgeBaseQuerySource {
    /// Required. Knowledge bases to query. Format: `projects//locations//knowledgeBases/`. Currently, at most 5 knowledge bases are supported.
    #[serde(rename="knowledgeBases")]
    
    pub knowledge_bases: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigKnowledgeBaseQuerySource {}


/// Settings of suggestion trigger.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionTriggerSettings {
    /// Do not trigger if last utterance is small talk.
    #[serde(rename="noSmalltalk")]
    
    pub no_smalltalk: Option<bool>,
    /// Only trigger suggestion if participant role of last utterance is END_USER.
    #[serde(rename="onlyEndUser")]
    
    pub only_end_user: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionTriggerSettings {}


/// Defines the hand off to a live agent, typically on which external agent service provider to connect to a conversation. Currently, this feature is not general available, please contact Google to get access.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2HumanAgentHandoffConfig {
    /// Uses LivePerson (https://www.liveperson.com).
    #[serde(rename="livePersonConfig")]
    
    pub live_person_config: Option<GoogleCloudDialogflowV2HumanAgentHandoffConfigLivePersonConfig>,
    /// Uses Salesforce Live Agent.
    #[serde(rename="salesforceLiveAgentConfig")]
    
    pub salesforce_live_agent_config: Option<GoogleCloudDialogflowV2HumanAgentHandoffConfigSalesforceLiveAgentConfig>,
}

impl client::Part for GoogleCloudDialogflowV2HumanAgentHandoffConfig {}


/// Configuration specific to LivePerson (https://www.liveperson.com).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2HumanAgentHandoffConfigLivePersonConfig {
    /// Required. Account number of the LivePerson account to connect. This is the account number you input at the login page.
    #[serde(rename="accountNumber")]
    
    pub account_number: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2HumanAgentHandoffConfigLivePersonConfig {}


/// Configuration specific to Salesforce Live Agent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2HumanAgentHandoffConfigSalesforceLiveAgentConfig {
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

impl client::Part for GoogleCloudDialogflowV2HumanAgentHandoffConfigSalesforceLiveAgentConfig {}


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
pub struct GoogleCloudDialogflowV2ImportAgentRequest {
    /// Zip compressed raw byte content for agent.
    #[serde(rename="agentContent")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub agent_content: Option<Vec<u8>>,
    /// The URI to a Google Cloud Storage file containing the agent to import. Note: The URI must start with "gs://". Dialogflow performs a read operation for the Cloud Storage object on the caller's behalf, so your request authentication must have read permissions for the object. For more information, see [Dialogflow access control](https://cloud.google.com/dialogflow/cx/docs/concept/access-control#storage).
    #[serde(rename="agentUri")]
    
    pub agent_uri: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2ImportAgentRequest {}


/// The request message for ConversationDatasets.ImportConversationData.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversation datasets import conversation data projects](ProjectConversationDatasetImportConversationDataCall) (request)
/// * [locations conversation datasets import conversation data projects](ProjectLocationConversationDatasetImportConversationDataCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2ImportConversationDataRequest {
    /// Required. Configuration describing where to import data from.
    #[serde(rename="inputConfig")]
    
    pub input_config: Option<GoogleCloudDialogflowV2InputConfig>,
}

impl client::RequestValue for GoogleCloudDialogflowV2ImportConversationDataRequest {}


/// The template used for importing documents.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2ImportDocumentTemplate {
    /// Required. The knowledge type of document content.
    #[serde(rename="knowledgeTypes")]
    
    pub knowledge_types: Option<Vec<String>>,
    /// Metadata for the document. The metadata supports arbitrary key-value pairs. Suggested use cases include storing a document's title, an external URL distinct from the document's content_uri, etc. The max size of a `key` or a `value` of the metadata is 1024 bytes.
    
    pub metadata: Option<HashMap<String, String>>,
    /// Required. The MIME type of the document.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2ImportDocumentTemplate {}


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
pub struct GoogleCloudDialogflowV2ImportDocumentsRequest {
    /// Required. Document template used for importing all the documents.
    #[serde(rename="documentTemplate")]
    
    pub document_template: Option<GoogleCloudDialogflowV2ImportDocumentTemplate>,
    /// The Google Cloud Storage location for the documents. The path can include a wildcard. These URIs may have the forms `gs:///`. `gs:////*.`.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GoogleCloudDialogflowV2GcsSources>,
    /// Whether to import custom metadata from Google Cloud Storage. Only valid when the document source is Google Cloud Storage URI.
    #[serde(rename="importGcsCustomMetadata")]
    
    pub import_gcs_custom_metadata: Option<bool>,
}

impl client::RequestValue for GoogleCloudDialogflowV2ImportDocumentsRequest {}


/// Instructs the speech recognizer how to process the audio content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2InputAudioConfig {
    /// Required. Audio encoding of the audio content to process.
    #[serde(rename="audioEncoding")]
    
    pub audio_encoding: Option<String>,
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
    
    pub model_variant: Option<String>,
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
    
    pub speech_contexts: Option<Vec<GoogleCloudDialogflowV2SpeechContext>>,
}

impl client::Part for GoogleCloudDialogflowV2InputAudioConfig {}


/// Represents the configuration of importing a set of conversation files in Google Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2InputConfig {
    /// The Cloud Storage URI has the form gs:////agent*.json. Wildcards are allowed and will be expanded into all matched JSON files, which will be read as one conversation per file.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GoogleCloudDialogflowV2GcsSources>,
}

impl client::Part for GoogleCloudDialogflowV2InputConfig {}


/// InputDataset used to create model or do evaluation. NextID:5
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2InputDataset {
    /// Required. ConversationDataset resource name. Format: `projects//locations//conversationDatasets/`
    
    pub dataset: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2InputDataset {}


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
pub struct GoogleCloudDialogflowV2Intent {
    /// Optional. The name of the action associated with the intent. Note: The action name must not contain whitespaces.
    
    pub action: Option<String>,
    /// Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform).
    #[serde(rename="defaultResponsePlatforms")]
    
    pub default_response_platforms: Option<Vec<String>>,
    /// Required. The name of this intent.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. Indicates that this intent ends an interaction. Some integrations (e.g., Actions on Google or Dialogflow phone gateway) use this information to close interaction with an end user. Default is false.
    #[serde(rename="endInteraction")]
    
    pub end_interaction: Option<bool>,
    /// Optional. The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of the contexts must be present in the active user session for an event to trigger this intent. Event names are limited to 150 characters.
    
    pub events: Option<Vec<String>>,
    /// Output only. Read-only. Information about all followup intents that have this intent as a direct or indirect parent. We populate this field only in the output.
    #[serde(rename="followupIntentInfo")]
    
    pub followup_intent_info: Option<Vec<GoogleCloudDialogflowV2IntentFollowupIntentInfo>>,
    /// Optional. The list of context names required for this intent to be triggered. Format: `projects//agent/sessions/-/contexts/`.
    #[serde(rename="inputContextNames")]
    
    pub input_context_names: Option<Vec<String>>,
    /// Optional. Indicates whether this is a fallback intent.
    #[serde(rename="isFallback")]
    
    pub is_fallback: Option<bool>,
    /// Optional. Indicates that a live agent should be brought in to handle the interaction with the user. In most cases, when you set this flag to true, you would also want to set end_interaction to true as well. Default is false.
    #[serde(rename="liveAgentHandoff")]
    
    pub live_agent_handoff: Option<bool>,
    /// Optional. The collection of rich messages corresponding to the `Response` field in the Dialogflow console.
    
    pub messages: Option<Vec<GoogleCloudDialogflowV2IntentMessage>>,
    /// Optional. Indicates whether Machine Learning is disabled for the intent. Note: If `ml_disabled` setting is set to true, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off.
    #[serde(rename="mlDisabled")]
    
    pub ml_disabled: Option<bool>,
    /// Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Format: `projects//agent/intents/`.
    
    pub name: Option<String>,
    /// Optional. The collection of contexts that are activated when the intent is matched. Context messages in this collection should not set the parameters field. Setting the `lifespan_count` to 0 will reset the context when the intent is matched. Format: `projects//agent/sessions/-/contexts/`.
    #[serde(rename="outputContexts")]
    
    pub output_contexts: Option<Vec<GoogleCloudDialogflowV2Context>>,
    /// Optional. The collection of parameters associated with the intent.
    
    pub parameters: Option<Vec<GoogleCloudDialogflowV2IntentParameter>>,
    /// Read-only after creation. The unique identifier of the parent intent in the chain of followup intents. You can set this field when creating an intent, for example with CreateIntent or BatchUpdateIntents, in order to make this intent a followup intent. It identifies the parent followup intent. Format: `projects//agent/intents/`.
    #[serde(rename="parentFollowupIntentName")]
    
    pub parent_followup_intent_name: Option<String>,
    /// Optional. The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests.
    
    pub priority: Option<i32>,
    /// Optional. Indicates whether to delete all contexts in the current session when this intent is matched.
    #[serde(rename="resetContexts")]
    
    pub reset_contexts: Option<bool>,
    /// Output only. Read-only. The unique identifier of the root intent in the chain of followup intents. It identifies the correct followup intents chain for this intent. We populate this field only in the output. Format: `projects//agent/intents/`.
    #[serde(rename="rootFollowupIntentName")]
    
    pub root_followup_intent_name: Option<String>,
    /// Optional. The collection of examples that the agent is trained on.
    #[serde(rename="trainingPhrases")]
    
    pub training_phrases: Option<Vec<GoogleCloudDialogflowV2IntentTrainingPhrase>>,
    /// Optional. Indicates whether webhooks are enabled for the intent.
    #[serde(rename="webhookState")]
    
    pub webhook_state: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2Intent {}
impl client::ResponseResult for GoogleCloudDialogflowV2Intent {}


/// This message is a wrapper around a collection of intents.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentBatch {
    /// A collection of intents.
    
    pub intents: Option<Vec<GoogleCloudDialogflowV2Intent>>,
}

impl client::Part for GoogleCloudDialogflowV2IntentBatch {}


/// Represents a single followup intent in the chain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentFollowupIntentInfo {
    /// The unique identifier of the followup intent. Format: `projects//agent/intents/`.
    #[serde(rename="followupIntentName")]
    
    pub followup_intent_name: Option<String>,
    /// The unique identifier of the followup intent's parent. Format: `projects//agent/intents/`.
    #[serde(rename="parentFollowupIntentName")]
    
    pub parent_followup_intent_name: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentFollowupIntentInfo {}


/// A rich response message. Corresponds to the intent `Response` field in the Dialogflow console. For more information, see [Rich response messages](https://cloud.google.com/dialogflow/docs/intents-rich-messages).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessage {
    /// The basic card response for Actions on Google.
    #[serde(rename="basicCard")]
    
    pub basic_card: Option<GoogleCloudDialogflowV2IntentMessageBasicCard>,
    /// Browse carousel card for Actions on Google.
    #[serde(rename="browseCarouselCard")]
    
    pub browse_carousel_card: Option<GoogleCloudDialogflowV2IntentMessageBrowseCarouselCard>,
    /// The card response.
    
    pub card: Option<GoogleCloudDialogflowV2IntentMessageCard>,
    /// The carousel card response for Actions on Google.
    #[serde(rename="carouselSelect")]
    
    pub carousel_select: Option<GoogleCloudDialogflowV2IntentMessageCarouselSelect>,
    /// The image response.
    
    pub image: Option<GoogleCloudDialogflowV2IntentMessageImage>,
    /// The link out suggestion chip for Actions on Google.
    #[serde(rename="linkOutSuggestion")]
    
    pub link_out_suggestion: Option<GoogleCloudDialogflowV2IntentMessageLinkOutSuggestion>,
    /// The list card response for Actions on Google.
    #[serde(rename="listSelect")]
    
    pub list_select: Option<GoogleCloudDialogflowV2IntentMessageListSelect>,
    /// The media content card for Actions on Google.
    #[serde(rename="mediaContent")]
    
    pub media_content: Option<GoogleCloudDialogflowV2IntentMessageMediaContent>,
    /// A custom platform-specific response.
    
    pub payload: Option<HashMap<String, json::Value>>,
    /// Optional. The platform that this message is intended for.
    
    pub platform: Option<String>,
    /// The quick replies response.
    #[serde(rename="quickReplies")]
    
    pub quick_replies: Option<GoogleCloudDialogflowV2IntentMessageQuickReplies>,
    /// The voice and text-only responses for Actions on Google.
    #[serde(rename="simpleResponses")]
    
    pub simple_responses: Option<GoogleCloudDialogflowV2IntentMessageSimpleResponses>,
    /// The suggestion chips for Actions on Google.
    
    pub suggestions: Option<GoogleCloudDialogflowV2IntentMessageSuggestions>,
    /// Table card for Actions on Google.
    #[serde(rename="tableCard")]
    
    pub table_card: Option<GoogleCloudDialogflowV2IntentMessageTableCard>,
    /// The text response.
    
    pub text: Option<GoogleCloudDialogflowV2IntentMessageText>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessage {}


/// The basic card message. Useful for displaying information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageBasicCard {
    /// Optional. The collection of card buttons.
    
    pub buttons: Option<Vec<GoogleCloudDialogflowV2IntentMessageBasicCardButton>>,
    /// Required, unless image is present. The body text of the card.
    #[serde(rename="formattedText")]
    
    pub formatted_text: Option<String>,
    /// Optional. The image for the card.
    
    pub image: Option<GoogleCloudDialogflowV2IntentMessageImage>,
    /// Optional. The subtitle of the card.
    
    pub subtitle: Option<String>,
    /// Optional. The title of the card.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageBasicCard {}


/// The button object that appears at the bottom of a card.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageBasicCardButton {
    /// Required. Action to take when a user taps on the button.
    #[serde(rename="openUriAction")]
    
    pub open_uri_action: Option<GoogleCloudDialogflowV2IntentMessageBasicCardButtonOpenUriAction>,
    /// Required. The title of the button.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageBasicCardButton {}


/// Opens the given URI.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageBasicCardButtonOpenUriAction {
    /// Required. The HTTP or HTTPS scheme URI.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageBasicCardButtonOpenUriAction {}


/// Browse Carousel Card for Actions on Google. https://developers.google.com/actions/assistant/responses#browsing_carousel
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageBrowseCarouselCard {
    /// Optional. Settings for displaying the image. Applies to every image in items.
    #[serde(rename="imageDisplayOptions")]
    
    pub image_display_options: Option<String>,
    /// Required. List of items in the Browse Carousel Card. Minimum of two items, maximum of ten.
    
    pub items: Option<Vec<GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItem>>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageBrowseCarouselCard {}


/// Browsing carousel tile
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItem {
    /// Optional. Description of the carousel item. Maximum of four lines of text.
    
    pub description: Option<String>,
    /// Optional. Text that appears at the bottom of the Browse Carousel Card. Maximum of one line of text.
    
    pub footer: Option<String>,
    /// Optional. Hero image for the carousel item.
    
    pub image: Option<GoogleCloudDialogflowV2IntentMessageImage>,
    /// Required. Action to present to the user.
    #[serde(rename="openUriAction")]
    
    pub open_uri_action: Option<GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction>,
    /// Required. Title of the carousel item. Maximum of two lines of text.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItem {}


/// Actions on Google action to open a given url.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction {
    /// Required. URL
    
    pub url: Option<String>,
    /// Optional. Specifies the type of viewer that is used when opening the URL. Defaults to opening via web browser.
    #[serde(rename="urlTypeHint")]
    
    pub url_type_hint: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction {}


/// The card response message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageCard {
    /// Optional. The collection of card buttons.
    
    pub buttons: Option<Vec<GoogleCloudDialogflowV2IntentMessageCardButton>>,
    /// Optional. The public URI to an image file for the card.
    #[serde(rename="imageUri")]
    
    pub image_uri: Option<String>,
    /// Optional. The subtitle of the card.
    
    pub subtitle: Option<String>,
    /// Optional. The title of the card.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageCard {}


/// Contains information about a button.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageCardButton {
    /// Optional. The text to send back to the Dialogflow API or a URI to open.
    
    pub postback: Option<String>,
    /// Optional. The text to show on the button.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageCardButton {}


/// The card for presenting a carousel of options to select from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageCarouselSelect {
    /// Required. Carousel items.
    
    pub items: Option<Vec<GoogleCloudDialogflowV2IntentMessageCarouselSelectItem>>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageCarouselSelect {}


/// An item in the carousel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageCarouselSelectItem {
    /// Optional. The body text of the card.
    
    pub description: Option<String>,
    /// Optional. The image to display.
    
    pub image: Option<GoogleCloudDialogflowV2IntentMessageImage>,
    /// Required. Additional info about the option item.
    
    pub info: Option<GoogleCloudDialogflowV2IntentMessageSelectItemInfo>,
    /// Required. Title of the carousel item.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageCarouselSelectItem {}


/// Column properties for TableCard.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageColumnProperties {
    /// Required. Column heading.
    
    pub header: Option<String>,
    /// Optional. Defines text alignment for all cells in this column.
    #[serde(rename="horizontalAlignment")]
    
    pub horizontal_alignment: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageColumnProperties {}


/// The image response message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageImage {
    /// Optional. A text description of the image to be used for accessibility, e.g., screen readers.
    #[serde(rename="accessibilityText")]
    
    pub accessibility_text: Option<String>,
    /// Optional. The public URI to an image file.
    #[serde(rename="imageUri")]
    
    pub image_uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageImage {}


/// The suggestion chip message that allows the user to jump out to the app or website associated with this agent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageLinkOutSuggestion {
    /// Required. The name of the app or site this chip is linking to.
    #[serde(rename="destinationName")]
    
    pub destination_name: Option<String>,
    /// Required. The URI of the app or site to open when the user taps the suggestion chip.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageLinkOutSuggestion {}


/// The card for presenting a list of options to select from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageListSelect {
    /// Required. List items.
    
    pub items: Option<Vec<GoogleCloudDialogflowV2IntentMessageListSelectItem>>,
    /// Optional. Subtitle of the list.
    
    pub subtitle: Option<String>,
    /// Optional. The overall title of the list.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageListSelect {}


/// An item in the list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageListSelectItem {
    /// Optional. The main text describing the item.
    
    pub description: Option<String>,
    /// Optional. The image to display.
    
    pub image: Option<GoogleCloudDialogflowV2IntentMessageImage>,
    /// Required. Additional information about this option.
    
    pub info: Option<GoogleCloudDialogflowV2IntentMessageSelectItemInfo>,
    /// Required. The title of the list item.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageListSelectItem {}


/// The media content card for Actions on Google.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageMediaContent {
    /// Required. List of media objects.
    #[serde(rename="mediaObjects")]
    
    pub media_objects: Option<Vec<GoogleCloudDialogflowV2IntentMessageMediaContentResponseMediaObject>>,
    /// Optional. What type of media is the content (ie "audio").
    #[serde(rename="mediaType")]
    
    pub media_type: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageMediaContent {}


/// Response media object for media content card.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageMediaContentResponseMediaObject {
    /// Required. Url where the media is stored.
    #[serde(rename="contentUrl")]
    
    pub content_url: Option<String>,
    /// Optional. Description of media card.
    
    pub description: Option<String>,
    /// Optional. Icon to display above media content.
    
    pub icon: Option<GoogleCloudDialogflowV2IntentMessageImage>,
    /// Optional. Image to display above media content.
    #[serde(rename="largeImage")]
    
    pub large_image: Option<GoogleCloudDialogflowV2IntentMessageImage>,
    /// Required. Name of media card.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageMediaContentResponseMediaObject {}


/// The quick replies response message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageQuickReplies {
    /// Optional. The collection of quick replies.
    #[serde(rename="quickReplies")]
    
    pub quick_replies: Option<Vec<String>>,
    /// Optional. The title of the collection of quick replies.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageQuickReplies {}


/// Additional info about the select item for when it is triggered in a dialog.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageSelectItemInfo {
    /// Required. A unique key that will be sent back to the agent if this response is given.
    
    pub key: Option<String>,
    /// Optional. A list of synonyms that can also be used to trigger this item in dialog.
    
    pub synonyms: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageSelectItemInfo {}


/// The simple response message containing speech or text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageSimpleResponse {
    /// Optional. The text to display.
    #[serde(rename="displayText")]
    
    pub display_text: Option<String>,
    /// One of text_to_speech or ssml must be provided. Structured spoken response to the user in the SSML format. Mutually exclusive with text_to_speech.
    
    pub ssml: Option<String>,
    /// One of text_to_speech or ssml must be provided. The plain text of the speech output. Mutually exclusive with ssml.
    #[serde(rename="textToSpeech")]
    
    pub text_to_speech: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageSimpleResponse {}


/// The collection of simple response candidates. This message in `QueryResult.fulfillment_messages` and `WebhookResponse.fulfillment_messages` should contain only one `SimpleResponse`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageSimpleResponses {
    /// Required. The list of simple responses.
    #[serde(rename="simpleResponses")]
    
    pub simple_responses: Option<Vec<GoogleCloudDialogflowV2IntentMessageSimpleResponse>>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageSimpleResponses {}


/// The suggestion chip message that the user can tap to quickly post a reply to the conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageSuggestion {
    /// Required. The text shown the in the suggestion chip.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageSuggestion {}


/// The collection of suggestions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageSuggestions {
    /// Required. The list of suggested replies.
    
    pub suggestions: Option<Vec<GoogleCloudDialogflowV2IntentMessageSuggestion>>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageSuggestions {}


/// Table card for Actions on Google.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageTableCard {
    /// Optional. List of buttons for the card.
    
    pub buttons: Option<Vec<GoogleCloudDialogflowV2IntentMessageBasicCardButton>>,
    /// Optional. Display properties for the columns in this table.
    #[serde(rename="columnProperties")]
    
    pub column_properties: Option<Vec<GoogleCloudDialogflowV2IntentMessageColumnProperties>>,
    /// Optional. Image which should be displayed on the card.
    
    pub image: Option<GoogleCloudDialogflowV2IntentMessageImage>,
    /// Optional. Rows in this table of data.
    
    pub rows: Option<Vec<GoogleCloudDialogflowV2IntentMessageTableCardRow>>,
    /// Optional. Subtitle to the title.
    
    pub subtitle: Option<String>,
    /// Required. Title of the card.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageTableCard {}


/// Cell of TableCardRow.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageTableCardCell {
    /// Required. Text in this cell.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageTableCardCell {}


/// Row of TableCard.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageTableCardRow {
    /// Optional. List of cells that make up this row.
    
    pub cells: Option<Vec<GoogleCloudDialogflowV2IntentMessageTableCardCell>>,
    /// Optional. Whether to add a visual divider after this row.
    #[serde(rename="dividerAfter")]
    
    pub divider_after: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageTableCardRow {}


/// The text response message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentMessageText {
    /// Optional. The collection of the agent's responses.
    
    pub text: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowV2IntentMessageText {}


/// Represents intent parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentParameter {
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

impl client::Part for GoogleCloudDialogflowV2IntentParameter {}


/// Represents an example that the agent is trained on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentTrainingPhrase {
    /// Output only. The unique identifier of this training phrase.
    
    pub name: Option<String>,
    /// Required. The ordered list of training phrase parts. The parts are concatenated in order to form the training phrase. Note: The API does not automatically annotate training phrases like the Dialogflow Console does. Note: Do not forget to include whitespace at part boundaries, so the training phrase is well formatted when the parts are concatenated. If the training phrase does not need to be annotated with parameters, you just need a single part with only the Part.text field set. If you want to annotate the training phrase, you must create multiple parts, where the fields of each part are populated in one of two ways: - `Part.text` is set to a part of the phrase that has no parameters. - `Part.text` is set to a part of the phrase that you want to annotate, and the `entity_type`, `alias`, and `user_defined` fields are all set.
    
    pub parts: Option<Vec<GoogleCloudDialogflowV2IntentTrainingPhrasePart>>,
    /// Optional. Indicates how many times this example was added to the intent. Each time a developer adds an existing sample by editing an intent or training, this counter is increased.
    #[serde(rename="timesAddedCount")]
    
    pub times_added_count: Option<i32>,
    /// Required. The type of the training phrase.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2IntentTrainingPhrase {}


/// Represents a part of a training phrase.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2IntentTrainingPhrasePart {
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

impl client::Part for GoogleCloudDialogflowV2IntentTrainingPhrasePart {}


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
pub struct GoogleCloudDialogflowV2KnowledgeBase {
    /// Required. The display name of the knowledge base. The name must be 1024 bytes or less; otherwise, the creation request fails.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Language which represents the KnowledgeBase. When the KnowledgeBase is created/updated, expect this to be present for non en-us languages. When unspecified, the default language code en-us applies.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// The knowledge base resource name. The name must be empty when creating a knowledge base. Format: `projects//locations//knowledgeBases/`.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2KnowledgeBase {}
impl client::ResponseResult for GoogleCloudDialogflowV2KnowledgeBase {}


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
pub struct GoogleCloudDialogflowV2ListAnswerRecordsResponse {
    /// The list of answer records.
    #[serde(rename="answerRecords")]
    
    pub answer_records: Option<Vec<GoogleCloudDialogflowV2AnswerRecord>>,
    /// A token to retrieve next page of results. Or empty if there are no more results. Pass this value in the ListAnswerRecordsRequest.page_token field in the subsequent call to `ListAnswerRecords` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListAnswerRecordsResponse {}


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
pub struct GoogleCloudDialogflowV2ListContextsResponse {
    /// The list of contexts. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub contexts: Option<Vec<GoogleCloudDialogflowV2Context>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListContextsResponse {}


/// The response message for ConversationDatasets.ListConversationDatasets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversation datasets list projects](ProjectConversationDatasetListCall) (response)
/// * [locations conversation datasets list projects](ProjectLocationConversationDatasetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2ListConversationDatasetsResponse {
    /// The list of datasets to return.
    #[serde(rename="conversationDatasets")]
    
    pub conversation_datasets: Option<Vec<GoogleCloudDialogflowV2ConversationDataset>>,
    /// The token to use to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListConversationDatasetsResponse {}


/// The response message for ConversationModels.ListConversationModelEvaluations
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversation models evaluations list projects](ProjectConversationModelEvaluationListCall) (response)
/// * [locations conversation models evaluations list projects](ProjectLocationConversationModelEvaluationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2ListConversationModelEvaluationsResponse {
    /// The list of evaluations to return.
    #[serde(rename="conversationModelEvaluations")]
    
    pub conversation_model_evaluations: Option<Vec<GoogleCloudDialogflowV2ConversationModelEvaluation>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListConversationModelEvaluationsResponse {}


/// The response message for ConversationModels.ListConversationModels
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversation models list projects](ProjectConversationModelListCall) (response)
/// * [locations conversation models list projects](ProjectLocationConversationModelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2ListConversationModelsResponse {
    /// The list of models to return.
    #[serde(rename="conversationModels")]
    
    pub conversation_models: Option<Vec<GoogleCloudDialogflowV2ConversationModel>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListConversationModelsResponse {}


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
pub struct GoogleCloudDialogflowV2ListConversationProfilesResponse {
    /// The list of project conversation profiles. There is a maximum number of items returned based on the page_size field in the request.
    #[serde(rename="conversationProfiles")]
    
    pub conversation_profiles: Option<Vec<GoogleCloudDialogflowV2ConversationProfile>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListConversationProfilesResponse {}


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
pub struct GoogleCloudDialogflowV2ListConversationsResponse {
    /// The list of conversations. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub conversations: Option<Vec<GoogleCloudDialogflowV2Conversation>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListConversationsResponse {}


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
pub struct GoogleCloudDialogflowV2ListDocumentsResponse {
    /// The list of documents.
    
    pub documents: Option<Vec<GoogleCloudDialogflowV2Document>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListDocumentsResponse {}


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
pub struct GoogleCloudDialogflowV2ListEntityTypesResponse {
    /// The list of agent entity types. There will be a maximum number of items returned based on the page_size field in the request.
    #[serde(rename="entityTypes")]
    
    pub entity_types: Option<Vec<GoogleCloudDialogflowV2EntityType>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListEntityTypesResponse {}


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
pub struct GoogleCloudDialogflowV2ListEnvironmentsResponse {
    /// The list of agent environments. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub environments: Option<Vec<GoogleCloudDialogflowV2Environment>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListEnvironmentsResponse {}


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
pub struct GoogleCloudDialogflowV2ListIntentsResponse {
    /// The list of agent intents. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub intents: Option<Vec<GoogleCloudDialogflowV2Intent>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListIntentsResponse {}


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
pub struct GoogleCloudDialogflowV2ListKnowledgeBasesResponse {
    /// The list of knowledge bases.
    #[serde(rename="knowledgeBases")]
    
    pub knowledge_bases: Option<Vec<GoogleCloudDialogflowV2KnowledgeBase>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListKnowledgeBasesResponse {}


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
pub struct GoogleCloudDialogflowV2ListMessagesResponse {
    /// The list of messages. There will be a maximum number of items returned based on the page_size field in the request. `messages` is sorted by `create_time` in descending order.
    
    pub messages: Option<Vec<GoogleCloudDialogflowV2Message>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListMessagesResponse {}


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
pub struct GoogleCloudDialogflowV2ListParticipantsResponse {
    /// Token to retrieve the next page of results or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of participants. There is a maximum number of items returned based on the page_size field in the request.
    
    pub participants: Option<Vec<GoogleCloudDialogflowV2Participant>>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListParticipantsResponse {}


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
pub struct GoogleCloudDialogflowV2ListSessionEntityTypesResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of session entity types. There will be a maximum number of items returned based on the page_size field in the request.
    #[serde(rename="sessionEntityTypes")]
    
    pub session_entity_types: Option<Vec<GoogleCloudDialogflowV2SessionEntityType>>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListSessionEntityTypesResponse {}


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
pub struct GoogleCloudDialogflowV2ListVersionsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of agent versions. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub versions: Option<Vec<GoogleCloudDialogflowV2Version>>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ListVersionsResponse {}


/// Defines logging behavior for conversation lifecycle events.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2LoggingConfig {
    /// Whether to log conversation events like CONVERSATION_STARTED to Stackdriver in the conversation project as JSON format ConversationEvent protos.
    #[serde(rename="enableStackdriverLogging")]
    
    pub enable_stackdriver_logging: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2LoggingConfig {}


/// Represents a message posted into a conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2Message {
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
    
    pub message_annotation: Option<GoogleCloudDialogflowV2MessageAnnotation>,
    /// Optional. The unique identifier of the message. Format: `projects//locations//conversations//messages/`.
    
    pub name: Option<String>,
    /// Output only. The participant that sends this message.
    
    pub participant: Option<String>,
    /// Output only. The role of the participant.
    #[serde(rename="participantRole")]
    
    pub participant_role: Option<String>,
    /// Optional. The time when the message was sent.
    #[serde(rename="sendTime")]
    
    pub send_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The sentiment analysis result for the message.
    #[serde(rename="sentimentAnalysis")]
    
    pub sentiment_analysis: Option<GoogleCloudDialogflowV2SentimentAnalysisResult>,
}

impl client::Part for GoogleCloudDialogflowV2Message {}


/// Represents the result of annotation for the message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2MessageAnnotation {
    /// Indicates whether the text message contains entities.
    #[serde(rename="containEntities")]
    
    pub contain_entities: Option<bool>,
    /// The collection of annotated message parts ordered by their position in the message. You can recover the annotated message by concatenating [AnnotatedMessagePart.text].
    
    pub parts: Option<Vec<GoogleCloudDialogflowV2AnnotatedMessagePart>>,
}

impl client::Part for GoogleCloudDialogflowV2MessageAnnotation {}


/// Defines notification behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2NotificationConfig {
    /// Format of message.
    #[serde(rename="messageFormat")]
    
    pub message_format: Option<String>,
    /// Name of the Pub/Sub topic to publish conversation events like CONVERSATION_STARTED as serialized ConversationEvent protos. For telephony integration to receive notification, make sure either this topic is in the same project as the conversation or you grant `service-@gcp-sa-dialogflow.iam.gserviceaccount.com` the `Dialogflow Service Agent` role in the topic project. For chat integration to receive notification, make sure API caller has been granted the `Dialogflow Service Agent` role for the topic. Format: `projects//locations//topics/`.
    
    pub topic: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2NotificationConfig {}


/// Represents the natural language speech audio to be played to the end user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2OutputAudio {
    /// The natural language speech audio.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub audio: Option<Vec<u8>>,
    /// Instructs the speech synthesizer how to generate the speech audio.
    
    pub config: Option<GoogleCloudDialogflowV2OutputAudioConfig>,
}

impl client::Part for GoogleCloudDialogflowV2OutputAudio {}


/// Instructs the speech synthesizer on how to generate the output audio content. If this audio config is supplied in a request, it overrides all existing text-to-speech settings applied to the agent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2OutputAudioConfig {
    /// Required. Audio encoding of the synthesized audio content.
    #[serde(rename="audioEncoding")]
    
    pub audio_encoding: Option<String>,
    /// The synthesis sample rate (in hertz) for this audio. If not provided, then the synthesizer will use the default sample rate based on the audio encoding. If this is different from the voice's natural sample rate, then the synthesizer will honor this request by converting to the desired sample rate (which might result in worse audio quality).
    #[serde(rename="sampleRateHertz")]
    
    pub sample_rate_hertz: Option<i32>,
    /// Configuration of how speech should be synthesized.
    #[serde(rename="synthesizeSpeechConfig")]
    
    pub synthesize_speech_config: Option<GoogleCloudDialogflowV2SynthesizeSpeechConfig>,
}

impl client::Part for GoogleCloudDialogflowV2OutputAudioConfig {}


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
pub struct GoogleCloudDialogflowV2Participant {
    /// Optional. Key-value filters on the metadata of documents returned by article suggestion. If specified, article suggestion only returns suggested documents that match all filters in their Document.metadata. Multiple values for a metadata key should be concatenated by comma. For example, filters to match all documents that have 'US' or 'CA' in their market metadata values and 'agent' in their user metadata values will be ``` documents_metadata_filters { key: "market" value: "US,CA" } documents_metadata_filters { key: "user" value: "agent" } ```
    #[serde(rename="documentsMetadataFilters")]
    
    pub documents_metadata_filters: Option<HashMap<String, String>>,
    /// Optional. The unique identifier of this participant. Format: `projects//locations//conversations//participants/`.
    
    pub name: Option<String>,
    /// Optional. Obfuscated user id that should be associated with the created participant. You can specify a user id as follows: 1. If you set this field in CreateParticipantRequest or UpdateParticipantRequest, Dialogflow adds the obfuscated user id with the participant. 2. If you set this field in AnalyzeContent or StreamingAnalyzeContent, Dialogflow will update Participant.obfuscated_external_user_id. Dialogflow returns an error if you try to add a user id for a non-END_USER participant. Dialogflow uses this user id for billing and measurement purposes. For example, Dialogflow determines whether a user in one conversation returned in a later conversation. Note: * Please never pass raw user ids to Dialogflow. Always obfuscate your user id first. * Dialogflow only accepts a UTF-8 encoded string, e.g., a hex digest of a hash function like SHA-512. * The length of the user id must be <= 256 characters.
    #[serde(rename="obfuscatedExternalUserId")]
    
    pub obfuscated_external_user_id: Option<String>,
    /// Immutable. The role this participant plays in the conversation. This field must be set during participant creation and is then immutable.
    
    pub role: Option<String>,
    /// Optional. Label applied to streams representing this participant in SIPREC XML metadata and SDP. This is used to assign transcriptions from that media stream to this participant. This field can be updated.
    #[serde(rename="sipRecordingMediaLabel")]
    
    pub sip_recording_media_label: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2Participant {}
impl client::ResponseResult for GoogleCloudDialogflowV2Participant {}


/// Represents the query input. It can contain either: 1. An audio config which instructs the speech recognizer how to process the speech audio. 2. A conversational query in the form of text,. 3. An event that specifies which intent to trigger.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2QueryInput {
    /// Instructs the speech recognizer how to process the speech audio.
    #[serde(rename="audioConfig")]
    
    pub audio_config: Option<GoogleCloudDialogflowV2InputAudioConfig>,
    /// The event to be processed.
    
    pub event: Option<GoogleCloudDialogflowV2EventInput>,
    /// The natural language text to be processed. Text length must not exceed 256 character for virtual agent interactions.
    
    pub text: Option<GoogleCloudDialogflowV2TextInput>,
}

impl client::Part for GoogleCloudDialogflowV2QueryInput {}


/// Represents the parameters of the conversational query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2QueryParameters {
    /// The collection of contexts to be activated before this query is executed.
    
    pub contexts: Option<Vec<GoogleCloudDialogflowV2Context>>,
    /// The geo location of this conversational query.
    #[serde(rename="geoLocation")]
    
    pub geo_location: Option<GoogleTypeLatLng>,
    /// This field can be used to pass custom data to your webhook. Arbitrary JSON objects are supported. If supplied, the value is used to populate the `WebhookRequest.original_detect_intent_request.payload` field sent to your webhook.
    
    pub payload: Option<HashMap<String, json::Value>>,
    /// Specifies whether to delete all contexts in the current session before the new ones are activated.
    #[serde(rename="resetContexts")]
    
    pub reset_contexts: Option<bool>,
    /// Configures the type of sentiment analysis to perform. If not provided, sentiment analysis is not performed.
    #[serde(rename="sentimentAnalysisRequestConfig")]
    
    pub sentiment_analysis_request_config: Option<GoogleCloudDialogflowV2SentimentAnalysisRequestConfig>,
    /// Additional session entity types to replace or extend developer entity types with. The entity synonyms apply to all languages and persist for the session of this query.
    #[serde(rename="sessionEntityTypes")]
    
    pub session_entity_types: Option<Vec<GoogleCloudDialogflowV2SessionEntityType>>,
    /// The time zone of this conversational query from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. If not provided, the time zone specified in agent settings is used.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// This field can be used to pass HTTP headers for a webhook call. These headers will be sent to webhook along with the headers that have been configured through the Dialogflow web console. The headers defined within this field will overwrite the headers configured through the Dialogflow console if there is a conflict. Header names are case-insensitive. Google's specified headers are not allowed. Including: "Host", "Content-Length", "Connection", "From", "User-Agent", "Accept-Encoding", "If-Modified-Since", "If-None-Match", "X-Forwarded-For", etc.
    #[serde(rename="webhookHeaders")]
    
    pub webhook_headers: Option<HashMap<String, String>>,
}

impl client::Part for GoogleCloudDialogflowV2QueryParameters {}


/// Represents the result of conversational query or event processing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2QueryResult {
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
    
    pub fulfillment_messages: Option<Vec<GoogleCloudDialogflowV2IntentMessage>>,
    /// The text to be pronounced to the user or shown on the screen. Note: This is a legacy field, `fulfillment_messages` should be preferred.
    #[serde(rename="fulfillmentText")]
    
    pub fulfillment_text: Option<String>,
    /// The intent that matched the conversational query. Some, not all fields are filled in this message, including but not limited to: `name`, `display_name`, `end_interaction` and `is_fallback`.
    
    pub intent: Option<GoogleCloudDialogflowV2Intent>,
    /// The intent detection confidence. Values range from 0.0 (completely uncertain) to 1.0 (completely certain). This value is for informational purpose only and is only used to help match the best intent within the classification threshold. This value may change for the same end-user expression at any time due to a model retraining or change in implementation. If there are `multiple knowledge_answers` messages, this value is set to the greatest `knowledgeAnswers.match_confidence` value in the list.
    #[serde(rename="intentDetectionConfidence")]
    
    pub intent_detection_confidence: Option<f32>,
    /// The language that was triggered during intent detection. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// The collection of output contexts. If applicable, `output_contexts.parameters` contains entries with name `.original` containing the original parameter values before the query.
    #[serde(rename="outputContexts")]
    
    pub output_contexts: Option<Vec<GoogleCloudDialogflowV2Context>>,
    /// The collection of extracted parameters. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value
    
    pub parameters: Option<HashMap<String, json::Value>>,
    /// The original conversational query text: - If natural language text was provided as input, `query_text` contains a copy of the input. - If natural language speech audio was provided as input, `query_text` contains the speech recognition result. If speech recognizer produced multiple alternatives, a particular one is picked. - If automatic spell correction is enabled, `query_text` will contain the corrected user input.
    #[serde(rename="queryText")]
    
    pub query_text: Option<String>,
    /// The sentiment analysis result, which depends on the `sentiment_analysis_request_config` specified in the request.
    #[serde(rename="sentimentAnalysisResult")]
    
    pub sentiment_analysis_result: Option<GoogleCloudDialogflowV2SentimentAnalysisResult>,
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

impl client::Part for GoogleCloudDialogflowV2QueryResult {}


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
pub struct GoogleCloudDialogflowV2ReloadDocumentRequest {
    /// Optional. The path of gcs source file for reloading document content. For now, only gcs uri is supported. For documents stored in Google Cloud Storage, these URIs must have the form `gs:///`.
    #[serde(rename="contentUri")]
    
    pub content_uri: Option<String>,
    /// Optional. Whether to import custom metadata from Google Cloud Storage. Only valid when the document source is Google Cloud Storage URI.
    #[serde(rename="importGcsCustomMetadata")]
    
    pub import_gcs_custom_metadata: Option<bool>,
    /// Optional. When enabled, the reload request is to apply partial update to the smart messaging allowlist.
    #[serde(rename="smartMessagingPartialUpdate")]
    
    pub smart_messaging_partial_update: Option<bool>,
}

impl client::RequestValue for GoogleCloudDialogflowV2ReloadDocumentRequest {}


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
pub struct GoogleCloudDialogflowV2RestoreAgentRequest {
    /// Zip compressed raw byte content for agent.
    #[serde(rename="agentContent")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub agent_content: Option<Vec<u8>>,
    /// The URI to a Google Cloud Storage file containing the agent to restore. Note: The URI must start with "gs://". Dialogflow performs a read operation for the Cloud Storage object on the caller's behalf, so your request authentication must have read permissions for the object. For more information, see [Dialogflow access control](https://cloud.google.com/dialogflow/cx/docs/concept/access-control#storage).
    #[serde(rename="agentUri")]
    
    pub agent_uri: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2RestoreAgentRequest {}


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
pub struct GoogleCloudDialogflowV2SearchAgentsResponse {
    /// The list of agents. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub agents: Option<Vec<GoogleCloudDialogflowV2Agent>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2SearchAgentsResponse {}


/// The sentiment, such as positive/negative feeling or association, for a unit of analysis, such as the query text. See: https://cloud.google.com/natural-language/docs/basics#interpreting_sentiment_analysis_values for how to interpret the result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2Sentiment {
    /// A non-negative number in the [0, +inf) range, which represents the absolute magnitude of sentiment, regardless of score (positive or negative).
    
    pub magnitude: Option<f32>,
    /// Sentiment score between -1.0 (negative sentiment) and 1.0 (positive sentiment).
    
    pub score: Option<f32>,
}

impl client::Part for GoogleCloudDialogflowV2Sentiment {}


/// Configures the types of sentiment analysis to perform.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2SentimentAnalysisRequestConfig {
    /// Instructs the service to perform sentiment analysis on `query_text`. If not provided, sentiment analysis is not performed on `query_text`.
    #[serde(rename="analyzeQueryTextSentiment")]
    
    pub analyze_query_text_sentiment: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowV2SentimentAnalysisRequestConfig {}


/// The result of sentiment analysis. Sentiment analysis inspects user input and identifies the prevailing subjective opinion, especially to determine a user's attitude as positive, negative, or neutral. For Participants.DetectIntent, it needs to be configured in DetectIntentRequest.query_params. For Participants.StreamingDetectIntent, it needs to be configured in StreamingDetectIntentRequest.query_params. And for Participants.AnalyzeContent and Participants.StreamingAnalyzeContent, it needs to be configured in ConversationProfile.human_agent_assistant_config
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2SentimentAnalysisResult {
    /// The sentiment analysis result for `query_text`.
    #[serde(rename="queryTextSentiment")]
    
    pub query_text_sentiment: Option<GoogleCloudDialogflowV2Sentiment>,
}

impl client::Part for GoogleCloudDialogflowV2SentimentAnalysisResult {}


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
pub struct GoogleCloudDialogflowV2SessionEntityType {
    /// Required. The collection of entities associated with this session entity type.
    
    pub entities: Option<Vec<GoogleCloudDialogflowV2EntityTypeEntity>>,
    /// Required. Indicates whether the additional data should override or supplement the custom entity type definition.
    #[serde(rename="entityOverrideMode")]
    
    pub entity_override_mode: Option<String>,
    /// Required. The unique identifier of this session entity type. Format: `projects//agent/sessions//entityTypes/`, or `projects//agent/environments//users//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. `` must be the display name of an existing entity type in the same agent that will be overridden or supplemented.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2SessionEntityType {}
impl client::ResponseResult for GoogleCloudDialogflowV2SessionEntityType {}


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
pub struct GoogleCloudDialogflowV2SetSuggestionFeatureConfigRequest {
    /// Required. The participant role to add or update the suggestion feature config. Only HUMAN_AGENT or END_USER can be used.
    #[serde(rename="participantRole")]
    
    pub participant_role: Option<String>,
    /// Required. The suggestion feature config to add or update.
    #[serde(rename="suggestionFeatureConfig")]
    
    pub suggestion_feature_config: Option<GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionFeatureConfig>,
}

impl client::RequestValue for GoogleCloudDialogflowV2SetSuggestionFeatureConfigRequest {}


/// Represents a smart reply answer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2SmartReplyAnswer {
    /// The name of answer record, in the format of "projects//locations//answerRecords/"
    #[serde(rename="answerRecord")]
    
    pub answer_record: Option<String>,
    /// Smart reply confidence. The system's confidence score that this reply is a good match for this conversation, as a value from 0.0 (completely uncertain) to 1.0 (completely certain).
    
    pub confidence: Option<f32>,
    /// The content of the reply.
    
    pub reply: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2SmartReplyAnswer {}


/// The evaluation metrics for smart reply model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2SmartReplyMetrics {
    /// Percentage of target participant messages in the evaluation dataset for which similar messages have appeared at least once in the allowlist. Should be [0, 1].
    #[serde(rename="allowlistCoverage")]
    
    pub allowlist_coverage: Option<f32>,
    /// Total number of conversations used to generate this metric.
    #[serde(rename="conversationCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub conversation_count: Option<i64>,
    /// Metrics of top n smart replies, sorted by TopNMetric.n.
    #[serde(rename="topNMetrics")]
    
    pub top_n_metrics: Option<Vec<GoogleCloudDialogflowV2SmartReplyMetricsTopNMetrics>>,
}

impl client::Part for GoogleCloudDialogflowV2SmartReplyMetrics {}


/// Evaluation metrics when retrieving `n` smart replies with the model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2SmartReplyMetricsTopNMetrics {
    /// Number of retrieved smart replies. For example, when `n` is 3, this evaluation contains metrics for when Dialogflow retrieves 3 smart replies with the model.
    
    pub n: Option<i32>,
    /// Defined as `number of queries whose top n smart replies have at least one similar (token match similarity above the defined threshold) reply as the real reply` divided by `number of queries with at least one smart reply`. Value ranges from 0.0 to 1.0 inclusive.
    
    pub recall: Option<f32>,
}

impl client::Part for GoogleCloudDialogflowV2SmartReplyMetricsTopNMetrics {}


/// Metadata for smart reply models.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2SmartReplyModelMetadata {
    /// Optional. Type of the smart reply model. If not provided, model_type is used.
    #[serde(rename="trainingModelType")]
    
    pub training_model_type: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2SmartReplyModelMetadata {}


/// Hints for the speech recognizer to help with recognition in a specific conversation state.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2SpeechContext {
    /// Optional. Boost for this context compared to other contexts: * If the boost is positive, Dialogflow will increase the probability that the phrases in this context are recognized over similar sounding phrases. * If the boost is unspecified or non-positive, Dialogflow will not apply any boost. Dialogflow recommends that you use boosts in the range (0, 20] and that you find a value that fits your use case with binary search.
    
    pub boost: Option<f32>,
    /// Optional. A list of strings containing words and phrases that the speech recognizer should recognize with higher likelihood. This list can be used to: * improve accuracy for words and phrases you expect the user to say, e.g. typical commands for your Dialogflow agent * add additional words to the speech recognizer vocabulary * ... See the [Cloud Speech documentation](https://cloud.google.com/speech-to-text/quotas) for usage limits.
    
    pub phrases: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowV2SpeechContext {}


/// Configures speech transcription for ConversationProfile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2SpeechToTextConfig {
    /// Which Speech model to select. Select the model best suited to your domain to get best results. If a model is not explicitly specified, then a default model is used. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics#select-model) for more details.
    
    pub model: Option<String>,
    /// The speech model used in speech to text. `SPEECH_MODEL_VARIANT_UNSPECIFIED`, `USE_BEST_AVAILABLE` will be treated as `USE_ENHANCED`. It can be overridden in AnalyzeContentRequest and StreamingAnalyzeContentRequest request. If enhanced model variant is specified and an enhanced version of the specified model for the language does not exist, then it would emit an error.
    #[serde(rename="speechModelVariant")]
    
    pub speech_model_variant: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2SpeechToTextConfig {}


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
pub struct GoogleCloudDialogflowV2SuggestArticlesRequest {
    /// Parameters for a human assist query.
    #[serde(rename="assistQueryParams")]
    
    pub assist_query_params: Option<GoogleCloudDialogflowV2AssistQueryParameters>,
    /// Optional. Max number of messages prior to and including latest_message to use as context when compiling the suggestion. By default 20 and at most 50.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// Optional. The name of the latest conversation message to compile suggestion for. If empty, it will be the latest message of the conversation. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2SuggestArticlesRequest {}


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
pub struct GoogleCloudDialogflowV2SuggestArticlesResponse {
    /// Articles ordered by score in descending order.
    #[serde(rename="articleAnswers")]
    
    pub article_answers: Option<Vec<GoogleCloudDialogflowV2ArticleAnswer>>,
    /// Number of messages prior to and including latest_message to compile the suggestion. It may be smaller than the SuggestArticlesRequest.context_size field in the request if there aren't that many messages in the conversation.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// The name of the latest conversation message used to compile suggestion for. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2SuggestArticlesResponse {}


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
pub struct GoogleCloudDialogflowV2SuggestConversationSummaryRequest {
    /// Max number of messages prior to and including [latest_message] to use as context when compiling the suggestion. By default 500 and at most 1000.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// The name of the latest conversation message used as context for compiling suggestion. If empty, the latest message of the conversation will be used. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2SuggestConversationSummaryRequest {}


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
pub struct GoogleCloudDialogflowV2SuggestConversationSummaryResponse {
    /// Number of messages prior to and including last_conversation_message used to compile the suggestion. It may be smaller than the SuggestSummaryRequest.context_size field in the request if there weren't that many messages in the conversation.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// The name of the latest conversation message used as context for compiling suggestion. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
    /// Generated summary.
    
    pub summary: Option<GoogleCloudDialogflowV2SuggestConversationSummaryResponseSummary>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2SuggestConversationSummaryResponse {}


/// Generated summary for a conversation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2SuggestConversationSummaryResponseSummary {
    /// The name of the answer record. Format: "projects//answerRecords/"
    #[serde(rename="answerRecord")]
    
    pub answer_record: Option<String>,
    /// The summary content that is concatenated into one string.
    
    pub text: Option<String>,
    /// The summary content that is divided into sections. The key is the section's name and the value is the section's content. There is no specific format for the key or value.
    #[serde(rename="textSections")]
    
    pub text_sections: Option<HashMap<String, String>>,
}

impl client::Part for GoogleCloudDialogflowV2SuggestConversationSummaryResponseSummary {}


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
pub struct GoogleCloudDialogflowV2SuggestFaqAnswersRequest {
    /// Parameters for a human assist query.
    #[serde(rename="assistQueryParams")]
    
    pub assist_query_params: Option<GoogleCloudDialogflowV2AssistQueryParameters>,
    /// Optional. Max number of messages prior to and including [latest_message] to use as context when compiling the suggestion. By default 20 and at most 50.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// Optional. The name of the latest conversation message to compile suggestion for. If empty, it will be the latest message of the conversation. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2SuggestFaqAnswersRequest {}


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
pub struct GoogleCloudDialogflowV2SuggestFaqAnswersResponse {
    /// Number of messages prior to and including latest_message to compile the suggestion. It may be smaller than the SuggestFaqAnswersRequest.context_size field in the request if there aren't that many messages in the conversation.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// Answers extracted from FAQ documents.
    #[serde(rename="faqAnswers")]
    
    pub faq_answers: Option<Vec<GoogleCloudDialogflowV2FaqAnswer>>,
    /// The name of the latest conversation message used to compile suggestion for. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2SuggestFaqAnswersResponse {}


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
pub struct GoogleCloudDialogflowV2SuggestSmartRepliesRequest {
    /// Max number of messages prior to and including [latest_message] to use as context when compiling the suggestion. By default 20 and at most 50.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// The current natural language text segment to compile suggestion for. This provides a way for user to get follow up smart reply suggestion after a smart reply selection, without sending a text message.
    #[serde(rename="currentTextInput")]
    
    pub current_text_input: Option<GoogleCloudDialogflowV2TextInput>,
    /// The name of the latest conversation message to compile suggestion for. If empty, it will be the latest message of the conversation. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowV2SuggestSmartRepliesRequest {}


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
pub struct GoogleCloudDialogflowV2SuggestSmartRepliesResponse {
    /// Number of messages prior to and including latest_message to compile the suggestion. It may be smaller than the SuggestSmartRepliesRequest.context_size field in the request if there aren't that many messages in the conversation.
    #[serde(rename="contextSize")]
    
    pub context_size: Option<i32>,
    /// The name of the latest conversation message used to compile suggestion for. Format: `projects//locations//conversations//messages/`.
    #[serde(rename="latestMessage")]
    
    pub latest_message: Option<String>,
    /// Output only. Multiple reply options provided by smart reply service. The order is based on the rank of the model prediction. The maximum number of the returned replies is set in SmartReplyConfig.
    #[serde(rename="smartReplyAnswers")]
    
    pub smart_reply_answers: Option<Vec<GoogleCloudDialogflowV2SmartReplyAnswer>>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2SuggestSmartRepliesResponse {}


/// The type of Human Agent Assistant API suggestion to perform, and the maximum number of results to return for that type. Multiple `Feature` objects can be specified in the `features` list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2SuggestionFeature {
    /// Type of Human Agent Assistant API feature to request.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2SuggestionFeature {}


/// One response of different type of suggestion response which is used in the response of Participants.AnalyzeContent and Participants.AnalyzeContent, as well as HumanAgentAssistantEvent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2SuggestionResult {
    /// Error status if the request failed.
    
    pub error: Option<GoogleRpcStatus>,
    /// SuggestArticlesResponse if request is for ARTICLE_SUGGESTION.
    #[serde(rename="suggestArticlesResponse")]
    
    pub suggest_articles_response: Option<GoogleCloudDialogflowV2SuggestArticlesResponse>,
    /// SuggestFaqAnswersResponse if request is for FAQ_ANSWER.
    #[serde(rename="suggestFaqAnswersResponse")]
    
    pub suggest_faq_answers_response: Option<GoogleCloudDialogflowV2SuggestFaqAnswersResponse>,
    /// SuggestSmartRepliesResponse if request is for SMART_REPLY.
    #[serde(rename="suggestSmartRepliesResponse")]
    
    pub suggest_smart_replies_response: Option<GoogleCloudDialogflowV2SuggestSmartRepliesResponse>,
}

impl client::Part for GoogleCloudDialogflowV2SuggestionResult {}


/// Configuration of how speech should be synthesized.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2SynthesizeSpeechConfig {
    /// Optional. An identifier which selects 'audio effects' profiles that are applied on (post synthesized) text to speech. Effects are applied on top of each other in the order they are given.
    #[serde(rename="effectsProfileId")]
    
    pub effects_profile_id: Option<Vec<String>>,
    /// Optional. Speaking pitch, in the range [-20.0, 20.0]. 20 means increase 20 semitones from the original pitch. -20 means decrease 20 semitones from the original pitch.
    
    pub pitch: Option<f64>,
    /// Optional. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is the normal native speed supported by the specific voice. 2.0 is twice as fast, and 0.5 is half as fast. If unset(0.0), defaults to the native 1.0 speed. Any other values < 0.25 or > 4.0 will return an error.
    #[serde(rename="speakingRate")]
    
    pub speaking_rate: Option<f64>,
    /// Optional. The desired voice of the synthesized audio.
    
    pub voice: Option<GoogleCloudDialogflowV2VoiceSelectionParams>,
    /// Optional. Volume gain (in dB) of the normal native volume supported by the specific voice, in the range [-96.0, 16.0]. If unset, or set to a value of 0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB) will play at approximately half the amplitude of the normal native signal amplitude. A value of +6.0 (dB) will play at approximately twice the amplitude of the normal native signal amplitude. We strongly recommend not to exceed +10 (dB) as there's usually no effective increase in loudness for any value greater than that.
    #[serde(rename="volumeGainDb")]
    
    pub volume_gain_db: Option<f64>,
}

impl client::Part for GoogleCloudDialogflowV2SynthesizeSpeechConfig {}


/// ============================================================================ Auxiliary proto messages. Represents the natural language text to be processed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2TextInput {
    /// Required. The language of this conversational query. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Required. The UTF-8 encoded natural language text to be processed. Text length must not exceed 256 characters for virtual agent interactions.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2TextInput {}


/// Instructs the speech synthesizer on how to generate the output audio content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2TextToSpeechSettings {
    /// Optional. Indicates whether text to speech is enabled. Even when this field is false, other settings in this proto are still retained.
    #[serde(rename="enableTextToSpeech")]
    
    pub enable_text_to_speech: Option<bool>,
    /// Required. Audio encoding of the synthesized audio content.
    #[serde(rename="outputAudioEncoding")]
    
    pub output_audio_encoding: Option<String>,
    /// Optional. The synthesis sample rate (in hertz) for this audio. If not provided, then the synthesizer will use the default sample rate based on the audio encoding. If this is different from the voice's natural sample rate, then the synthesizer will honor this request by converting to the desired sample rate (which might result in worse audio quality).
    #[serde(rename="sampleRateHertz")]
    
    pub sample_rate_hertz: Option<i32>,
    /// Optional. Configuration of how speech should be synthesized, mapping from language (https://cloud.google.com/dialogflow/docs/reference/language) to SynthesizeSpeechConfig.
    #[serde(rename="synthesizeSpeechConfigs")]
    
    pub synthesize_speech_configs: Option<HashMap<String, GoogleCloudDialogflowV2SynthesizeSpeechConfig>>,
}

impl client::Part for GoogleCloudDialogflowV2TextToSpeechSettings {}


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
pub struct GoogleCloudDialogflowV2TrainAgentRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDialogflowV2TrainAgentRequest {}


/// The request message for ConversationModels.UndeployConversationModel
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [conversation models undeploy projects](ProjectConversationModelUndeployCall) (request)
/// * [locations conversation models undeploy projects](ProjectLocationConversationModelUndeployCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2UndeployConversationModelRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDialogflowV2UndeployConversationModelRequest {}


/// Represents a single validation error.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2ValidationError {
    /// The names of the entries that the error is associated with. Format: - "projects//agent", if the error is associated with the entire agent. - "projects//agent/intents/", if the error is associated with certain intents. - "projects//agent/intents//trainingPhrases/", if the error is associated with certain intent training phrases. - "projects//agent/intents//parameters/", if the error is associated with certain intent parameters. - "projects//agent/entities/", if the error is associated with certain entities.
    
    pub entries: Option<Vec<String>>,
    /// The detailed error message.
    #[serde(rename="errorMessage")]
    
    pub error_message: Option<String>,
    /// The severity of the error.
    
    pub severity: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2ValidationError {}


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
pub struct GoogleCloudDialogflowV2ValidationResult {
    /// Contains all validation errors.
    #[serde(rename="validationErrors")]
    
    pub validation_errors: Option<Vec<GoogleCloudDialogflowV2ValidationError>>,
}

impl client::ResponseResult for GoogleCloudDialogflowV2ValidationResult {}


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
pub struct GoogleCloudDialogflowV2Version {
    /// Output only. The creation time of this version. This field is read-only, i.e., it cannot be set by create and update methods.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The developer-provided description of this version.
    
    pub description: Option<String>,
    /// Output only. The unique identifier of this agent version. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/`
    
    pub name: Option<String>,
    /// Output only. The status of this version. This field is read-only and cannot be set by create and update methods.
    
    pub status: Option<String>,
    /// Output only. The sequential number of this version. This field is read-only which means it cannot be set by create and update methods.
    #[serde(rename="versionNumber")]
    
    pub version_number: Option<i32>,
}

impl client::RequestValue for GoogleCloudDialogflowV2Version {}
impl client::ResponseResult for GoogleCloudDialogflowV2Version {}


/// Description of which voice to use for speech synthesis.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowV2VoiceSelectionParams {
    /// Optional. The name of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and ssml_gender.
    
    pub name: Option<String>,
    /// Optional. The preferred gender of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and name. Note that this is only a preference, not requirement. If a voice of the appropriate gender is not available, the synthesizer should substitute a voice with a different gender rather than failing the request.
    #[serde(rename="ssmlGender")]
    
    pub ssml_gender: Option<String>,
}

impl client::Part for GoogleCloudDialogflowV2VoiceSelectionParams {}


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
/// * [conversation datasets import conversation data projects](ProjectConversationDatasetImportConversationDataCall) (response)
/// * [conversation models create projects](ProjectConversationModelCreateCall) (response)
/// * [conversation models delete projects](ProjectConversationModelDeleteCall) (response)
/// * [conversation models deploy projects](ProjectConversationModelDeployCall) (response)
/// * [conversation models undeploy projects](ProjectConversationModelUndeployCall) (response)
/// * [conversation profiles clear suggestion feature config projects](ProjectConversationProfileClearSuggestionFeatureConfigCall) (response)
/// * [conversation profiles set suggestion feature config projects](ProjectConversationProfileSetSuggestionFeatureConfigCall) (response)
/// * [knowledge bases documents create projects](ProjectKnowledgeBaseDocumentCreateCall) (response)
/// * [knowledge bases documents delete projects](ProjectKnowledgeBaseDocumentDeleteCall) (response)
/// * [knowledge bases documents export projects](ProjectKnowledgeBaseDocumentExportCall) (response)
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
/// * [locations conversation datasets create projects](ProjectLocationConversationDatasetCreateCall) (response)
/// * [locations conversation datasets delete projects](ProjectLocationConversationDatasetDeleteCall) (response)
/// * [locations conversation datasets import conversation data projects](ProjectLocationConversationDatasetImportConversationDataCall) (response)
/// * [locations conversation models evaluations create projects](ProjectLocationConversationModelEvaluationCreateCall) (response)
/// * [locations conversation models create projects](ProjectLocationConversationModelCreateCall) (response)
/// * [locations conversation models delete projects](ProjectLocationConversationModelDeleteCall) (response)
/// * [locations conversation models deploy projects](ProjectLocationConversationModelDeployCall) (response)
/// * [locations conversation models undeploy projects](ProjectLocationConversationModelUndeployCall) (response)
/// * [locations conversation profiles clear suggestion feature config projects](ProjectLocationConversationProfileClearSuggestionFeatureConfigCall) (response)
/// * [locations conversation profiles set suggestion feature config projects](ProjectLocationConversationProfileSetSuggestionFeatureConfigCall) (response)
/// * [locations knowledge bases documents create projects](ProjectLocationKnowledgeBaseDocumentCreateCall) (response)
/// * [locations knowledge bases documents delete projects](ProjectLocationKnowledgeBaseDocumentDeleteCall) (response)
/// * [locations knowledge bases documents export projects](ProjectLocationKnowledgeBaseDocumentExportCall) (response)
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


