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
/// extern crate google_dialogflow3 as dialogflow3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dialogflow3::{Dialogflow, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dialogflow::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_agents_changelogs_get(...)`, `locations_agents_changelogs_list(...)`, `locations_agents_create(...)`, `locations_agents_delete(...)`, `locations_agents_entity_types_create(...)`, `locations_agents_entity_types_delete(...)`, `locations_agents_entity_types_get(...)`, `locations_agents_entity_types_list(...)`, `locations_agents_entity_types_patch(...)`, `locations_agents_environments_continuous_test_results_list(...)`, `locations_agents_environments_create(...)`, `locations_agents_environments_delete(...)`, `locations_agents_environments_deploy_flow(...)`, `locations_agents_environments_deployments_get(...)`, `locations_agents_environments_deployments_list(...)`, `locations_agents_environments_experiments_create(...)`, `locations_agents_environments_experiments_delete(...)`, `locations_agents_environments_experiments_get(...)`, `locations_agents_environments_experiments_list(...)`, `locations_agents_environments_experiments_patch(...)`, `locations_agents_environments_experiments_start(...)`, `locations_agents_environments_experiments_stop(...)`, `locations_agents_environments_get(...)`, `locations_agents_environments_list(...)`, `locations_agents_environments_lookup_environment_history(...)`, `locations_agents_environments_patch(...)`, `locations_agents_environments_run_continuous_test(...)`, `locations_agents_environments_sessions_detect_intent(...)`, `locations_agents_environments_sessions_entity_types_create(...)`, `locations_agents_environments_sessions_entity_types_delete(...)`, `locations_agents_environments_sessions_entity_types_get(...)`, `locations_agents_environments_sessions_entity_types_list(...)`, `locations_agents_environments_sessions_entity_types_patch(...)`, `locations_agents_environments_sessions_fulfill_intent(...)`, `locations_agents_environments_sessions_match_intent(...)`, `locations_agents_export(...)`, `locations_agents_flows_create(...)`, `locations_agents_flows_delete(...)`, `locations_agents_flows_export(...)`, `locations_agents_flows_get(...)`, `locations_agents_flows_get_validation_result(...)`, `locations_agents_flows_import(...)`, `locations_agents_flows_list(...)`, `locations_agents_flows_pages_create(...)`, `locations_agents_flows_pages_delete(...)`, `locations_agents_flows_pages_get(...)`, `locations_agents_flows_pages_list(...)`, `locations_agents_flows_pages_patch(...)`, `locations_agents_flows_patch(...)`, `locations_agents_flows_train(...)`, `locations_agents_flows_transition_route_groups_create(...)`, `locations_agents_flows_transition_route_groups_delete(...)`, `locations_agents_flows_transition_route_groups_get(...)`, `locations_agents_flows_transition_route_groups_list(...)`, `locations_agents_flows_transition_route_groups_patch(...)`, `locations_agents_flows_validate(...)`, `locations_agents_flows_versions_compare_versions(...)`, `locations_agents_flows_versions_create(...)`, `locations_agents_flows_versions_delete(...)`, `locations_agents_flows_versions_get(...)`, `locations_agents_flows_versions_list(...)`, `locations_agents_flows_versions_load(...)`, `locations_agents_flows_versions_patch(...)`, `locations_agents_get(...)`, `locations_agents_get_validation_result(...)`, `locations_agents_intents_create(...)`, `locations_agents_intents_delete(...)`, `locations_agents_intents_get(...)`, `locations_agents_intents_list(...)`, `locations_agents_intents_patch(...)`, `locations_agents_list(...)`, `locations_agents_patch(...)`, `locations_agents_restore(...)`, `locations_agents_sessions_detect_intent(...)`, `locations_agents_sessions_entity_types_create(...)`, `locations_agents_sessions_entity_types_delete(...)`, `locations_agents_sessions_entity_types_get(...)`, `locations_agents_sessions_entity_types_list(...)`, `locations_agents_sessions_entity_types_patch(...)`, `locations_agents_sessions_fulfill_intent(...)`, `locations_agents_sessions_match_intent(...)`, `locations_agents_test_cases_batch_delete(...)`, `locations_agents_test_cases_batch_run(...)`, `locations_agents_test_cases_calculate_coverage(...)`, `locations_agents_test_cases_create(...)`, `locations_agents_test_cases_export(...)`, `locations_agents_test_cases_get(...)`, `locations_agents_test_cases_import(...)`, `locations_agents_test_cases_list(...)`, `locations_agents_test_cases_patch(...)`, `locations_agents_test_cases_results_get(...)`, `locations_agents_test_cases_results_list(...)`, `locations_agents_test_cases_run(...)`, `locations_agents_validate(...)`, `locations_agents_webhooks_create(...)`, `locations_agents_webhooks_delete(...)`, `locations_agents_webhooks_get(...)`, `locations_agents_webhooks_list(...)`, `locations_agents_webhooks_patch(...)`, `locations_get(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_security_settings_create(...)`, `locations_security_settings_delete(...)`, `locations_security_settings_get(...)`, `locations_security_settings_list(...)`, `locations_security_settings_patch(...)`, `operations_cancel(...)`, `operations_get(...)` and `operations_list(...)`
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
    /// Retrieves the specified Changelog.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the changelog to get. Format: `projects//locations//agents//changelogs/`.
    pub fn locations_agents_changelogs_get(&self, name: &str) -> ProjectLocationAgentChangelogGetCall<'a, S> {
        ProjectLocationAgentChangelogGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of Changelogs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The agent containing the changelogs. Format: `projects//locations//agents/`.
    pub fn locations_agents_changelogs_list(&self, parent: &str) -> ProjectLocationAgentChangelogListCall<'a, S> {
        ProjectLocationAgentChangelogListCall {
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
    /// Creates an entity type in the specified agent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to create a entity type for. Format: `projects//locations//agents/`.
    pub fn locations_agents_entity_types_create(&self, request: GoogleCloudDialogflowCxV3EntityType, parent: &str) -> ProjectLocationAgentEntityTypeCreateCall<'a, S> {
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
    /// Deletes the specified entity type. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the entity type to delete. Format: `projects//locations//agents//entityTypes/`.
    pub fn locations_agents_entity_types_delete(&self, name: &str) -> ProjectLocationAgentEntityTypeDeleteCall<'a, S> {
        ProjectLocationAgentEntityTypeDeleteCall {
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
    /// Retrieves the specified entity type.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the entity type. Format: `projects//locations//agents//entityTypes/`.
    pub fn locations_agents_entity_types_get(&self, name: &str) -> ProjectLocationAgentEntityTypeGetCall<'a, S> {
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
    /// * `parent` - Required. The agent to list all entity types for. Format: `projects//locations//agents/`.
    pub fn locations_agents_entity_types_list(&self, parent: &str) -> ProjectLocationAgentEntityTypeListCall<'a, S> {
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
    /// Updates the specified entity type. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType. Format: `projects//locations//agents//entityTypes/`.
    pub fn locations_agents_entity_types_patch(&self, request: GoogleCloudDialogflowCxV3EntityType, name: &str) -> ProjectLocationAgentEntityTypePatchCall<'a, S> {
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
    /// Fetches a list of continuous test results for a given environment.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The environment to list results for. Format: `projects//locations//agents// environments/`.
    pub fn locations_agents_environments_continuous_test_results_list(&self, parent: &str) -> ProjectLocationAgentEnvironmentContinuousTestResultListCall<'a, S> {
        ProjectLocationAgentEnvironmentContinuousTestResultListCall {
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
    /// Retrieves the specified Deployment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Deployment. Format: `projects//locations//agents//environments//deployments/`.
    pub fn locations_agents_environments_deployments_get(&self, name: &str) -> ProjectLocationAgentEnvironmentDeploymentGetCall<'a, S> {
        ProjectLocationAgentEnvironmentDeploymentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all deployments in the specified Environment.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The Environment to list all environments for. Format: `projects//locations//agents//environments/`.
    pub fn locations_agents_environments_deployments_list(&self, parent: &str) -> ProjectLocationAgentEnvironmentDeploymentListCall<'a, S> {
        ProjectLocationAgentEnvironmentDeploymentListCall {
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
    /// Creates an Experiment in the specified Environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The Agent to create an Environment for. Format: `projects//locations//agents//environments/`.
    pub fn locations_agents_environments_experiments_create(&self, request: GoogleCloudDialogflowCxV3Experiment, parent: &str) -> ProjectLocationAgentEnvironmentExperimentCreateCall<'a, S> {
        ProjectLocationAgentEnvironmentExperimentCreateCall {
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
    /// Deletes the specified Experiment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Environment to delete. Format: `projects//locations//agents//environments//experiments/`.
    pub fn locations_agents_environments_experiments_delete(&self, name: &str) -> ProjectLocationAgentEnvironmentExperimentDeleteCall<'a, S> {
        ProjectLocationAgentEnvironmentExperimentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified Experiment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Environment. Format: `projects//locations//agents//environments//experiments/`.
    pub fn locations_agents_environments_experiments_get(&self, name: &str) -> ProjectLocationAgentEnvironmentExperimentGetCall<'a, S> {
        ProjectLocationAgentEnvironmentExperimentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all experiments in the specified Environment.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The Environment to list all environments for. Format: `projects//locations//agents//environments/`.
    pub fn locations_agents_environments_experiments_list(&self, parent: &str) -> ProjectLocationAgentEnvironmentExperimentListCall<'a, S> {
        ProjectLocationAgentEnvironmentExperimentListCall {
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
    /// Updates the specified Experiment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the experiment. Format: projects//locations//agents//environments//experiments/..
    pub fn locations_agents_environments_experiments_patch(&self, request: GoogleCloudDialogflowCxV3Experiment, name: &str) -> ProjectLocationAgentEnvironmentExperimentPatchCall<'a, S> {
        ProjectLocationAgentEnvironmentExperimentPatchCall {
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
    /// Starts the specified Experiment. This rpc only changes the state of experiment from PENDING to RUNNING.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of the experiment to start. Format: `projects//locations//agents//environments//experiments/`.
    pub fn locations_agents_environments_experiments_start(&self, request: GoogleCloudDialogflowCxV3StartExperimentRequest, name: &str) -> ProjectLocationAgentEnvironmentExperimentStartCall<'a, S> {
        ProjectLocationAgentEnvironmentExperimentStartCall {
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
    /// Stops the specified Experiment. This rpc only changes the state of experiment from RUNNING to DONE.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of the experiment to stop. Format: `projects//locations//agents//environments//experiments/`.
    pub fn locations_agents_environments_experiments_stop(&self, request: GoogleCloudDialogflowCxV3StopExperimentRequest, name: &str) -> ProjectLocationAgentEnvironmentExperimentStopCall<'a, S> {
        ProjectLocationAgentEnvironmentExperimentStopCall {
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
    /// Creates a session entity type.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The session to create a session entity type for. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment.
    pub fn locations_agents_environments_sessions_entity_types_create(&self, request: GoogleCloudDialogflowCxV3SessionEntityType, parent: &str) -> ProjectLocationAgentEnvironmentSessionEntityTypeCreateCall<'a, S> {
        ProjectLocationAgentEnvironmentSessionEntityTypeCreateCall {
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
    /// Deletes the specified session entity type.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the session entity type to delete. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment.
    pub fn locations_agents_environments_sessions_entity_types_delete(&self, name: &str) -> ProjectLocationAgentEnvironmentSessionEntityTypeDeleteCall<'a, S> {
        ProjectLocationAgentEnvironmentSessionEntityTypeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified session entity type.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the session entity type. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment.
    pub fn locations_agents_environments_sessions_entity_types_get(&self, name: &str) -> ProjectLocationAgentEnvironmentSessionEntityTypeGetCall<'a, S> {
        ProjectLocationAgentEnvironmentSessionEntityTypeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all session entity types in the specified session.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The session to list all session entity types from. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment.
    pub fn locations_agents_environments_sessions_entity_types_list(&self, parent: &str) -> ProjectLocationAgentEnvironmentSessionEntityTypeListCall<'a, S> {
        ProjectLocationAgentEnvironmentSessionEntityTypeListCall {
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
    /// Updates the specified session entity type.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique identifier of the session entity type. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment.
    pub fn locations_agents_environments_sessions_entity_types_patch(&self, request: GoogleCloudDialogflowCxV3SessionEntityType, name: &str) -> ProjectLocationAgentEnvironmentSessionEntityTypePatchCall<'a, S> {
        ProjectLocationAgentEnvironmentSessionEntityTypePatchCall {
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
    /// Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause session entity types to be updated, which in turn might affect results of future queries. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/cx/docs/concept/version).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/cx/docs/concept/version).
    pub fn locations_agents_environments_sessions_detect_intent(&self, request: GoogleCloudDialogflowCxV3DetectIntentRequest, session: &str) -> ProjectLocationAgentEnvironmentSessionDetectIntentCall<'a, S> {
        ProjectLocationAgentEnvironmentSessionDetectIntentCall {
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
    /// Fulfills a matched intent returned by MatchIntent. Must be called after MatchIntent, with input from MatchIntentResponse. Otherwise, the behavior is undefined.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session).
    pub fn locations_agents_environments_sessions_fulfill_intent(&self, request: GoogleCloudDialogflowCxV3FulfillIntentRequest, session: &str) -> ProjectLocationAgentEnvironmentSessionFulfillIntentCall<'a, S> {
        ProjectLocationAgentEnvironmentSessionFulfillIntentCall {
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
    /// Returns preliminary intent match results, doesn't change the session status.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session).
    pub fn locations_agents_environments_sessions_match_intent(&self, request: GoogleCloudDialogflowCxV3MatchIntentRequest, session: &str) -> ProjectLocationAgentEnvironmentSessionMatchIntentCall<'a, S> {
        ProjectLocationAgentEnvironmentSessionMatchIntentCall {
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
    /// Creates an Environment in the specified Agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: Environment
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The Agent to create an Environment for. Format: `projects//locations//agents/`.
    pub fn locations_agents_environments_create(&self, request: GoogleCloudDialogflowCxV3Environment, parent: &str) -> ProjectLocationAgentEnvironmentCreateCall<'a, S> {
        ProjectLocationAgentEnvironmentCreateCall {
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
    /// Deletes the specified Environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Environment to delete. Format: `projects//locations//agents//environments/`.
    pub fn locations_agents_environments_delete(&self, name: &str) -> ProjectLocationAgentEnvironmentDeleteCall<'a, S> {
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
    /// Deploys a flow to the specified Environment. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: DeployFlowMetadata - `response`: DeployFlowResponse
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `environment` - Required. The environment to deploy the flow to. Format: `projects//locations//agents// environments/`.
    pub fn locations_agents_environments_deploy_flow(&self, request: GoogleCloudDialogflowCxV3DeployFlowRequest, environment: &str) -> ProjectLocationAgentEnvironmentDeployFlowCall<'a, S> {
        ProjectLocationAgentEnvironmentDeployFlowCall {
            hub: self.hub,
            _request: request,
            _environment: environment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified Environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Environment. Format: `projects//locations//agents//environments/`.
    pub fn locations_agents_environments_get(&self, name: &str) -> ProjectLocationAgentEnvironmentGetCall<'a, S> {
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
    /// Returns the list of all environments in the specified Agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The Agent to list all environments for. Format: `projects//locations//agents/`.
    pub fn locations_agents_environments_list(&self, parent: &str) -> ProjectLocationAgentEnvironmentListCall<'a, S> {
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
    /// Looks up the history of the specified Environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the environment to look up the history for. Format: `projects//locations//agents//environments/`.
    pub fn locations_agents_environments_lookup_environment_history(&self, name: &str) -> ProjectLocationAgentEnvironmentLookupEnvironmentHistoryCall<'a, S> {
        ProjectLocationAgentEnvironmentLookupEnvironmentHistoryCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified Environment. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: Environment
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the environment. Format: `projects//locations//agents//environments/`.
    pub fn locations_agents_environments_patch(&self, request: GoogleCloudDialogflowCxV3Environment, name: &str) -> ProjectLocationAgentEnvironmentPatchCall<'a, S> {
        ProjectLocationAgentEnvironmentPatchCall {
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
    /// Kicks off a continuous test under the specified Environment. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: RunContinuousTestMetadata - `response`: RunContinuousTestResponse
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `environment` - Required. Format: `projects//locations//agents//environments/`.
    pub fn locations_agents_environments_run_continuous_test(&self, request: GoogleCloudDialogflowCxV3RunContinuousTestRequest, environment: &str) -> ProjectLocationAgentEnvironmentRunContinuousTestCall<'a, S> {
        ProjectLocationAgentEnvironmentRunContinuousTestCall {
            hub: self.hub,
            _request: request,
            _environment: environment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a page in the specified flow. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The flow to create a page for. Format: `projects//locations//agents//flows/`.
    pub fn locations_agents_flows_pages_create(&self, request: GoogleCloudDialogflowCxV3Page, parent: &str) -> ProjectLocationAgentFlowPageCreateCall<'a, S> {
        ProjectLocationAgentFlowPageCreateCall {
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
    /// Deletes the specified page. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the page to delete. Format: `projects//locations//agents//Flows//pages/`.
    pub fn locations_agents_flows_pages_delete(&self, name: &str) -> ProjectLocationAgentFlowPageDeleteCall<'a, S> {
        ProjectLocationAgentFlowPageDeleteCall {
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
    /// Retrieves the specified page.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the page. Format: `projects//locations//agents//flows//pages/`.
    pub fn locations_agents_flows_pages_get(&self, name: &str) -> ProjectLocationAgentFlowPageGetCall<'a, S> {
        ProjectLocationAgentFlowPageGetCall {
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
    /// Returns the list of all pages in the specified flow.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The flow to list all pages for. Format: `projects//locations//agents//flows/`.
    pub fn locations_agents_flows_pages_list(&self, parent: &str) -> ProjectLocationAgentFlowPageListCall<'a, S> {
        ProjectLocationAgentFlowPageListCall {
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
    /// Updates the specified page. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique identifier of the page. Required for the Pages.UpdatePage method. Pages.CreatePage populates the name automatically. Format: `projects//locations//agents//flows//pages/`.
    pub fn locations_agents_flows_pages_patch(&self, request: GoogleCloudDialogflowCxV3Page, name: &str) -> ProjectLocationAgentFlowPagePatchCall<'a, S> {
        ProjectLocationAgentFlowPagePatchCall {
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
    /// Creates an TransitionRouteGroup in the specified flow. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The flow to create an TransitionRouteGroup for. Format: `projects//locations//agents//flows/`.
    pub fn locations_agents_flows_transition_route_groups_create(&self, request: GoogleCloudDialogflowCxV3TransitionRouteGroup, parent: &str) -> ProjectLocationAgentFlowTransitionRouteGroupCreateCall<'a, S> {
        ProjectLocationAgentFlowTransitionRouteGroupCreateCall {
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
    /// Deletes the specified TransitionRouteGroup. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the TransitionRouteGroup to delete. Format: `projects//locations//agents//flows//transitionRouteGroups/`.
    pub fn locations_agents_flows_transition_route_groups_delete(&self, name: &str) -> ProjectLocationAgentFlowTransitionRouteGroupDeleteCall<'a, S> {
        ProjectLocationAgentFlowTransitionRouteGroupDeleteCall {
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
    /// Retrieves the specified TransitionRouteGroup.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the TransitionRouteGroup. Format: `projects//locations//agents//flows//transitionRouteGroups/`.
    pub fn locations_agents_flows_transition_route_groups_get(&self, name: &str) -> ProjectLocationAgentFlowTransitionRouteGroupGetCall<'a, S> {
        ProjectLocationAgentFlowTransitionRouteGroupGetCall {
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
    /// Returns the list of all transition route groups in the specified flow.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The flow to list all transition route groups for. Format: `projects//locations//agents//flows/`.
    pub fn locations_agents_flows_transition_route_groups_list(&self, parent: &str) -> ProjectLocationAgentFlowTransitionRouteGroupListCall<'a, S> {
        ProjectLocationAgentFlowTransitionRouteGroupListCall {
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
    /// Updates the specified TransitionRouteGroup. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique identifier of the transition route group. TransitionRouteGroups.CreateTransitionRouteGroup populates the name automatically. Format: `projects//locations//agents//flows//transitionRouteGroups/`.
    pub fn locations_agents_flows_transition_route_groups_patch(&self, request: GoogleCloudDialogflowCxV3TransitionRouteGroup, name: &str) -> ProjectLocationAgentFlowTransitionRouteGroupPatchCall<'a, S> {
        ProjectLocationAgentFlowTransitionRouteGroupPatchCall {
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
    /// Compares the specified base version with target version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `baseVersion` - Required. Name of the base flow version to compare with the target version. Use version ID `0` to indicate the draft version of the specified flow. Format: `projects//locations//agents/ /flows//versions/`.
    pub fn locations_agents_flows_versions_compare_versions(&self, request: GoogleCloudDialogflowCxV3CompareVersionsRequest, base_version: &str) -> ProjectLocationAgentFlowVersionCompareVersionCall<'a, S> {
        ProjectLocationAgentFlowVersionCompareVersionCall {
            hub: self.hub,
            _request: request,
            _base_version: base_version.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Version in the specified Flow. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: CreateVersionOperationMetadata - `response`: Version
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The Flow to create an Version for. Format: `projects//locations//agents//flows/`.
    pub fn locations_agents_flows_versions_create(&self, request: GoogleCloudDialogflowCxV3Version, parent: &str) -> ProjectLocationAgentFlowVersionCreateCall<'a, S> {
        ProjectLocationAgentFlowVersionCreateCall {
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
    /// Deletes the specified Version.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Version to delete. Format: `projects//locations//agents//flows//versions/`.
    pub fn locations_agents_flows_versions_delete(&self, name: &str) -> ProjectLocationAgentFlowVersionDeleteCall<'a, S> {
        ProjectLocationAgentFlowVersionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified Version.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Version. Format: `projects//locations//agents//flows//versions/`.
    pub fn locations_agents_flows_versions_get(&self, name: &str) -> ProjectLocationAgentFlowVersionGetCall<'a, S> {
        ProjectLocationAgentFlowVersionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all versions in the specified Flow.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The Flow to list all versions for. Format: `projects//locations//agents//flows/`.
    pub fn locations_agents_flows_versions_list(&self, parent: &str) -> ProjectLocationAgentFlowVersionListCall<'a, S> {
        ProjectLocationAgentFlowVersionListCall {
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
    /// Loads resources in the specified version to the draft flow. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The Version to be loaded to draft flow. Format: `projects//locations//agents//flows//versions/`.
    pub fn locations_agents_flows_versions_load(&self, request: GoogleCloudDialogflowCxV3LoadVersionRequest, name: &str) -> ProjectLocationAgentFlowVersionLoadCall<'a, S> {
        ProjectLocationAgentFlowVersionLoadCall {
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
    /// Updates the specified Version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Format: projects//locations//agents//flows//versions/. Version ID is a self-increasing number generated by Dialogflow upon version creation.
    pub fn locations_agents_flows_versions_patch(&self, request: GoogleCloudDialogflowCxV3Version, name: &str) -> ProjectLocationAgentFlowVersionPatchCall<'a, S> {
        ProjectLocationAgentFlowVersionPatchCall {
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
    /// Creates a flow in the specified agent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to create a flow for. Format: `projects//locations//agents/`.
    pub fn locations_agents_flows_create(&self, request: GoogleCloudDialogflowCxV3Flow, parent: &str) -> ProjectLocationAgentFlowCreateCall<'a, S> {
        ProjectLocationAgentFlowCreateCall {
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
    /// Deletes a specified flow.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the flow to delete. Format: `projects//locations//agents//flows/`.
    pub fn locations_agents_flows_delete(&self, name: &str) -> ProjectLocationAgentFlowDeleteCall<'a, S> {
        ProjectLocationAgentFlowDeleteCall {
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
    /// Exports the specified flow to a binary file. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: ExportFlowResponse Note that resources (e.g. intents, entities, webhooks) that the flow references will also be exported.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the flow to export. Format: `projects//locations//agents//flows/`.
    pub fn locations_agents_flows_export(&self, request: GoogleCloudDialogflowCxV3ExportFlowRequest, name: &str) -> ProjectLocationAgentFlowExportCall<'a, S> {
        ProjectLocationAgentFlowExportCall {
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
    /// Retrieves the specified flow.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the flow to get. Format: `projects//locations//agents//flows/`.
    pub fn locations_agents_flows_get(&self, name: &str) -> ProjectLocationAgentFlowGetCall<'a, S> {
        ProjectLocationAgentFlowGetCall {
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
    /// Gets the latest flow validation result. Flow validation is performed when ValidateFlow is called.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The flow name. Format: `projects//locations//agents//flows//validationResult`.
    pub fn locations_agents_flows_get_validation_result(&self, name: &str) -> ProjectLocationAgentFlowGetValidationResultCall<'a, S> {
        ProjectLocationAgentFlowGetValidationResultCall {
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
    /// Imports the specified flow to the specified agent from a binary file. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: ImportFlowResponse Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to import the flow into. Format: `projects//locations//agents/`.
    pub fn locations_agents_flows_import(&self, request: GoogleCloudDialogflowCxV3ImportFlowRequest, parent: &str) -> ProjectLocationAgentFlowImportCall<'a, S> {
        ProjectLocationAgentFlowImportCall {
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
    /// Returns the list of all flows in the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The agent containing the flows. Format: `projects//locations//agents/`.
    pub fn locations_agents_flows_list(&self, parent: &str) -> ProjectLocationAgentFlowListCall<'a, S> {
        ProjectLocationAgentFlowListCall {
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
    /// Updates the specified flow. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique identifier of the flow. Format: `projects//locations//agents//flows/`.
    pub fn locations_agents_flows_patch(&self, request: GoogleCloudDialogflowCxV3Flow, name: &str) -> ProjectLocationAgentFlowPatchCall<'a, S> {
        ProjectLocationAgentFlowPatchCall {
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
    /// Trains the specified flow. Note that only the flow in 'draft' environment is trained. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The flow to train. Format: `projects//locations//agents//flows/`.
    pub fn locations_agents_flows_train(&self, request: GoogleCloudDialogflowCxV3TrainFlowRequest, name: &str) -> ProjectLocationAgentFlowTrainCall<'a, S> {
        ProjectLocationAgentFlowTrainCall {
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
    /// Validates the specified flow and creates or updates validation results. Please call this API after the training is completed to get the complete validation results.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The flow to validate. Format: `projects//locations//agents//flows/`.
    pub fn locations_agents_flows_validate(&self, request: GoogleCloudDialogflowCxV3ValidateFlowRequest, name: &str) -> ProjectLocationAgentFlowValidateCall<'a, S> {
        ProjectLocationAgentFlowValidateCall {
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
    /// Creates an intent in the specified agent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to create an intent for. Format: `projects//locations//agents/`.
    pub fn locations_agents_intents_create(&self, request: GoogleCloudDialogflowCxV3Intent, parent: &str) -> ProjectLocationAgentIntentCreateCall<'a, S> {
        ProjectLocationAgentIntentCreateCall {
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
    /// Deletes the specified intent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the intent to delete. Format: `projects//locations//agents//intents/`.
    pub fn locations_agents_intents_delete(&self, name: &str) -> ProjectLocationAgentIntentDeleteCall<'a, S> {
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
    /// * `name` - Required. The name of the intent. Format: `projects//locations//agents//intents/`.
    pub fn locations_agents_intents_get(&self, name: &str) -> ProjectLocationAgentIntentGetCall<'a, S> {
        ProjectLocationAgentIntentGetCall {
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
    /// Returns the list of all intents in the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The agent to list all intents for. Format: `projects//locations//agents/`.
    pub fn locations_agents_intents_list(&self, parent: &str) -> ProjectLocationAgentIntentListCall<'a, S> {
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
    /// Updates the specified intent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique identifier of the intent. Required for the Intents.UpdateIntent method. Intents.CreateIntent populates the name automatically. Format: `projects//locations//agents//intents/`.
    pub fn locations_agents_intents_patch(&self, request: GoogleCloudDialogflowCxV3Intent, name: &str) -> ProjectLocationAgentIntentPatchCall<'a, S> {
        ProjectLocationAgentIntentPatchCall {
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
    /// Creates a session entity type.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The session to create a session entity type for. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment.
    pub fn locations_agents_sessions_entity_types_create(&self, request: GoogleCloudDialogflowCxV3SessionEntityType, parent: &str) -> ProjectLocationAgentSessionEntityTypeCreateCall<'a, S> {
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
    /// Deletes the specified session entity type.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the session entity type to delete. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment.
    pub fn locations_agents_sessions_entity_types_delete(&self, name: &str) -> ProjectLocationAgentSessionEntityTypeDeleteCall<'a, S> {
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
    /// Retrieves the specified session entity type.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the session entity type. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment.
    pub fn locations_agents_sessions_entity_types_get(&self, name: &str) -> ProjectLocationAgentSessionEntityTypeGetCall<'a, S> {
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
    /// Returns the list of all session entity types in the specified session.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The session to list all session entity types from. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment.
    pub fn locations_agents_sessions_entity_types_list(&self, parent: &str) -> ProjectLocationAgentSessionEntityTypeListCall<'a, S> {
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
    /// Updates the specified session entity type.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique identifier of the session entity type. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment.
    pub fn locations_agents_sessions_entity_types_patch(&self, request: GoogleCloudDialogflowCxV3SessionEntityType, name: &str) -> ProjectLocationAgentSessionEntityTypePatchCall<'a, S> {
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
    /// Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause session entity types to be updated, which in turn might affect results of future queries. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/cx/docs/concept/version).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/cx/docs/concept/version).
    pub fn locations_agents_sessions_detect_intent(&self, request: GoogleCloudDialogflowCxV3DetectIntentRequest, session: &str) -> ProjectLocationAgentSessionDetectIntentCall<'a, S> {
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
    /// Fulfills a matched intent returned by MatchIntent. Must be called after MatchIntent, with input from MatchIntentResponse. Otherwise, the behavior is undefined.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session).
    pub fn locations_agents_sessions_fulfill_intent(&self, request: GoogleCloudDialogflowCxV3FulfillIntentRequest, session: &str) -> ProjectLocationAgentSessionFulfillIntentCall<'a, S> {
        ProjectLocationAgentSessionFulfillIntentCall {
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
    /// Returns preliminary intent match results, doesn't change the session status.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session).
    pub fn locations_agents_sessions_match_intent(&self, request: GoogleCloudDialogflowCxV3MatchIntentRequest, session: &str) -> ProjectLocationAgentSessionMatchIntentCall<'a, S> {
        ProjectLocationAgentSessionMatchIntentCall {
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
    /// Gets a test case result.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the testcase. Format: `projects//locations//agents//testCases//results/`.
    pub fn locations_agents_test_cases_results_get(&self, name: &str) -> ProjectLocationAgentTestCaseResultGetCall<'a, S> {
        ProjectLocationAgentTestCaseResultGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches a list of results for a given test case.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The test case to list results for. Format: `projects//locations//agents// testCases/`. Specify a `-` as a wildcard for TestCase ID to list results across multiple test cases.
    pub fn locations_agents_test_cases_results_list(&self, parent: &str) -> ProjectLocationAgentTestCaseResultListCall<'a, S> {
        ProjectLocationAgentTestCaseResultListCall {
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
    /// Batch deletes test cases.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to delete test cases from. Format: `projects//locations//agents/`.
    pub fn locations_agents_test_cases_batch_delete(&self, request: GoogleCloudDialogflowCxV3BatchDeleteTestCasesRequest, parent: &str) -> ProjectLocationAgentTestCaseBatchDeleteCall<'a, S> {
        ProjectLocationAgentTestCaseBatchDeleteCall {
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
    /// Kicks off a batch run of test cases. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: BatchRunTestCasesMetadata - `response`: BatchRunTestCasesResponse
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Agent name. Format: `projects//locations//agents/ `.
    pub fn locations_agents_test_cases_batch_run(&self, request: GoogleCloudDialogflowCxV3BatchRunTestCasesRequest, parent: &str) -> ProjectLocationAgentTestCaseBatchRunCall<'a, S> {
        ProjectLocationAgentTestCaseBatchRunCall {
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
    /// Calculates the test coverage for an agent.
    /// 
    /// # Arguments
    ///
    /// * `agent` - Required. The agent to calculate coverage for. Format: `projects//locations//agents/`.
    pub fn locations_agents_test_cases_calculate_coverage(&self, agent: &str) -> ProjectLocationAgentTestCaseCalculateCoverageCall<'a, S> {
        ProjectLocationAgentTestCaseCalculateCoverageCall {
            hub: self.hub,
            _agent: agent.to_string(),
            _type_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a test case for the given agent.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to create the test case for. Format: `projects//locations//agents/`.
    pub fn locations_agents_test_cases_create(&self, request: GoogleCloudDialogflowCxV3TestCase, parent: &str) -> ProjectLocationAgentTestCaseCreateCall<'a, S> {
        ProjectLocationAgentTestCaseCreateCall {
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
    /// Exports the test cases under the agent to a Cloud Storage bucket or a local file. Filter can be applied to export a subset of test cases. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: ExportTestCasesMetadata - `response`: ExportTestCasesResponse
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent where to export test cases from. Format: `projects//locations//agents/`.
    pub fn locations_agents_test_cases_export(&self, request: GoogleCloudDialogflowCxV3ExportTestCasesRequest, parent: &str) -> ProjectLocationAgentTestCaseExportCall<'a, S> {
        ProjectLocationAgentTestCaseExportCall {
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
    /// Gets a test case.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the testcase. Format: `projects//locations//agents//testCases/`.
    pub fn locations_agents_test_cases_get(&self, name: &str) -> ProjectLocationAgentTestCaseGetCall<'a, S> {
        ProjectLocationAgentTestCaseGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Imports the test cases from a Cloud Storage bucket or a local file. It always creates new test cases and won't overwrite any existing ones. The provided ID in the imported test case is neglected. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: ImportTestCasesMetadata - `response`: ImportTestCasesResponse
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to import test cases to. Format: `projects//locations//agents/`.
    pub fn locations_agents_test_cases_import(&self, request: GoogleCloudDialogflowCxV3ImportTestCasesRequest, parent: &str) -> ProjectLocationAgentTestCaseImportCall<'a, S> {
        ProjectLocationAgentTestCaseImportCall {
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
    /// Fetches a list of test cases for a given agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The agent to list all pages for. Format: `projects//locations//agents/`.
    pub fn locations_agents_test_cases_list(&self, parent: &str) -> ProjectLocationAgentTestCaseListCall<'a, S> {
        ProjectLocationAgentTestCaseListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _view: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified test case.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique identifier of the test case. TestCases.CreateTestCase will populate the name automatically. Otherwise use format: `projects//locations//agents/ /testCases/`.
    pub fn locations_agents_test_cases_patch(&self, request: GoogleCloudDialogflowCxV3TestCase, name: &str) -> ProjectLocationAgentTestCasePatchCall<'a, S> {
        ProjectLocationAgentTestCasePatchCall {
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
    /// Kicks off a test case run. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: RunTestCaseMetadata - `response`: RunTestCaseResponse
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format of test case name to run: `projects//locations/ /agents//testCases/`.
    pub fn locations_agents_test_cases_run(&self, request: GoogleCloudDialogflowCxV3RunTestCaseRequest, name: &str) -> ProjectLocationAgentTestCaseRunCall<'a, S> {
        ProjectLocationAgentTestCaseRunCall {
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
    /// Creates a webhook in the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The agent to create a webhook for. Format: `projects//locations//agents/`.
    pub fn locations_agents_webhooks_create(&self, request: GoogleCloudDialogflowCxV3Webhook, parent: &str) -> ProjectLocationAgentWebhookCreateCall<'a, S> {
        ProjectLocationAgentWebhookCreateCall {
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
    /// Deletes the specified webhook.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the webhook to delete. Format: `projects//locations//agents//webhooks/`.
    pub fn locations_agents_webhooks_delete(&self, name: &str) -> ProjectLocationAgentWebhookDeleteCall<'a, S> {
        ProjectLocationAgentWebhookDeleteCall {
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
    /// Retrieves the specified webhook.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the webhook. Format: `projects//locations//agents//webhooks/`.
    pub fn locations_agents_webhooks_get(&self, name: &str) -> ProjectLocationAgentWebhookGetCall<'a, S> {
        ProjectLocationAgentWebhookGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all webhooks in the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The agent to list all webhooks for. Format: `projects//locations//agents/`.
    pub fn locations_agents_webhooks_list(&self, parent: &str) -> ProjectLocationAgentWebhookListCall<'a, S> {
        ProjectLocationAgentWebhookListCall {
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
    /// Updates the specified webhook.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique identifier of the webhook. Required for the Webhooks.UpdateWebhook method. Webhooks.CreateWebhook populates the name automatically. Format: `projects//locations//agents//webhooks/`.
    pub fn locations_agents_webhooks_patch(&self, request: GoogleCloudDialogflowCxV3Webhook, name: &str) -> ProjectLocationAgentWebhookPatchCall<'a, S> {
        ProjectLocationAgentWebhookPatchCall {
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
    /// Creates an agent in the specified location. Note: You should always train flows prior to sending them queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The location to create a agent for. Format: `projects//locations/`.
    pub fn locations_agents_create(&self, request: GoogleCloudDialogflowCxV3Agent, parent: &str) -> ProjectLocationAgentCreateCall<'a, S> {
        ProjectLocationAgentCreateCall {
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
    /// Deletes the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the agent to delete. Format: `projects//locations//agents/`.
    pub fn locations_agents_delete(&self, name: &str) -> ProjectLocationAgentDeleteCall<'a, S> {
        ProjectLocationAgentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports the specified agent to a binary file. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: ExportAgentResponse
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the agent to export. Format: `projects//locations//agents/`.
    pub fn locations_agents_export(&self, request: GoogleCloudDialogflowCxV3ExportAgentRequest, name: &str) -> ProjectLocationAgentExportCall<'a, S> {
        ProjectLocationAgentExportCall {
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
    /// Retrieves the specified agent.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the agent. Format: `projects//locations//agents/`.
    pub fn locations_agents_get(&self, name: &str) -> ProjectLocationAgentGetCall<'a, S> {
        ProjectLocationAgentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest agent validation result. Agent validation is performed when ValidateAgent is called.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The agent name. Format: `projects//locations//agents//validationResult`.
    pub fn locations_agents_get_validation_result(&self, name: &str) -> ProjectLocationAgentGetValidationResultCall<'a, S> {
        ProjectLocationAgentGetValidationResultCall {
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
    /// Returns the list of all agents in the specified location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The location to list all agents for. Format: `projects//locations/`.
    pub fn locations_agents_list(&self, parent: &str) -> ProjectLocationAgentListCall<'a, S> {
        ProjectLocationAgentListCall {
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
    /// Updates the specified agent. Note: You should always train flows prior to sending them queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique identifier of the agent. Required for the Agents.UpdateAgent method. Agents.CreateAgent populates the name automatically. Format: `projects//locations//agents/`.
    pub fn locations_agents_patch(&self, request: GoogleCloudDialogflowCxV3Agent, name: &str) -> ProjectLocationAgentPatchCall<'a, S> {
        ProjectLocationAgentPatchCall {
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
    /// Restores the specified agent from a binary file. Replaces the current agent with a new one. Note that all existing resources in agent (e.g. intents, entity types, flows) will be removed. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train flows prior to sending them queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the agent to restore into. Format: `projects//locations//agents/`.
    pub fn locations_agents_restore(&self, request: GoogleCloudDialogflowCxV3RestoreAgentRequest, name: &str) -> ProjectLocationAgentRestoreCall<'a, S> {
        ProjectLocationAgentRestoreCall {
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
    /// Validates the specified agent and creates or updates validation results. The agent in draft version is validated. Please call this API after the training is completed to get the complete validation results.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The agent to validate. Format: `projects//locations//agents/`.
    pub fn locations_agents_validate(&self, request: GoogleCloudDialogflowCxV3ValidateAgentRequest, name: &str) -> ProjectLocationAgentValidateCall<'a, S> {
        ProjectLocationAgentValidateCall {
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
    /// Create security settings in the specified location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The location to create an SecuritySettings for. Format: `projects//locations/`.
    pub fn locations_security_settings_create(&self, request: GoogleCloudDialogflowCxV3SecuritySettings, parent: &str) -> ProjectLocationSecuritySettingCreateCall<'a, S> {
        ProjectLocationSecuritySettingCreateCall {
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
    /// Deletes the specified SecuritySettings.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the SecuritySettings to delete. Format: `projects//locations//securitySettings/`.
    pub fn locations_security_settings_delete(&self, name: &str) -> ProjectLocationSecuritySettingDeleteCall<'a, S> {
        ProjectLocationSecuritySettingDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified SecuritySettings. The returned settings may be stale by up to 1 minute.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the settings. Format: `projects//locations//securitySettings/`.
    pub fn locations_security_settings_get(&self, name: &str) -> ProjectLocationSecuritySettingGetCall<'a, S> {
        ProjectLocationSecuritySettingGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all security settings in the specified location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The location to list all security settings for. Format: `projects//locations/`.
    pub fn locations_security_settings_list(&self, parent: &str) -> ProjectLocationSecuritySettingListCall<'a, S> {
        ProjectLocationSecuritySettingListCall {
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
    /// Updates the specified SecuritySettings.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name of the settings. Required for the SecuritySettingsService.UpdateSecuritySettings method. SecuritySettingsService.CreateSecuritySettings populates the name automatically. Format: `projects//locations//securitySettings/`.
    pub fn locations_security_settings_patch(&self, request: GoogleCloudDialogflowCxV3SecuritySettings, name: &str) -> ProjectLocationSecuritySettingPatchCall<'a, S> {
        ProjectLocationSecuritySettingPatchCall {
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
}



