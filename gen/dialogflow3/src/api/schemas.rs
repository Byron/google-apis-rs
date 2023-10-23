use super::*;
/// Hierarchical advanced settings for agent/flow/page/fulfillment/parameter. Settings exposed at lower level overrides the settings exposed at higher level. Overriding occurs at the sub-setting level. For example, the playback_interruption_settings at fulfillment level only overrides the playback_interruption_settings at the agent level, leaving other settings at the agent level unchanged. DTMF settings does not override each other. DTMF settings set at different levels define DTMF detections running in parallel. Hierarchy: Agent->Flow->Page->Fulfillment/Parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3AdvancedSettings {
    /// Settings for logging. Settings for Dialogflow History, Contact Center messages, StackDriver logs, and speech logging. Exposed at the following levels: - Agent level.
    #[serde(rename="loggingSettings")]
    
    pub logging_settings: Option<GoogleCloudDialogflowCxV3AdvancedSettingsLoggingSettings>,
}

impl client::Part for GoogleCloudDialogflowCxV3AdvancedSettings {}


/// Define behaviors on logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3AdvancedSettingsLoggingSettings {
    /// If true, DF Interaction logging is currently enabled.
    #[serde(rename="enableInteractionLogging")]
    
    pub enable_interaction_logging: Option<bool>,
    /// If true, StackDriver logging is currently enabled.
    #[serde(rename="enableStackdriverLogging")]
    
    pub enable_stackdriver_logging: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowCxV3AdvancedSettingsLoggingSettings {}


/// Agents are best described as Natural Language Understanding (NLU) modules that transform user requests into actionable data. You can include agents in your app, product, or service to determine user intent and respond to the user in a natural way. After you create an agent, you can add Intents, Entity Types, Flows, Fulfillments, Webhooks, and so on to manage the conversation flows..
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents create projects](ProjectLocationAgentCreateCall) (request|response)
/// * [locations agents get projects](ProjectLocationAgentGetCall) (response)
/// * [locations agents patch projects](ProjectLocationAgentPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3Agent {
    /// Hierarchical advanced settings for this agent. The settings exposed at the lower level overrides the settings exposed at the higher level.
    #[serde(rename="advancedSettings")]
    
    pub advanced_settings: Option<GoogleCloudDialogflowCxV3AdvancedSettings>,
    /// The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration.
    #[serde(rename="avatarUri")]
    
    pub avatar_uri: Option<String>,
    /// Required. Immutable. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the Agents.UpdateAgent method.
    #[serde(rename="defaultLanguageCode")]
    
    pub default_language_code: Option<String>,
    /// The description of the agent. The maximum length is 500 characters. If exceeded, the request is rejected.
    
    pub description: Option<String>,
    /// Required. The human-readable name of the agent, unique within the location.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Indicates if automatic spell correction is enabled in detect intent requests.
    #[serde(rename="enableSpellCorrection")]
    
    pub enable_spell_correction: Option<bool>,
    /// Indicates if stackdriver logging is enabled for the agent. Please use agent.advanced_settings instead.
    #[serde(rename="enableStackdriverLogging")]
    
    pub enable_stackdriver_logging: Option<bool>,
    /// Indicates whether the agent is locked for changes. If the agent is locked, modifications to the agent will be rejected except for RestoreAgent.
    
    pub locked: Option<bool>,
    /// The unique identifier of the agent. Required for the Agents.UpdateAgent method. Agents.CreateAgent populates the name automatically. Format: `projects//locations//agents/`.
    
    pub name: Option<String>,
    /// Name of the SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`.
    #[serde(rename="securitySettings")]
    
    pub security_settings: Option<String>,
    /// Speech recognition related settings.
    #[serde(rename="speechToTextSettings")]
    
    pub speech_to_text_settings: Option<GoogleCloudDialogflowCxV3SpeechToTextSettings>,
    /// Immutable. Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//flows/`.
    #[serde(rename="startFlow")]
    
    pub start_flow: Option<String>,
    /// The list of all languages supported by the agent (except for the `default_language_code`).
    #[serde(rename="supportedLanguageCodes")]
    
    pub supported_language_codes: Option<Vec<String>>,
    /// Required. The time zone of the agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3Agent {}
impl client::ResponseResult for GoogleCloudDialogflowCxV3Agent {}


/// The response message for Agents.GetAgentValidationResult.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents get validation result projects](ProjectLocationAgentGetValidationResultCall) (response)
/// * [locations agents validate projects](ProjectLocationAgentValidateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3AgentValidationResult {
    /// Contains all flow validation results.
    #[serde(rename="flowValidationResults")]
    
    pub flow_validation_results: Option<Vec<GoogleCloudDialogflowCxV3FlowValidationResult>>,
    /// The unique identifier of the agent validation result. Format: `projects//locations//agents//validationResult`.
    
    pub name: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3AgentValidationResult {}


/// Represents the natural speech audio to be processed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3AudioInput {
    /// The natural language speech audio to be processed. A single request can contain up to 1 minute of speech audio data. The transcribed text cannot contain more than 256 bytes. For non-streaming audio detect intent, both `config` and `audio` must be provided. For streaming audio detect intent, `config` must be provided in the first request and `audio` must be provided in all following requests.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub audio: Option<Vec<u8>>,
    /// Required. Instructs the speech recognizer how to process the speech audio.
    
    pub config: Option<GoogleCloudDialogflowCxV3InputAudioConfig>,
}

impl client::Part for GoogleCloudDialogflowCxV3AudioInput {}


/// The request message for TestCases.BatchDeleteTestCases.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents test cases batch delete projects](ProjectLocationAgentTestCaseBatchDeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3BatchDeleteTestCasesRequest {
    /// Required. Format of test case names: `projects//locations/ /agents//testCases/`.
    
    pub names: Option<Vec<String>>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3BatchDeleteTestCasesRequest {}


/// The request message for TestCases.BatchRunTestCases.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents test cases batch run projects](ProjectLocationAgentTestCaseBatchRunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3BatchRunTestCasesRequest {
    /// Optional. If not set, draft environment is assumed. Format: `projects//locations//agents//environments/`.
    
    pub environment: Option<String>,
    /// Required. Format: `projects//locations//agents//testCases/`.
    #[serde(rename="testCases")]
    
    pub test_cases: Option<Vec<String>>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3BatchRunTestCasesRequest {}


/// The response message for TestCases.CalculateCoverage.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents test cases calculate coverage projects](ProjectLocationAgentTestCaseCalculateCoverageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3CalculateCoverageResponse {
    /// The agent to calculate coverage for. Format: `projects//locations//agents/`.
    
    pub agent: Option<String>,
    /// Intent coverage.
    #[serde(rename="intentCoverage")]
    
    pub intent_coverage: Option<GoogleCloudDialogflowCxV3IntentCoverage>,
    /// Transition route group coverage.
    #[serde(rename="routeGroupCoverage")]
    
    pub route_group_coverage: Option<GoogleCloudDialogflowCxV3TransitionRouteGroupCoverage>,
    /// Transition (excluding transition route groups) coverage.
    #[serde(rename="transitionCoverage")]
    
    pub transition_coverage: Option<GoogleCloudDialogflowCxV3TransitionCoverage>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3CalculateCoverageResponse {}


/// Changelogs represents a change made to a given agent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents changelogs get projects](ProjectLocationAgentChangelogGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3Changelog {
    /// The action of the change.
    
    pub action: Option<String>,
    /// The timestamp of the change.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The affected resource display name of the change.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The unique identifier of the changelog. Format: `projects//locations//agents//changelogs/`.
    
    pub name: Option<String>,
    /// The affected resource name of the change.
    
    pub resource: Option<String>,
    /// The affected resource type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Email address of the authenticated user.
    #[serde(rename="userEmail")]
    
    pub user_email: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3Changelog {}


/// The request message for Versions.CompareVersions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows versions compare versions projects](ProjectLocationAgentFlowVersionCompareVersionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3CompareVersionsRequest {
    /// The language to compare the flow versions for. If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Required. Name of the target flow version to compare with the base version. Use version ID `0` to indicate the draft version of the specified flow. Format: `projects//locations//agents//flows//versions/`.
    #[serde(rename="targetVersion")]
    
    pub target_version: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3CompareVersionsRequest {}


/// The response message for Versions.CompareVersions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows versions compare versions projects](ProjectLocationAgentFlowVersionCompareVersionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3CompareVersionsResponse {
    /// JSON representation of the base version content.
    #[serde(rename="baseVersionContentJson")]
    
    pub base_version_content_json: Option<String>,
    /// The timestamp when the two version compares.
    #[serde(rename="compareTime")]
    
    pub compare_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// JSON representation of the target version content.
    #[serde(rename="targetVersionContentJson")]
    
    pub target_version_content_json: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3CompareVersionsResponse {}


/// Represents a result from running a test case in an agent environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ContinuousTestResult {
    /// The resource name for the continuous test result. Format: `projects//locations//agents//environments//continuousTestResults/`.
    
    pub name: Option<String>,
    /// The result of this continuous test run, i.e. whether all the tests in this continuous test run pass or not.
    
    pub result: Option<GoogleCloudDialogflowCxV3ContinuousTestResultResultEnum>,
    /// Time when the continuous testing run starts.
    #[serde(rename="runTime")]
    
    pub run_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A list of individual test case results names in this continuous test run.
    #[serde(rename="testCaseResults")]
    
    pub test_case_results: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowCxV3ContinuousTestResult {}


/// One interaction between a human and virtual agent. The human provides some input and the virtual agent provides a response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ConversationTurn {
    /// The user input.
    #[serde(rename="userInput")]
    
    pub user_input: Option<GoogleCloudDialogflowCxV3ConversationTurnUserInput>,
    /// The virtual agent output.
    #[serde(rename="virtualAgentOutput")]
    
    pub virtual_agent_output: Option<GoogleCloudDialogflowCxV3ConversationTurnVirtualAgentOutput>,
}

impl client::Part for GoogleCloudDialogflowCxV3ConversationTurn {}


/// The input from the human user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ConversationTurnUserInput {
    /// Whether sentiment analysis is enabled.
    #[serde(rename="enableSentimentAnalysis")]
    
    pub enable_sentiment_analysis: Option<bool>,
    /// Parameters that need to be injected into the conversation during intent detection.
    #[serde(rename="injectedParameters")]
    
    pub injected_parameters: Option<HashMap<String, json::Value>>,
    /// Supports text input, event input, dtmf input in the test case.
    
    pub input: Option<GoogleCloudDialogflowCxV3QueryInput>,
    /// If webhooks should be allowed to trigger in response to the user utterance. Often if parameters are injected, webhooks should not be enabled.
    #[serde(rename="isWebhookEnabled")]
    
    pub is_webhook_enabled: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowCxV3ConversationTurnUserInput {}


/// The output from the virtual agent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ConversationTurnVirtualAgentOutput {
    /// The Page on which the utterance was spoken. Only name and displayName will be set.
    #[serde(rename="currentPage")]
    
    pub current_page: Option<GoogleCloudDialogflowCxV3Page>,
    /// Required. Input only. The diagnostic info output for the turn. Required to calculate the testing coverage.
    #[serde(rename="diagnosticInfo")]
    
    pub diagnostic_info: Option<HashMap<String, json::Value>>,
    /// Output only. If this is part of a result conversation turn, the list of differences between the original run and the replay for this output, if any.
    
    pub differences: Option<Vec<GoogleCloudDialogflowCxV3TestRunDifference>>,
    /// The session parameters available to the bot at this point.
    #[serde(rename="sessionParameters")]
    
    pub session_parameters: Option<HashMap<String, json::Value>>,
    /// Response error from the agent in the test result. If set, other output is empty.
    
    pub status: Option<GoogleRpcStatus>,
    /// The text responses from the agent for the turn.
    #[serde(rename="textResponses")]
    
    pub text_responses: Option<Vec<GoogleCloudDialogflowCxV3ResponseMessageText>>,
    /// The Intent that triggered the response. Only name and displayName will be set.
    #[serde(rename="triggeredIntent")]
    
    pub triggered_intent: Option<GoogleCloudDialogflowCxV3Intent>,
}

impl client::Part for GoogleCloudDialogflowCxV3ConversationTurnVirtualAgentOutput {}


/// The request message for Environments.DeployFlow.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments deploy flow projects](ProjectLocationAgentEnvironmentDeployFlowCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3DeployFlowRequest {
    /// Required. The flow version to deploy. Format: `projects//locations//agents// flows//versions/`.
    #[serde(rename="flowVersion")]
    
    pub flow_version: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3DeployFlowRequest {}


/// Represents an deployment in an environment. A deployment happens when a flow version configured to be active in the environment. You can configure running pre-deployment steps, e.g. running validation test cases, experiment auto-rollout, etc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments deployments get projects](ProjectLocationAgentEnvironmentDeploymentGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3Deployment {
    /// End time of this deployment.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The name of the flow version for this deployment. Format: projects//locations//agents//flows//versions/.
    #[serde(rename="flowVersion")]
    
    pub flow_version: Option<String>,
    /// The name of the deployment. Format: projects//locations//agents//environments//deployments/.
    
    pub name: Option<String>,
    /// Result of the deployment.
    
    pub result: Option<GoogleCloudDialogflowCxV3DeploymentResult>,
    /// Start time of this deployment.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The current state of the deployment.
    
    pub state: Option<GoogleCloudDialogflowCxV3DeploymentStateEnum>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3Deployment {}


/// Result of the deployment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3DeploymentResult {
    /// Results of test cases running before the deployment. Format: `projects//locations//agents//testCases//results/`.
    #[serde(rename="deploymentTestResults")]
    
    pub deployment_test_results: Option<Vec<String>>,
    /// The name of the experiment triggered by this deployment. Format: projects//locations//agents//environments//experiments/.
    
    pub experiment: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3DeploymentResult {}


/// The request to detect user’s intent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments sessions detect intent projects](ProjectLocationAgentEnvironmentSessionDetectIntentCall) (request)
/// * [locations agents sessions detect intent projects](ProjectLocationAgentSessionDetectIntentCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3DetectIntentRequest {
    /// Instructs the speech synthesizer how to generate the output audio.
    #[serde(rename="outputAudioConfig")]
    
    pub output_audio_config: Option<GoogleCloudDialogflowCxV3OutputAudioConfig>,
    /// Required. The input specification.
    #[serde(rename="queryInput")]
    
    pub query_input: Option<GoogleCloudDialogflowCxV3QueryInput>,
    /// The parameters of this query.
    #[serde(rename="queryParams")]
    
    pub query_params: Option<GoogleCloudDialogflowCxV3QueryParameters>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3DetectIntentRequest {}


/// The message returned from the DetectIntent method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments sessions detect intent projects](ProjectLocationAgentEnvironmentSessionDetectIntentCall) (response)
/// * [locations agents sessions detect intent projects](ProjectLocationAgentSessionDetectIntentCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3DetectIntentResponse {
    /// Indicates whether the partial response can be cancelled when a later response arrives. e.g. if the agent specified some music as partial response, it can be cancelled.
    #[serde(rename="allowCancellation")]
    
    pub allow_cancellation: Option<bool>,
    /// The audio data bytes encoded as specified in the request. Note: The output audio is generated based on the values of default platform text responses found in the `query_result.response_messages` field. If multiple default text responses exist, they will be concatenated when generating audio. If no default platform text responses exist, the generated audio content will be empty. In some scenarios, multiple output audio fields may be present in the response structure. In these cases, only the top-most-level audio output has content.
    #[serde(rename="outputAudio")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub output_audio: Option<Vec<u8>>,
    /// The config used by the speech synthesizer to generate the output audio.
    #[serde(rename="outputAudioConfig")]
    
    pub output_audio_config: Option<GoogleCloudDialogflowCxV3OutputAudioConfig>,
    /// The result of the conversational query.
    #[serde(rename="queryResult")]
    
    pub query_result: Option<GoogleCloudDialogflowCxV3QueryResult>,
    /// Output only. The unique identifier of the response. It can be used to locate a response in the training example set or for reporting issues.
    #[serde(rename="responseId")]
    
    pub response_id: Option<String>,
    /// Response type.
    #[serde(rename="responseType")]
    
    pub response_type: Option<GoogleCloudDialogflowCxV3DetectIntentResponseResponseTypeEnum>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3DetectIntentResponse {}


/// Represents the input for dtmf event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3DtmfInput {
    /// The dtmf digits.
    
    pub digits: Option<String>,
    /// The finish digit (if any).
    #[serde(rename="finishDigit")]
    
    pub finish_digit: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3DtmfInput {}


/// Entities are extracted from user input and represent parameters that are meaningful to your application. For example, a date range, a proper name such as a geographic location or landmark, and so on. Entities represent actionable data for your application. When you define an entity, you can also include synonyms that all map to that entity. For example, “soft drink”, “soda”, “pop”, and so on. There are three types of entities: * **System** - entities that are defined by the Dialogflow API for common data types such as date, time, currency, and so on. A system entity is represented by the `EntityType` type. * **Custom** - entities that are defined by you that represent actionable data that is meaningful to your application. For example, you could define a `pizza.sauce` entity for red or white pizza sauce, a `pizza.cheese` entity for the different types of cheese on a pizza, a `pizza.topping` entity for different toppings, and so on. A custom entity is represented by the `EntityType` type. * **User** - entities that are built for an individual user such as favorites, preferences, playlists, and so on. A user entity is represented by the SessionEntityType type. For more information about entity types, see the [Dialogflow documentation](https://cloud.google.com/dialogflow/docs/entities-overview).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents entity types create projects](ProjectLocationAgentEntityTypeCreateCall) (request|response)
/// * [locations agents entity types get projects](ProjectLocationAgentEntityTypeGetCall) (response)
/// * [locations agents entity types patch projects](ProjectLocationAgentEntityTypePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3EntityType {
    /// Indicates whether the entity type can be automatically expanded.
    #[serde(rename="autoExpansionMode")]
    
    pub auto_expansion_mode: Option<GoogleCloudDialogflowCxV3EntityTypeAutoExpansionModeEnum>,
    /// Required. The human-readable name of the entity type, unique within the agent.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Enables fuzzy entity extraction during classification.
    #[serde(rename="enableFuzzyExtraction")]
    
    pub enable_fuzzy_extraction: Option<bool>,
    /// The collection of entity entries associated with the entity type.
    
    pub entities: Option<Vec<GoogleCloudDialogflowCxV3EntityTypeEntity>>,
    /// Collection of exceptional words and phrases that shouldn't be matched. For example, if you have a size entity type with entry `giant`(an adjective), you might consider adding `giants`(a noun) as an exclusion. If the kind of entity type is `KIND_MAP`, then the phrases specified by entities and excluded phrases should be mutually exclusive.
    #[serde(rename="excludedPhrases")]
    
    pub excluded_phrases: Option<Vec<GoogleCloudDialogflowCxV3EntityTypeExcludedPhrase>>,
    /// Required. Indicates the kind of entity type.
    
    pub kind: Option<GoogleCloudDialogflowCxV3EntityTypeKindEnum>,
    /// The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType. Format: `projects//locations//agents//entityTypes/`.
    
    pub name: Option<String>,
    /// Indicates whether parameters of the entity type should be redacted in log. If redaction is enabled, page parameters and intent parameters referring to the entity type will be replaced by parameter name when logging.
    
    pub redact: Option<bool>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3EntityType {}
impl client::ResponseResult for GoogleCloudDialogflowCxV3EntityType {}


/// An **entity entry** for an associated entity type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3EntityTypeEntity {
    /// Required. A collection of value synonyms. For example, if the entity type is *vegetable*, and `value` is *scallions*, a synonym could be *green onions*. For `KIND_LIST` entity types: * This collection must contain exactly one synonym equal to `value`.
    
    pub synonyms: Option<Vec<String>>,
    /// Required. The primary value associated with this entity entry. For example, if the entity type is *vegetable*, the value could be *scallions*. For `KIND_MAP` entity types: * A canonical value to be used in place of synonyms. For `KIND_LIST` entity types: * A string that can contain references to other entity types (with or without aliases).
    
    pub value: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3EntityTypeEntity {}


/// An excluded entity phrase that should not be matched.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3EntityTypeExcludedPhrase {
    /// Required. The word or phrase to be excluded.
    
    pub value: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3EntityTypeExcludedPhrase {}


/// Represents an environment for an agent. You can create multiple versions of your agent and publish them to separate environments. When you edit an agent, you are editing the draft agent. At any point, you can save the draft agent as an agent version, which is an immutable snapshot of your agent. When you save the draft agent, it is published to the default environment. When you create agent versions, you can publish them to custom environments. You can create a variety of custom environments for testing, development, production, etc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments create projects](ProjectLocationAgentEnvironmentCreateCall) (request)
/// * [locations agents environments get projects](ProjectLocationAgentEnvironmentGetCall) (response)
/// * [locations agents environments patch projects](ProjectLocationAgentEnvironmentPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3Environment {
    /// The human-readable description of the environment. The maximum length is 500 characters. If exceeded, the request is rejected.
    
    pub description: Option<String>,
    /// Required. The human-readable name of the environment (unique in an agent). Limit of 64 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The name of the environment. Format: `projects//locations//agents//environments/`.
    
    pub name: Option<String>,
    /// The test cases config for continuous tests of this environment.
    #[serde(rename="testCasesConfig")]
    
    pub test_cases_config: Option<GoogleCloudDialogflowCxV3EnvironmentTestCasesConfig>,
    /// Output only. Update time of this environment.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. A list of configurations for flow versions. You should include version configs for all flows that are reachable from `Start Flow` in the agent. Otherwise, an error will be returned.
    #[serde(rename="versionConfigs")]
    
    pub version_configs: Option<Vec<GoogleCloudDialogflowCxV3EnvironmentVersionConfig>>,
    /// The webhook configuration for this environment.
    #[serde(rename="webhookConfig")]
    
    pub webhook_config: Option<GoogleCloudDialogflowCxV3EnvironmentWebhookConfig>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3Environment {}
impl client::ResponseResult for GoogleCloudDialogflowCxV3Environment {}


/// The configuration for continuous tests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3EnvironmentTestCasesConfig {
    /// Whether to run test cases in TestCasesConfig.test_cases periodically. Default false. If set to true, run once a day.
    #[serde(rename="enableContinuousRun")]
    
    pub enable_continuous_run: Option<bool>,
    /// Whether to run test cases in TestCasesConfig.test_cases before deploying a flow version to the environment. Default false.
    #[serde(rename="enablePredeploymentRun")]
    
    pub enable_predeployment_run: Option<bool>,
    /// A list of test case names to run. They should be under the same agent. Format of each test case name: `projects//locations/ /agents//testCases/`
    #[serde(rename="testCases")]
    
    pub test_cases: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowCxV3EnvironmentTestCasesConfig {}


/// Configuration for the version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3EnvironmentVersionConfig {
    /// Required. Format: projects//locations//agents//flows//versions/.
    
    pub version: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3EnvironmentVersionConfig {}


/// Configuration for webhooks.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3EnvironmentWebhookConfig {
    /// The list of webhooks to override for the agent environment. The webhook must exist in the agent. You can override fields in `generic_web_service` and `service_directory`.
    #[serde(rename="webhookOverrides")]
    
    pub webhook_overrides: Option<Vec<GoogleCloudDialogflowCxV3Webhook>>,
}

impl client::Part for GoogleCloudDialogflowCxV3EnvironmentWebhookConfig {}


/// An event handler specifies an event that can be handled during a session. When the specified event happens, the following actions are taken in order: * If there is a `trigger_fulfillment` associated with the event, it will be called. * If there is a `target_page` associated with the event, the session will transition into the specified page. * If there is a `target_flow` associated with the event, the session will transition into the specified flow.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3EventHandler {
    /// Required. The name of the event to handle.
    
    pub event: Option<String>,
    /// Output only. The unique identifier of this event handler.
    
    pub name: Option<String>,
    /// The target flow to transition to. Format: `projects//locations//agents//flows/`.
    #[serde(rename="targetFlow")]
    
    pub target_flow: Option<String>,
    /// The target page to transition to. Format: `projects//locations//agents//flows//pages/`.
    #[serde(rename="targetPage")]
    
    pub target_page: Option<String>,
    /// The fulfillment to call when the event occurs. Handling webhook errors with a fulfillment enabled with webhook could cause infinite loop. It is invalid to specify such fulfillment for a handler handling webhooks.
    #[serde(rename="triggerFulfillment")]
    
    pub trigger_fulfillment: Option<GoogleCloudDialogflowCxV3Fulfillment>,
}

impl client::Part for GoogleCloudDialogflowCxV3EventHandler {}


/// Represents the event to trigger.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3EventInput {
    /// Name of the event.
    
    pub event: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3EventInput {}


/// Represents an experiment in an environment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments experiments create projects](ProjectLocationAgentEnvironmentExperimentCreateCall) (request|response)
/// * [locations agents environments experiments get projects](ProjectLocationAgentEnvironmentExperimentGetCall) (response)
/// * [locations agents environments experiments patch projects](ProjectLocationAgentEnvironmentExperimentPatchCall) (request|response)
/// * [locations agents environments experiments start projects](ProjectLocationAgentEnvironmentExperimentStartCall) (response)
/// * [locations agents environments experiments stop projects](ProjectLocationAgentEnvironmentExperimentStopCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3Experiment {
    /// Creation time of this experiment.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The definition of the experiment.
    
    pub definition: Option<GoogleCloudDialogflowCxV3ExperimentDefinition>,
    /// The human-readable description of the experiment.
    
    pub description: Option<String>,
    /// Required. The human-readable name of the experiment (unique in an environment). Limit of 64 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// End time of this experiment.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Maximum number of days to run the experiment/rollout. If auto-rollout is not enabled, default value and maximum will be 30 days. If auto-rollout is enabled, default value and maximum will be 6 days.
    #[serde(rename="experimentLength")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub experiment_length: Option<client::chrono::Duration>,
    /// Last update time of this experiment.
    #[serde(rename="lastUpdateTime")]
    
    pub last_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The name of the experiment. Format: projects//locations//agents//environments//experiments/..
    
    pub name: Option<String>,
    /// Inference result of the experiment.
    
    pub result: Option<GoogleCloudDialogflowCxV3ExperimentResult>,
    /// The configuration for auto rollout. If set, there should be exactly two variants in the experiment (control variant being the default version of the flow), the traffic allocation for the non-control variant will gradually increase to 100% when conditions are met, and eventually replace the control variant to become the default version of the flow.
    #[serde(rename="rolloutConfig")]
    
    pub rollout_config: Option<GoogleCloudDialogflowCxV3RolloutConfig>,
    /// The reason why rollout has failed. Should only be set when state is ROLLOUT_FAILED.
    #[serde(rename="rolloutFailureReason")]
    
    pub rollout_failure_reason: Option<String>,
    /// State of the auto rollout process.
    #[serde(rename="rolloutState")]
    
    pub rollout_state: Option<GoogleCloudDialogflowCxV3RolloutState>,
    /// Start time of this experiment.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The current state of the experiment. Transition triggered by Experiments.StartExperiment: DRAFT->RUNNING. Transition triggered by Experiments.CancelExperiment: DRAFT->DONE or RUNNING->DONE.
    
    pub state: Option<GoogleCloudDialogflowCxV3ExperimentStateEnum>,
    /// The history of updates to the experiment variants.
    #[serde(rename="variantsHistory")]
    
    pub variants_history: Option<Vec<GoogleCloudDialogflowCxV3VariantsHistory>>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3Experiment {}
impl client::ResponseResult for GoogleCloudDialogflowCxV3Experiment {}


/// Definition of the experiment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ExperimentDefinition {
    /// The condition defines which subset of sessions are selected for this experiment. If not specified, all sessions are eligible. E.g. "query_input.language_code=en" See the [conditions reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition).
    
    pub condition: Option<String>,
    /// The flow versions as the variants of this experiment.
    #[serde(rename="versionVariants")]
    
    pub version_variants: Option<GoogleCloudDialogflowCxV3VersionVariants>,
}

impl client::Part for GoogleCloudDialogflowCxV3ExperimentDefinition {}


/// The inference result which includes an objective metric to optimize and the confidence interval.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ExperimentResult {
    /// The last time the experiment's stats data was updated. Will have default value if stats have never been computed for this experiment.
    #[serde(rename="lastUpdateTime")]
    
    pub last_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Version variants and metrics.
    #[serde(rename="versionMetrics")]
    
    pub version_metrics: Option<Vec<GoogleCloudDialogflowCxV3ExperimentResultVersionMetrics>>,
}

impl client::Part for GoogleCloudDialogflowCxV3ExperimentResult {}


/// A confidence interval is a range of possible values for the experiment objective you are trying to measure.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ExperimentResultConfidenceInterval {
    /// The confidence level used to construct the interval, i.e. there is X% chance that the true value is within this interval.
    #[serde(rename="confidenceLevel")]
    
    pub confidence_level: Option<f64>,
    /// Lower bound of the interval.
    #[serde(rename="lowerBound")]
    
    pub lower_bound: Option<f64>,
    /// The percent change between an experiment metric's value and the value for its control.
    
    pub ratio: Option<f64>,
    /// Upper bound of the interval.
    #[serde(rename="upperBound")]
    
    pub upper_bound: Option<f64>,
}

impl client::Part for GoogleCloudDialogflowCxV3ExperimentResultConfidenceInterval {}


/// Metric and corresponding confidence intervals.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ExperimentResultMetric {
    /// The probability that the treatment is better than all other treatments in the experiment
    #[serde(rename="confidenceInterval")]
    
    pub confidence_interval: Option<GoogleCloudDialogflowCxV3ExperimentResultConfidenceInterval>,
    /// Count value of a metric.
    
    pub count: Option<f64>,
    /// Count-based metric type. Only one of type or count_type is specified in each Metric.
    #[serde(rename="countType")]
    
    pub count_type: Option<GoogleCloudDialogflowCxV3ExperimentResultMetricCountTypeEnum>,
    /// Ratio value of a metric.
    
    pub ratio: Option<f64>,
    /// Ratio-based metric type. Only one of type or count_type is specified in each Metric.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum>,
}

impl client::Part for GoogleCloudDialogflowCxV3ExperimentResultMetric {}


/// Version variant and associated metrics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ExperimentResultVersionMetrics {
    /// The metrics and corresponding confidence intervals in the inference result.
    
    pub metrics: Option<Vec<GoogleCloudDialogflowCxV3ExperimentResultMetric>>,
    /// Number of sessions that were allocated to this version.
    #[serde(rename="sessionCount")]
    
    pub session_count: Option<i32>,
    /// The name of the flow Version. Format: `projects//locations//agents//flows//versions/`.
    
    pub version: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3ExperimentResultVersionMetrics {}


/// The request message for Agents.ExportAgent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents export projects](ProjectLocationAgentExportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ExportAgentRequest {
    /// Optional. The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to export the agent to. The format of this URI must be `gs:///`. If left unspecified, the serialized agent is returned inline. Dialogflow performs a write operation for the Cloud Storage object on the caller's behalf, so your request authentication must have write permissions for the object. For more information, see [Dialogflow access control](https://cloud.google.com/dialogflow/cx/docs/concept/access-control#storage).
    #[serde(rename="agentUri")]
    
    pub agent_uri: Option<String>,
    /// Optional. The data format of the exported agent. If not specified, `BLOB` is assumed.
    #[serde(rename="dataFormat")]
    
    pub data_format: Option<GoogleCloudDialogflowCxV3ExportAgentRequestDataFormatEnum>,
    /// Optional. Environment name. If not set, draft environment is assumed. Format: `projects//locations//agents//environments/`.
    
    pub environment: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3ExportAgentRequest {}


/// The request message for Flows.ExportFlow.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows export projects](ProjectLocationAgentFlowExportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ExportFlowRequest {
    /// Optional. The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to export the flow to. The format of this URI must be `gs:///`. If left unspecified, the serialized flow is returned inline. Dialogflow performs a write operation for the Cloud Storage object on the caller's behalf, so your request authentication must have write permissions for the object. For more information, see [Dialogflow access control](https://cloud.google.com/dialogflow/cx/docs/concept/access-control#storage).
    #[serde(rename="flowUri")]
    
    pub flow_uri: Option<String>,
    /// Optional. Whether to export flows referenced by the specified flow.
    #[serde(rename="includeReferencedFlows")]
    
    pub include_referenced_flows: Option<bool>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3ExportFlowRequest {}


/// The request message for TestCases.ExportTestCases.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents test cases export projects](ProjectLocationAgentTestCaseExportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ExportTestCasesRequest {
    /// The data format of the exported test cases. If not specified, `BLOB` is assumed.
    #[serde(rename="dataFormat")]
    
    pub data_format: Option<GoogleCloudDialogflowCxV3ExportTestCasesRequestDataFormatEnum>,
    /// The filter expression used to filter exported test cases, see [API Filtering](https://aip.dev/160). The expression is case insensitive and supports the following syntax: name = [OR name = ] ... For example: * "name = t1 OR name = t2" matches the test case with the exact resource name "t1" or "t2".
    
    pub filter: Option<String>,
    /// The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to export the test cases to. The format of this URI must be `gs:///`. If unspecified, the serialized test cases is returned inline. Dialogflow performs a write operation for the Cloud Storage object on the caller's behalf, so your request authentication must have write permissions for the object. For more information, see [Dialogflow access control](https://cloud.google.com/dialogflow/cx/docs/concept/access-control#storage).
    #[serde(rename="gcsUri")]
    
    pub gcs_uri: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3ExportTestCasesRequest {}


/// Flows represents the conversation flows when you build your chatbot agent. A flow consists of many pages connected by the transition routes. Conversations always start with the built-in Start Flow (with an all-0 ID). Transition routes can direct the conversation session from the current flow (parent flow) to another flow (sub flow). When the sub flow is finished, Dialogflow will bring the session back to the parent flow, where the sub flow is started. Usually, when a transition route is followed by a matched intent, the intent will be “consumed”. This means the intent won’t activate more transition routes. However, when the followed transition route moves the conversation session into a different flow, the matched intent can be carried over and to be consumed in the target flow.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows create projects](ProjectLocationAgentFlowCreateCall) (request|response)
/// * [locations agents flows get projects](ProjectLocationAgentFlowGetCall) (response)
/// * [locations agents flows patch projects](ProjectLocationAgentFlowPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3Flow {
    /// The description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected.
    
    pub description: Option<String>,
    /// Required. The human-readable name of the flow.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// A flow's event handlers serve two purposes: * They are responsible for handling events (e.g. no match, webhook errors) in the flow. * They are inherited by every page's event handlers, which can be used to handle common events regardless of the current page. Event handlers defined in the page have higher priority than those defined in the flow. Unlike transition_routes, these handlers are evaluated on a first-match basis. The first one that matches the event get executed, with the rest being ignored.
    #[serde(rename="eventHandlers")]
    
    pub event_handlers: Option<Vec<GoogleCloudDialogflowCxV3EventHandler>>,
    /// The unique identifier of the flow. Format: `projects//locations//agents//flows/`.
    
    pub name: Option<String>,
    /// NLU related settings of the flow.
    #[serde(rename="nluSettings")]
    
    pub nlu_settings: Option<GoogleCloudDialogflowCxV3NluSettings>,
    /// A flow's transition route group serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition route groups. Transition route groups defined in the page have higher priority than those defined in the flow. Format:`projects//locations//agents//flows//transitionRouteGroups/`.
    #[serde(rename="transitionRouteGroups")]
    
    pub transition_route_groups: Option<Vec<String>>,
    /// A flow's transition routes serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition routes and can support use cases such as the user saying "help" or "can I talk to a human?", which can be handled in a common way regardless of the current page. Transition routes defined in the page have higher priority than those defined in the flow. TransitionRoutes are evalauted in the following order: * TransitionRoutes with intent specified. * TransitionRoutes with only condition specified. TransitionRoutes with intent specified are inherited by pages in the flow.
    #[serde(rename="transitionRoutes")]
    
    pub transition_routes: Option<Vec<GoogleCloudDialogflowCxV3TransitionRoute>>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3Flow {}
impl client::ResponseResult for GoogleCloudDialogflowCxV3Flow {}


/// The response message for Flows.GetFlowValidationResult.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows get validation result projects](ProjectLocationAgentFlowGetValidationResultCall) (response)
/// * [locations agents flows validate projects](ProjectLocationAgentFlowValidateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3FlowValidationResult {
    /// The unique identifier of the flow validation result. Format: `projects//locations//agents//flows//validationResult`.
    
    pub name: Option<String>,
    /// Last time the flow was validated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Contains all validation messages.
    #[serde(rename="validationMessages")]
    
    pub validation_messages: Option<Vec<GoogleCloudDialogflowCxV3ValidationMessage>>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3FlowValidationResult {}


/// A form is a data model that groups related parameters that can be collected from the user. The process in which the agent prompts the user and collects parameter values from the user is called form filling. A form can be added to a page. When form filling is done, the filled parameters will be written to the session.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3Form {
    /// Parameters to collect from the user.
    
    pub parameters: Option<Vec<GoogleCloudDialogflowCxV3FormParameter>>,
}

impl client::Part for GoogleCloudDialogflowCxV3Form {}


/// Represents a form parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3FormParameter {
    /// The default value of an optional parameter. If the parameter is required, the default value will be ignored.
    #[serde(rename="defaultValue")]
    
    pub default_value: Option<json::Value>,
    /// Required. The human-readable name of the parameter, unique within the form.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. The entity type of the parameter. Format: `projects/-/locations/-/agents/-/entityTypes/` for system entity types (for example, `projects/-/locations/-/agents/-/entityTypes/sys.date`), or `projects//locations//agents//entityTypes/` for developer entity types.
    #[serde(rename="entityType")]
    
    pub entity_type: Option<String>,
    /// Required. Defines fill behavior for the parameter.
    #[serde(rename="fillBehavior")]
    
    pub fill_behavior: Option<GoogleCloudDialogflowCxV3FormParameterFillBehavior>,
    /// Indicates whether the parameter represents a list of values.
    #[serde(rename="isList")]
    
    pub is_list: Option<bool>,
    /// Indicates whether the parameter content should be redacted in log. If redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled.
    
    pub redact: Option<bool>,
    /// Indicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them. Required parameters must be filled before form filling concludes.
    
    pub required: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowCxV3FormParameter {}


/// Configuration for how the filling of a parameter should be handled.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3FormParameterFillBehavior {
    /// Required. The fulfillment to provide the initial prompt that the agent can present to the user in order to fill the parameter.
    #[serde(rename="initialPromptFulfillment")]
    
    pub initial_prompt_fulfillment: Option<GoogleCloudDialogflowCxV3Fulfillment>,
    /// The handlers for parameter-level events, used to provide reprompt for the parameter or transition to a different page/flow. The supported events are: * `sys.no-match-`, where N can be from 1 to 6 * `sys.no-match-default` * `sys.no-input-`, where N can be from 1 to 6 * `sys.no-input-default` * `sys.invalid-parameter` `initial_prompt_fulfillment` provides the first prompt for the parameter. If the user's response does not fill the parameter, a no-match/no-input event will be triggered, and the fulfillment associated with the `sys.no-match-1`/`sys.no-input-1` handler (if defined) will be called to provide a prompt. The `sys.no-match-2`/`sys.no-input-2` handler (if defined) will respond to the next no-match/no-input event, and so on. A `sys.no-match-default` or `sys.no-input-default` handler will be used to handle all following no-match/no-input events after all numbered no-match/no-input handlers for the parameter are consumed. A `sys.invalid-parameter` handler can be defined to handle the case where the parameter values have been `invalidated` by webhook. For example, if the user's response fill the parameter, however the parameter was invalidated by webhook, the fulfillment associated with the `sys.invalid-parameter` handler (if defined) will be called to provide a prompt. If the event handler for the corresponding event can't be found on the parameter, `initial_prompt_fulfillment` will be re-prompted.
    #[serde(rename="repromptEventHandlers")]
    
    pub reprompt_event_handlers: Option<Vec<GoogleCloudDialogflowCxV3EventHandler>>,
}

impl client::Part for GoogleCloudDialogflowCxV3FormParameterFillBehavior {}


/// Request of FulfillIntent
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments sessions fulfill intent projects](ProjectLocationAgentEnvironmentSessionFulfillIntentCall) (request)
/// * [locations agents sessions fulfill intent projects](ProjectLocationAgentSessionFulfillIntentCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3FulfillIntentRequest {
    /// The matched intent/event to fulfill.
    #[serde(rename="match")]
    
    pub match_: Option<GoogleCloudDialogflowCxV3Match>,
    /// Must be same as the corresponding MatchIntent request, otherwise the behavior is undefined.
    #[serde(rename="matchIntentRequest")]
    
    pub match_intent_request: Option<GoogleCloudDialogflowCxV3MatchIntentRequest>,
    /// Instructs the speech synthesizer how to generate output audio.
    #[serde(rename="outputAudioConfig")]
    
    pub output_audio_config: Option<GoogleCloudDialogflowCxV3OutputAudioConfig>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3FulfillIntentRequest {}


/// Response of FulfillIntent
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments sessions fulfill intent projects](ProjectLocationAgentEnvironmentSessionFulfillIntentCall) (response)
/// * [locations agents sessions fulfill intent projects](ProjectLocationAgentSessionFulfillIntentCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3FulfillIntentResponse {
    /// The audio data bytes encoded as specified in the request. Note: The output audio is generated based on the values of default platform text responses found in the `query_result.response_messages` field. If multiple default text responses exist, they will be concatenated when generating audio. If no default platform text responses exist, the generated audio content will be empty. In some scenarios, multiple output audio fields may be present in the response structure. In these cases, only the top-most-level audio output has content.
    #[serde(rename="outputAudio")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub output_audio: Option<Vec<u8>>,
    /// The config used by the speech synthesizer to generate the output audio.
    #[serde(rename="outputAudioConfig")]
    
    pub output_audio_config: Option<GoogleCloudDialogflowCxV3OutputAudioConfig>,
    /// The result of the conversational query.
    #[serde(rename="queryResult")]
    
    pub query_result: Option<GoogleCloudDialogflowCxV3QueryResult>,
    /// Output only. The unique identifier of the response. It can be used to locate a response in the training example set or for reporting issues.
    #[serde(rename="responseId")]
    
    pub response_id: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3FulfillIntentResponse {}


/// A fulfillment can do one or more of the following actions at the same time: * Generate rich message responses. * Set parameter values. * Call the webhook. Fulfillments can be called at various stages in the Page or Form lifecycle. For example, when a DetectIntentRequest drives a session to enter a new page, the page's entry fulfillment can add a static response to the QueryResult in the returning DetectIntentResponse, call the webhook (for example, to load user data from a database), or both.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3Fulfillment {
    /// Conditional cases for this fulfillment.
    #[serde(rename="conditionalCases")]
    
    pub conditional_cases: Option<Vec<GoogleCloudDialogflowCxV3FulfillmentConditionalCases>>,
    /// The list of rich message responses to present to the user.
    
    pub messages: Option<Vec<GoogleCloudDialogflowCxV3ResponseMessage>>,
    /// Whether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks.
    #[serde(rename="returnPartialResponses")]
    
    pub return_partial_responses: Option<bool>,
    /// Set parameter values before executing the webhook.
    #[serde(rename="setParameterActions")]
    
    pub set_parameter_actions: Option<Vec<GoogleCloudDialogflowCxV3FulfillmentSetParameterAction>>,
    /// The value of this field will be populated in the WebhookRequest `fulfillmentInfo.tag` field by Dialogflow when the associated webhook is called. The tag is typically used by the webhook service to identify which fulfillment is being called, but it could be used for other purposes. This field is required if `webhook` is specified.
    
    pub tag: Option<String>,
    /// The webhook to call. Format: `projects//locations//agents//webhooks/`.
    
    pub webhook: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3Fulfillment {}


/// A list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3FulfillmentConditionalCases {
    /// A list of cascading if-else conditions.
    
    pub cases: Option<Vec<GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCase>>,
}

impl client::Part for GoogleCloudDialogflowCxV3FulfillmentConditionalCases {}


/// Each case has a Boolean condition. When it is evaluated to be True, the corresponding messages will be selected and evaluated recursively.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCase {
    /// A list of case content.
    #[serde(rename="caseContent")]
    
    pub case_content: Option<Vec<GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCaseCaseContent>>,
    /// The condition to activate and select this case. Empty means the condition is always true. The condition is evaluated against form parameters or session parameters. See the [conditions reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition).
    
    pub condition: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCase {}


/// The list of messages or conditional cases to activate for this case.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCaseCaseContent {
    /// Additional cases to be evaluated.
    #[serde(rename="additionalCases")]
    
    pub additional_cases: Option<GoogleCloudDialogflowCxV3FulfillmentConditionalCases>,
    /// Returned message.
    
    pub message: Option<GoogleCloudDialogflowCxV3ResponseMessage>,
}

impl client::Part for GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCaseCaseContent {}


/// Setting a parameter value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3FulfillmentSetParameterAction {
    /// Display name of the parameter.
    
    pub parameter: Option<String>,
    /// The new value of the parameter. A null value clears the parameter.
    
    pub value: Option<json::Value>,
}

impl client::Part for GoogleCloudDialogflowCxV3FulfillmentSetParameterAction {}


/// The request message for Flows.ImportFlow.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows import projects](ProjectLocationAgentFlowImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ImportFlowRequest {
    /// Uncompressed raw byte content for flow.
    #[serde(rename="flowContent")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub flow_content: Option<Vec<u8>>,
    /// The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to import flow from. The format of this URI must be `gs:///`. Dialogflow performs a read operation for the Cloud Storage object on the caller's behalf, so your request authentication must have read permissions for the object. For more information, see [Dialogflow access control](https://cloud.google.com/dialogflow/cx/docs/concept/access-control#storage).
    #[serde(rename="flowUri")]
    
    pub flow_uri: Option<String>,
    /// Flow import mode. If not specified, `KEEP` is assumed.
    #[serde(rename="importOption")]
    
    pub import_option: Option<GoogleCloudDialogflowCxV3ImportFlowRequestImportOptionEnum>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3ImportFlowRequest {}


/// The request message for TestCases.ImportTestCases.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents test cases import projects](ProjectLocationAgentTestCaseImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ImportTestCasesRequest {
    /// Uncompressed raw byte content for test cases.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub content: Option<Vec<u8>>,
    /// The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to import test cases from. The format of this URI must be `gs:///`. Dialogflow performs a read operation for the Cloud Storage object on the caller's behalf, so your request authentication must have read permissions for the object. For more information, see [Dialogflow access control](https://cloud.google.com/dialogflow/cx/docs/concept/access-control#storage).
    #[serde(rename="gcsUri")]
    
    pub gcs_uri: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3ImportTestCasesRequest {}


/// Instructs the speech recognizer on how to process the audio content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3InputAudioConfig {
    /// Required. Audio encoding of the audio content to process.
    #[serde(rename="audioEncoding")]
    
    pub audio_encoding: Option<GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum>,
    /// Optional. If `true`, Dialogflow returns SpeechWordInfo in StreamingRecognitionResult with information about the recognized speech words, e.g. start and end time offsets. If false or unspecified, Speech doesn't return any word-level information.
    #[serde(rename="enableWordInfo")]
    
    pub enable_word_info: Option<bool>,
    /// Optional. Which Speech model to select for the given request. Select the model best suited to your domain to get best results. If a model is not explicitly specified, then we auto-select a model based on the parameters in the InputAudioConfig. If enhanced speech model is enabled for the agent and an enhanced version of the specified model for the language does not exist, then the speech is recognized using the standard version of the specified model. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics#select-model) for more details.
    
    pub model: Option<String>,
    /// Optional. Which variant of the Speech model to use.
    #[serde(rename="modelVariant")]
    
    pub model_variant: Option<GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum>,
    /// Optional. A list of strings containing words and phrases that the speech recognizer should recognize with higher likelihood. See [the Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints) for more details.
    #[serde(rename="phraseHints")]
    
    pub phrase_hints: Option<Vec<String>>,
    /// Sample rate (in Hertz) of the audio content sent in the query. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics) for more details.
    #[serde(rename="sampleRateHertz")]
    
    pub sample_rate_hertz: Option<i32>,
    /// Optional. If `false` (default), recognition does not cease until the client closes the stream. If `true`, the recognizer will detect a single spoken utterance in input audio. Recognition ceases when it detects the audio's voice has stopped or paused. In this case, once a detected intent is received, the client should close the stream and start a new request with a new stream as needed. Note: This setting is relevant only for streaming methods.
    #[serde(rename="singleUtterance")]
    
    pub single_utterance: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowCxV3InputAudioConfig {}


/// An intent represents a user’s intent to interact with a conversational agent. You can provide information for the Dialogflow API to use to match user input to an intent by adding training phrases (i.e., examples of user input) to your intent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents intents create projects](ProjectLocationAgentIntentCreateCall) (request|response)
/// * [locations agents intents get projects](ProjectLocationAgentIntentGetCall) (response)
/// * [locations agents intents patch projects](ProjectLocationAgentIntentPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3Intent {
    /// Human readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters.
    
    pub description: Option<String>,
    /// Required. The human-readable name of the intent, unique within the agent.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Indicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation. Adding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event.
    #[serde(rename="isFallback")]
    
    pub is_fallback: Option<bool>,
    /// The key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes. Prefix "sys-" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. "sys-head" means the intent is a head intent. "sys.contextual" means the intent is a contextual intent.
    
    pub labels: Option<HashMap<String, String>>,
    /// The unique identifier of the intent. Required for the Intents.UpdateIntent method. Intents.CreateIntent populates the name automatically. Format: `projects//locations//agents//intents/`.
    
    pub name: Option<String>,
    /// The collection of parameters associated with the intent.
    
    pub parameters: Option<Vec<GoogleCloudDialogflowCxV3IntentParameter>>,
    /// The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests.
    
    pub priority: Option<i32>,
    /// The collection of training phrases the agent is trained on to identify the intent.
    #[serde(rename="trainingPhrases")]
    
    pub training_phrases: Option<Vec<GoogleCloudDialogflowCxV3IntentTrainingPhrase>>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3Intent {}
impl client::ResponseResult for GoogleCloudDialogflowCxV3Intent {}


/// Intent coverage represents the percentage of all possible intents in the agent that are triggered in any of a parent's test cases.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3IntentCoverage {
    /// The percent of intents in the agent that are covered.
    #[serde(rename="coverageScore")]
    
    pub coverage_score: Option<f32>,
    /// The list of Intents present in the agent
    
    pub intents: Option<Vec<GoogleCloudDialogflowCxV3IntentCoverageIntent>>,
}

impl client::Part for GoogleCloudDialogflowCxV3IntentCoverage {}


/// The agent's intent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3IntentCoverageIntent {
    /// Whether or not the intent is covered by at least one of the agent's test cases.
    
    pub covered: Option<bool>,
    /// The intent full resource name
    
    pub intent: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3IntentCoverageIntent {}


/// Represents the intent to trigger programmatically rather than as a result of natural language processing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3IntentInput {
    /// Required. The unique identifier of the intent. Format: `projects//locations//agents//intents/`.
    
    pub intent: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3IntentInput {}


/// Represents an intent parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3IntentParameter {
    /// Required. The entity type of the parameter. Format: `projects/-/locations/-/agents/-/entityTypes/` for system entity types (for example, `projects/-/locations/-/agents/-/entityTypes/sys.date`), or `projects//locations//agents//entityTypes/` for developer entity types.
    #[serde(rename="entityType")]
    
    pub entity_type: Option<String>,
    /// Required. The unique identifier of the parameter. This field is used by training phrases to annotate their parts.
    
    pub id: Option<String>,
    /// Indicates whether the parameter represents a list of values.
    #[serde(rename="isList")]
    
    pub is_list: Option<bool>,
    /// Indicates whether the parameter content should be redacted in log. If redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled.
    
    pub redact: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowCxV3IntentParameter {}


/// Represents an example that the agent is trained on to identify the intent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3IntentTrainingPhrase {
    /// Output only. The unique identifier of the training phrase.
    
    pub id: Option<String>,
    /// Required. The ordered list of training phrase parts. The parts are concatenated in order to form the training phrase. Note: The API does not automatically annotate training phrases like the Dialogflow Console does. Note: Do not forget to include whitespace at part boundaries, so the training phrase is well formatted when the parts are concatenated. If the training phrase does not need to be annotated with parameters, you just need a single part with only the Part.text field set. If you want to annotate the training phrase, you must create multiple parts, where the fields of each part are populated in one of two ways: - `Part.text` is set to a part of the phrase that has no parameters. - `Part.text` is set to a part of the phrase that you want to annotate, and the `parameter_id` field is set.
    
    pub parts: Option<Vec<GoogleCloudDialogflowCxV3IntentTrainingPhrasePart>>,
    /// Indicates how many times this example was added to the intent.
    #[serde(rename="repeatCount")]
    
    pub repeat_count: Option<i32>,
}

impl client::Part for GoogleCloudDialogflowCxV3IntentTrainingPhrase {}


/// Represents a part of a training phrase.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3IntentTrainingPhrasePart {
    /// The parameter used to annotate this part of the training phrase. This field is required for annotated parts of the training phrase.
    #[serde(rename="parameterId")]
    
    pub parameter_id: Option<String>,
    /// Required. The text for this part.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3IntentTrainingPhrasePart {}


/// The response message for Agents.ListAgents.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents list projects](ProjectLocationAgentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListAgentsResponse {
    /// The list of agents. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub agents: Option<Vec<GoogleCloudDialogflowCxV3Agent>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListAgentsResponse {}


/// The response message for Changelogs.ListChangelogs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents changelogs list projects](ProjectLocationAgentChangelogListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListChangelogsResponse {
    /// The list of changelogs. There will be a maximum number of items returned based on the page_size field in the request. The changelogs will be ordered by timestamp.
    
    pub changelogs: Option<Vec<GoogleCloudDialogflowCxV3Changelog>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListChangelogsResponse {}


/// The response message for Environments.ListTestCaseResults.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments continuous test results list projects](ProjectLocationAgentEnvironmentContinuousTestResultListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListContinuousTestResultsResponse {
    /// The list of continuous test results.
    #[serde(rename="continuousTestResults")]
    
    pub continuous_test_results: Option<Vec<GoogleCloudDialogflowCxV3ContinuousTestResult>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListContinuousTestResultsResponse {}


/// The response message for Deployments.ListDeployments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments deployments list projects](ProjectLocationAgentEnvironmentDeploymentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListDeploymentsResponse {
    /// The list of deployments. There will be a maximum number of items returned based on the page_size field in the request. The list may in some cases be empty or contain fewer entries than page_size even if this isn't the last page.
    
    pub deployments: Option<Vec<GoogleCloudDialogflowCxV3Deployment>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListDeploymentsResponse {}


/// The response message for EntityTypes.ListEntityTypes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents entity types list projects](ProjectLocationAgentEntityTypeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListEntityTypesResponse {
    /// The list of entity types. There will be a maximum number of items returned based on the page_size field in the request.
    #[serde(rename="entityTypes")]
    
    pub entity_types: Option<Vec<GoogleCloudDialogflowCxV3EntityType>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListEntityTypesResponse {}


/// The response message for Environments.ListEnvironments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments list projects](ProjectLocationAgentEnvironmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListEnvironmentsResponse {
    /// The list of environments. There will be a maximum number of items returned based on the page_size field in the request. The list may in some cases be empty or contain fewer entries than page_size even if this isn't the last page.
    
    pub environments: Option<Vec<GoogleCloudDialogflowCxV3Environment>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListEnvironmentsResponse {}


/// The response message for Experiments.ListExperiments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments experiments list projects](ProjectLocationAgentEnvironmentExperimentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListExperimentsResponse {
    /// The list of experiments. There will be a maximum number of items returned based on the page_size field in the request. The list may in some cases be empty or contain fewer entries than page_size even if this isn't the last page.
    
    pub experiments: Option<Vec<GoogleCloudDialogflowCxV3Experiment>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListExperimentsResponse {}


/// The response message for Flows.ListFlows.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows list projects](ProjectLocationAgentFlowListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListFlowsResponse {
    /// The list of flows. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub flows: Option<Vec<GoogleCloudDialogflowCxV3Flow>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListFlowsResponse {}


/// The response message for Intents.ListIntents.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents intents list projects](ProjectLocationAgentIntentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListIntentsResponse {
    /// The list of intents. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub intents: Option<Vec<GoogleCloudDialogflowCxV3Intent>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListIntentsResponse {}


/// The response message for Pages.ListPages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows pages list projects](ProjectLocationAgentFlowPageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListPagesResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of pages. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub pages: Option<Vec<GoogleCloudDialogflowCxV3Page>>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListPagesResponse {}


/// The response message for SecuritySettings.ListSecuritySettings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations security settings list projects](ProjectLocationSecuritySettingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListSecuritySettingsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of security settings.
    #[serde(rename="securitySettings")]
    
    pub security_settings: Option<Vec<GoogleCloudDialogflowCxV3SecuritySettings>>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListSecuritySettingsResponse {}


/// The response message for SessionEntityTypes.ListSessionEntityTypes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments sessions entity types list projects](ProjectLocationAgentEnvironmentSessionEntityTypeListCall) (response)
/// * [locations agents sessions entity types list projects](ProjectLocationAgentSessionEntityTypeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListSessionEntityTypesResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of session entity types. There will be a maximum number of items returned based on the page_size field in the request.
    #[serde(rename="sessionEntityTypes")]
    
    pub session_entity_types: Option<Vec<GoogleCloudDialogflowCxV3SessionEntityType>>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListSessionEntityTypesResponse {}


/// The response message for TestCases.ListTestCaseResults.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents test cases results list projects](ProjectLocationAgentTestCaseResultListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListTestCaseResultsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of test case results.
    #[serde(rename="testCaseResults")]
    
    pub test_case_results: Option<Vec<GoogleCloudDialogflowCxV3TestCaseResult>>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListTestCaseResultsResponse {}


/// The response message for TestCases.ListTestCases.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents test cases list projects](ProjectLocationAgentTestCaseListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListTestCasesResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of test cases. There will be a maximum number of items returned based on the page_size field in the request.
    #[serde(rename="testCases")]
    
    pub test_cases: Option<Vec<GoogleCloudDialogflowCxV3TestCase>>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListTestCasesResponse {}


/// The response message for TransitionRouteGroups.ListTransitionRouteGroups.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows transition route groups list projects](ProjectLocationAgentFlowTransitionRouteGroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListTransitionRouteGroupsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of transition route groups. There will be a maximum number of items returned based on the page_size field in the request. The list may in some cases be empty or contain fewer entries than page_size even if this isn't the last page.
    #[serde(rename="transitionRouteGroups")]
    
    pub transition_route_groups: Option<Vec<GoogleCloudDialogflowCxV3TransitionRouteGroup>>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListTransitionRouteGroupsResponse {}


/// The response message for Versions.ListVersions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows versions list projects](ProjectLocationAgentFlowVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListVersionsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of versions. There will be a maximum number of items returned based on the page_size field in the request. The list may in some cases be empty or contain fewer entries than page_size even if this isn't the last page.
    
    pub versions: Option<Vec<GoogleCloudDialogflowCxV3Version>>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListVersionsResponse {}


/// The response message for Webhooks.ListWebhooks.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents webhooks list projects](ProjectLocationAgentWebhookListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ListWebhooksResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of webhooks. There will be a maximum number of items returned based on the page_size field in the request.
    
    pub webhooks: Option<Vec<GoogleCloudDialogflowCxV3Webhook>>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3ListWebhooksResponse {}


/// The request message for Versions.LoadVersion.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows versions load projects](ProjectLocationAgentFlowVersionLoadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3LoadVersionRequest {
    /// This field is used to prevent accidental overwrite of other agent resources, which can potentially impact other flow's behavior. If `allow_override_agent_resources` is false, conflicted agent-level resources will not be overridden (i.e. intents, entities, webhooks).
    #[serde(rename="allowOverrideAgentResources")]
    
    pub allow_override_agent_resources: Option<bool>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3LoadVersionRequest {}


/// The response message for Environments.LookupEnvironmentHistory.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments lookup environment history projects](ProjectLocationAgentEnvironmentLookupEnvironmentHistoryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3LookupEnvironmentHistoryResponse {
    /// Represents a list of snapshots for an environment. Time of the snapshots is stored in `update_time`.
    
    pub environments: Option<Vec<GoogleCloudDialogflowCxV3Environment>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3LookupEnvironmentHistoryResponse {}


/// Represents one match result of MatchIntent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3Match {
    /// The confidence of this match. Values range from 0.0 (completely uncertain) to 1.0 (completely certain). This value is for informational purpose only and is only used to help match the best intent within the classification threshold. This value may change for the same end-user expression at any time due to a model retraining or change in implementation.
    
    pub confidence: Option<f32>,
    /// The event that matched the query. Filled for `EVENT`, `NO_MATCH` and `NO_INPUT` match types.
    
    pub event: Option<String>,
    /// The Intent that matched the query. Some, not all fields are filled in this message, including but not limited to: `name` and `display_name`. Only filled for `INTENT` match type.
    
    pub intent: Option<GoogleCloudDialogflowCxV3Intent>,
    /// Type of this Match.
    #[serde(rename="matchType")]
    
    pub match_type: Option<GoogleCloudDialogflowCxV3MatchMatchTypeEnum>,
    /// The collection of parameters extracted from the query. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: * MapKey type: string * MapKey value: parameter name * MapValue type: If parameter's entity type is a composite entity then use map, otherwise, depending on the parameter value type, it could be one of string, number, boolean, null, list or map. * MapValue value: If parameter's entity type is a composite entity then use map from composite entity property names to property values, otherwise, use parameter value.
    
    pub parameters: Option<HashMap<String, json::Value>>,
    /// Final text input which was matched during MatchIntent. This value can be different from original input sent in request because of spelling correction or other processing.
    #[serde(rename="resolvedInput")]
    
    pub resolved_input: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3Match {}


/// Request of MatchIntent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments sessions match intent projects](ProjectLocationAgentEnvironmentSessionMatchIntentCall) (request)
/// * [locations agents sessions match intent projects](ProjectLocationAgentSessionMatchIntentCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3MatchIntentRequest {
    /// Required. The input specification.
    #[serde(rename="queryInput")]
    
    pub query_input: Option<GoogleCloudDialogflowCxV3QueryInput>,
    /// The parameters of this query.
    #[serde(rename="queryParams")]
    
    pub query_params: Option<GoogleCloudDialogflowCxV3QueryParameters>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3MatchIntentRequest {}


/// Response of MatchIntent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments sessions match intent projects](ProjectLocationAgentEnvironmentSessionMatchIntentCall) (response)
/// * [locations agents sessions match intent projects](ProjectLocationAgentSessionMatchIntentCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3MatchIntentResponse {
    /// The current Page. Some, not all fields are filled in this message, including but not limited to `name` and `display_name`.
    #[serde(rename="currentPage")]
    
    pub current_page: Option<GoogleCloudDialogflowCxV3Page>,
    /// Match results, if more than one, ordered descendingly by the confidence we have that the particular intent matches the query.
    
    pub matches: Option<Vec<GoogleCloudDialogflowCxV3Match>>,
    /// If natural language text was provided as input, this field will contain a copy of the text.
    
    pub text: Option<String>,
    /// If natural language speech audio was provided as input, this field will contain the transcript for the audio.
    
    pub transcript: Option<String>,
    /// If an event was provided as input, this field will contain a copy of the event name.
    #[serde(rename="triggerEvent")]
    
    pub trigger_event: Option<String>,
    /// If an intent was provided as input, this field will contain a copy of the intent identifier. Format: `projects//locations//agents//intents/`.
    #[serde(rename="triggerIntent")]
    
    pub trigger_intent: Option<String>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3MatchIntentResponse {}


/// Settings related to NLU.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3NluSettings {
    /// To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a no-match event will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used.
    #[serde(rename="classificationThreshold")]
    
    pub classification_threshold: Option<f32>,
    /// Indicates NLU model training mode.
    #[serde(rename="modelTrainingMode")]
    
    pub model_training_mode: Option<GoogleCloudDialogflowCxV3NluSettingModelTrainingModeEnum>,
    /// Indicates the type of NLU model.
    #[serde(rename="modelType")]
    
    pub model_type: Option<GoogleCloudDialogflowCxV3NluSettingModelTypeEnum>,
}

impl client::Part for GoogleCloudDialogflowCxV3NluSettings {}


/// Instructs the speech synthesizer how to generate the output audio content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3OutputAudioConfig {
    /// Required. Audio encoding of the synthesized audio content.
    #[serde(rename="audioEncoding")]
    
    pub audio_encoding: Option<GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum>,
    /// Optional. The synthesis sample rate (in hertz) for this audio. If not provided, then the synthesizer will use the default sample rate based on the audio encoding. If this is different from the voice's natural sample rate, then the synthesizer will honor this request by converting to the desired sample rate (which might result in worse audio quality).
    #[serde(rename="sampleRateHertz")]
    
    pub sample_rate_hertz: Option<i32>,
    /// Optional. Configuration of how speech should be synthesized.
    #[serde(rename="synthesizeSpeechConfig")]
    
    pub synthesize_speech_config: Option<GoogleCloudDialogflowCxV3SynthesizeSpeechConfig>,
}

impl client::Part for GoogleCloudDialogflowCxV3OutputAudioConfig {}


/// A Dialogflow CX conversation (session) can be described and visualized as a state machine. The states of a CX session are represented by pages. For each flow, you define many pages, where your combined pages can handle a complete conversation on the topics the flow is designed for. At any given moment, exactly one page is the current page, the current page is considered active, and the flow associated with that page is considered active. Every flow has a special start page. When a flow initially becomes active, the start page page becomes the current page. For each conversational turn, the current page will either stay the same or transition to another page. You configure each page to collect information from the end-user that is relevant for the conversational state represented by the page. For more information, see the [Page guide](https://cloud.google.com/dialogflow/cx/docs/concept/page).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows pages create projects](ProjectLocationAgentFlowPageCreateCall) (request|response)
/// * [locations agents flows pages get projects](ProjectLocationAgentFlowPageGetCall) (response)
/// * [locations agents flows pages patch projects](ProjectLocationAgentFlowPagePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3Page {
    /// Required. The human-readable name of the page, unique within the flow.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The fulfillment to call when the session is entering the page.
    #[serde(rename="entryFulfillment")]
    
    pub entry_fulfillment: Option<GoogleCloudDialogflowCxV3Fulfillment>,
    /// Handlers associated with the page to handle events such as webhook errors, no match or no input.
    #[serde(rename="eventHandlers")]
    
    pub event_handlers: Option<Vec<GoogleCloudDialogflowCxV3EventHandler>>,
    /// The form associated with the page, used for collecting parameters relevant to the page.
    
    pub form: Option<GoogleCloudDialogflowCxV3Form>,
    /// The unique identifier of the page. Required for the Pages.UpdatePage method. Pages.CreatePage populates the name automatically. Format: `projects//locations//agents//flows//pages/`.
    
    pub name: Option<String>,
    /// Ordered list of `TransitionRouteGroups` associated with the page. Transition route groups must be unique within a page. * If multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes. * If multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence. Format:`projects//locations//agents//flows//transitionRouteGroups/`.
    #[serde(rename="transitionRouteGroups")]
    
    pub transition_route_groups: Option<Vec<String>>,
    /// A list of transitions for the transition rules of this page. They route the conversation to another page in the same flow, or another flow. When we are in a certain page, the TransitionRoutes are evalauted in the following order: * TransitionRoutes defined in the page with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in flow with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in the page with only condition specified. * TransitionRoutes defined in the transition route groups with only condition specified.
    #[serde(rename="transitionRoutes")]
    
    pub transition_routes: Option<Vec<GoogleCloudDialogflowCxV3TransitionRoute>>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3Page {}
impl client::ResponseResult for GoogleCloudDialogflowCxV3Page {}


/// Represents the query input. It can contain one of: 1. A conversational query in the form of text. 2. An intent query that specifies which intent to trigger. 3. Natural language speech audio to be processed. 4. An event to be triggered. 
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3QueryInput {
    /// The natural language speech audio to be processed.
    
    pub audio: Option<GoogleCloudDialogflowCxV3AudioInput>,
    /// The DTMF event to be handled.
    
    pub dtmf: Option<GoogleCloudDialogflowCxV3DtmfInput>,
    /// The event to be triggered.
    
    pub event: Option<GoogleCloudDialogflowCxV3EventInput>,
    /// The intent to be triggered.
    
    pub intent: Option<GoogleCloudDialogflowCxV3IntentInput>,
    /// Required. The language of the input. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// The natural language text to be processed.
    
    pub text: Option<GoogleCloudDialogflowCxV3TextInput>,
}

impl client::Part for GoogleCloudDialogflowCxV3QueryInput {}


/// Represents the parameters of a conversational query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3QueryParameters {
    /// Configures whether sentiment analysis should be performed. If not provided, sentiment analysis is not performed.
    #[serde(rename="analyzeQueryTextSentiment")]
    
    pub analyze_query_text_sentiment: Option<bool>,
    /// The channel which this query is for. If specified, only the ResponseMessage associated with the channel will be returned. If no ResponseMessage is associated with the channel, it falls back to the ResponseMessage with unspecified channel. If unspecified, the ResponseMessage with unspecified channel will be returned.
    
    pub channel: Option<String>,
    /// The unique identifier of the page to override the current page in the session. Format: `projects//locations//agents//flows//pages/`. If `current_page` is specified, the previous state of the session will be ignored by Dialogflow, including the previous page and the previous session parameters. In most cases, current_page and parameters should be configured together to direct a session to a specific state.
    #[serde(rename="currentPage")]
    
    pub current_page: Option<String>,
    /// Whether to disable webhook calls for this request.
    #[serde(rename="disableWebhook")]
    
    pub disable_webhook: Option<bool>,
    /// A list of flow versions to override for the request. Format: `projects//locations//agents//flows//versions/`. If version 1 of flow X is included in this list, the traffic of flow X will go through version 1 regardless of the version configuration in the environment. Each flow can have at most one version specified in this list.
    #[serde(rename="flowVersions")]
    
    pub flow_versions: Option<Vec<String>>,
    /// The geo location of this conversational query.
    #[serde(rename="geoLocation")]
    
    pub geo_location: Option<GoogleTypeLatLng>,
    /// Additional parameters to be put into session parameters. To remove a parameter from the session, clients should explicitly set the parameter value to null. You can reference the session parameters in the agent with the following format: $session.params.parameter-id. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: * MapKey type: string * MapKey value: parameter name * MapValue type: If parameter's entity type is a composite entity then use map, otherwise, depending on the parameter value type, it could be one of string, number, boolean, null, list or map. * MapValue value: If parameter's entity type is a composite entity then use map from composite entity property names to property values, otherwise, use parameter value.
    
    pub parameters: Option<HashMap<String, json::Value>>,
    /// This field can be used to pass custom data into the webhook associated with the agent. Arbitrary JSON objects are supported. Some integrations that query a Dialogflow agent may provide additional information in the payload. In particular, for the Dialogflow Phone Gateway integration, this field has the form: ``` { "telephony": { "caller_id": "+18558363987" } } ```
    
    pub payload: Option<HashMap<String, json::Value>>,
    /// Additional session entity types to replace or extend developer entity types with. The entity synonyms apply to all languages and persist for the session of this query.
    #[serde(rename="sessionEntityTypes")]
    
    pub session_entity_types: Option<Vec<GoogleCloudDialogflowCxV3SessionEntityType>>,
    /// The time zone of this conversational query from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. If not provided, the time zone specified in the agent is used.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// This field can be used to pass HTTP headers for a webhook call. These headers will be sent to webhook along with the headers that have been configured through Dialogflow web console. The headers defined within this field will overwrite the headers configured through Dialogflow console if there is a conflict. Header names are case-insensitive. Google's specified headers are not allowed. Including: "Host", "Content-Length", "Connection", "From", "User-Agent", "Accept-Encoding", "If-Modified-Since", "If-None-Match", "X-Forwarded-For", etc.
    #[serde(rename="webhookHeaders")]
    
    pub webhook_headers: Option<HashMap<String, String>>,
}

impl client::Part for GoogleCloudDialogflowCxV3QueryParameters {}


/// Represents the result of a conversational query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3QueryResult {
    /// The current Page. Some, not all fields are filled in this message, including but not limited to `name` and `display_name`.
    #[serde(rename="currentPage")]
    
    pub current_page: Option<GoogleCloudDialogflowCxV3Page>,
    /// The free-form diagnostic info. For example, this field could contain webhook call latency. The fields of this data can change without notice, so you should not write code that depends on its structure. One of the fields is called "Alternative Matched Intents", which may aid with debugging. The following describes these intent results: - The list is empty if no intent was matched to end-user input. - Only intents that are referenced in the currently active flow are included. - The matched intent is included. - Other intents that could have matched end-user input, but did not match because they are referenced by intent routes that are out of [scope](https://cloud.google.com/dialogflow/cx/docs/concept/handler#scope), are included. - Other intents referenced by intent routes in scope that matched end-user input, but had a lower confidence score.
    #[serde(rename="diagnosticInfo")]
    
    pub diagnostic_info: Option<HashMap<String, json::Value>>,
    /// If a DTMF was provided as input, this field will contain a copy of the DTMFInput.
    
    pub dtmf: Option<GoogleCloudDialogflowCxV3DtmfInput>,
    /// The Intent that matched the conversational query. Some, not all fields are filled in this message, including but not limited to: `name` and `display_name`. This field is deprecated, please use QueryResult.match instead.
    
    pub intent: Option<GoogleCloudDialogflowCxV3Intent>,
    /// The intent detection confidence. Values range from 0.0 (completely uncertain) to 1.0 (completely certain). This value is for informational purpose only and is only used to help match the best intent within the classification threshold. This value may change for the same end-user expression at any time due to a model retraining or change in implementation. This field is deprecated, please use QueryResult.match instead.
    #[serde(rename="intentDetectionConfidence")]
    
    pub intent_detection_confidence: Option<f32>,
    /// The language that was triggered during intent detection. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Intent match result, could be an intent or an event.
    #[serde(rename="match")]
    
    pub match_: Option<GoogleCloudDialogflowCxV3Match>,
    /// The collected session parameters. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: * MapKey type: string * MapKey value: parameter name * MapValue type: If parameter's entity type is a composite entity then use map, otherwise, depending on the parameter value type, it could be one of string, number, boolean, null, list or map. * MapValue value: If parameter's entity type is a composite entity then use map from composite entity property names to property values, otherwise, use parameter value.
    
    pub parameters: Option<HashMap<String, json::Value>>,
    /// The list of rich messages returned to the client. Responses vary from simple text messages to more sophisticated, structured payloads used to drive complex logic.
    #[serde(rename="responseMessages")]
    
    pub response_messages: Option<Vec<GoogleCloudDialogflowCxV3ResponseMessage>>,
    /// The sentiment analyss result, which depends on `analyze_query_text_sentiment`, specified in the request.
    #[serde(rename="sentimentAnalysisResult")]
    
    pub sentiment_analysis_result: Option<GoogleCloudDialogflowCxV3SentimentAnalysisResult>,
    /// If natural language text was provided as input, this field will contain a copy of the text.
    
    pub text: Option<String>,
    /// If natural language speech audio was provided as input, this field will contain the transcript for the audio.
    
    pub transcript: Option<String>,
    /// If an event was provided as input, this field will contain the name of the event.
    #[serde(rename="triggerEvent")]
    
    pub trigger_event: Option<String>,
    /// If an intent was provided as input, this field will contain a copy of the intent identifier. Format: `projects//locations//agents//intents/`.
    #[serde(rename="triggerIntent")]
    
    pub trigger_intent: Option<String>,
    /// The list of webhook payload in WebhookResponse.payload, in the order of call sequence. If some webhook call fails or doesn't return any payload, an empty `Struct` would be used instead.
    #[serde(rename="webhookPayloads")]
    
    pub webhook_payloads: Option<Vec<HashMap<String, json::Value>>>,
    /// The list of webhook call status in the order of call sequence.
    #[serde(rename="webhookStatuses")]
    
    pub webhook_statuses: Option<Vec<GoogleRpcStatus>>,
}

impl client::Part for GoogleCloudDialogflowCxV3QueryResult {}


/// Resource name and display name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ResourceName {
    /// Display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Name.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3ResourceName {}


/// Represents a response message that can be returned by a conversational agent. Response messages are also used for output audio synthesis. The approach is as follows: * If at least one OutputAudioText response is present, then all OutputAudioText responses are linearly concatenated, and the result is used for output audio synthesis. * If the OutputAudioText responses are a mixture of text and SSML, then the concatenated result is treated as SSML; otherwise, the result is treated as either text or SSML as appropriate. The agent designer should ideally use either text or SSML consistently throughout the bot design. * Otherwise, all Text responses are linearly concatenated, and the result is used for output audio synthesis. This approach allows for more sophisticated user experience scenarios, where the text displayed to the user may differ from what is heard.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ResponseMessage {
    /// The channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned.
    
    pub channel: Option<String>,
    /// Indicates that the conversation succeeded.
    #[serde(rename="conversationSuccess")]
    
    pub conversation_success: Option<GoogleCloudDialogflowCxV3ResponseMessageConversationSuccess>,
    /// Output only. A signal that indicates the interaction with the Dialogflow agent has ended. This message is generated by Dialogflow only when the conversation reaches `END_SESSION` page. It is not supposed to be defined by the user. It's guaranteed that there is at most one such message in each response.
    #[serde(rename="endInteraction")]
    
    pub end_interaction: Option<GoogleCloudDialogflowCxV3ResponseMessageEndInteraction>,
    /// Hands off conversation to a human agent.
    #[serde(rename="liveAgentHandoff")]
    
    pub live_agent_handoff: Option<GoogleCloudDialogflowCxV3ResponseMessageLiveAgentHandoff>,
    /// Output only. An audio response message composed of both the synthesized Dialogflow agent responses and responses defined via play_audio. This message is generated by Dialogflow only and not supposed to be defined by the user.
    #[serde(rename="mixedAudio")]
    
    pub mixed_audio: Option<GoogleCloudDialogflowCxV3ResponseMessageMixedAudio>,
    /// A text or ssml response that is preferentially used for TTS output audio synthesis, as described in the comment on the ResponseMessage message.
    #[serde(rename="outputAudioText")]
    
    pub output_audio_text: Option<GoogleCloudDialogflowCxV3ResponseMessageOutputAudioText>,
    /// Returns a response containing a custom, platform-specific payload.
    
    pub payload: Option<HashMap<String, json::Value>>,
    /// Signal that the client should play an audio clip hosted at a client-specific URI. Dialogflow uses this to construct mixed_audio. However, Dialogflow itself does not try to read or process the URI in any way.
    #[serde(rename="playAudio")]
    
    pub play_audio: Option<GoogleCloudDialogflowCxV3ResponseMessagePlayAudio>,
    /// A signal that the client should transfer the phone call connected to this agent to a third-party endpoint.
    #[serde(rename="telephonyTransferCall")]
    
    pub telephony_transfer_call: Option<GoogleCloudDialogflowCxV3ResponseMessageTelephonyTransferCall>,
    /// Returns a text response.
    
    pub text: Option<GoogleCloudDialogflowCxV3ResponseMessageText>,
}

impl client::Part for GoogleCloudDialogflowCxV3ResponseMessage {}


/// Indicates that the conversation succeeded, i.e., the bot handled the issue that the customer talked to it about. Dialogflow only uses this to determine which conversations should be counted as successful and doesn't process the metadata in this message in any way. Note that Dialogflow also considers conversations that get to the conversation end page as successful even if they don't return ConversationSuccess. You may set this, for example: * In the entry_fulfillment of a Page if entering the page indicates that the conversation succeeded. * In a webhook response when you determine that you handled the customer issue.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ResponseMessageConversationSuccess {
    /// Custom metadata. Dialogflow doesn't impose any structure on this.
    
    pub metadata: Option<HashMap<String, json::Value>>,
}

impl client::Part for GoogleCloudDialogflowCxV3ResponseMessageConversationSuccess {}


/// Indicates that interaction with the Dialogflow agent has ended. This message is generated by Dialogflow only and not supposed to be defined by the user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ResponseMessageEndInteraction { _never_set: Option<bool> }

impl client::Part for GoogleCloudDialogflowCxV3ResponseMessageEndInteraction {}


/// Indicates that the conversation should be handed off to a live agent. Dialogflow only uses this to determine which conversations were handed off to a human agent for measurement purposes. What else to do with this signal is up to you and your handoff procedures. You may set this, for example: * In the entry_fulfillment of a Page if entering the page indicates something went extremely wrong in the conversation. * In a webhook response when you determine that the customer issue can only be handled by a human.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ResponseMessageLiveAgentHandoff {
    /// Custom metadata for your handoff procedure. Dialogflow doesn't impose any structure on this.
    
    pub metadata: Option<HashMap<String, json::Value>>,
}

impl client::Part for GoogleCloudDialogflowCxV3ResponseMessageLiveAgentHandoff {}


/// Represents an audio message that is composed of both segments synthesized from the Dialogflow agent prompts and ones hosted externally at the specified URIs. The external URIs are specified via play_audio. This message is generated by Dialogflow only and not supposed to be defined by the user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ResponseMessageMixedAudio {
    /// Segments this audio response is composed of.
    
    pub segments: Option<Vec<GoogleCloudDialogflowCxV3ResponseMessageMixedAudioSegment>>,
}

impl client::Part for GoogleCloudDialogflowCxV3ResponseMessageMixedAudio {}


/// Represents one segment of audio.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ResponseMessageMixedAudioSegment {
    /// Output only. Whether the playback of this segment can be interrupted by the end user's speech and the client should then start the next Dialogflow request.
    #[serde(rename="allowPlaybackInterruption")]
    
    pub allow_playback_interruption: Option<bool>,
    /// Raw audio synthesized from the Dialogflow agent's response using the output config specified in the request.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub audio: Option<Vec<u8>>,
    /// Client-specific URI that points to an audio clip accessible to the client. Dialogflow does not impose any validation on it.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3ResponseMessageMixedAudioSegment {}


/// A text or ssml response that is preferentially used for TTS output audio synthesis, as described in the comment on the ResponseMessage message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ResponseMessageOutputAudioText {
    /// Output only. Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request.
    #[serde(rename="allowPlaybackInterruption")]
    
    pub allow_playback_interruption: Option<bool>,
    /// The SSML text to be synthesized. For more information, see [SSML](https://cloud.google.com/speech/text-to-speech/docs/ssml).
    
    pub ssml: Option<String>,
    /// The raw text to be synthesized.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3ResponseMessageOutputAudioText {}


/// Specifies an audio clip to be played by the client as part of the response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ResponseMessagePlayAudio {
    /// Output only. Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request.
    #[serde(rename="allowPlaybackInterruption")]
    
    pub allow_playback_interruption: Option<bool>,
    /// Required. URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it.
    #[serde(rename="audioUri")]
    
    pub audio_uri: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3ResponseMessagePlayAudio {}


/// Represents the signal that telles the client to transfer the phone call connected to the agent to a third-party endpoint.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ResponseMessageTelephonyTransferCall {
    /// Transfer the call to a phone number in [E.164 format](https://en.wikipedia.org/wiki/E.164).
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3ResponseMessageTelephonyTransferCall {}


/// The text response message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ResponseMessageText {
    /// Output only. Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request.
    #[serde(rename="allowPlaybackInterruption")]
    
    pub allow_playback_interruption: Option<bool>,
    /// Required. A collection of text responses.
    
    pub text: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowCxV3ResponseMessageText {}


/// The request message for Agents.RestoreAgent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents restore projects](ProjectLocationAgentRestoreCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3RestoreAgentRequest {
    /// Uncompressed raw byte content for agent.
    #[serde(rename="agentContent")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub agent_content: Option<Vec<u8>>,
    /// The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to restore agent from. The format of this URI must be `gs:///`. Dialogflow performs a read operation for the Cloud Storage object on the caller's behalf, so your request authentication must have read permissions for the object. For more information, see [Dialogflow access control](https://cloud.google.com/dialogflow/cx/docs/concept/access-control#storage).
    #[serde(rename="agentUri")]
    
    pub agent_uri: Option<String>,
    /// Agent restore mode. If not specified, `KEEP` is assumed.
    #[serde(rename="restoreOption")]
    
    pub restore_option: Option<GoogleCloudDialogflowCxV3RestoreAgentRequestRestoreOptionEnum>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3RestoreAgentRequest {}


/// The configuration for auto rollout.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3RolloutConfig {
    /// The conditions that are used to evaluate the failure of a rollout step. If not specified, no rollout steps will fail. E.g. "containment_rate < 10% OR average_turn_count < 3". See the [conditions reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition).
    #[serde(rename="failureCondition")]
    
    pub failure_condition: Option<String>,
    /// The conditions that are used to evaluate the success of a rollout step. If not specified, all rollout steps will proceed to the next one unless failure conditions are met. E.g. "containment_rate > 60% AND callback_rate < 20%". See the [conditions reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition).
    #[serde(rename="rolloutCondition")]
    
    pub rollout_condition: Option<String>,
    /// Steps to roll out a flow version. Steps should be sorted by percentage in ascending order.
    #[serde(rename="rolloutSteps")]
    
    pub rollout_steps: Option<Vec<GoogleCloudDialogflowCxV3RolloutConfigRolloutStep>>,
}

impl client::Part for GoogleCloudDialogflowCxV3RolloutConfig {}


/// A single rollout step with specified traffic allocation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3RolloutConfigRolloutStep {
    /// The name of the rollout step;
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The minimum time that this step should last. Should be longer than 1 hour. If not set, the default minimum duration for each step will be 1 hour.
    #[serde(rename="minDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub min_duration: Option<client::chrono::Duration>,
    /// The percentage of traffic allocated to the flow version of this rollout step. (0%, 100%].
    #[serde(rename="trafficPercent")]
    
    pub traffic_percent: Option<i32>,
}

impl client::Part for GoogleCloudDialogflowCxV3RolloutConfigRolloutStep {}


/// State of the auto-rollout process.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3RolloutState {
    /// Start time of the current step.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Display name of the current auto rollout step.
    
    pub step: Option<String>,
    /// Index of the current step in the auto rollout steps list.
    #[serde(rename="stepIndex")]
    
    pub step_index: Option<i32>,
}

impl client::Part for GoogleCloudDialogflowCxV3RolloutState {}


/// The request message for Environments.RunContinuousTest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments run continuous test projects](ProjectLocationAgentEnvironmentRunContinuousTestCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3RunContinuousTestRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDialogflowCxV3RunContinuousTestRequest {}


/// The request message for TestCases.RunTestCase.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents test cases run projects](ProjectLocationAgentTestCaseRunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3RunTestCaseRequest {
    /// Optional. Environment name. If not set, draft environment is assumed. Format: `projects//locations//agents//environments/`.
    
    pub environment: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3RunTestCaseRequest {}


/// Represents the settings related to security issues, such as data redaction and data retention. It may take hours for updates on the settings to propagate to all the related components and take effect.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations security settings create projects](ProjectLocationSecuritySettingCreateCall) (request|response)
/// * [locations security settings get projects](ProjectLocationSecuritySettingGetCall) (response)
/// * [locations security settings patch projects](ProjectLocationSecuritySettingPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3SecuritySettings {
    /// Controls audio export settings for post-conversation analytics when ingesting audio to conversations via Participants.AnalyzeContent or Participants.StreamingAnalyzeContent. If retention_strategy is set to REMOVE_AFTER_CONVERSATION or audio_export_settings.gcs_bucket is empty, audio export is disabled. If audio export is enabled, audio is recorded and saved to audio_export_settings.gcs_bucket, subject to retention policy of audio_export_settings.gcs_bucket. This setting won't effect audio input for implicit sessions via Sessions.DetectIntent or Sessions.StreamingDetectIntent.
    #[serde(rename="audioExportSettings")]
    
    pub audio_export_settings: Option<GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettings>,
    /// [DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. The `DLP De-identify Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, Dialogflow replaces sensitive info with `[redacted]` text. The template name will have one of the following formats: `projects//locations//deidentifyTemplates/` OR `organizations//locations//deidentifyTemplates/` Note: `deidentify_template` must be located in the same region as the `SecuritySettings`.
    #[serde(rename="deidentifyTemplate")]
    
    pub deidentify_template: Option<String>,
    /// Required. The human-readable name of the security settings, unique within the location.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Controls conversation exporting settings to Insights after conversation is completed. If retention_strategy is set to REMOVE_AFTER_CONVERSATION, Insights export is disabled no matter what you configure here.
    #[serde(rename="insightsExportSettings")]
    
    pub insights_export_settings: Option<GoogleCloudDialogflowCxV3SecuritySettingsInsightsExportSettings>,
    /// [DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. The `DLP Inspect Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, we use the default DLP inspect config. The template name will have one of the following formats: `projects//locations//inspectTemplates/` OR `organizations//locations//inspectTemplates/` Note: `inspect_template` must be located in the same region as the `SecuritySettings`.
    #[serde(rename="inspectTemplate")]
    
    pub inspect_template: Option<String>,
    /// Resource name of the settings. Required for the SecuritySettingsService.UpdateSecuritySettings method. SecuritySettingsService.CreateSecuritySettings populates the name automatically. Format: `projects//locations//securitySettings/`.
    
    pub name: Option<String>,
    /// List of types of data to remove when retention settings triggers purge.
    #[serde(rename="purgeDataTypes")]
    
    pub purge_data_types: Option<Vec<GoogleCloudDialogflowCxV3SecuritySettingPurgeDataTypesEnum>>,
    /// Defines the data for which Dialogflow applies redaction. Dialogflow does not redact data that it does not have access to – for example, Cloud logging.
    #[serde(rename="redactionScope")]
    
    pub redaction_scope: Option<GoogleCloudDialogflowCxV3SecuritySettingRedactionScopeEnum>,
    /// Strategy that defines how we do redaction.
    #[serde(rename="redactionStrategy")]
    
    pub redaction_strategy: Option<GoogleCloudDialogflowCxV3SecuritySettingRedactionStrategyEnum>,
    /// Retains the data for the specified number of days. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL.
    #[serde(rename="retentionWindowDays")]
    
    pub retention_window_days: Option<i32>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3SecuritySettings {}
impl client::ResponseResult for GoogleCloudDialogflowCxV3SecuritySettings {}


/// Settings for exporting audio.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettings {
    /// Filename pattern for exported audio.
    #[serde(rename="audioExportPattern")]
    
    pub audio_export_pattern: Option<String>,
    /// File format for exported audio file. Currently only in telephony recordings.
    #[serde(rename="audioFormat")]
    
    pub audio_format: Option<GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettingAudioFormatEnum>,
    /// Enable audio redaction if it is true.
    #[serde(rename="enableAudioRedaction")]
    
    pub enable_audio_redaction: Option<bool>,
    /// Cloud Storage bucket to export audio record to. Setting this field would grant the Storage Object Creator role to the Dialogflow Service Agent. API caller that tries to modify this field should have the permission of storage.buckets.setIamPolicy.
    #[serde(rename="gcsBucket")]
    
    pub gcs_bucket: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettings {}


/// Settings for exporting conversations to [Insights](https://cloud.google.com/contact-center/insights/docs).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3SecuritySettingsInsightsExportSettings {
    /// If enabled, we will automatically exports conversations to Insights and Insights runs its analyzers.
    #[serde(rename="enableInsightsExport")]
    
    pub enable_insights_export: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowCxV3SecuritySettingsInsightsExportSettings {}


/// The result of sentiment analysis. Sentiment analysis inspects user input and identifies the prevailing subjective opinion, especially to determine a user's attitude as positive, negative, or neutral.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3SentimentAnalysisResult {
    /// A non-negative number in the [0, +inf) range, which represents the absolute magnitude of sentiment, regardless of score (positive or negative).
    
    pub magnitude: Option<f32>,
    /// Sentiment score between -1.0 (negative sentiment) and 1.0 (positive sentiment).
    
    pub score: Option<f32>,
}

impl client::Part for GoogleCloudDialogflowCxV3SentimentAnalysisResult {}


/// Session entity types are referred to as **User** entity types and are entities that are built for an individual user such as favorites, preferences, playlists, and so on. You can redefine a session entity type at the session level to extend or replace a custom entity type at the user session level (we refer to the entity types defined at the agent level as “custom entity types”). Note: session entity types apply to all queries, regardless of the language. For more information about entity types, see the [Dialogflow documentation](https://cloud.google.com/dialogflow/docs/entities-overview).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments sessions entity types create projects](ProjectLocationAgentEnvironmentSessionEntityTypeCreateCall) (request|response)
/// * [locations agents environments sessions entity types get projects](ProjectLocationAgentEnvironmentSessionEntityTypeGetCall) (response)
/// * [locations agents environments sessions entity types patch projects](ProjectLocationAgentEnvironmentSessionEntityTypePatchCall) (request|response)
/// * [locations agents sessions entity types create projects](ProjectLocationAgentSessionEntityTypeCreateCall) (request|response)
/// * [locations agents sessions entity types get projects](ProjectLocationAgentSessionEntityTypeGetCall) (response)
/// * [locations agents sessions entity types patch projects](ProjectLocationAgentSessionEntityTypePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3SessionEntityType {
    /// Required. The collection of entities to override or supplement the custom entity type.
    
    pub entities: Option<Vec<GoogleCloudDialogflowCxV3EntityTypeEntity>>,
    /// Required. Indicates whether the additional data should override or supplement the custom entity type definition.
    #[serde(rename="entityOverrideMode")]
    
    pub entity_override_mode: Option<GoogleCloudDialogflowCxV3SessionEntityTypeEntityOverrideModeEnum>,
    /// Required. The unique identifier of the session entity type. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3SessionEntityType {}
impl client::ResponseResult for GoogleCloudDialogflowCxV3SessionEntityType {}


/// Settings related to speech recognition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3SpeechToTextSettings {
    /// Whether to use speech adaptation for speech recognition.
    #[serde(rename="enableSpeechAdaptation")]
    
    pub enable_speech_adaptation: Option<bool>,
}

impl client::Part for GoogleCloudDialogflowCxV3SpeechToTextSettings {}


/// The request message for Experiments.StartExperiment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments experiments start projects](ProjectLocationAgentEnvironmentExperimentStartCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3StartExperimentRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDialogflowCxV3StartExperimentRequest {}


/// The request message for Experiments.StopExperiment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents environments experiments stop projects](ProjectLocationAgentEnvironmentExperimentStopCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3StopExperimentRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDialogflowCxV3StopExperimentRequest {}


/// Configuration of how speech should be synthesized.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3SynthesizeSpeechConfig {
    /// Optional. An identifier which selects 'audio effects' profiles that are applied on (post synthesized) text to speech. Effects are applied on top of each other in the order they are given.
    #[serde(rename="effectsProfileId")]
    
    pub effects_profile_id: Option<Vec<String>>,
    /// Optional. Speaking pitch, in the range [-20.0, 20.0]. 20 means increase 20 semitones from the original pitch. -20 means decrease 20 semitones from the original pitch.
    
    pub pitch: Option<f64>,
    /// Optional. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is the normal native speed supported by the specific voice. 2.0 is twice as fast, and 0.5 is half as fast. If unset(0.0), defaults to the native 1.0 speed. Any other values < 0.25 or > 4.0 will return an error.
    #[serde(rename="speakingRate")]
    
    pub speaking_rate: Option<f64>,
    /// Optional. The desired voice of the synthesized audio.
    
    pub voice: Option<GoogleCloudDialogflowCxV3VoiceSelectionParams>,
    /// Optional. Volume gain (in dB) of the normal native volume supported by the specific voice, in the range [-96.0, 16.0]. If unset, or set to a value of 0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB) will play at approximately half the amplitude of the normal native signal amplitude. A value of +6.0 (dB) will play at approximately twice the amplitude of the normal native signal amplitude. We strongly recommend not to exceed +10 (dB) as there's usually no effective increase in loudness for any value greater than that.
    #[serde(rename="volumeGainDb")]
    
    pub volume_gain_db: Option<f64>,
}

impl client::Part for GoogleCloudDialogflowCxV3SynthesizeSpeechConfig {}


/// Represents a test case.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents test cases create projects](ProjectLocationAgentTestCaseCreateCall) (request|response)
/// * [locations agents test cases get projects](ProjectLocationAgentTestCaseGetCall) (response)
/// * [locations agents test cases patch projects](ProjectLocationAgentTestCasePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3TestCase {
    /// Output only. When the test was created.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The human-readable name of the test case, unique within the agent. Limit of 200 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The latest test result.
    #[serde(rename="lastTestResult")]
    
    pub last_test_result: Option<GoogleCloudDialogflowCxV3TestCaseResult>,
    /// The unique identifier of the test case. TestCases.CreateTestCase will populate the name automatically. Otherwise use format: `projects//locations//agents/ /testCases/`.
    
    pub name: Option<String>,
    /// Additional freeform notes about the test case. Limit of 400 characters.
    
    pub notes: Option<String>,
    /// Tags are short descriptions that users may apply to test cases for organizational and filtering purposes. Each tag should start with "#" and has a limit of 30 characters.
    
    pub tags: Option<Vec<String>>,
    /// The conversation turns uttered when the test case was created, in chronological order. These include the canonical set of agent utterances that should occur when the agent is working properly.
    #[serde(rename="testCaseConversationTurns")]
    
    pub test_case_conversation_turns: Option<Vec<GoogleCloudDialogflowCxV3ConversationTurn>>,
    /// Config for the test case.
    #[serde(rename="testConfig")]
    
    pub test_config: Option<GoogleCloudDialogflowCxV3TestConfig>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3TestCase {}
impl client::ResponseResult for GoogleCloudDialogflowCxV3TestCase {}


/// Represents a result from running a test case in an agent environment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents test cases results get projects](ProjectLocationAgentTestCaseResultGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3TestCaseResult {
    /// The conversation turns uttered during the test case replay in chronological order.
    #[serde(rename="conversationTurns")]
    
    pub conversation_turns: Option<Vec<GoogleCloudDialogflowCxV3ConversationTurn>>,
    /// Environment where the test was run. If not set, it indicates the draft environment.
    
    pub environment: Option<String>,
    /// The resource name for the test case result. Format: `projects//locations//agents//testCases/ /results/`.
    
    pub name: Option<String>,
    /// Whether the test case passed in the agent environment.
    #[serde(rename="testResult")]
    
    pub test_result: Option<GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum>,
    /// The time that the test was run.
    #[serde(rename="testTime")]
    
    pub test_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for GoogleCloudDialogflowCxV3TestCaseResult {}


/// Represents configurations for a test case.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3TestConfig {
    /// Flow name to start the test case with. Format: `projects//locations//agents//flows/`. Only one of `flow` and `page` should be set to indicate the starting point of the test case. If both are set, `page` takes precedence over `flow`. If neither is set, the test case will start with start page on the default start flow.
    
    pub flow: Option<String>,
    /// The page to start the test case with. Format: `projects//locations//agents//flows//pages/`. Only one of `flow` and `page` should be set to indicate the starting point of the test case. If both are set, `page` takes precedence over `flow`. If neither is set, the test case will start with start page on the default start flow.
    
    pub page: Option<String>,
    /// Session parameters to be compared when calculating differences.
    #[serde(rename="trackingParameters")]
    
    pub tracking_parameters: Option<Vec<String>>,
}

impl client::Part for GoogleCloudDialogflowCxV3TestConfig {}


/// The description of differences between original and replayed agent output.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3TestRunDifference {
    /// A description of the diff, showing the actual output vs expected output.
    
    pub description: Option<String>,
    /// The type of diff.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum>,
}

impl client::Part for GoogleCloudDialogflowCxV3TestRunDifference {}


/// Represents the natural language text to be processed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3TextInput {
    /// Required. The UTF-8 encoded natural language text to be processed. Text length must not exceed 256 characters.
    
    pub text: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3TextInput {}


/// The request message for Flows.TrainFlow.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows train projects](ProjectLocationAgentFlowTrainCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3TrainFlowRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudDialogflowCxV3TrainFlowRequest {}


/// Transition coverage represents the percentage of all possible page transitions (page-level transition routes and event handlers, excluding transition route groups) present within any of a parent's test cases.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3TransitionCoverage {
    /// The percent of transitions in the agent that are covered.
    #[serde(rename="coverageScore")]
    
    pub coverage_score: Option<f32>,
    /// The list of Transitions present in the agent.
    
    pub transitions: Option<Vec<GoogleCloudDialogflowCxV3TransitionCoverageTransition>>,
}

impl client::Part for GoogleCloudDialogflowCxV3TransitionCoverage {}


/// A transition in a page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3TransitionCoverageTransition {
    /// Whether or not the transition is covered by at least one of the agent's test cases.
    
    pub covered: Option<bool>,
    /// Event handler.
    #[serde(rename="eventHandler")]
    
    pub event_handler: Option<GoogleCloudDialogflowCxV3EventHandler>,
    /// The index of a transition in the transition list. Starting from 0.
    
    pub index: Option<i32>,
    /// The start node of a transition.
    
    pub source: Option<GoogleCloudDialogflowCxV3TransitionCoverageTransitionNode>,
    /// The end node of a transition.
    
    pub target: Option<GoogleCloudDialogflowCxV3TransitionCoverageTransitionNode>,
    /// Intent route or condition route.
    #[serde(rename="transitionRoute")]
    
    pub transition_route: Option<GoogleCloudDialogflowCxV3TransitionRoute>,
}

impl client::Part for GoogleCloudDialogflowCxV3TransitionCoverageTransition {}


/// The source or target of a transition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3TransitionCoverageTransitionNode {
    /// Indicates a transition to a Flow. Only some fields such as name and displayname will be set.
    
    pub flow: Option<GoogleCloudDialogflowCxV3Flow>,
    /// Indicates a transition to a Page. Only some fields such as name and displayname will be set.
    
    pub page: Option<GoogleCloudDialogflowCxV3Page>,
}

impl client::Part for GoogleCloudDialogflowCxV3TransitionCoverageTransitionNode {}


/// A transition route specifies a intent that can be matched and/or a data condition that can be evaluated during a session. When a specified transition is matched, the following actions are taken in order: * If there is a `trigger_fulfillment` associated with the transition, it will be called. * If there is a `target_page` associated with the transition, the session will transition into the specified page. * If there is a `target_flow` associated with the transition, the session will transition into the specified flow.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3TransitionRoute {
    /// The condition to evaluate against form parameters or session parameters. See the [conditions reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition). At least one of `intent` or `condition` must be specified. When both `intent` and `condition` are specified, the transition can only happen when both are fulfilled.
    
    pub condition: Option<String>,
    /// The unique identifier of an Intent. Format: `projects//locations//agents//intents/`. Indicates that the transition can only happen when the given intent is matched. At least one of `intent` or `condition` must be specified. When both `intent` and `condition` are specified, the transition can only happen when both are fulfilled.
    
    pub intent: Option<String>,
    /// Output only. The unique identifier of this transition route.
    
    pub name: Option<String>,
    /// The target flow to transition to. Format: `projects//locations//agents//flows/`.
    #[serde(rename="targetFlow")]
    
    pub target_flow: Option<String>,
    /// The target page to transition to. Format: `projects//locations//agents//flows//pages/`.
    #[serde(rename="targetPage")]
    
    pub target_page: Option<String>,
    /// The fulfillment to call when the condition is satisfied. At least one of `trigger_fulfillment` and `target` must be specified. When both are defined, `trigger_fulfillment` is executed first.
    #[serde(rename="triggerFulfillment")]
    
    pub trigger_fulfillment: Option<GoogleCloudDialogflowCxV3Fulfillment>,
}

impl client::Part for GoogleCloudDialogflowCxV3TransitionRoute {}


/// An TransitionRouteGroup represents a group of `TransitionRoutes` to be used by a Page.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows transition route groups create projects](ProjectLocationAgentFlowTransitionRouteGroupCreateCall) (request|response)
/// * [locations agents flows transition route groups get projects](ProjectLocationAgentFlowTransitionRouteGroupGetCall) (response)
/// * [locations agents flows transition route groups patch projects](ProjectLocationAgentFlowTransitionRouteGroupPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3TransitionRouteGroup {
    /// Required. The human-readable name of the transition route group, unique within the flow. The display name can be no longer than 30 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The unique identifier of the transition route group. TransitionRouteGroups.CreateTransitionRouteGroup populates the name automatically. Format: `projects//locations//agents//flows//transitionRouteGroups/`.
    
    pub name: Option<String>,
    /// Transition routes associated with the TransitionRouteGroup.
    #[serde(rename="transitionRoutes")]
    
    pub transition_routes: Option<Vec<GoogleCloudDialogflowCxV3TransitionRoute>>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3TransitionRouteGroup {}
impl client::ResponseResult for GoogleCloudDialogflowCxV3TransitionRouteGroup {}


/// Transition route group coverage represents the percentage of all possible transition routes present within any of a parent's test cases. The results are grouped by the transition route group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3TransitionRouteGroupCoverage {
    /// The percent of transition routes in all the transition route groups that are covered.
    #[serde(rename="coverageScore")]
    
    pub coverage_score: Option<f32>,
    /// Transition route group coverages.
    
    pub coverages: Option<Vec<GoogleCloudDialogflowCxV3TransitionRouteGroupCoverageCoverage>>,
}

impl client::Part for GoogleCloudDialogflowCxV3TransitionRouteGroupCoverage {}


/// Coverage result message for one transition route group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3TransitionRouteGroupCoverageCoverage {
    /// The percent of transition routes in the transition route group that are covered.
    #[serde(rename="coverageScore")]
    
    pub coverage_score: Option<f32>,
    /// Transition route group metadata. Only name and displayName will be set.
    #[serde(rename="routeGroup")]
    
    pub route_group: Option<GoogleCloudDialogflowCxV3TransitionRouteGroup>,
    /// The list of transition routes and coverage in the transition route group.
    
    pub transitions: Option<Vec<GoogleCloudDialogflowCxV3TransitionRouteGroupCoverageCoverageTransition>>,
}

impl client::Part for GoogleCloudDialogflowCxV3TransitionRouteGroupCoverageCoverage {}


/// A transition coverage in a transition route group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3TransitionRouteGroupCoverageCoverageTransition {
    /// Whether or not the transition route is covered by at least one of the agent's test cases.
    
    pub covered: Option<bool>,
    /// Intent route or condition route.
    #[serde(rename="transitionRoute")]
    
    pub transition_route: Option<GoogleCloudDialogflowCxV3TransitionRoute>,
}

impl client::Part for GoogleCloudDialogflowCxV3TransitionRouteGroupCoverageCoverageTransition {}


/// The request message for Agents.ValidateAgent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents validate projects](ProjectLocationAgentValidateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ValidateAgentRequest {
    /// If not specified, the agent's default language is used.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3ValidateAgentRequest {}


/// The request message for Flows.ValidateFlow.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows validate projects](ProjectLocationAgentFlowValidateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ValidateFlowRequest {
    /// If not specified, the agent's default language is used.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3ValidateFlowRequest {}


/// Agent/flow validation message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3ValidationMessage {
    /// The message detail.
    
    pub detail: Option<String>,
    /// The resource names of the resources where the message is found.
    #[serde(rename="resourceNames")]
    
    pub resource_names: Option<Vec<GoogleCloudDialogflowCxV3ResourceName>>,
    /// The type of the resources where the message is found.
    #[serde(rename="resourceType")]
    
    pub resource_type: Option<GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum>,
    /// The names of the resources where the message is found.
    
    pub resources: Option<Vec<String>>,
    /// Indicates the severity of the message.
    
    pub severity: Option<GoogleCloudDialogflowCxV3ValidationMessageSeverityEnum>,
}

impl client::Part for GoogleCloudDialogflowCxV3ValidationMessage {}


/// The history of variants update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3VariantsHistory {
    /// Update time of the variants.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The flow versions as the variants.
    #[serde(rename="versionVariants")]
    
    pub version_variants: Option<GoogleCloudDialogflowCxV3VersionVariants>,
}

impl client::Part for GoogleCloudDialogflowCxV3VariantsHistory {}


/// Represents a version of a flow.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents flows versions create projects](ProjectLocationAgentFlowVersionCreateCall) (request)
/// * [locations agents flows versions get projects](ProjectLocationAgentFlowVersionGetCall) (response)
/// * [locations agents flows versions patch projects](ProjectLocationAgentFlowVersionPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3Version {
    /// Output only. Create time of the version.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The description of the version. The maximum length is 500 characters. If exceeded, the request is rejected.
    
    pub description: Option<String>,
    /// Required. The human-readable name of the version. Limit of 64 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Format: projects//locations//agents//flows//versions/. Version ID is a self-increasing number generated by Dialogflow upon version creation.
    
    pub name: Option<String>,
    /// Output only. The NLU settings of the flow at version creation.
    #[serde(rename="nluSettings")]
    
    pub nlu_settings: Option<GoogleCloudDialogflowCxV3NluSettings>,
    /// Output only. The state of this version. This field is read-only and cannot be set by create and update methods.
    
    pub state: Option<GoogleCloudDialogflowCxV3VersionStateEnum>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3Version {}
impl client::ResponseResult for GoogleCloudDialogflowCxV3Version {}


/// A list of flow version variants.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3VersionVariants {
    /// A list of flow version variants.
    
    pub variants: Option<Vec<GoogleCloudDialogflowCxV3VersionVariantsVariant>>,
}

impl client::Part for GoogleCloudDialogflowCxV3VersionVariants {}


/// A single flow version with specified traffic allocation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3VersionVariantsVariant {
    /// Whether the variant is for the control group.
    #[serde(rename="isControlGroup")]
    
    pub is_control_group: Option<bool>,
    /// Percentage of the traffic which should be routed to this version of flow. Traffic allocation for a single flow must sum up to 1.0.
    #[serde(rename="trafficAllocation")]
    
    pub traffic_allocation: Option<f32>,
    /// The name of the flow version. Format: `projects//locations//agents//flows//versions/`.
    
    pub version: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3VersionVariantsVariant {}


/// Description of which voice to use for speech synthesis.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3VoiceSelectionParams {
    /// Optional. The name of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and ssml_gender. For the list of available voices, please refer to [Supported voices and languages](https://cloud.google.com/text-to-speech/docs/voices).
    
    pub name: Option<String>,
    /// Optional. The preferred gender of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and name. Note that this is only a preference, not requirement. If a voice of the appropriate gender is not available, the synthesizer substitutes a voice with a different gender rather than failing the request.
    #[serde(rename="ssmlGender")]
    
    pub ssml_gender: Option<GoogleCloudDialogflowCxV3VoiceSelectionParamSsmlGenderEnum>,
}

impl client::Part for GoogleCloudDialogflowCxV3VoiceSelectionParams {}


/// Webhooks host the developer’s business logic. During a session, webhooks allow the developer to use the data extracted by Dialogflow’s natural language processing to generate dynamic responses, validate collected data, or trigger actions on the backend.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations agents webhooks create projects](ProjectLocationAgentWebhookCreateCall) (request|response)
/// * [locations agents webhooks get projects](ProjectLocationAgentWebhookGetCall) (response)
/// * [locations agents webhooks patch projects](ProjectLocationAgentWebhookPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3Webhook {
    /// Indicates whether the webhook is disabled.
    
    pub disabled: Option<bool>,
    /// Required. The human-readable name of the webhook, unique within the agent.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Configuration for a generic web service.
    #[serde(rename="genericWebService")]
    
    pub generic_web_service: Option<GoogleCloudDialogflowCxV3WebhookGenericWebService>,
    /// The unique identifier of the webhook. Required for the Webhooks.UpdateWebhook method. Webhooks.CreateWebhook populates the name automatically. Format: `projects//locations//agents//webhooks/`.
    
    pub name: Option<String>,
    /// Configuration for a [Service Directory](https://cloud.google.com/service-directory) service.
    #[serde(rename="serviceDirectory")]
    
    pub service_directory: Option<GoogleCloudDialogflowCxV3WebhookServiceDirectoryConfig>,
    /// Webhook execution timeout. Execution is considered failed if Dialogflow doesn't receive a response from webhook at the end of the timeout period. Defaults to 5 seconds, maximum allowed timeout is 30 seconds.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub timeout: Option<client::chrono::Duration>,
}

impl client::RequestValue for GoogleCloudDialogflowCxV3Webhook {}
impl client::ResponseResult for GoogleCloudDialogflowCxV3Webhook {}


/// Represents configuration for a generic web service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3WebhookGenericWebService {
    /// Optional. Specifies a list of allowed custom CA certificates (in DER format) for HTTPS verification. This overrides the default SSL trust store. If this is empty or unspecified, Dialogflow will use Google's default trust store to verify certificates. N.B. Make sure the HTTPS server certificates are signed with "subject alt name". For instance a certificate can be self-signed using the following command, ``` openssl x509 -req -days 200 -in example.com.csr \ -signkey example.com.key \ -out example.com.crt \ -extfile <(printf "\nsubjectAltName='DNS:www.example.com'") ```
    #[serde(rename="allowedCaCerts")]
    
    #[serde_as(as = "Option<Vec<::client::serde::urlsafe_base64::Wrapper>>")]
    pub allowed_ca_certs: Option<Vec<Vec<u8>>>,
    /// The password for HTTP Basic authentication.
    
    pub password: Option<String>,
    /// The HTTP request headers to send together with webhook requests.
    #[serde(rename="requestHeaders")]
    
    pub request_headers: Option<HashMap<String, String>>,
    /// Required. The webhook URI for receiving POST requests. It must use https protocol.
    
    pub uri: Option<String>,
    /// The user name for HTTP Basic authentication.
    
    pub username: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3WebhookGenericWebService {}


/// Represents configuration for a [Service Directory](https://cloud.google.com/service-directory) service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudDialogflowCxV3WebhookServiceDirectoryConfig {
    /// Generic Service configuration of this webhook.
    #[serde(rename="genericWebService")]
    
    pub generic_web_service: Option<GoogleCloudDialogflowCxV3WebhookGenericWebService>,
    /// Required. The name of [Service Directory](https://cloud.google.com/service-directory) service. Format: `projects//locations//namespaces//services/`. `Location ID` of the service directory must be the same as the location of the agent.
    
    pub service: Option<String>,
}

impl client::Part for GoogleCloudDialogflowCxV3WebhookServiceDirectoryConfig {}


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
/// * [locations agents environments create projects](ProjectLocationAgentEnvironmentCreateCall) (response)
/// * [locations agents environments deploy flow projects](ProjectLocationAgentEnvironmentDeployFlowCall) (response)
/// * [locations agents environments patch projects](ProjectLocationAgentEnvironmentPatchCall) (response)
/// * [locations agents environments run continuous test projects](ProjectLocationAgentEnvironmentRunContinuousTestCall) (response)
/// * [locations agents flows versions create projects](ProjectLocationAgentFlowVersionCreateCall) (response)
/// * [locations agents flows versions load projects](ProjectLocationAgentFlowVersionLoadCall) (response)
/// * [locations agents flows export projects](ProjectLocationAgentFlowExportCall) (response)
/// * [locations agents flows import projects](ProjectLocationAgentFlowImportCall) (response)
/// * [locations agents flows train projects](ProjectLocationAgentFlowTrainCall) (response)
/// * [locations agents test cases batch run projects](ProjectLocationAgentTestCaseBatchRunCall) (response)
/// * [locations agents test cases export projects](ProjectLocationAgentTestCaseExportCall) (response)
/// * [locations agents test cases import projects](ProjectLocationAgentTestCaseImportCall) (response)
/// * [locations agents test cases run projects](ProjectLocationAgentTestCaseRunCall) (response)
/// * [locations agents export projects](ProjectLocationAgentExportCall) (response)
/// * [locations agents restore projects](ProjectLocationAgentRestoreCall) (response)
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
/// * [locations agents entity types delete projects](ProjectLocationAgentEntityTypeDeleteCall) (response)
/// * [locations agents environments experiments delete projects](ProjectLocationAgentEnvironmentExperimentDeleteCall) (response)
/// * [locations agents environments sessions entity types delete projects](ProjectLocationAgentEnvironmentSessionEntityTypeDeleteCall) (response)
/// * [locations agents environments delete projects](ProjectLocationAgentEnvironmentDeleteCall) (response)
/// * [locations agents flows pages delete projects](ProjectLocationAgentFlowPageDeleteCall) (response)
/// * [locations agents flows transition route groups delete projects](ProjectLocationAgentFlowTransitionRouteGroupDeleteCall) (response)
/// * [locations agents flows versions delete projects](ProjectLocationAgentFlowVersionDeleteCall) (response)
/// * [locations agents flows delete projects](ProjectLocationAgentFlowDeleteCall) (response)
/// * [locations agents intents delete projects](ProjectLocationAgentIntentDeleteCall) (response)
/// * [locations agents sessions entity types delete projects](ProjectLocationAgentSessionEntityTypeDeleteCall) (response)
/// * [locations agents test cases batch delete projects](ProjectLocationAgentTestCaseBatchDeleteCall) (response)
/// * [locations agents webhooks delete projects](ProjectLocationAgentWebhookDeleteCall) (response)
/// * [locations agents delete projects](ProjectLocationAgentDeleteCall) (response)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations security settings delete projects](ProjectLocationSecuritySettingDeleteCall) (response)
/// * [operations cancel projects](ProjectOperationCancelCall) (response)
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


