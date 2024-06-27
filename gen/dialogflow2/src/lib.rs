// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Dialogflow* crate version *5.0.5+20240614*, where *20240614* is the exact revision of the *dialogflow:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Dialogflow* *v2* API can be found at the
//! [official documentation site](https://cloud.google.com/dialogflow/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/dialogflow2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](Dialogflow) ... 
//! 
//! * projects
//!  * [*agent entity types batch delete*](api::ProjectAgentEntityTypeBatchDeleteCall), [*agent entity types batch update*](api::ProjectAgentEntityTypeBatchUpdateCall), [*agent entity types create*](api::ProjectAgentEntityTypeCreateCall), [*agent entity types delete*](api::ProjectAgentEntityTypeDeleteCall), [*agent entity types entities batch create*](api::ProjectAgentEntityTypeEntityBatchCreateCall), [*agent entity types entities batch delete*](api::ProjectAgentEntityTypeEntityBatchDeleteCall), [*agent entity types entities batch update*](api::ProjectAgentEntityTypeEntityBatchUpdateCall), [*agent entity types get*](api::ProjectAgentEntityTypeGetCall), [*agent entity types list*](api::ProjectAgentEntityTypeListCall), [*agent entity types patch*](api::ProjectAgentEntityTypePatchCall), [*agent environments create*](api::ProjectAgentEnvironmentCreateCall), [*agent environments delete*](api::ProjectAgentEnvironmentDeleteCall), [*agent environments get*](api::ProjectAgentEnvironmentGetCall), [*agent environments get history*](api::ProjectAgentEnvironmentGetHistoryCall), [*agent environments intents list*](api::ProjectAgentEnvironmentIntentListCall), [*agent environments list*](api::ProjectAgentEnvironmentListCall), [*agent environments patch*](api::ProjectAgentEnvironmentPatchCall), [*agent environments users sessions contexts create*](api::ProjectAgentEnvironmentUserSessionContextCreateCall), [*agent environments users sessions contexts delete*](api::ProjectAgentEnvironmentUserSessionContextDeleteCall), [*agent environments users sessions contexts get*](api::ProjectAgentEnvironmentUserSessionContextGetCall), [*agent environments users sessions contexts list*](api::ProjectAgentEnvironmentUserSessionContextListCall), [*agent environments users sessions contexts patch*](api::ProjectAgentEnvironmentUserSessionContextPatchCall), [*agent environments users sessions delete contexts*](api::ProjectAgentEnvironmentUserSessionDeleteContextCall), [*agent environments users sessions detect intent*](api::ProjectAgentEnvironmentUserSessionDetectIntentCall), [*agent environments users sessions entity types create*](api::ProjectAgentEnvironmentUserSessionEntityTypeCreateCall), [*agent environments users sessions entity types delete*](api::ProjectAgentEnvironmentUserSessionEntityTypeDeleteCall), [*agent environments users sessions entity types get*](api::ProjectAgentEnvironmentUserSessionEntityTypeGetCall), [*agent environments users sessions entity types list*](api::ProjectAgentEnvironmentUserSessionEntityTypeListCall), [*agent environments users sessions entity types patch*](api::ProjectAgentEnvironmentUserSessionEntityTypePatchCall), [*agent export*](api::ProjectAgentExportCall), [*agent get fulfillment*](api::ProjectAgentGetFulfillmentCall), [*agent get validation result*](api::ProjectAgentGetValidationResultCall), [*agent import*](api::ProjectAgentImportCall), [*agent intents batch delete*](api::ProjectAgentIntentBatchDeleteCall), [*agent intents batch update*](api::ProjectAgentIntentBatchUpdateCall), [*agent intents create*](api::ProjectAgentIntentCreateCall), [*agent intents delete*](api::ProjectAgentIntentDeleteCall), [*agent intents get*](api::ProjectAgentIntentGetCall), [*agent intents list*](api::ProjectAgentIntentListCall), [*agent intents patch*](api::ProjectAgentIntentPatchCall), [*agent knowledge bases create*](api::ProjectAgentKnowledgeBaseCreateCall), [*agent knowledge bases delete*](api::ProjectAgentKnowledgeBaseDeleteCall), [*agent knowledge bases documents create*](api::ProjectAgentKnowledgeBaseDocumentCreateCall), [*agent knowledge bases documents delete*](api::ProjectAgentKnowledgeBaseDocumentDeleteCall), [*agent knowledge bases documents get*](api::ProjectAgentKnowledgeBaseDocumentGetCall), [*agent knowledge bases documents list*](api::ProjectAgentKnowledgeBaseDocumentListCall), [*agent knowledge bases documents patch*](api::ProjectAgentKnowledgeBaseDocumentPatchCall), [*agent knowledge bases documents reload*](api::ProjectAgentKnowledgeBaseDocumentReloadCall), [*agent knowledge bases get*](api::ProjectAgentKnowledgeBaseGetCall), [*agent knowledge bases list*](api::ProjectAgentKnowledgeBaseListCall), [*agent knowledge bases patch*](api::ProjectAgentKnowledgeBasePatchCall), [*agent restore*](api::ProjectAgentRestoreCall), [*agent search*](api::ProjectAgentSearchCall), [*agent sessions contexts create*](api::ProjectAgentSessionContextCreateCall), [*agent sessions contexts delete*](api::ProjectAgentSessionContextDeleteCall), [*agent sessions contexts get*](api::ProjectAgentSessionContextGetCall), [*agent sessions contexts list*](api::ProjectAgentSessionContextListCall), [*agent sessions contexts patch*](api::ProjectAgentSessionContextPatchCall), [*agent sessions delete contexts*](api::ProjectAgentSessionDeleteContextCall), [*agent sessions detect intent*](api::ProjectAgentSessionDetectIntentCall), [*agent sessions entity types create*](api::ProjectAgentSessionEntityTypeCreateCall), [*agent sessions entity types delete*](api::ProjectAgentSessionEntityTypeDeleteCall), [*agent sessions entity types get*](api::ProjectAgentSessionEntityTypeGetCall), [*agent sessions entity types list*](api::ProjectAgentSessionEntityTypeListCall), [*agent sessions entity types patch*](api::ProjectAgentSessionEntityTypePatchCall), [*agent train*](api::ProjectAgentTrainCall), [*agent update fulfillment*](api::ProjectAgentUpdateFulfillmentCall), [*agent versions create*](api::ProjectAgentVersionCreateCall), [*agent versions delete*](api::ProjectAgentVersionDeleteCall), [*agent versions get*](api::ProjectAgentVersionGetCall), [*agent versions list*](api::ProjectAgentVersionListCall), [*agent versions patch*](api::ProjectAgentVersionPatchCall), [*answer records list*](api::ProjectAnswerRecordListCall), [*answer records patch*](api::ProjectAnswerRecordPatchCall), [*conversation datasets get*](api::ProjectConversationDatasetGetCall), [*conversation datasets import conversation data*](api::ProjectConversationDatasetImportConversationDataCall), [*conversation datasets list*](api::ProjectConversationDatasetListCall), [*conversation models create*](api::ProjectConversationModelCreateCall), [*conversation models delete*](api::ProjectConversationModelDeleteCall), [*conversation models deploy*](api::ProjectConversationModelDeployCall), [*conversation models evaluations get*](api::ProjectConversationModelEvaluationGetCall), [*conversation models evaluations list*](api::ProjectConversationModelEvaluationListCall), [*conversation models get*](api::ProjectConversationModelGetCall), [*conversation models list*](api::ProjectConversationModelListCall), [*conversation models undeploy*](api::ProjectConversationModelUndeployCall), [*conversation profiles clear suggestion feature config*](api::ProjectConversationProfileClearSuggestionFeatureConfigCall), [*conversation profiles create*](api::ProjectConversationProfileCreateCall), [*conversation profiles delete*](api::ProjectConversationProfileDeleteCall), [*conversation profiles get*](api::ProjectConversationProfileGetCall), [*conversation profiles list*](api::ProjectConversationProfileListCall), [*conversation profiles patch*](api::ProjectConversationProfilePatchCall), [*conversation profiles set suggestion feature config*](api::ProjectConversationProfileSetSuggestionFeatureConfigCall), [*conversations complete*](api::ProjectConversationCompleteCall), [*conversations create*](api::ProjectConversationCreateCall), [*conversations get*](api::ProjectConversationGetCall), [*conversations list*](api::ProjectConversationListCall), [*conversations messages list*](api::ProjectConversationMessageListCall), [*conversations participants analyze content*](api::ProjectConversationParticipantAnalyzeContentCall), [*conversations participants create*](api::ProjectConversationParticipantCreateCall), [*conversations participants get*](api::ProjectConversationParticipantGetCall), [*conversations participants list*](api::ProjectConversationParticipantListCall), [*conversations participants patch*](api::ProjectConversationParticipantPatchCall), [*conversations participants suggestions suggest articles*](api::ProjectConversationParticipantSuggestionSuggestArticleCall), [*conversations participants suggestions suggest faq answers*](api::ProjectConversationParticipantSuggestionSuggestFaqAnswerCall), [*conversations participants suggestions suggest knowledge assist*](api::ProjectConversationParticipantSuggestionSuggestKnowledgeAssistCall), [*conversations participants suggestions suggest smart replies*](api::ProjectConversationParticipantSuggestionSuggestSmartReplyCall), [*conversations suggestions search knowledge*](api::ProjectConversationSuggestionSearchKnowledgeCall), [*conversations suggestions suggest conversation summary*](api::ProjectConversationSuggestionSuggestConversationSummaryCall), [*delete agent*](api::ProjectDeleteAgentCall), [*generators create*](api::ProjectGeneratorCreateCall), [*generators list*](api::ProjectGeneratorListCall), [*get agent*](api::ProjectGetAgentCall), [*knowledge bases create*](api::ProjectKnowledgeBaseCreateCall), [*knowledge bases delete*](api::ProjectKnowledgeBaseDeleteCall), [*knowledge bases documents create*](api::ProjectKnowledgeBaseDocumentCreateCall), [*knowledge bases documents delete*](api::ProjectKnowledgeBaseDocumentDeleteCall), [*knowledge bases documents export*](api::ProjectKnowledgeBaseDocumentExportCall), [*knowledge bases documents get*](api::ProjectKnowledgeBaseDocumentGetCall), [*knowledge bases documents import*](api::ProjectKnowledgeBaseDocumentImportCall), [*knowledge bases documents list*](api::ProjectKnowledgeBaseDocumentListCall), [*knowledge bases documents patch*](api::ProjectKnowledgeBaseDocumentPatchCall), [*knowledge bases documents reload*](api::ProjectKnowledgeBaseDocumentReloadCall), [*knowledge bases get*](api::ProjectKnowledgeBaseGetCall), [*knowledge bases list*](api::ProjectKnowledgeBaseListCall), [*knowledge bases patch*](api::ProjectKnowledgeBasePatchCall), [*locations agent entity types batch delete*](api::ProjectLocationAgentEntityTypeBatchDeleteCall), [*locations agent entity types batch update*](api::ProjectLocationAgentEntityTypeBatchUpdateCall), [*locations agent entity types create*](api::ProjectLocationAgentEntityTypeCreateCall), [*locations agent entity types delete*](api::ProjectLocationAgentEntityTypeDeleteCall), [*locations agent entity types entities batch create*](api::ProjectLocationAgentEntityTypeEntityBatchCreateCall), [*locations agent entity types entities batch delete*](api::ProjectLocationAgentEntityTypeEntityBatchDeleteCall), [*locations agent entity types entities batch update*](api::ProjectLocationAgentEntityTypeEntityBatchUpdateCall), [*locations agent entity types get*](api::ProjectLocationAgentEntityTypeGetCall), [*locations agent entity types list*](api::ProjectLocationAgentEntityTypeListCall), [*locations agent entity types patch*](api::ProjectLocationAgentEntityTypePatchCall), [*locations agent environments create*](api::ProjectLocationAgentEnvironmentCreateCall), [*locations agent environments delete*](api::ProjectLocationAgentEnvironmentDeleteCall), [*locations agent environments get*](api::ProjectLocationAgentEnvironmentGetCall), [*locations agent environments get history*](api::ProjectLocationAgentEnvironmentGetHistoryCall), [*locations agent environments intents list*](api::ProjectLocationAgentEnvironmentIntentListCall), [*locations agent environments list*](api::ProjectLocationAgentEnvironmentListCall), [*locations agent environments patch*](api::ProjectLocationAgentEnvironmentPatchCall), [*locations agent environments users sessions contexts create*](api::ProjectLocationAgentEnvironmentUserSessionContextCreateCall), [*locations agent environments users sessions contexts delete*](api::ProjectLocationAgentEnvironmentUserSessionContextDeleteCall), [*locations agent environments users sessions contexts get*](api::ProjectLocationAgentEnvironmentUserSessionContextGetCall), [*locations agent environments users sessions contexts list*](api::ProjectLocationAgentEnvironmentUserSessionContextListCall), [*locations agent environments users sessions contexts patch*](api::ProjectLocationAgentEnvironmentUserSessionContextPatchCall), [*locations agent environments users sessions delete contexts*](api::ProjectLocationAgentEnvironmentUserSessionDeleteContextCall), [*locations agent environments users sessions detect intent*](api::ProjectLocationAgentEnvironmentUserSessionDetectIntentCall), [*locations agent environments users sessions entity types create*](api::ProjectLocationAgentEnvironmentUserSessionEntityTypeCreateCall), [*locations agent environments users sessions entity types delete*](api::ProjectLocationAgentEnvironmentUserSessionEntityTypeDeleteCall), [*locations agent environments users sessions entity types get*](api::ProjectLocationAgentEnvironmentUserSessionEntityTypeGetCall), [*locations agent environments users sessions entity types list*](api::ProjectLocationAgentEnvironmentUserSessionEntityTypeListCall), [*locations agent environments users sessions entity types patch*](api::ProjectLocationAgentEnvironmentUserSessionEntityTypePatchCall), [*locations agent export*](api::ProjectLocationAgentExportCall), [*locations agent get fulfillment*](api::ProjectLocationAgentGetFulfillmentCall), [*locations agent get validation result*](api::ProjectLocationAgentGetValidationResultCall), [*locations agent import*](api::ProjectLocationAgentImportCall), [*locations agent intents batch delete*](api::ProjectLocationAgentIntentBatchDeleteCall), [*locations agent intents batch update*](api::ProjectLocationAgentIntentBatchUpdateCall), [*locations agent intents create*](api::ProjectLocationAgentIntentCreateCall), [*locations agent intents delete*](api::ProjectLocationAgentIntentDeleteCall), [*locations agent intents get*](api::ProjectLocationAgentIntentGetCall), [*locations agent intents list*](api::ProjectLocationAgentIntentListCall), [*locations agent intents patch*](api::ProjectLocationAgentIntentPatchCall), [*locations agent restore*](api::ProjectLocationAgentRestoreCall), [*locations agent search*](api::ProjectLocationAgentSearchCall), [*locations agent sessions contexts create*](api::ProjectLocationAgentSessionContextCreateCall), [*locations agent sessions contexts delete*](api::ProjectLocationAgentSessionContextDeleteCall), [*locations agent sessions contexts get*](api::ProjectLocationAgentSessionContextGetCall), [*locations agent sessions contexts list*](api::ProjectLocationAgentSessionContextListCall), [*locations agent sessions contexts patch*](api::ProjectLocationAgentSessionContextPatchCall), [*locations agent sessions delete contexts*](api::ProjectLocationAgentSessionDeleteContextCall), [*locations agent sessions detect intent*](api::ProjectLocationAgentSessionDetectIntentCall), [*locations agent sessions entity types create*](api::ProjectLocationAgentSessionEntityTypeCreateCall), [*locations agent sessions entity types delete*](api::ProjectLocationAgentSessionEntityTypeDeleteCall), [*locations agent sessions entity types get*](api::ProjectLocationAgentSessionEntityTypeGetCall), [*locations agent sessions entity types list*](api::ProjectLocationAgentSessionEntityTypeListCall), [*locations agent sessions entity types patch*](api::ProjectLocationAgentSessionEntityTypePatchCall), [*locations agent train*](api::ProjectLocationAgentTrainCall), [*locations agent update fulfillment*](api::ProjectLocationAgentUpdateFulfillmentCall), [*locations agent versions create*](api::ProjectLocationAgentVersionCreateCall), [*locations agent versions delete*](api::ProjectLocationAgentVersionDeleteCall), [*locations agent versions get*](api::ProjectLocationAgentVersionGetCall), [*locations agent versions list*](api::ProjectLocationAgentVersionListCall), [*locations agent versions patch*](api::ProjectLocationAgentVersionPatchCall), [*locations answer records list*](api::ProjectLocationAnswerRecordListCall), [*locations answer records patch*](api::ProjectLocationAnswerRecordPatchCall), [*locations conversation datasets create*](api::ProjectLocationConversationDatasetCreateCall), [*locations conversation datasets delete*](api::ProjectLocationConversationDatasetDeleteCall), [*locations conversation datasets get*](api::ProjectLocationConversationDatasetGetCall), [*locations conversation datasets import conversation data*](api::ProjectLocationConversationDatasetImportConversationDataCall), [*locations conversation datasets list*](api::ProjectLocationConversationDatasetListCall), [*locations conversation models create*](api::ProjectLocationConversationModelCreateCall), [*locations conversation models delete*](api::ProjectLocationConversationModelDeleteCall), [*locations conversation models deploy*](api::ProjectLocationConversationModelDeployCall), [*locations conversation models evaluations create*](api::ProjectLocationConversationModelEvaluationCreateCall), [*locations conversation models evaluations get*](api::ProjectLocationConversationModelEvaluationGetCall), [*locations conversation models evaluations list*](api::ProjectLocationConversationModelEvaluationListCall), [*locations conversation models get*](api::ProjectLocationConversationModelGetCall), [*locations conversation models list*](api::ProjectLocationConversationModelListCall), [*locations conversation models undeploy*](api::ProjectLocationConversationModelUndeployCall), [*locations conversation profiles clear suggestion feature config*](api::ProjectLocationConversationProfileClearSuggestionFeatureConfigCall), [*locations conversation profiles create*](api::ProjectLocationConversationProfileCreateCall), [*locations conversation profiles delete*](api::ProjectLocationConversationProfileDeleteCall), [*locations conversation profiles get*](api::ProjectLocationConversationProfileGetCall), [*locations conversation profiles list*](api::ProjectLocationConversationProfileListCall), [*locations conversation profiles patch*](api::ProjectLocationConversationProfilePatchCall), [*locations conversation profiles set suggestion feature config*](api::ProjectLocationConversationProfileSetSuggestionFeatureConfigCall), [*locations conversations complete*](api::ProjectLocationConversationCompleteCall), [*locations conversations create*](api::ProjectLocationConversationCreateCall), [*locations conversations get*](api::ProjectLocationConversationGetCall), [*locations conversations list*](api::ProjectLocationConversationListCall), [*locations conversations messages list*](api::ProjectLocationConversationMessageListCall), [*locations conversations participants analyze content*](api::ProjectLocationConversationParticipantAnalyzeContentCall), [*locations conversations participants create*](api::ProjectLocationConversationParticipantCreateCall), [*locations conversations participants get*](api::ProjectLocationConversationParticipantGetCall), [*locations conversations participants list*](api::ProjectLocationConversationParticipantListCall), [*locations conversations participants patch*](api::ProjectLocationConversationParticipantPatchCall), [*locations conversations participants suggestions suggest articles*](api::ProjectLocationConversationParticipantSuggestionSuggestArticleCall), [*locations conversations participants suggestions suggest faq answers*](api::ProjectLocationConversationParticipantSuggestionSuggestFaqAnswerCall), [*locations conversations participants suggestions suggest knowledge assist*](api::ProjectLocationConversationParticipantSuggestionSuggestKnowledgeAssistCall), [*locations conversations participants suggestions suggest smart replies*](api::ProjectLocationConversationParticipantSuggestionSuggestSmartReplyCall), [*locations conversations suggestions search knowledge*](api::ProjectLocationConversationSuggestionSearchKnowledgeCall), [*locations conversations suggestions suggest conversation summary*](api::ProjectLocationConversationSuggestionSuggestConversationSummaryCall), [*locations delete agent*](api::ProjectLocationDeleteAgentCall), [*locations generators create*](api::ProjectLocationGeneratorCreateCall), [*locations generators delete*](api::ProjectLocationGeneratorDeleteCall), [*locations generators get*](api::ProjectLocationGeneratorGetCall), [*locations generators list*](api::ProjectLocationGeneratorListCall), [*locations generators patch*](api::ProjectLocationGeneratorPatchCall), [*locations get*](api::ProjectLocationGetCall), [*locations get agent*](api::ProjectLocationGetAgentCall), [*locations knowledge bases create*](api::ProjectLocationKnowledgeBaseCreateCall), [*locations knowledge bases delete*](api::ProjectLocationKnowledgeBaseDeleteCall), [*locations knowledge bases documents create*](api::ProjectLocationKnowledgeBaseDocumentCreateCall), [*locations knowledge bases documents delete*](api::ProjectLocationKnowledgeBaseDocumentDeleteCall), [*locations knowledge bases documents export*](api::ProjectLocationKnowledgeBaseDocumentExportCall), [*locations knowledge bases documents get*](api::ProjectLocationKnowledgeBaseDocumentGetCall), [*locations knowledge bases documents import*](api::ProjectLocationKnowledgeBaseDocumentImportCall), [*locations knowledge bases documents list*](api::ProjectLocationKnowledgeBaseDocumentListCall), [*locations knowledge bases documents patch*](api::ProjectLocationKnowledgeBaseDocumentPatchCall), [*locations knowledge bases documents reload*](api::ProjectLocationKnowledgeBaseDocumentReloadCall), [*locations knowledge bases get*](api::ProjectLocationKnowledgeBaseGetCall), [*locations knowledge bases list*](api::ProjectLocationKnowledgeBaseListCall), [*locations knowledge bases patch*](api::ProjectLocationKnowledgeBasePatchCall), [*locations list*](api::ProjectLocationListCall), [*locations operations cancel*](api::ProjectLocationOperationCancelCall), [*locations operations get*](api::ProjectLocationOperationGetCall), [*locations operations list*](api::ProjectLocationOperationListCall), [*locations set agent*](api::ProjectLocationSetAgentCall), [*locations stateless suggestion generate*](api::ProjectLocationStatelessSuggestionGenerateCall), [*locations suggestions generate stateless summary*](api::ProjectLocationSuggestionGenerateStatelessSummaryCall), [*locations suggestions search knowledge*](api::ProjectLocationSuggestionSearchKnowledgeCall), [*operations cancel*](api::ProjectOperationCancelCall), [*operations get*](api::ProjectOperationGetCall), [*operations list*](api::ProjectOperationListCall), [*set agent*](api::ProjectSetAgentCall), [*suggestions generate stateless summary*](api::ProjectSuggestionGenerateStatelessSummaryCall) and [*suggestions search knowledge*](api::ProjectSuggestionSearchKnowledgeCall)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](Dialogflow)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](client::MethodsBuilder) which in turn
//!       allow access to individual [*Call Builders*](client::CallBuilder)
//! * **[Resources](client::Resource)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](client::Part)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](client::CallBuilder)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit().await
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.projects().agent_entity_types_entities_batch_create(...).doit().await
//! let r = hub.projects().agent_entity_types_entities_batch_delete(...).doit().await
//! let r = hub.projects().agent_entity_types_entities_batch_update(...).doit().await
//! let r = hub.projects().agent_entity_types_batch_delete(...).doit().await
//! let r = hub.projects().agent_entity_types_batch_update(...).doit().await
//! let r = hub.projects().agent_intents_batch_delete(...).doit().await
//! let r = hub.projects().agent_intents_batch_update(...).doit().await
//! let r = hub.projects().agent_knowledge_bases_documents_create(...).doit().await
//! let r = hub.projects().agent_knowledge_bases_documents_delete(...).doit().await
//! let r = hub.projects().agent_knowledge_bases_documents_patch(...).doit().await
//! let r = hub.projects().agent_knowledge_bases_documents_reload(...).doit().await
//! let r = hub.projects().agent_export(...).doit().await
//! let r = hub.projects().agent_import(...).doit().await
//! let r = hub.projects().agent_restore(...).doit().await
//! let r = hub.projects().agent_train(...).doit().await
//! let r = hub.projects().conversation_datasets_import_conversation_data(...).doit().await
//! let r = hub.projects().conversation_models_create(...).doit().await
//! let r = hub.projects().conversation_models_delete(...).doit().await
//! let r = hub.projects().conversation_models_deploy(...).doit().await
//! let r = hub.projects().conversation_models_undeploy(...).doit().await
//! let r = hub.projects().conversation_profiles_clear_suggestion_feature_config(...).doit().await
//! let r = hub.projects().conversation_profiles_set_suggestion_feature_config(...).doit().await
//! let r = hub.projects().knowledge_bases_documents_create(...).doit().await
//! let r = hub.projects().knowledge_bases_documents_delete(...).doit().await
//! let r = hub.projects().knowledge_bases_documents_export(...).doit().await
//! let r = hub.projects().knowledge_bases_documents_import(...).doit().await
//! let r = hub.projects().knowledge_bases_documents_patch(...).doit().await
//! let r = hub.projects().knowledge_bases_documents_reload(...).doit().await
//! let r = hub.projects().locations_agent_entity_types_entities_batch_create(...).doit().await
//! let r = hub.projects().locations_agent_entity_types_entities_batch_delete(...).doit().await
//! let r = hub.projects().locations_agent_entity_types_entities_batch_update(...).doit().await
//! let r = hub.projects().locations_agent_entity_types_batch_delete(...).doit().await
//! let r = hub.projects().locations_agent_entity_types_batch_update(...).doit().await
//! let r = hub.projects().locations_agent_intents_batch_delete(...).doit().await
//! let r = hub.projects().locations_agent_intents_batch_update(...).doit().await
//! let r = hub.projects().locations_agent_export(...).doit().await
//! let r = hub.projects().locations_agent_import(...).doit().await
//! let r = hub.projects().locations_agent_restore(...).doit().await
//! let r = hub.projects().locations_agent_train(...).doit().await
//! let r = hub.projects().locations_conversation_datasets_create(...).doit().await
//! let r = hub.projects().locations_conversation_datasets_delete(...).doit().await
//! let r = hub.projects().locations_conversation_datasets_import_conversation_data(...).doit().await
//! let r = hub.projects().locations_conversation_models_evaluations_create(...).doit().await
//! let r = hub.projects().locations_conversation_models_create(...).doit().await
//! let r = hub.projects().locations_conversation_models_delete(...).doit().await
//! let r = hub.projects().locations_conversation_models_deploy(...).doit().await
//! let r = hub.projects().locations_conversation_models_undeploy(...).doit().await
//! let r = hub.projects().locations_conversation_profiles_clear_suggestion_feature_config(...).doit().await
//! let r = hub.projects().locations_conversation_profiles_set_suggestion_feature_config(...).doit().await
//! let r = hub.projects().locations_knowledge_bases_documents_create(...).doit().await
//! let r = hub.projects().locations_knowledge_bases_documents_delete(...).doit().await
//! let r = hub.projects().locations_knowledge_bases_documents_export(...).doit().await
//! let r = hub.projects().locations_knowledge_bases_documents_import(...).doit().await
//! let r = hub.projects().locations_knowledge_bases_documents_patch(...).doit().await
//! let r = hub.projects().locations_knowledge_bases_documents_reload(...).doit().await
//! let r = hub.projects().locations_operations_get(...).doit().await
//! let r = hub.projects().operations_get(...).doit().await
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-dialogflow2 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_dialogflow2 as dialogflow2;
//! use dialogflow2::api::GoogleCloudDialogflowV2Document;
//! use dialogflow2::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use dialogflow2::{Dialogflow, oauth2, hyper, hyper_rustls, chrono, FieldMask};
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = oauth2::InstalledFlowAuthenticator::builder(
//!         secret,
//!         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//!     ).build().await.unwrap();
//! let mut hub = Dialogflow::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = GoogleCloudDialogflowV2Document::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().agent_knowledge_bases_documents_patch(req, "name")
//!              .update_mask(FieldMask::new::<&str>(&[]))
//!              .doit().await;
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::Io(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](client::Result) enumeration as return value of
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](client::Result), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](client::ResponseResult), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](client::Delegate) to the 
//! [Method Builder](client::CallBuilder) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](client::Delegate) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](client::RequestValue) and 
//! [decodable](client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](client::Part) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](client::RequestValue) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! ## Cargo Features
//! 
//! * `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
//! the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
//! generated `openapi` spec would be invalid.
//! 
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

// Re-export the hyper and hyper_rustls crate, they are required to build the hub
pub use hyper;
pub use hyper_rustls;
pub extern crate google_apis_common as client;
pub use client::chrono;
pub mod api;

// Re-export the hub type and some basic client structs
pub use api::Dialogflow;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;