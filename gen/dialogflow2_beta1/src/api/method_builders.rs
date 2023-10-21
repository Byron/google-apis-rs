use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Dialogflow`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dialogflow2_beta1 as dialogflow2_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dialogflow2_beta1::{Dialogflow, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dialogflow::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `agent_entity_types_batch_delete(...)`, `agent_entity_types_batch_update(...)`, `agent_entity_types_create(...)`, `agent_entity_types_delete(...)`, `agent_entity_types_entities_batch_create(...)`, `agent_entity_types_entities_batch_delete(...)`, `agent_entity_types_entities_batch_update(...)`, `agent_entity_types_get(...)`, `agent_entity_types_list(...)`, `agent_entity_types_patch(...)`, `agent_environments_create(...)`, `agent_environments_delete(...)`, `agent_environments_get(...)`, `agent_environments_get_history(...)`, `agent_environments_intents_list(...)`, `agent_environments_list(...)`, `agent_environments_patch(...)`, `agent_environments_users_sessions_contexts_create(...)`, `agent_environments_users_sessions_contexts_delete(...)`, `agent_environments_users_sessions_contexts_get(...)`, `agent_environments_users_sessions_contexts_list(...)`, `agent_environments_users_sessions_contexts_patch(...)`, `agent_environments_users_sessions_delete_contexts(...)`, `agent_environments_users_sessions_detect_intent(...)`, `agent_environments_users_sessions_entity_types_create(...)`, `agent_environments_users_sessions_entity_types_delete(...)`, `agent_environments_users_sessions_entity_types_get(...)`, `agent_environments_users_sessions_entity_types_list(...)`, `agent_environments_users_sessions_entity_types_patch(...)`, `agent_export(...)`, `agent_get_fulfillment(...)`, `agent_get_validation_result(...)`, `agent_import(...)`, `agent_intents_batch_delete(...)`, `agent_intents_batch_update(...)`, `agent_intents_create(...)`, `agent_intents_delete(...)`, `agent_intents_get(...)`, `agent_intents_list(...)`, `agent_intents_patch(...)`, `agent_knowledge_bases_create(...)`, `agent_knowledge_bases_delete(...)`, `agent_knowledge_bases_documents_create(...)`, `agent_knowledge_bases_documents_delete(...)`, `agent_knowledge_bases_documents_get(...)`, `agent_knowledge_bases_documents_list(...)`, `agent_knowledge_bases_documents_patch(...)`, `agent_knowledge_bases_documents_reload(...)`, `agent_knowledge_bases_get(...)`, `agent_knowledge_bases_list(...)`, `agent_knowledge_bases_patch(...)`, `agent_restore(...)`, `agent_search(...)`, `agent_sessions_contexts_create(...)`, `agent_sessions_contexts_delete(...)`, `agent_sessions_contexts_get(...)`, `agent_sessions_contexts_list(...)`, `agent_sessions_contexts_patch(...)`, `agent_sessions_delete_contexts(...)`, `agent_sessions_detect_intent(...)`, `agent_sessions_entity_types_create(...)`, `agent_sessions_entity_types_delete(...)`, `agent_sessions_entity_types_get(...)`, `agent_sessions_entity_types_list(...)`, `agent_sessions_entity_types_patch(...)`, `agent_train(...)`, `agent_update_fulfillment(...)`, `agent_versions_create(...)`, `agent_versions_delete(...)`, `agent_versions_get(...)`, `agent_versions_list(...)`, `agent_versions_patch(...)`, `answer_records_get(...)`, `answer_records_list(...)`, `answer_records_patch(...)`, `conversation_profiles_clear_suggestion_feature_config(...)`, `conversation_profiles_create(...)`, `conversation_profiles_delete(...)`, `conversation_profiles_get(...)`, `conversation_profiles_list(...)`, `conversation_profiles_patch(...)`, `conversation_profiles_set_suggestion_feature_config(...)`, `conversations_complete(...)`, `conversations_create(...)`, `conversations_get(...)`, `conversations_list(...)`, `conversations_messages_batch_create(...)`, `conversations_messages_list(...)`, `conversations_participants_analyze_content(...)`, `conversations_participants_create(...)`, `conversations_participants_get(...)`, `conversations_participants_list(...)`, `conversations_participants_patch(...)`, `conversations_participants_suggestions_compile(...)`, `conversations_participants_suggestions_list(...)`, `conversations_participants_suggestions_suggest_articles(...)`, `conversations_participants_suggestions_suggest_faq_answers(...)`, `conversations_participants_suggestions_suggest_smart_replies(...)`, `conversations_suggestions_suggest_conversation_summary(...)`, `delete_agent(...)`, `get_agent(...)`, `knowledge_bases_create(...)`, `knowledge_bases_delete(...)`, `knowledge_bases_documents_create(...)`, `knowledge_bases_documents_delete(...)`, `knowledge_bases_documents_get(...)`, `knowledge_bases_documents_import(...)`, `knowledge_bases_documents_list(...)`, `knowledge_bases_documents_patch(...)`, `knowledge_bases_documents_reload(...)`, `knowledge_bases_get(...)`, `knowledge_bases_list(...)`, `knowledge_bases_patch(...)`, `locations_agent_entity_types_batch_delete(...)`, `locations_agent_entity_types_batch_update(...)`, `locations_agent_entity_types_create(...)`, `locations_agent_entity_types_delete(...)`, `locations_agent_entity_types_entities_batch_create(...)`, `locations_agent_entity_types_entities_batch_delete(...)`, `locations_agent_entity_types_entities_batch_update(...)`, `locations_agent_entity_types_get(...)`, `locations_agent_entity_types_list(...)`, `locations_agent_entity_types_patch(...)`, `locations_agent_environments_create(...)`, `locations_agent_environments_delete(...)`, `locations_agent_environments_get(...)`, `locations_agent_environments_get_history(...)`, `locations_agent_environments_intents_list(...)`, `locations_agent_environments_list(...)`, `locations_agent_environments_patch(...)`, `locations_agent_environments_users_sessions_contexts_create(...)`, `locations_agent_environments_users_sessions_contexts_delete(...)`, `locations_agent_environments_users_sessions_contexts_get(...)`, `locations_agent_environments_users_sessions_contexts_list(...)`, `locations_agent_environments_users_sessions_contexts_patch(...)`, `locations_agent_environments_users_sessions_delete_contexts(...)`, `locations_agent_environments_users_sessions_detect_intent(...)`, `locations_agent_environments_users_sessions_entity_types_create(...)`, `locations_agent_environments_users_sessions_entity_types_delete(...)`, `locations_agent_environments_users_sessions_entity_types_get(...)`, `locations_agent_environments_users_sessions_entity_types_list(...)`, `locations_agent_environments_users_sessions_entity_types_patch(...)`, `locations_agent_export(...)`, `locations_agent_get_fulfillment(...)`, `locations_agent_get_validation_result(...)`, `locations_agent_import(...)`, `locations_agent_intents_batch_delete(...)`, `locations_agent_intents_batch_update(...)`, `locations_agent_intents_create(...)`, `locations_agent_intents_delete(...)`, `locations_agent_intents_get(...)`, `locations_agent_intents_list(...)`, `locations_agent_intents_patch(...)`, `locations_agent_restore(...)`, `locations_agent_search(...)`, `locations_agent_sessions_contexts_create(...)`, `locations_agent_sessions_contexts_delete(...)`, `locations_agent_sessions_contexts_get(...)`, `locations_agent_sessions_contexts_list(...)`, `locations_agent_sessions_contexts_patch(...)`, `locations_agent_sessions_delete_contexts(...)`, `locations_agent_sessions_detect_intent(...)`, `locations_agent_sessions_entity_types_create(...)`, `locations_agent_sessions_entity_types_delete(...)`, `locations_agent_sessions_entity_types_get(...)`, `locations_agent_sessions_entity_types_list(...)`, `locations_agent_sessions_entity_types_patch(...)`, `locations_agent_train(...)`, `locations_agent_update_fulfillment(...)`, `locations_agent_versions_create(...)`, `locations_agent_versions_delete(...)`, `locations_agent_versions_get(...)`, `locations_agent_versions_list(...)`, `locations_agent_versions_patch(...)`, `locations_answer_records_get(...)`, `locations_answer_records_list(...)`, `locations_answer_records_patch(...)`, `locations_conversation_profiles_clear_suggestion_feature_config(...)`, `locations_conversation_profiles_create(...)`, `locations_conversation_profiles_delete(...)`, `locations_conversation_profiles_get(...)`, `locations_conversation_profiles_list(...)`, `locations_conversation_profiles_patch(...)`, `locations_conversation_profiles_set_suggestion_feature_config(...)`, `locations_conversations_complete(...)`, `locations_conversations_create(...)`, `locations_conversations_get(...)`, `locations_conversations_list(...)`, `locations_conversations_messages_batch_create(...)`, `locations_conversations_messages_list(...)`, `locations_conversations_participants_analyze_content(...)`, `locations_conversations_participants_create(...)`, `locations_conversations_participants_get(...)`, `locations_conversations_participants_list(...)`, `locations_conversations_participants_patch(...)`, `locations_conversations_participants_suggestions_suggest_articles(...)`, `locations_conversations_participants_suggestions_suggest_faq_answers(...)`, `locations_conversations_participants_suggestions_suggest_smart_replies(...)`, `locations_conversations_suggestions_suggest_conversation_summary(...)`, `locations_delete_agent(...)`, `locations_get(...)`, `locations_get_agent(...)`, `locations_knowledge_bases_create(...)`, `locations_knowledge_bases_delete(...)`, `locations_knowledge_bases_documents_create(...)`, `locations_knowledge_bases_documents_delete(...)`, `locations_knowledge_bases_documents_get(...)`, `locations_knowledge_bases_documents_import(...)`, `locations_knowledge_bases_documents_list(...)`, `locations_knowledge_bases_documents_patch(...)`, `locations_knowledge_bases_documents_reload(...)`, `locations_knowledge_bases_get(...)`, `locations_knowledge_bases_list(...)`, `locations_knowledge_bases_patch(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_set_agent(...)`, `operations_cancel(...)`, `operations_get(...)`, `operations_list(...)` and `set_agent(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dialogflow<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates multiple new entities in the specified entity type. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the entity type to create entities in. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`
    pub fn agent_entity_types_entities_batch_create(&self, request: GoogleCloudDialogflowV2beta1BatchCreateEntitiesRequest, parent: &str) -> ProjectAgentEntityTypeEntityBatchCreateCall<'a, S> {
        ProjectAgentEntityTypeEntityBatchCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes entities in the specified entity type. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the entity type to delete entries for. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`
    pub fn agent_entity_types_entities_batch_delete(&self, request: GoogleCloudDialogflowV2beta1BatchDeleteEntitiesRequest, parent: &str) -> ProjectAgentEntityTypeEntityBatchDeleteCall<'a, S> {
        ProjectAgentEntityTypeEntityBatchDeleteCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates or creates multiple entities in the specified entity type. This method does not affect entities in the entity type that aren't explicitly specified in the request. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training). This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the entity type to update or create entities in. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`
    pub fn agent_entity_types_entities_batch_update(&self, request: GoogleCloudDialogflowV2beta1BatchUpdateEntitiesRequest, parent: &str) -> ProjectAgentEntityTypeEntityBatchUpdateCall<'a, S> {
        ProjectAgentEntityTypeEntityBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes entity types in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the agent to delete all entities types for. Supported formats: - `projects//agent`, - `projects//locations//agent`.
    pub fn agent_entity_types_batch_delete(&self, request: GoogleCloudDialogflowV2beta1BatchDeleteEntityTypesRequest, parent: &str) -> ProjectAgentEntityTypeBatchDeleteCall<'a, S> {
        ProjectAgentEntityTypeBatchDeleteCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates/Creates multiple entity types in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: BatchUpdateEntityTypesResponse Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the agent to update or create entity types in. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn agent_entity_types_batch_update(&self, request: GoogleCloudDialogflowV2beta1BatchUpdateEntityTypesRequest, parent: &str) -> ProjectAgentEntityTypeBatchUpdateCall<'a, S> {
        ProjectAgentEntityTypeBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an entity type in the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to create a entity type for. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn agent_entity_types_create(&self, request: GoogleCloudDialogflowV2beta1EntityType, parent: &str) -> ProjectAgentEntityTypeCreateCall<'a, S> {
        ProjectAgentEntityTypeCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified entity type. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the entity type to delete. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`
    pub fn agent_entity_types_delete(&self, name: &str) -> ProjectAgentEntityTypeDeleteCall<'a, S> {
        ProjectAgentEntityTypeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified entity type.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the entity type. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`
    pub fn agent_entity_types_get(&self, name: &str) -> ProjectAgentEntityTypeGetCall<'a, S> {
        ProjectAgentEntityTypeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all entity types in the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The agent to list all entity types from. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn agent_entity_types_list(&self, parent: &str) -> ProjectAgentEntityTypeListCall<'a, S> {
        ProjectAgentEntityTypeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified entity type. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType and EntityTypes.BatchUpdateEntityTypes methods. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`
    pub fn agent_entity_types_patch(&self, request: GoogleCloudDialogflowV2beta1EntityType, name: &str) -> ProjectAgentEntityTypePatchCall<'a, S> {
        ProjectAgentEntityTypePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all intents in the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The agent to list all intents from. Format: `projects//agent` or `projects//locations//agent`. Alternatively, you can specify the environment to list intents for. Format: `projects//agent/environments/` or `projects//locations//agent/environments/`. Note: training phrases of the intents will not be returned for non-draft environment.
    pub fn agent_environments_intents_list(&self, parent: &str) -> ProjectAgentEnvironmentIntentListCall<'a, S> {
        ProjectAgentEnvironmentIntentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _intent_view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a context. If the specified context already exists, overrides the context.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The session to create a context for. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_environments_users_sessions_contexts_create(&self, request: GoogleCloudDialogflowV2beta1Context, parent: &str) -> ProjectAgentEnvironmentUserSessionContextCreateCall<'a, S> {
        ProjectAgentEnvironmentUserSessionContextCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified context.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the context to delete. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_environments_users_sessions_contexts_delete(&self, name: &str) -> ProjectAgentEnvironmentUserSessionContextDeleteCall<'a, S> {
        ProjectAgentEnvironmentUserSessionContextDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified context.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the context. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_environments_users_sessions_contexts_get(&self, name: &str) -> ProjectAgentEnvironmentUserSessionContextGetCall<'a, S> {
        ProjectAgentEnvironmentUserSessionContextGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all contexts in the specified session.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The session to list all contexts from. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_environments_users_sessions_contexts_list(&self, parent: &str) -> ProjectAgentEnvironmentUserSessionContextListCall<'a, S> {
        ProjectAgentEnvironmentUserSessionContextListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified context.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique identifier of the context. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, The `Context ID` is always converted to lowercase, may only contain characters in a-zA-Z0-9_-% and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size`
    pub fn agent_environments_users_sessions_contexts_patch(&self, request: GoogleCloudDialogflowV2beta1Context, name: &str) -> ProjectAgentEnvironmentUserSessionContextPatchCall<'a, S> {
        ProjectAgentEnvironmentUserSessionContextPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a session entity type. If the specified session entity type already exists, overrides the session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The session to create a session entity type for. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_environments_users_sessions_entity_types_create(&self, request: GoogleCloudDialogflowV2beta1SessionEntityType, parent: &str) -> ProjectAgentEnvironmentUserSessionEntityTypeCreateCall<'a, S> {
        ProjectAgentEnvironmentUserSessionEntityTypeCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the entity type to delete. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_environments_users_sessions_entity_types_delete(&self, name: &str) -> ProjectAgentEnvironmentUserSessionEntityTypeDeleteCall<'a, S> {
        ProjectAgentEnvironmentUserSessionEntityTypeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the session entity type. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_environments_users_sessions_entity_types_get(&self, name: &str) -> ProjectAgentEnvironmentUserSessionEntityTypeGetCall<'a, S> {
        ProjectAgentEnvironmentUserSessionEntityTypeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all session entity types in the specified session. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The session to list all session entity types from. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_environments_users_sessions_entity_types_list(&self, parent: &str) -> ProjectAgentEnvironmentUserSessionEntityTypeListCall<'a, S> {
        ProjectAgentEnvironmentUserSessionEntityTypeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique identifier of this session entity type. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. `` must be the display name of an existing entity type in the same agent that will be overridden or supplemented.
    pub fn agent_environments_users_sessions_entity_types_patch(&self, request: GoogleCloudDialogflowV2beta1SessionEntityType, name: &str) -> ProjectAgentEnvironmentUserSessionEntityTypePatchCall<'a, S> {
        ProjectAgentEnvironmentUserSessionEntityTypePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes all active contexts in the specified session.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the session to delete all contexts from. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_environments_users_sessions_delete_contexts(&self, parent: &str) -> ProjectAgentEnvironmentUserSessionDeleteContextCall<'a, S> {
        ProjectAgentEnvironmentUserSessionDeleteContextCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause contexts and session entity types to be updated, which in turn might affect results of future queries. If you might use [Agent Assist](https://cloud.google.com/dialogflow/docs/#aa) or other CCAI products now or in the future, consider using AnalyzeContent instead of `DetectIntent`. `AnalyzeContent` has additional functionality for Agent Assist and other CCAI products. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The name of the session this query is sent to. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment (`Environment ID` might be referred to as environment name at some places). If `User ID` is not specified, we are using "-". It's up to the API caller to choose an appropriate `Session ID` and `User Id`. They can be a random number or some type of user and session identifiers (preferably hashed). The length of the `Session ID` and `User ID` must not exceed 36 characters. For more information, see the [API interactions guide](https://cloud.google.com/dialogflow/docs/api-overview). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).
    pub fn agent_environments_users_sessions_detect_intent(&self, request: GoogleCloudDialogflowV2beta1DetectIntentRequest, session: &str) -> ProjectAgentEnvironmentUserSessionDetectIntentCall<'a, S> {
        ProjectAgentEnvironmentUserSessionDetectIntentCall {
            hub: self.hub,
            _request: request,
            _session: session.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an agent environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to create an environment for. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn agent_environments_create(&self, request: GoogleCloudDialogflowV2beta1Environment, parent: &str) -> ProjectAgentEnvironmentCreateCall<'a, S> {
        ProjectAgentEnvironmentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _environment_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified agent environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the environment to delete. / Format: - `projects//agent/environments/` - `projects//locations//agent/environments/`
    pub fn agent_environments_delete(&self, name: &str) -> ProjectAgentEnvironmentDeleteCall<'a, S> {
        ProjectAgentEnvironmentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified agent environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the environment. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/`
    pub fn agent_environments_get(&self, name: &str) -> ProjectAgentEnvironmentGetCall<'a, S> {
        ProjectAgentEnvironmentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the history of the specified environment.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the environment to retrieve history for. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/`
    pub fn agent_environments_get_history(&self, parent: &str) -> ProjectAgentEnvironmentGetHistoryCall<'a, S> {
        ProjectAgentEnvironmentGetHistoryCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all non-draft environments of the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The agent to list all environments from. Format: - `projects//agent` - `projects//locations//agent`
    pub fn agent_environments_list(&self, parent: &str) -> ProjectAgentEnvironmentListCall<'a, S> {
        ProjectAgentEnvironmentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified agent environment. This method allows you to deploy new agent versions into the environment. When an environment is pointed to a new agent version by setting `environment.agent_version`, the environment is temporarily set to the `LOADING` state. During that time, the environment keeps on serving the previous version of the agent. After the new agent version is done loading, the environment is set back to the `RUNNING` state. You can use "-" as Environment ID in environment name to update version in "draft" environment. WARNING: this will negate all recent changes to draft and can't be undone. You may want to save the draft to a version before calling this function.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The unique identifier of this agent environment. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/`
    pub fn agent_environments_patch(&self, request: GoogleCloudDialogflowV2beta1Environment, name: &str) -> ProjectAgentEnvironmentPatchCall<'a, S> {
        ProjectAgentEnvironmentPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _allow_load_to_draft_and_discard_changes: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes intents in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the agent to delete all entities types for. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn agent_intents_batch_delete(&self, request: GoogleCloudDialogflowV2beta1BatchDeleteIntentsRequest, parent: &str) -> ProjectAgentIntentBatchDeleteCall<'a, S> {
        ProjectAgentIntentBatchDeleteCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates/Creates multiple intents in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: BatchUpdateIntentsResponse Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the agent to update or create intents in. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn agent_intents_batch_update(&self, request: GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequest, parent: &str) -> ProjectAgentIntentBatchUpdateCall<'a, S> {
        ProjectAgentIntentBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an intent in the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to create a intent for. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn agent_intents_create(&self, request: GoogleCloudDialogflowV2beta1Intent, parent: &str) -> ProjectAgentIntentCreateCall<'a, S> {
        ProjectAgentIntentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _language_code: Default::default(),
            _intent_view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified intent and its direct or indirect followup intents. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the intent to delete. If this intent has direct or indirect followup intents, we also delete them. Supported formats: - `projects//agent/intents/` - `projects//locations//agent/intents/`
    pub fn agent_intents_delete(&self, name: &str) -> ProjectAgentIntentDeleteCall<'a, S> {
        ProjectAgentIntentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified intent.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the intent. Supported formats: - `projects//agent/intents/` - `projects//locations//agent/intents/`
    pub fn agent_intents_get(&self, name: &str) -> ProjectAgentIntentGetCall<'a, S> {
        ProjectAgentIntentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _language_code: Default::default(),
            _intent_view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all intents in the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The agent to list all intents from. Format: `projects//agent` or `projects//locations//agent`. Alternatively, you can specify the environment to list intents for. Format: `projects//agent/environments/` or `projects//locations//agent/environments/`. Note: training phrases of the intents will not be returned for non-draft environment.
    pub fn agent_intents_list(&self, parent: &str) -> ProjectAgentIntentListCall<'a, S> {
        ProjectAgentIntentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _intent_view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified intent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Supported formats: - `projects//agent/intents/` - `projects//locations//agent/intents/`
    pub fn agent_intents_patch(&self, request: GoogleCloudDialogflowV2beta1Intent, name: &str) -> ProjectAgentIntentPatchCall<'a, S> {
        ProjectAgentIntentPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _language_code: Default::default(),
            _intent_view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The knowledge base to create a document for. Format: `projects//locations//knowledgeBases/`.
    pub fn agent_knowledge_bases_documents_create(&self, request: GoogleCloudDialogflowV2beta1Document, parent: &str) -> ProjectAgentKnowledgeBaseDocumentCreateCall<'a, S> {
        ProjectAgentKnowledgeBaseDocumentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _import_gcs_custom_metadata: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the document to delete. Format: `projects//locations//knowledgeBases//documents/`.
    pub fn agent_knowledge_bases_documents_delete(&self, name: &str) -> ProjectAgentKnowledgeBaseDocumentDeleteCall<'a, S> {
        ProjectAgentKnowledgeBaseDocumentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified document. Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the document to retrieve. Format `projects//locations//knowledgeBases//documents/`.
    pub fn agent_knowledge_bases_documents_get(&self, name: &str) -> ProjectAgentKnowledgeBaseDocumentGetCall<'a, S> {
        ProjectAgentKnowledgeBaseDocumentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all documents of the knowledge base. Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The knowledge base to list all documents for. Format: `projects//locations//knowledgeBases/`.
    pub fn agent_knowledge_bases_documents_list(&self, parent: &str) -> ProjectAgentKnowledgeBaseDocumentListCall<'a, S> {
        ProjectAgentKnowledgeBaseDocumentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Optional. The document resource name. The name must be empty when creating a document. Format: `projects//locations//knowledgeBases//documents/`.
    pub fn agent_knowledge_bases_documents_patch(&self, request: GoogleCloudDialogflowV2beta1Document, name: &str) -> ProjectAgentKnowledgeBaseDocumentPatchCall<'a, S> {
        ProjectAgentKnowledgeBaseDocumentPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reloads the specified document from its specified source, content_uri or content. The previously loaded content of the document will be deleted. Note: Even when the content of the document has not changed, there still may be side effects because of internal implementation changes. Note: If the document source is Google Cloud Storage URI, its metadata will be replaced with the custom metadata from Google Cloud Storage if the `import_gcs_custom_metadata` field is set to true in the request. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the document to reload. Format: `projects//locations//knowledgeBases//documents/`
    pub fn agent_knowledge_bases_documents_reload(&self, request: GoogleCloudDialogflowV2beta1ReloadDocumentRequest, name: &str) -> ProjectAgentKnowledgeBaseDocumentReloadCall<'a, S> {
        ProjectAgentKnowledgeBaseDocumentReloadCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a knowledge base. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project to create a knowledge base for. Format: `projects//locations/`.
    pub fn agent_knowledge_bases_create(&self, request: GoogleCloudDialogflowV2beta1KnowledgeBase, parent: &str) -> ProjectAgentKnowledgeBaseCreateCall<'a, S> {
        ProjectAgentKnowledgeBaseCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified knowledge base. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the knowledge base to delete. Format: `projects//locations//knowledgeBases/`.
    pub fn agent_knowledge_bases_delete(&self, name: &str) -> ProjectAgentKnowledgeBaseDeleteCall<'a, S> {
        ProjectAgentKnowledgeBaseDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified knowledge base. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the knowledge base to retrieve. Format `projects//locations//knowledgeBases/`.
    pub fn agent_knowledge_bases_get(&self, name: &str) -> ProjectAgentKnowledgeBaseGetCall<'a, S> {
        ProjectAgentKnowledgeBaseGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all knowledge bases of the specified agent. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project to list of knowledge bases for. Format: `projects//locations/`.
    pub fn agent_knowledge_bases_list(&self, parent: &str) -> ProjectAgentKnowledgeBaseListCall<'a, S> {
        ProjectAgentKnowledgeBaseListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified knowledge base. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The knowledge base resource name. The name must be empty when creating a knowledge base. Format: `projects//locations//knowledgeBases/`.
    pub fn agent_knowledge_bases_patch(&self, request: GoogleCloudDialogflowV2beta1KnowledgeBase, name: &str) -> ProjectAgentKnowledgeBasePatchCall<'a, S> {
        ProjectAgentKnowledgeBasePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a context. If the specified context already exists, overrides the context.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The session to create a context for. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_sessions_contexts_create(&self, request: GoogleCloudDialogflowV2beta1Context, parent: &str) -> ProjectAgentSessionContextCreateCall<'a, S> {
        ProjectAgentSessionContextCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified context.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the context to delete. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_sessions_contexts_delete(&self, name: &str) -> ProjectAgentSessionContextDeleteCall<'a, S> {
        ProjectAgentSessionContextDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified context.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the context. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_sessions_contexts_get(&self, name: &str) -> ProjectAgentSessionContextGetCall<'a, S> {
        ProjectAgentSessionContextGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all contexts in the specified session.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The session to list all contexts from. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_sessions_contexts_list(&self, parent: &str) -> ProjectAgentSessionContextListCall<'a, S> {
        ProjectAgentSessionContextListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified context.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique identifier of the context. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, The `Context ID` is always converted to lowercase, may only contain characters in a-zA-Z0-9_-% and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size`
    pub fn agent_sessions_contexts_patch(&self, request: GoogleCloudDialogflowV2beta1Context, name: &str) -> ProjectAgentSessionContextPatchCall<'a, S> {
        ProjectAgentSessionContextPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a session entity type. If the specified session entity type already exists, overrides the session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The session to create a session entity type for. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_sessions_entity_types_create(&self, request: GoogleCloudDialogflowV2beta1SessionEntityType, parent: &str) -> ProjectAgentSessionEntityTypeCreateCall<'a, S> {
        ProjectAgentSessionEntityTypeCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the entity type to delete. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_sessions_entity_types_delete(&self, name: &str) -> ProjectAgentSessionEntityTypeDeleteCall<'a, S> {
        ProjectAgentSessionEntityTypeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the session entity type. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_sessions_entity_types_get(&self, name: &str) -> ProjectAgentSessionEntityTypeGetCall<'a, S> {
        ProjectAgentSessionEntityTypeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all session entity types in the specified session. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The session to list all session entity types from. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_sessions_entity_types_list(&self, parent: &str) -> ProjectAgentSessionEntityTypeListCall<'a, S> {
        ProjectAgentSessionEntityTypeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique identifier of this session entity type. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. `` must be the display name of an existing entity type in the same agent that will be overridden or supplemented.
    pub fn agent_sessions_entity_types_patch(&self, request: GoogleCloudDialogflowV2beta1SessionEntityType, name: &str) -> ProjectAgentSessionEntityTypePatchCall<'a, S> {
        ProjectAgentSessionEntityTypePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes all active contexts in the specified session.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the session to delete all contexts from. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn agent_sessions_delete_contexts(&self, parent: &str) -> ProjectAgentSessionDeleteContextCall<'a, S> {
        ProjectAgentSessionDeleteContextCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause contexts and session entity types to be updated, which in turn might affect results of future queries. If you might use [Agent Assist](https://cloud.google.com/dialogflow/docs/#aa) or other CCAI products now or in the future, consider using AnalyzeContent instead of `DetectIntent`. `AnalyzeContent` has additional functionality for Agent Assist and other CCAI products. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The name of the session this query is sent to. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment (`Environment ID` might be referred to as environment name at some places). If `User ID` is not specified, we are using "-". It's up to the API caller to choose an appropriate `Session ID` and `User Id`. They can be a random number or some type of user and session identifiers (preferably hashed). The length of the `Session ID` and `User ID` must not exceed 36 characters. For more information, see the [API interactions guide](https://cloud.google.com/dialogflow/docs/api-overview). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).
    pub fn agent_sessions_detect_intent(&self, request: GoogleCloudDialogflowV2beta1DetectIntentRequest, session: &str) -> ProjectAgentSessionDetectIntentCall<'a, S> {
        ProjectAgentSessionDetectIntentCall {
            hub: self.hub,
            _request: request,
            _session: session.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an agent version. The new version points to the agent instance in the "default" environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to create a version for. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn agent_versions_create(&self, request: GoogleCloudDialogflowV2beta1Version, parent: &str) -> ProjectAgentVersionCreateCall<'a, S> {
        ProjectAgentVersionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete the specified agent version.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the version to delete. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/`
    pub fn agent_versions_delete(&self, name: &str) -> ProjectAgentVersionDeleteCall<'a, S> {
        ProjectAgentVersionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified agent version.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the version. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/`
    pub fn agent_versions_get(&self, name: &str) -> ProjectAgentVersionGetCall<'a, S> {
        ProjectAgentVersionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all versions of the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The agent to list all versions from. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn agent_versions_list(&self, parent: &str) -> ProjectAgentVersionListCall<'a, S> {
        ProjectAgentVersionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified agent version. Note that this method does not allow you to update the state of the agent the given version points to. It allows you to update only mutable properties of the version resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The unique identifier of this agent version. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/`
    pub fn agent_versions_patch(&self, request: GoogleCloudDialogflowV2beta1Version, name: &str) -> ProjectAgentVersionPatchCall<'a, S> {
        ProjectAgentVersionPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports the specified agent to a ZIP file. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: ExportAgentResponse
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project that the agent to export is associated with. Format: `projects/` or `projects//locations/`.
    pub fn agent_export(&self, request: GoogleCloudDialogflowV2beta1ExportAgentRequest, parent: &str) -> ProjectAgentExportCall<'a, S> {
        ProjectAgentExportCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the fulfillment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the fulfillment. Supported formats: - `projects//agent/fulfillment` - `projects//locations//agent/fulfillment`
    pub fn agent_get_fulfillment(&self, name: &str) -> ProjectAgentGetFulfillmentCall<'a, S> {
        ProjectAgentGetFulfillmentCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets agent validation result. Agent validation is performed during training time and is updated automatically when training is completed.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project that the agent is associated with. Format: `projects/` or `projects//locations/`.
    pub fn agent_get_validation_result(&self, parent: &str) -> ProjectAgentGetValidationResultCall<'a, S> {
        ProjectAgentGetValidationResultCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Imports the specified agent from a ZIP file. Uploads new intents and entity types without deleting the existing ones. Intents and entity types with the same name are replaced with the new versions from ImportAgentRequest. After the import, the imported draft agent will be trained automatically (unless disabled in agent settings). However, once the import is done, training may not be completed yet. Please call TrainAgent and wait for the operation it returns in order to train explicitly. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) The operation only tracks when importing is complete, not when it is done training. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project that the agent to import is associated with. Format: `projects/` or `projects//locations/`.
    pub fn agent_import(&self, request: GoogleCloudDialogflowV2beta1ImportAgentRequest, parent: &str) -> ProjectAgentImportCall<'a, S> {
        ProjectAgentImportCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Restores the specified agent from a ZIP file. Replaces the current agent version with a new one. All the intents and entity types in the older version are deleted. After the restore, the restored draft agent will be trained automatically (unless disabled in agent settings). However, once the restore is done, training may not be completed yet. Please call TrainAgent and wait for the operation it returns in order to train explicitly. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) The operation only tracks when restoring is complete, not when it is done training. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project that the agent to restore is associated with. Format: `projects/` or `projects//locations/`.
    pub fn agent_restore(&self, request: GoogleCloudDialogflowV2beta1RestoreAgentRequest, parent: &str) -> ProjectAgentRestoreCall<'a, S> {
        ProjectAgentRestoreCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of agents. Since there is at most one conversational agent per project, this method is useful primarily for listing all agents across projects the caller has access to. One can achieve that with a wildcard project collection id "-". Refer to [List Sub-Collections](https://cloud.google.com/apis/design/design_patterns#list_sub-collections).
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project to list agents from. Format: `projects/` or `projects//locations/`.
    pub fn agent_search(&self, parent: &str) -> ProjectAgentSearchCall<'a, S> {
        ProjectAgentSearchCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Trains the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project that the agent to train is associated with. Format: `projects/` or `projects//locations/`.
    pub fn agent_train(&self, request: GoogleCloudDialogflowV2beta1TrainAgentRequest, parent: &str) -> ProjectAgentTrainCall<'a, S> {
        ProjectAgentTrainCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the fulfillment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique identifier of the fulfillment. Supported formats: - `projects//agent/fulfillment` - `projects//locations//agent/fulfillment` This field is not used for Fulfillment in an Environment.
    pub fn agent_update_fulfillment(&self, request: GoogleCloudDialogflowV2beta1Fulfillment, name: &str) -> ProjectAgentUpdateFulfillmentCall<'a, S> {
        ProjectAgentUpdateFulfillmentCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated. Retrieves a specific answer record.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the answer record to retrieve. Format: `projects//locations//answerRecords/`.
    pub fn answer_records_get(&self, name: &str) -> ProjectAnswerRecordGetCall<'a, S> {
        ProjectAnswerRecordGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all answer records in the specified project in reverse chronological order.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project to list all answer records for in reverse chronological order. Format: `projects//locations/`.
    pub fn answer_records_list(&self, parent: &str) -> ProjectAnswerRecordListCall<'a, S> {
        ProjectAnswerRecordListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified answer record.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique identifier of this answer record. Required for AnswerRecords.UpdateAnswerRecord method. Format: `projects//locations//answerRecords/`.
    pub fn answer_records_patch(&self, request: GoogleCloudDialogflowV2beta1AnswerRecord, name: &str) -> ProjectAnswerRecordPatchCall<'a, S> {
        ProjectAnswerRecordPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears a suggestion feature from a conversation profile for the given participant role. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: ClearSuggestionFeatureConfigOperationMetadata - `response`: ConversationProfile
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `conversationProfile` - Required. The Conversation Profile to add or update the suggestion feature config. Format: `projects//locations//conversationProfiles/`.
    pub fn conversation_profiles_clear_suggestion_feature_config(&self, request: GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequest, conversation_profile: &str) -> ProjectConversationProfileClearSuggestionFeatureConfigCall<'a, S> {
        ProjectConversationProfileClearSuggestionFeatureConfigCall {
            hub: self.hub,
            _request: request,
            _conversation_profile: conversation_profile.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a conversation profile in the specified project. ConversationProfile.CreateTime and ConversationProfile.UpdateTime aren't populated in the response. You can retrieve them via GetConversationProfile API.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project to create a conversation profile for. Format: `projects//locations/`.
    pub fn conversation_profiles_create(&self, request: GoogleCloudDialogflowV2beta1ConversationProfile, parent: &str) -> ProjectConversationProfileCreateCall<'a, S> {
        ProjectConversationProfileCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified conversation profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the conversation profile to delete. Format: `projects//locations//conversationProfiles/`.
    pub fn conversation_profiles_delete(&self, name: &str) -> ProjectConversationProfileDeleteCall<'a, S> {
        ProjectConversationProfileDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified conversation profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the conversation profile. Format: `projects//locations//conversationProfiles/`.
    pub fn conversation_profiles_get(&self, name: &str) -> ProjectConversationProfileGetCall<'a, S> {
        ProjectConversationProfileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all conversation profiles in the specified project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project to list all conversation profiles from. Format: `projects//locations/`.
    pub fn conversation_profiles_list(&self, parent: &str) -> ProjectConversationProfileListCall<'a, S> {
        ProjectConversationProfileListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified conversation profile. ConversationProfile.CreateTime and ConversationProfile.UpdateTime aren't populated in the response. You can retrieve them via GetConversationProfile API.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique identifier of this conversation profile. Format: `projects//locations//conversationProfiles/`.
    pub fn conversation_profiles_patch(&self, request: GoogleCloudDialogflowV2beta1ConversationProfile, name: &str) -> ProjectConversationProfilePatchCall<'a, S> {
        ProjectConversationProfilePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds or updates a suggestion feature in a conversation profile. If the conversation profile contains the type of suggestion feature for the participant role, it will update it. Otherwise it will insert the suggestion feature. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: SetSuggestionFeatureConfigOperationMetadata - `response`: ConversationProfile If a long running operation to add or update suggestion feature config for the same conversation profile, participant role and suggestion feature type exists, please cancel the existing long running operation before sending such request, otherwise the request will be rejected.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `conversationProfile` - Required. The Conversation Profile to add or update the suggestion feature config. Format: `projects//locations//conversationProfiles/`.
    pub fn conversation_profiles_set_suggestion_feature_config(&self, request: GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequest, conversation_profile: &str) -> ProjectConversationProfileSetSuggestionFeatureConfigCall<'a, S> {
        ProjectConversationProfileSetSuggestionFeatureConfigCall {
            hub: self.hub,
            _request: request,
            _conversation_profile: conversation_profile.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Batch ingests messages to conversation. Customers can use this RPC to ingest historical messages to conversation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource identifier of the conversation to create message. Format: `projects//locations//conversations/`.
    pub fn conversations_messages_batch_create(&self, request: GoogleCloudDialogflowV2beta1BatchCreateMessagesRequest, parent: &str) -> ProjectConversationMessageBatchCreateCall<'a, S> {
        ProjectConversationMessageBatchCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists messages that belong to a given conversation. `messages` are ordered by `create_time` in descending order. To fetch updates without duplication, send request with filter `create_time_epoch_microseconds > [first item's create_time of previous request]` and empty page_token.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the conversation to list messages for. Format: `projects//locations//conversations/`
    pub fn conversations_messages_list(&self, parent: &str) -> ProjectConversationMessageListCall<'a, S> {
        ProjectConversationMessageListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated. use SuggestArticles and SuggestFaqAnswers instead. Gets suggestions for a participant based on specific historical messages. Note that ListSuggestions will only list the auto-generated suggestions, while CompileSuggestion will try to compile suggestion based on the provided conversation context in the real time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the participant to fetch suggestion for. Format: `projects//locations//conversations//participants/`.
    pub fn conversations_participants_suggestions_compile(&self, request: GoogleCloudDialogflowV2beta1CompileSuggestionRequest, parent: &str) -> ProjectConversationParticipantSuggestionCompileCall<'a, S> {
        ProjectConversationParticipantSuggestionCompileCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated: Use inline suggestion, event based suggestion or Suggestion* API instead. See HumanAgentAssistantConfig.name for more details. Removal Date: 2020-09-01. Retrieves suggestions for live agents. This method should be used by human agent client software to fetch auto generated suggestions in real-time, while the conversation with an end user is in progress. The functionality is implemented in terms of the [list pagination](https://cloud.google.com/apis/design/design_patterns#list_pagination) design pattern. The client app should use the `next_page_token` field to fetch the next batch of suggestions. `suggestions` are sorted by `create_time` in descending order. To fetch latest suggestion, just set `page_size` to 1. To fetch new suggestions without duplication, send request with filter `create_time_epoch_microseconds > [first item's create_time of previous request]` and empty page_token.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the participant to fetch suggestions for. Format: `projects//locations//conversations//participants/`.
    pub fn conversations_participants_suggestions_list(&self, parent: &str) -> ProjectConversationParticipantSuggestionListCall<'a, S> {
        ProjectConversationParticipantSuggestionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets suggested articles for a participant based on specific historical messages. Note that ListSuggestions will only list the auto-generated suggestions, while CompileSuggestion will try to compile suggestion based on the provided conversation context in the real time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the participant to fetch suggestion for. Format: `projects//locations//conversations//participants/`.
    pub fn conversations_participants_suggestions_suggest_articles(&self, request: GoogleCloudDialogflowV2beta1SuggestArticlesRequest, parent: &str) -> ProjectConversationParticipantSuggestionSuggestArticleCall<'a, S> {
        ProjectConversationParticipantSuggestionSuggestArticleCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets suggested faq answers for a participant based on specific historical messages.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the participant to fetch suggestion for. Format: `projects//locations//conversations//participants/`.
    pub fn conversations_participants_suggestions_suggest_faq_answers(&self, request: GoogleCloudDialogflowV2beta1SuggestFaqAnswersRequest, parent: &str) -> ProjectConversationParticipantSuggestionSuggestFaqAnswerCall<'a, S> {
        ProjectConversationParticipantSuggestionSuggestFaqAnswerCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets smart replies for a participant based on specific historical messages.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the participant to fetch suggestion for. Format: `projects//locations//conversations//participants/`.
    pub fn conversations_participants_suggestions_suggest_smart_replies(&self, request: GoogleCloudDialogflowV2beta1SuggestSmartRepliesRequest, parent: &str) -> ProjectConversationParticipantSuggestionSuggestSmartReplyCall<'a, S> {
        ProjectConversationParticipantSuggestionSuggestSmartReplyCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a text (chat, for example), or audio (phone recording, for example) message from a participant into the conversation. Note: Always use agent versions for production traffic sent to virtual agents. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `participant` - Required. The name of the participant this text comes from. Format: `projects//locations//conversations//participants/`.
    pub fn conversations_participants_analyze_content(&self, request: GoogleCloudDialogflowV2beta1AnalyzeContentRequest, participant: &str) -> ProjectConversationParticipantAnalyzeContentCall<'a, S> {
        ProjectConversationParticipantAnalyzeContentCall {
            hub: self.hub,
            _request: request,
            _participant: participant.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new participant in a conversation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource identifier of the conversation adding the participant. Format: `projects//locations//conversations/`.
    pub fn conversations_participants_create(&self, request: GoogleCloudDialogflowV2beta1Participant, parent: &str) -> ProjectConversationParticipantCreateCall<'a, S> {
        ProjectConversationParticipantCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a conversation participant.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the participant. Format: `projects//locations//conversations//participants/`.
    pub fn conversations_participants_get(&self, name: &str) -> ProjectConversationParticipantGetCall<'a, S> {
        ProjectConversationParticipantGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all participants in the specified conversation.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The conversation to list all participants from. Format: `projects//locations//conversations/`.
    pub fn conversations_participants_list(&self, parent: &str) -> ProjectConversationParticipantListCall<'a, S> {
        ProjectConversationParticipantListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified participant.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Optional. The unique identifier of this participant. Format: `projects//locations//conversations//participants/`.
    pub fn conversations_participants_patch(&self, request: GoogleCloudDialogflowV2beta1Participant, name: &str) -> ProjectConversationParticipantPatchCall<'a, S> {
        ProjectConversationParticipantPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Suggest summary for a conversation based on specific historical messages. The range of the messages to be used for summary can be specified in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `conversation` - Required. The conversation to fetch suggestion for. Format: `projects//locations//conversations/`.
    pub fn conversations_suggestions_suggest_conversation_summary(&self, request: GoogleCloudDialogflowV2beta1SuggestConversationSummaryRequest, conversation: &str) -> ProjectConversationSuggestionSuggestConversationSummaryCall<'a, S> {
        ProjectConversationSuggestionSuggestConversationSummaryCall {
            hub: self.hub,
            _request: request,
            _conversation: conversation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Completes the specified conversation. Finished conversations are purged from the database after 30 days.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource identifier of the conversation to close. Format: `projects//locations//conversations/`.
    pub fn conversations_complete(&self, request: GoogleCloudDialogflowV2beta1CompleteConversationRequest, name: &str) -> ProjectConversationCompleteCall<'a, S> {
        ProjectConversationCompleteCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new conversation. Conversations are auto-completed after 24 hours. Conversation Lifecycle: There are two stages during a conversation: Automated Agent Stage and Assist Stage. For Automated Agent Stage, there will be a dialogflow agent responding to user queries. For Assist Stage, there's no dialogflow agent responding to user queries. But we will provide suggestions which are generated from conversation. If Conversation.conversation_profile is configured for a dialogflow agent, conversation will start from `Automated Agent Stage`, otherwise, it will start from `Assist Stage`. And during `Automated Agent Stage`, once an Intent with Intent.live_agent_handoff is triggered, conversation will transfer to Assist Stage.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource identifier of the project creating the conversation. Format: `projects//locations/`.
    pub fn conversations_create(&self, request: GoogleCloudDialogflowV2beta1Conversation, parent: &str) -> ProjectConversationCreateCall<'a, S> {
        ProjectConversationCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _conversation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specific conversation.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the conversation. Format: `projects//locations//conversations/`.
    pub fn conversations_get(&self, name: &str) -> ProjectConversationGetCall<'a, S> {
        ProjectConversationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all conversations in the specified project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project from which to list all conversation. Format: `projects//locations/`.
    pub fn conversations_list(&self, parent: &str) -> ProjectConversationListCall<'a, S> {
        ProjectConversationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The knowledge base to create a document for. Format: `projects//locations//knowledgeBases/`.
    pub fn knowledge_bases_documents_create(&self, request: GoogleCloudDialogflowV2beta1Document, parent: &str) -> ProjectKnowledgeBaseDocumentCreateCall<'a, S> {
        ProjectKnowledgeBaseDocumentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _import_gcs_custom_metadata: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the document to delete. Format: `projects//locations//knowledgeBases//documents/`.
    pub fn knowledge_bases_documents_delete(&self, name: &str) -> ProjectKnowledgeBaseDocumentDeleteCall<'a, S> {
        ProjectKnowledgeBaseDocumentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified document. Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the document to retrieve. Format `projects//locations//knowledgeBases//documents/`.
    pub fn knowledge_bases_documents_get(&self, name: &str) -> ProjectKnowledgeBaseDocumentGetCall<'a, S> {
        ProjectKnowledgeBaseDocumentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create documents by importing data from external sources. Dialogflow supports up to 350 documents in each request. If you try to import more, Dialogflow will return an error. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: ImportDocumentsResponse
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The knowledge base to import documents into. Format: `projects//locations//knowledgeBases/`.
    pub fn knowledge_bases_documents_import(&self, request: GoogleCloudDialogflowV2beta1ImportDocumentsRequest, parent: &str) -> ProjectKnowledgeBaseDocumentImportCall<'a, S> {
        ProjectKnowledgeBaseDocumentImportCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all documents of the knowledge base. Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The knowledge base to list all documents for. Format: `projects//locations//knowledgeBases/`.
    pub fn knowledge_bases_documents_list(&self, parent: &str) -> ProjectKnowledgeBaseDocumentListCall<'a, S> {
        ProjectKnowledgeBaseDocumentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Optional. The document resource name. The name must be empty when creating a document. Format: `projects//locations//knowledgeBases//documents/`.
    pub fn knowledge_bases_documents_patch(&self, request: GoogleCloudDialogflowV2beta1Document, name: &str) -> ProjectKnowledgeBaseDocumentPatchCall<'a, S> {
        ProjectKnowledgeBaseDocumentPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reloads the specified document from its specified source, content_uri or content. The previously loaded content of the document will be deleted. Note: Even when the content of the document has not changed, there still may be side effects because of internal implementation changes. Note: If the document source is Google Cloud Storage URI, its metadata will be replaced with the custom metadata from Google Cloud Storage if the `import_gcs_custom_metadata` field is set to true in the request. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the document to reload. Format: `projects//locations//knowledgeBases//documents/`
    pub fn knowledge_bases_documents_reload(&self, request: GoogleCloudDialogflowV2beta1ReloadDocumentRequest, name: &str) -> ProjectKnowledgeBaseDocumentReloadCall<'a, S> {
        ProjectKnowledgeBaseDocumentReloadCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a knowledge base. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project to create a knowledge base for. Format: `projects//locations/`.
    pub fn knowledge_bases_create(&self, request: GoogleCloudDialogflowV2beta1KnowledgeBase, parent: &str) -> ProjectKnowledgeBaseCreateCall<'a, S> {
        ProjectKnowledgeBaseCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified knowledge base. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the knowledge base to delete. Format: `projects//locations//knowledgeBases/`.
    pub fn knowledge_bases_delete(&self, name: &str) -> ProjectKnowledgeBaseDeleteCall<'a, S> {
        ProjectKnowledgeBaseDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified knowledge base. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the knowledge base to retrieve. Format `projects//locations//knowledgeBases/`.
    pub fn knowledge_bases_get(&self, name: &str) -> ProjectKnowledgeBaseGetCall<'a, S> {
        ProjectKnowledgeBaseGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all knowledge bases of the specified agent. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project to list of knowledge bases for. Format: `projects//locations/`.
    pub fn knowledge_bases_list(&self, parent: &str) -> ProjectKnowledgeBaseListCall<'a, S> {
        ProjectKnowledgeBaseListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified knowledge base. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The knowledge base resource name. The name must be empty when creating a knowledge base. Format: `projects//locations//knowledgeBases/`.
    pub fn knowledge_bases_patch(&self, request: GoogleCloudDialogflowV2beta1KnowledgeBase, name: &str) -> ProjectKnowledgeBasePatchCall<'a, S> {
        ProjectKnowledgeBasePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates multiple new entities in the specified entity type. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the entity type to create entities in. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`
    pub fn locations_agent_entity_types_entities_batch_create(&self, request: GoogleCloudDialogflowV2beta1BatchCreateEntitiesRequest, parent: &str) -> ProjectLocationAgentEntityTypeEntityBatchCreateCall<'a, S> {
        ProjectLocationAgentEntityTypeEntityBatchCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes entities in the specified entity type. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the entity type to delete entries for. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`
    pub fn locations_agent_entity_types_entities_batch_delete(&self, request: GoogleCloudDialogflowV2beta1BatchDeleteEntitiesRequest, parent: &str) -> ProjectLocationAgentEntityTypeEntityBatchDeleteCall<'a, S> {
        ProjectLocationAgentEntityTypeEntityBatchDeleteCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates or creates multiple entities in the specified entity type. This method does not affect entities in the entity type that aren't explicitly specified in the request. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training). This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the entity type to update or create entities in. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`
    pub fn locations_agent_entity_types_entities_batch_update(&self, request: GoogleCloudDialogflowV2beta1BatchUpdateEntitiesRequest, parent: &str) -> ProjectLocationAgentEntityTypeEntityBatchUpdateCall<'a, S> {
        ProjectLocationAgentEntityTypeEntityBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes entity types in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the agent to delete all entities types for. Supported formats: - `projects//agent`, - `projects//locations//agent`.
    pub fn locations_agent_entity_types_batch_delete(&self, request: GoogleCloudDialogflowV2beta1BatchDeleteEntityTypesRequest, parent: &str) -> ProjectLocationAgentEntityTypeBatchDeleteCall<'a, S> {
        ProjectLocationAgentEntityTypeBatchDeleteCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates/Creates multiple entity types in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: BatchUpdateEntityTypesResponse Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the agent to update or create entity types in. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn locations_agent_entity_types_batch_update(&self, request: GoogleCloudDialogflowV2beta1BatchUpdateEntityTypesRequest, parent: &str) -> ProjectLocationAgentEntityTypeBatchUpdateCall<'a, S> {
        ProjectLocationAgentEntityTypeBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an entity type in the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to create a entity type for. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn locations_agent_entity_types_create(&self, request: GoogleCloudDialogflowV2beta1EntityType, parent: &str) -> ProjectLocationAgentEntityTypeCreateCall<'a, S> {
        ProjectLocationAgentEntityTypeCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified entity type. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the entity type to delete. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`
    pub fn locations_agent_entity_types_delete(&self, name: &str) -> ProjectLocationAgentEntityTypeDeleteCall<'a, S> {
        ProjectLocationAgentEntityTypeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified entity type.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the entity type. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`
    pub fn locations_agent_entity_types_get(&self, name: &str) -> ProjectLocationAgentEntityTypeGetCall<'a, S> {
        ProjectLocationAgentEntityTypeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all entity types in the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The agent to list all entity types from. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn locations_agent_entity_types_list(&self, parent: &str) -> ProjectLocationAgentEntityTypeListCall<'a, S> {
        ProjectLocationAgentEntityTypeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified entity type. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType and EntityTypes.BatchUpdateEntityTypes methods. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`
    pub fn locations_agent_entity_types_patch(&self, request: GoogleCloudDialogflowV2beta1EntityType, name: &str) -> ProjectLocationAgentEntityTypePatchCall<'a, S> {
        ProjectLocationAgentEntityTypePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all intents in the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The agent to list all intents from. Format: `projects//agent` or `projects//locations//agent`. Alternatively, you can specify the environment to list intents for. Format: `projects//agent/environments/` or `projects//locations//agent/environments/`. Note: training phrases of the intents will not be returned for non-draft environment.
    pub fn locations_agent_environments_intents_list(&self, parent: &str) -> ProjectLocationAgentEnvironmentIntentListCall<'a, S> {
        ProjectLocationAgentEnvironmentIntentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _intent_view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a context. If the specified context already exists, overrides the context.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The session to create a context for. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_environments_users_sessions_contexts_create(&self, request: GoogleCloudDialogflowV2beta1Context, parent: &str) -> ProjectLocationAgentEnvironmentUserSessionContextCreateCall<'a, S> {
        ProjectLocationAgentEnvironmentUserSessionContextCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified context.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the context to delete. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_environments_users_sessions_contexts_delete(&self, name: &str) -> ProjectLocationAgentEnvironmentUserSessionContextDeleteCall<'a, S> {
        ProjectLocationAgentEnvironmentUserSessionContextDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified context.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the context. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_environments_users_sessions_contexts_get(&self, name: &str) -> ProjectLocationAgentEnvironmentUserSessionContextGetCall<'a, S> {
        ProjectLocationAgentEnvironmentUserSessionContextGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all contexts in the specified session.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The session to list all contexts from. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_environments_users_sessions_contexts_list(&self, parent: &str) -> ProjectLocationAgentEnvironmentUserSessionContextListCall<'a, S> {
        ProjectLocationAgentEnvironmentUserSessionContextListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified context.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique identifier of the context. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, The `Context ID` is always converted to lowercase, may only contain characters in a-zA-Z0-9_-% and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size`
    pub fn locations_agent_environments_users_sessions_contexts_patch(&self, request: GoogleCloudDialogflowV2beta1Context, name: &str) -> ProjectLocationAgentEnvironmentUserSessionContextPatchCall<'a, S> {
        ProjectLocationAgentEnvironmentUserSessionContextPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a session entity type. If the specified session entity type already exists, overrides the session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The session to create a session entity type for. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_environments_users_sessions_entity_types_create(&self, request: GoogleCloudDialogflowV2beta1SessionEntityType, parent: &str) -> ProjectLocationAgentEnvironmentUserSessionEntityTypeCreateCall<'a, S> {
        ProjectLocationAgentEnvironmentUserSessionEntityTypeCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the entity type to delete. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_environments_users_sessions_entity_types_delete(&self, name: &str) -> ProjectLocationAgentEnvironmentUserSessionEntityTypeDeleteCall<'a, S> {
        ProjectLocationAgentEnvironmentUserSessionEntityTypeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the session entity type. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_environments_users_sessions_entity_types_get(&self, name: &str) -> ProjectLocationAgentEnvironmentUserSessionEntityTypeGetCall<'a, S> {
        ProjectLocationAgentEnvironmentUserSessionEntityTypeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all session entity types in the specified session. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The session to list all session entity types from. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_environments_users_sessions_entity_types_list(&self, parent: &str) -> ProjectLocationAgentEnvironmentUserSessionEntityTypeListCall<'a, S> {
        ProjectLocationAgentEnvironmentUserSessionEntityTypeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique identifier of this session entity type. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. `` must be the display name of an existing entity type in the same agent that will be overridden or supplemented.
    pub fn locations_agent_environments_users_sessions_entity_types_patch(&self, request: GoogleCloudDialogflowV2beta1SessionEntityType, name: &str) -> ProjectLocationAgentEnvironmentUserSessionEntityTypePatchCall<'a, S> {
        ProjectLocationAgentEnvironmentUserSessionEntityTypePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes all active contexts in the specified session.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the session to delete all contexts from. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_environments_users_sessions_delete_contexts(&self, parent: &str) -> ProjectLocationAgentEnvironmentUserSessionDeleteContextCall<'a, S> {
        ProjectLocationAgentEnvironmentUserSessionDeleteContextCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause contexts and session entity types to be updated, which in turn might affect results of future queries. If you might use [Agent Assist](https://cloud.google.com/dialogflow/docs/#aa) or other CCAI products now or in the future, consider using AnalyzeContent instead of `DetectIntent`. `AnalyzeContent` has additional functionality for Agent Assist and other CCAI products. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The name of the session this query is sent to. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment (`Environment ID` might be referred to as environment name at some places). If `User ID` is not specified, we are using "-". It's up to the API caller to choose an appropriate `Session ID` and `User Id`. They can be a random number or some type of user and session identifiers (preferably hashed). The length of the `Session ID` and `User ID` must not exceed 36 characters. For more information, see the [API interactions guide](https://cloud.google.com/dialogflow/docs/api-overview). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).
    pub fn locations_agent_environments_users_sessions_detect_intent(&self, request: GoogleCloudDialogflowV2beta1DetectIntentRequest, session: &str) -> ProjectLocationAgentEnvironmentUserSessionDetectIntentCall<'a, S> {
        ProjectLocationAgentEnvironmentUserSessionDetectIntentCall {
            hub: self.hub,
            _request: request,
            _session: session.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an agent environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to create an environment for. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn locations_agent_environments_create(&self, request: GoogleCloudDialogflowV2beta1Environment, parent: &str) -> ProjectLocationAgentEnvironmentCreateCall<'a, S> {
        ProjectLocationAgentEnvironmentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _environment_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified agent environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the environment to delete. / Format: - `projects//agent/environments/` - `projects//locations//agent/environments/`
    pub fn locations_agent_environments_delete(&self, name: &str) -> ProjectLocationAgentEnvironmentDeleteCall<'a, S> {
        ProjectLocationAgentEnvironmentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified agent environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the environment. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/`
    pub fn locations_agent_environments_get(&self, name: &str) -> ProjectLocationAgentEnvironmentGetCall<'a, S> {
        ProjectLocationAgentEnvironmentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the history of the specified environment.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the environment to retrieve history for. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/`
    pub fn locations_agent_environments_get_history(&self, parent: &str) -> ProjectLocationAgentEnvironmentGetHistoryCall<'a, S> {
        ProjectLocationAgentEnvironmentGetHistoryCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all non-draft environments of the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The agent to list all environments from. Format: - `projects//agent` - `projects//locations//agent`
    pub fn locations_agent_environments_list(&self, parent: &str) -> ProjectLocationAgentEnvironmentListCall<'a, S> {
        ProjectLocationAgentEnvironmentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified agent environment. This method allows you to deploy new agent versions into the environment. When an environment is pointed to a new agent version by setting `environment.agent_version`, the environment is temporarily set to the `LOADING` state. During that time, the environment keeps on serving the previous version of the agent. After the new agent version is done loading, the environment is set back to the `RUNNING` state. You can use "-" as Environment ID in environment name to update version in "draft" environment. WARNING: this will negate all recent changes to draft and can't be undone. You may want to save the draft to a version before calling this function.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The unique identifier of this agent environment. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/`
    pub fn locations_agent_environments_patch(&self, request: GoogleCloudDialogflowV2beta1Environment, name: &str) -> ProjectLocationAgentEnvironmentPatchCall<'a, S> {
        ProjectLocationAgentEnvironmentPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _allow_load_to_draft_and_discard_changes: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes intents in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the agent to delete all entities types for. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn locations_agent_intents_batch_delete(&self, request: GoogleCloudDialogflowV2beta1BatchDeleteIntentsRequest, parent: &str) -> ProjectLocationAgentIntentBatchDeleteCall<'a, S> {
        ProjectLocationAgentIntentBatchDeleteCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates/Creates multiple intents in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: BatchUpdateIntentsResponse Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the agent to update or create intents in. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn locations_agent_intents_batch_update(&self, request: GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequest, parent: &str) -> ProjectLocationAgentIntentBatchUpdateCall<'a, S> {
        ProjectLocationAgentIntentBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an intent in the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to create a intent for. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn locations_agent_intents_create(&self, request: GoogleCloudDialogflowV2beta1Intent, parent: &str) -> ProjectLocationAgentIntentCreateCall<'a, S> {
        ProjectLocationAgentIntentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _language_code: Default::default(),
            _intent_view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified intent and its direct or indirect followup intents. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the intent to delete. If this intent has direct or indirect followup intents, we also delete them. Supported formats: - `projects//agent/intents/` - `projects//locations//agent/intents/`
    pub fn locations_agent_intents_delete(&self, name: &str) -> ProjectLocationAgentIntentDeleteCall<'a, S> {
        ProjectLocationAgentIntentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified intent.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the intent. Supported formats: - `projects//agent/intents/` - `projects//locations//agent/intents/`
    pub fn locations_agent_intents_get(&self, name: &str) -> ProjectLocationAgentIntentGetCall<'a, S> {
        ProjectLocationAgentIntentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _language_code: Default::default(),
            _intent_view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all intents in the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The agent to list all intents from. Format: `projects//agent` or `projects//locations//agent`. Alternatively, you can specify the environment to list intents for. Format: `projects//agent/environments/` or `projects//locations//agent/environments/`. Note: training phrases of the intents will not be returned for non-draft environment.
    pub fn locations_agent_intents_list(&self, parent: &str) -> ProjectLocationAgentIntentListCall<'a, S> {
        ProjectLocationAgentIntentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _intent_view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified intent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Supported formats: - `projects//agent/intents/` - `projects//locations//agent/intents/`
    pub fn locations_agent_intents_patch(&self, request: GoogleCloudDialogflowV2beta1Intent, name: &str) -> ProjectLocationAgentIntentPatchCall<'a, S> {
        ProjectLocationAgentIntentPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _language_code: Default::default(),
            _intent_view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a context. If the specified context already exists, overrides the context.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The session to create a context for. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_sessions_contexts_create(&self, request: GoogleCloudDialogflowV2beta1Context, parent: &str) -> ProjectLocationAgentSessionContextCreateCall<'a, S> {
        ProjectLocationAgentSessionContextCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified context.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the context to delete. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_sessions_contexts_delete(&self, name: &str) -> ProjectLocationAgentSessionContextDeleteCall<'a, S> {
        ProjectLocationAgentSessionContextDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified context.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the context. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_sessions_contexts_get(&self, name: &str) -> ProjectLocationAgentSessionContextGetCall<'a, S> {
        ProjectLocationAgentSessionContextGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all contexts in the specified session.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The session to list all contexts from. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_sessions_contexts_list(&self, parent: &str) -> ProjectLocationAgentSessionContextListCall<'a, S> {
        ProjectLocationAgentSessionContextListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified context.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique identifier of the context. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, The `Context ID` is always converted to lowercase, may only contain characters in a-zA-Z0-9_-% and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size`
    pub fn locations_agent_sessions_contexts_patch(&self, request: GoogleCloudDialogflowV2beta1Context, name: &str) -> ProjectLocationAgentSessionContextPatchCall<'a, S> {
        ProjectLocationAgentSessionContextPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a session entity type. If the specified session entity type already exists, overrides the session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The session to create a session entity type for. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_sessions_entity_types_create(&self, request: GoogleCloudDialogflowV2beta1SessionEntityType, parent: &str) -> ProjectLocationAgentSessionEntityTypeCreateCall<'a, S> {
        ProjectLocationAgentSessionEntityTypeCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the entity type to delete. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_sessions_entity_types_delete(&self, name: &str) -> ProjectLocationAgentSessionEntityTypeDeleteCall<'a, S> {
        ProjectLocationAgentSessionEntityTypeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the session entity type. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_sessions_entity_types_get(&self, name: &str) -> ProjectLocationAgentSessionEntityTypeGetCall<'a, S> {
        ProjectLocationAgentSessionEntityTypeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all session entity types in the specified session. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The session to list all session entity types from. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_sessions_entity_types_list(&self, parent: &str) -> ProjectLocationAgentSessionEntityTypeListCall<'a, S> {
        ProjectLocationAgentSessionEntityTypeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique identifier of this session entity type. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. `` must be the display name of an existing entity type in the same agent that will be overridden or supplemented.
    pub fn locations_agent_sessions_entity_types_patch(&self, request: GoogleCloudDialogflowV2beta1SessionEntityType, name: &str) -> ProjectLocationAgentSessionEntityTypePatchCall<'a, S> {
        ProjectLocationAgentSessionEntityTypePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes all active contexts in the specified session.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the session to delete all contexts from. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
    pub fn locations_agent_sessions_delete_contexts(&self, parent: &str) -> ProjectLocationAgentSessionDeleteContextCall<'a, S> {
        ProjectLocationAgentSessionDeleteContextCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause contexts and session entity types to be updated, which in turn might affect results of future queries. If you might use [Agent Assist](https://cloud.google.com/dialogflow/docs/#aa) or other CCAI products now or in the future, consider using AnalyzeContent instead of `DetectIntent`. `AnalyzeContent` has additional functionality for Agent Assist and other CCAI products. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The name of the session this query is sent to. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment (`Environment ID` might be referred to as environment name at some places). If `User ID` is not specified, we are using "-". It's up to the API caller to choose an appropriate `Session ID` and `User Id`. They can be a random number or some type of user and session identifiers (preferably hashed). The length of the `Session ID` and `User ID` must not exceed 36 characters. For more information, see the [API interactions guide](https://cloud.google.com/dialogflow/docs/api-overview). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).
    pub fn locations_agent_sessions_detect_intent(&self, request: GoogleCloudDialogflowV2beta1DetectIntentRequest, session: &str) -> ProjectLocationAgentSessionDetectIntentCall<'a, S> {
        ProjectLocationAgentSessionDetectIntentCall {
            hub: self.hub,
            _request: request,
            _session: session.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an agent version. The new version points to the agent instance in the "default" environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to create a version for. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn locations_agent_versions_create(&self, request: GoogleCloudDialogflowV2beta1Version, parent: &str) -> ProjectLocationAgentVersionCreateCall<'a, S> {
        ProjectLocationAgentVersionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete the specified agent version.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the version to delete. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/`
    pub fn locations_agent_versions_delete(&self, name: &str) -> ProjectLocationAgentVersionDeleteCall<'a, S> {
        ProjectLocationAgentVersionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified agent version.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the version. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/`
    pub fn locations_agent_versions_get(&self, name: &str) -> ProjectLocationAgentVersionGetCall<'a, S> {
        ProjectLocationAgentVersionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all versions of the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The agent to list all versions from. Supported formats: - `projects//agent` - `projects//locations//agent`
    pub fn locations_agent_versions_list(&self, parent: &str) -> ProjectLocationAgentVersionListCall<'a, S> {
        ProjectLocationAgentVersionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified agent version. Note that this method does not allow you to update the state of the agent the given version points to. It allows you to update only mutable properties of the version resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The unique identifier of this agent version. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/`
    pub fn locations_agent_versions_patch(&self, request: GoogleCloudDialogflowV2beta1Version, name: &str) -> ProjectLocationAgentVersionPatchCall<'a, S> {
        ProjectLocationAgentVersionPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports the specified agent to a ZIP file. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: ExportAgentResponse
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project that the agent to export is associated with. Format: `projects/` or `projects//locations/`.
    pub fn locations_agent_export(&self, request: GoogleCloudDialogflowV2beta1ExportAgentRequest, parent: &str) -> ProjectLocationAgentExportCall<'a, S> {
        ProjectLocationAgentExportCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the fulfillment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the fulfillment. Supported formats: - `projects//agent/fulfillment` - `projects//locations//agent/fulfillment`
    pub fn locations_agent_get_fulfillment(&self, name: &str) -> ProjectLocationAgentGetFulfillmentCall<'a, S> {
        ProjectLocationAgentGetFulfillmentCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets agent validation result. Agent validation is performed during training time and is updated automatically when training is completed.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project that the agent is associated with. Format: `projects/` or `projects//locations/`.
    pub fn locations_agent_get_validation_result(&self, parent: &str) -> ProjectLocationAgentGetValidationResultCall<'a, S> {
        ProjectLocationAgentGetValidationResultCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Imports the specified agent from a ZIP file. Uploads new intents and entity types without deleting the existing ones. Intents and entity types with the same name are replaced with the new versions from ImportAgentRequest. After the import, the imported draft agent will be trained automatically (unless disabled in agent settings). However, once the import is done, training may not be completed yet. Please call TrainAgent and wait for the operation it returns in order to train explicitly. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) The operation only tracks when importing is complete, not when it is done training. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project that the agent to import is associated with. Format: `projects/` or `projects//locations/`.
    pub fn locations_agent_import(&self, request: GoogleCloudDialogflowV2beta1ImportAgentRequest, parent: &str) -> ProjectLocationAgentImportCall<'a, S> {
        ProjectLocationAgentImportCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Restores the specified agent from a ZIP file. Replaces the current agent version with a new one. All the intents and entity types in the older version are deleted. After the restore, the restored draft agent will be trained automatically (unless disabled in agent settings). However, once the restore is done, training may not be completed yet. Please call TrainAgent and wait for the operation it returns in order to train explicitly. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) The operation only tracks when restoring is complete, not when it is done training. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project that the agent to restore is associated with. Format: `projects/` or `projects//locations/`.
    pub fn locations_agent_restore(&self, request: GoogleCloudDialogflowV2beta1RestoreAgentRequest, parent: &str) -> ProjectLocationAgentRestoreCall<'a, S> {
        ProjectLocationAgentRestoreCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of agents. Since there is at most one conversational agent per project, this method is useful primarily for listing all agents across projects the caller has access to. One can achieve that with a wildcard project collection id "-". Refer to [List Sub-Collections](https://cloud.google.com/apis/design/design_patterns#list_sub-collections).
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project to list agents from. Format: `projects/` or `projects//locations/`.
    pub fn locations_agent_search(&self, parent: &str) -> ProjectLocationAgentSearchCall<'a, S> {
        ProjectLocationAgentSearchCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Trains the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project that the agent to train is associated with. Format: `projects/` or `projects//locations/`.
    pub fn locations_agent_train(&self, request: GoogleCloudDialogflowV2beta1TrainAgentRequest, parent: &str) -> ProjectLocationAgentTrainCall<'a, S> {
        ProjectLocationAgentTrainCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the fulfillment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique identifier of the fulfillment. Supported formats: - `projects//agent/fulfillment` - `projects//locations//agent/fulfillment` This field is not used for Fulfillment in an Environment.
    pub fn locations_agent_update_fulfillment(&self, request: GoogleCloudDialogflowV2beta1Fulfillment, name: &str) -> ProjectLocationAgentUpdateFulfillmentCall<'a, S> {
        ProjectLocationAgentUpdateFulfillmentCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated. Retrieves a specific answer record.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the answer record to retrieve. Format: `projects//locations//answerRecords/`.
    pub fn locations_answer_records_get(&self, name: &str) -> ProjectLocationAnswerRecordGetCall<'a, S> {
        ProjectLocationAnswerRecordGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all answer records in the specified project in reverse chronological order.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project to list all answer records for in reverse chronological order. Format: `projects//locations/`.
    pub fn locations_answer_records_list(&self, parent: &str) -> ProjectLocationAnswerRecordListCall<'a, S> {
        ProjectLocationAnswerRecordListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified answer record.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique identifier of this answer record. Required for AnswerRecords.UpdateAnswerRecord method. Format: `projects//locations//answerRecords/`.
    pub fn locations_answer_records_patch(&self, request: GoogleCloudDialogflowV2beta1AnswerRecord, name: &str) -> ProjectLocationAnswerRecordPatchCall<'a, S> {
        ProjectLocationAnswerRecordPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears a suggestion feature from a conversation profile for the given participant role. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: ClearSuggestionFeatureConfigOperationMetadata - `response`: ConversationProfile
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `conversationProfile` - Required. The Conversation Profile to add or update the suggestion feature config. Format: `projects//locations//conversationProfiles/`.
    pub fn locations_conversation_profiles_clear_suggestion_feature_config(&self, request: GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequest, conversation_profile: &str) -> ProjectLocationConversationProfileClearSuggestionFeatureConfigCall<'a, S> {
        ProjectLocationConversationProfileClearSuggestionFeatureConfigCall {
            hub: self.hub,
            _request: request,
            _conversation_profile: conversation_profile.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a conversation profile in the specified project. ConversationProfile.CreateTime and ConversationProfile.UpdateTime aren't populated in the response. You can retrieve them via GetConversationProfile API.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project to create a conversation profile for. Format: `projects//locations/`.
    pub fn locations_conversation_profiles_create(&self, request: GoogleCloudDialogflowV2beta1ConversationProfile, parent: &str) -> ProjectLocationConversationProfileCreateCall<'a, S> {
        ProjectLocationConversationProfileCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified conversation profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the conversation profile to delete. Format: `projects//locations//conversationProfiles/`.
    pub fn locations_conversation_profiles_delete(&self, name: &str) -> ProjectLocationConversationProfileDeleteCall<'a, S> {
        ProjectLocationConversationProfileDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified conversation profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the conversation profile. Format: `projects//locations//conversationProfiles/`.
    pub fn locations_conversation_profiles_get(&self, name: &str) -> ProjectLocationConversationProfileGetCall<'a, S> {
        ProjectLocationConversationProfileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all conversation profiles in the specified project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project to list all conversation profiles from. Format: `projects//locations/`.
    pub fn locations_conversation_profiles_list(&self, parent: &str) -> ProjectLocationConversationProfileListCall<'a, S> {
        ProjectLocationConversationProfileListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified conversation profile. ConversationProfile.CreateTime and ConversationProfile.UpdateTime aren't populated in the response. You can retrieve them via GetConversationProfile API.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique identifier of this conversation profile. Format: `projects//locations//conversationProfiles/`.
    pub fn locations_conversation_profiles_patch(&self, request: GoogleCloudDialogflowV2beta1ConversationProfile, name: &str) -> ProjectLocationConversationProfilePatchCall<'a, S> {
        ProjectLocationConversationProfilePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds or updates a suggestion feature in a conversation profile. If the conversation profile contains the type of suggestion feature for the participant role, it will update it. Otherwise it will insert the suggestion feature. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: SetSuggestionFeatureConfigOperationMetadata - `response`: ConversationProfile If a long running operation to add or update suggestion feature config for the same conversation profile, participant role and suggestion feature type exists, please cancel the existing long running operation before sending such request, otherwise the request will be rejected.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `conversationProfile` - Required. The Conversation Profile to add or update the suggestion feature config. Format: `projects//locations//conversationProfiles/`.
    pub fn locations_conversation_profiles_set_suggestion_feature_config(&self, request: GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequest, conversation_profile: &str) -> ProjectLocationConversationProfileSetSuggestionFeatureConfigCall<'a, S> {
        ProjectLocationConversationProfileSetSuggestionFeatureConfigCall {
            hub: self.hub,
            _request: request,
            _conversation_profile: conversation_profile.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Batch ingests messages to conversation. Customers can use this RPC to ingest historical messages to conversation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource identifier of the conversation to create message. Format: `projects//locations//conversations/`.
    pub fn locations_conversations_messages_batch_create(&self, request: GoogleCloudDialogflowV2beta1BatchCreateMessagesRequest, parent: &str) -> ProjectLocationConversationMessageBatchCreateCall<'a, S> {
        ProjectLocationConversationMessageBatchCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists messages that belong to a given conversation. `messages` are ordered by `create_time` in descending order. To fetch updates without duplication, send request with filter `create_time_epoch_microseconds > [first item's create_time of previous request]` and empty page_token.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the conversation to list messages for. Format: `projects//locations//conversations/`
    pub fn locations_conversations_messages_list(&self, parent: &str) -> ProjectLocationConversationMessageListCall<'a, S> {
        ProjectLocationConversationMessageListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets suggested articles for a participant based on specific historical messages. Note that ListSuggestions will only list the auto-generated suggestions, while CompileSuggestion will try to compile suggestion based on the provided conversation context in the real time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the participant to fetch suggestion for. Format: `projects//locations//conversations//participants/`.
    pub fn locations_conversations_participants_suggestions_suggest_articles(&self, request: GoogleCloudDialogflowV2beta1SuggestArticlesRequest, parent: &str) -> ProjectLocationConversationParticipantSuggestionSuggestArticleCall<'a, S> {
        ProjectLocationConversationParticipantSuggestionSuggestArticleCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets suggested faq answers for a participant based on specific historical messages.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the participant to fetch suggestion for. Format: `projects//locations//conversations//participants/`.
    pub fn locations_conversations_participants_suggestions_suggest_faq_answers(&self, request: GoogleCloudDialogflowV2beta1SuggestFaqAnswersRequest, parent: &str) -> ProjectLocationConversationParticipantSuggestionSuggestFaqAnswerCall<'a, S> {
        ProjectLocationConversationParticipantSuggestionSuggestFaqAnswerCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets smart replies for a participant based on specific historical messages.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the participant to fetch suggestion for. Format: `projects//locations//conversations//participants/`.
    pub fn locations_conversations_participants_suggestions_suggest_smart_replies(&self, request: GoogleCloudDialogflowV2beta1SuggestSmartRepliesRequest, parent: &str) -> ProjectLocationConversationParticipantSuggestionSuggestSmartReplyCall<'a, S> {
        ProjectLocationConversationParticipantSuggestionSuggestSmartReplyCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a text (chat, for example), or audio (phone recording, for example) message from a participant into the conversation. Note: Always use agent versions for production traffic sent to virtual agents. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `participant` - Required. The name of the participant this text comes from. Format: `projects//locations//conversations//participants/`.
    pub fn locations_conversations_participants_analyze_content(&self, request: GoogleCloudDialogflowV2beta1AnalyzeContentRequest, participant: &str) -> ProjectLocationConversationParticipantAnalyzeContentCall<'a, S> {
        ProjectLocationConversationParticipantAnalyzeContentCall {
            hub: self.hub,
            _request: request,
            _participant: participant.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new participant in a conversation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource identifier of the conversation adding the participant. Format: `projects//locations//conversations/`.
    pub fn locations_conversations_participants_create(&self, request: GoogleCloudDialogflowV2beta1Participant, parent: &str) -> ProjectLocationConversationParticipantCreateCall<'a, S> {
        ProjectLocationConversationParticipantCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a conversation participant.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the participant. Format: `projects//locations//conversations//participants/`.
    pub fn locations_conversations_participants_get(&self, name: &str) -> ProjectLocationConversationParticipantGetCall<'a, S> {
        ProjectLocationConversationParticipantGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all participants in the specified conversation.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The conversation to list all participants from. Format: `projects//locations//conversations/`.
    pub fn locations_conversations_participants_list(&self, parent: &str) -> ProjectLocationConversationParticipantListCall<'a, S> {
        ProjectLocationConversationParticipantListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified participant.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Optional. The unique identifier of this participant. Format: `projects//locations//conversations//participants/`.
    pub fn locations_conversations_participants_patch(&self, request: GoogleCloudDialogflowV2beta1Participant, name: &str) -> ProjectLocationConversationParticipantPatchCall<'a, S> {
        ProjectLocationConversationParticipantPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Suggest summary for a conversation based on specific historical messages. The range of the messages to be used for summary can be specified in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `conversation` - Required. The conversation to fetch suggestion for. Format: `projects//locations//conversations/`.
    pub fn locations_conversations_suggestions_suggest_conversation_summary(&self, request: GoogleCloudDialogflowV2beta1SuggestConversationSummaryRequest, conversation: &str) -> ProjectLocationConversationSuggestionSuggestConversationSummaryCall<'a, S> {
        ProjectLocationConversationSuggestionSuggestConversationSummaryCall {
            hub: self.hub,
            _request: request,
            _conversation: conversation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Completes the specified conversation. Finished conversations are purged from the database after 30 days.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource identifier of the conversation to close. Format: `projects//locations//conversations/`.
    pub fn locations_conversations_complete(&self, request: GoogleCloudDialogflowV2beta1CompleteConversationRequest, name: &str) -> ProjectLocationConversationCompleteCall<'a, S> {
        ProjectLocationConversationCompleteCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new conversation. Conversations are auto-completed after 24 hours. Conversation Lifecycle: There are two stages during a conversation: Automated Agent Stage and Assist Stage. For Automated Agent Stage, there will be a dialogflow agent responding to user queries. For Assist Stage, there's no dialogflow agent responding to user queries. But we will provide suggestions which are generated from conversation. If Conversation.conversation_profile is configured for a dialogflow agent, conversation will start from `Automated Agent Stage`, otherwise, it will start from `Assist Stage`. And during `Automated Agent Stage`, once an Intent with Intent.live_agent_handoff is triggered, conversation will transfer to Assist Stage.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource identifier of the project creating the conversation. Format: `projects//locations/`.
    pub fn locations_conversations_create(&self, request: GoogleCloudDialogflowV2beta1Conversation, parent: &str) -> ProjectLocationConversationCreateCall<'a, S> {
        ProjectLocationConversationCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _conversation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specific conversation.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the conversation. Format: `projects//locations//conversations/`.
    pub fn locations_conversations_get(&self, name: &str) -> ProjectLocationConversationGetCall<'a, S> {
        ProjectLocationConversationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all conversations in the specified project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project from which to list all conversation. Format: `projects//locations/`.
    pub fn locations_conversations_list(&self, parent: &str) -> ProjectLocationConversationListCall<'a, S> {
        ProjectLocationConversationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The knowledge base to create a document for. Format: `projects//locations//knowledgeBases/`.
    pub fn locations_knowledge_bases_documents_create(&self, request: GoogleCloudDialogflowV2beta1Document, parent: &str) -> ProjectLocationKnowledgeBaseDocumentCreateCall<'a, S> {
        ProjectLocationKnowledgeBaseDocumentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _import_gcs_custom_metadata: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the document to delete. Format: `projects//locations//knowledgeBases//documents/`.
    pub fn locations_knowledge_bases_documents_delete(&self, name: &str) -> ProjectLocationKnowledgeBaseDocumentDeleteCall<'a, S> {
        ProjectLocationKnowledgeBaseDocumentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified document. Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the document to retrieve. Format `projects//locations//knowledgeBases//documents/`.
    pub fn locations_knowledge_bases_documents_get(&self, name: &str) -> ProjectLocationKnowledgeBaseDocumentGetCall<'a, S> {
        ProjectLocationKnowledgeBaseDocumentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create documents by importing data from external sources. Dialogflow supports up to 350 documents in each request. If you try to import more, Dialogflow will return an error. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: ImportDocumentsResponse
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The knowledge base to import documents into. Format: `projects//locations//knowledgeBases/`.
    pub fn locations_knowledge_bases_documents_import(&self, request: GoogleCloudDialogflowV2beta1ImportDocumentsRequest, parent: &str) -> ProjectLocationKnowledgeBaseDocumentImportCall<'a, S> {
        ProjectLocationKnowledgeBaseDocumentImportCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all documents of the knowledge base. Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The knowledge base to list all documents for. Format: `projects//locations//knowledgeBases/`.
    pub fn locations_knowledge_bases_documents_list(&self, parent: &str) -> ProjectLocationKnowledgeBaseDocumentListCall<'a, S> {
        ProjectLocationKnowledgeBaseDocumentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Optional. The document resource name. The name must be empty when creating a document. Format: `projects//locations//knowledgeBases//documents/`.
    pub fn locations_knowledge_bases_documents_patch(&self, request: GoogleCloudDialogflowV2beta1Document, name: &str) -> ProjectLocationKnowledgeBaseDocumentPatchCall<'a, S> {
        ProjectLocationKnowledgeBaseDocumentPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reloads the specified document from its specified source, content_uri or content. The previously loaded content of the document will be deleted. Note: Even when the content of the document has not changed, there still may be side effects because of internal implementation changes. Note: If the document source is Google Cloud Storage URI, its metadata will be replaced with the custom metadata from Google Cloud Storage if the `import_gcs_custom_metadata` field is set to true in the request. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the document to reload. Format: `projects//locations//knowledgeBases//documents/`
    pub fn locations_knowledge_bases_documents_reload(&self, request: GoogleCloudDialogflowV2beta1ReloadDocumentRequest, name: &str) -> ProjectLocationKnowledgeBaseDocumentReloadCall<'a, S> {
        ProjectLocationKnowledgeBaseDocumentReloadCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a knowledge base. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project to create a knowledge base for. Format: `projects//locations/`.
    pub fn locations_knowledge_bases_create(&self, request: GoogleCloudDialogflowV2beta1KnowledgeBase, parent: &str) -> ProjectLocationKnowledgeBaseCreateCall<'a, S> {
        ProjectLocationKnowledgeBaseCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified knowledge base. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the knowledge base to delete. Format: `projects//locations//knowledgeBases/`.
    pub fn locations_knowledge_bases_delete(&self, name: &str) -> ProjectLocationKnowledgeBaseDeleteCall<'a, S> {
        ProjectLocationKnowledgeBaseDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified knowledge base. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the knowledge base to retrieve. Format `projects//locations//knowledgeBases/`.
    pub fn locations_knowledge_bases_get(&self, name: &str) -> ProjectLocationKnowledgeBaseGetCall<'a, S> {
        ProjectLocationKnowledgeBaseGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all knowledge bases of the specified agent. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project to list of knowledge bases for. Format: `projects//locations/`.
    pub fn locations_knowledge_bases_list(&self, parent: &str) -> ProjectLocationKnowledgeBaseListCall<'a, S> {
        ProjectLocationKnowledgeBaseListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified knowledge base. Note: The `projects.agent.knowledgeBases` resource is deprecated; only use `projects.knowledgeBases`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The knowledge base resource name. The name must be empty when creating a knowledge base. Format: `projects//locations//knowledgeBases/`.
    pub fn locations_knowledge_bases_patch(&self, request: GoogleCloudDialogflowV2beta1KnowledgeBase, name: &str) -> ProjectLocationKnowledgeBasePatchCall<'a, S> {
        ProjectLocationKnowledgeBasePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn locations_operations_cancel(&self, name: &str) -> ProjectLocationOperationCancelCall<'a, S> {
        ProjectLocationOperationCancelCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_operations_get(&self, name: &str) -> ProjectLocationOperationGetCall<'a, S> {
        ProjectLocationOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn locations_operations_list(&self, name: &str) -> ProjectLocationOperationListCall<'a, S> {
        ProjectLocationOperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project that the agent to delete is associated with. Format: `projects/` or `projects//locations/`.
    pub fn locations_delete_agent(&self, parent: &str) -> ProjectLocationDeleteAgentCall<'a, S> {
        ProjectLocationDeleteAgentCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Resource name for the location.
    pub fn locations_get(&self, name: &str) -> ProjectLocationGetCall<'a, S> {
        ProjectLocationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project that the agent to fetch is associated with. Format: `projects/` or `projects//locations/`.
    pub fn locations_get_agent(&self, parent: &str) -> ProjectLocationGetAgentCall<'a, S> {
        ProjectLocationGetAgentCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists information about the supported locations for this service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource that owns the locations collection, if applicable.
    pub fn locations_list(&self, name: &str) -> ProjectLocationListCall<'a, S> {
        ProjectLocationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates/updates the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project of this agent. Format: `projects/` or `projects//locations/`
    pub fn locations_set_agent(&self, request: GoogleCloudDialogflowV2beta1Agent, parent: &str) -> ProjectLocationSetAgentCall<'a, S> {
        ProjectLocationSetAgentCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn operations_cancel(&self, name: &str) -> ProjectOperationCancelCall<'a, S> {
        ProjectOperationCancelCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn operations_get(&self, name: &str) -> ProjectOperationGetCall<'a, S> {
        ProjectOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn operations_list(&self, name: &str) -> ProjectOperationListCall<'a, S> {
        ProjectOperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project that the agent to delete is associated with. Format: `projects/` or `projects//locations/`.
    pub fn delete_agent(&self, parent: &str) -> ProjectDeleteAgentCall<'a, S> {
        ProjectDeleteAgentCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project that the agent to fetch is associated with. Format: `projects/` or `projects//locations/`.
    pub fn get_agent(&self, parent: &str) -> ProjectGetAgentCall<'a, S> {
        ProjectGetAgentCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates/updates the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project of this agent. Format: `projects/` or `projects//locations/`
    pub fn set_agent(&self, request: GoogleCloudDialogflowV2beta1Agent, parent: &str) -> ProjectSetAgentCall<'a, S> {
        ProjectSetAgentCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



