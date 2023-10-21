use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Contactcenterinsights`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_contactcenterinsights1 as contactcenterinsights1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use contactcenterinsights1::{Contactcenterinsights, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Contactcenterinsights::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_conversations_analyses_create(...)`, `locations_conversations_analyses_delete(...)`, `locations_conversations_analyses_get(...)`, `locations_conversations_analyses_list(...)`, `locations_conversations_bulk_analyze(...)`, `locations_conversations_calculate_stats(...)`, `locations_conversations_create(...)`, `locations_conversations_delete(...)`, `locations_conversations_get(...)`, `locations_conversations_ingest(...)`, `locations_conversations_list(...)`, `locations_conversations_patch(...)`, `locations_get_settings(...)`, `locations_insightsdata_export(...)`, `locations_issue_models_calculate_issue_model_stats(...)`, `locations_issue_models_create(...)`, `locations_issue_models_delete(...)`, `locations_issue_models_deploy(...)`, `locations_issue_models_get(...)`, `locations_issue_models_issues_delete(...)`, `locations_issue_models_issues_get(...)`, `locations_issue_models_issues_list(...)`, `locations_issue_models_issues_patch(...)`, `locations_issue_models_list(...)`, `locations_issue_models_patch(...)`, `locations_issue_models_undeploy(...)`, `locations_operations_cancel(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_phrase_matchers_create(...)`, `locations_phrase_matchers_delete(...)`, `locations_phrase_matchers_get(...)`, `locations_phrase_matchers_list(...)`, `locations_phrase_matchers_patch(...)`, `locations_update_settings(...)`, `locations_views_create(...)`, `locations_views_delete(...)`, `locations_views_get(...)`, `locations_views_list(...)` and `locations_views_patch(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Contactcenterinsights<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an analysis. The long running operation is done when the analysis has completed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the analysis.
    pub fn locations_conversations_analyses_create(&self, request: GoogleCloudContactcenterinsightsV1Analysis, parent: &str) -> ProjectLocationConversationAnalysisCreateCall<'a, S> {
        ProjectLocationConversationAnalysisCreateCall {
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
    /// Deletes an analysis.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the analysis to delete.
    pub fn locations_conversations_analyses_delete(&self, name: &str) -> ProjectLocationConversationAnalysisDeleteCall<'a, S> {
        ProjectLocationConversationAnalysisDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an analysis.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the analysis to get.
    pub fn locations_conversations_analyses_get(&self, name: &str) -> ProjectLocationConversationAnalysisGetCall<'a, S> {
        ProjectLocationConversationAnalysisGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists analyses.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource of the analyses.
    pub fn locations_conversations_analyses_list(&self, parent: &str) -> ProjectLocationConversationAnalysisListCall<'a, S> {
        ProjectLocationConversationAnalysisListCall {
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
    /// Analyzes multiple conversations in a single request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource to create analyses in.
    pub fn locations_conversations_bulk_analyze(&self, request: GoogleCloudContactcenterinsightsV1BulkAnalyzeConversationsRequest, parent: &str) -> ProjectLocationConversationBulkAnalyzeCall<'a, S> {
        ProjectLocationConversationBulkAnalyzeCall {
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
    /// Gets conversation statistics.
    /// 
    /// # Arguments
    ///
    /// * `location` - Required. The location of the conversations.
    pub fn locations_conversations_calculate_stats(&self, location: &str) -> ProjectLocationConversationCalculateStatCall<'a, S> {
        ProjectLocationConversationCalculateStatCall {
            hub: self.hub,
            _location: location.to_string(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a conversation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the conversation.
    pub fn locations_conversations_create(&self, request: GoogleCloudContactcenterinsightsV1Conversation, parent: &str) -> ProjectLocationConversationCreateCall<'a, S> {
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
    /// Deletes a conversation.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the conversation to delete.
    pub fn locations_conversations_delete(&self, name: &str) -> ProjectLocationConversationDeleteCall<'a, S> {
        ProjectLocationConversationDeleteCall {
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
    /// Gets a conversation.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the conversation to get.
    pub fn locations_conversations_get(&self, name: &str) -> ProjectLocationConversationGetCall<'a, S> {
        ProjectLocationConversationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Imports conversations and processes them according to the user's configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource for new conversations.
    pub fn locations_conversations_ingest(&self, request: GoogleCloudContactcenterinsightsV1IngestConversationsRequest, parent: &str) -> ProjectLocationConversationIngestCall<'a, S> {
        ProjectLocationConversationIngestCall {
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
    /// Lists conversations.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource of the conversation.
    pub fn locations_conversations_list(&self, parent: &str) -> ProjectLocationConversationListCall<'a, S> {
        ProjectLocationConversationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _view: Default::default(),
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
    /// Updates a conversation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name of the conversation. Format: projects/{project}/locations/{location}/conversations/{conversation}
    pub fn locations_conversations_patch(&self, request: GoogleCloudContactcenterinsightsV1Conversation, name: &str) -> ProjectLocationConversationPatchCall<'a, S> {
        ProjectLocationConversationPatchCall {
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
    /// Export insights data to a destination defined in the request body.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource to export data from.
    pub fn locations_insightsdata_export(&self, request: GoogleCloudContactcenterinsightsV1ExportInsightsDataRequest, parent: &str) -> ProjectLocationInsightsdataExportCall<'a, S> {
        ProjectLocationInsightsdataExportCall {
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
    /// Deletes an issue.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the issue to delete.
    pub fn locations_issue_models_issues_delete(&self, name: &str) -> ProjectLocationIssueModelIssueDeleteCall<'a, S> {
        ProjectLocationIssueModelIssueDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an issue.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the issue to get.
    pub fn locations_issue_models_issues_get(&self, name: &str) -> ProjectLocationIssueModelIssueGetCall<'a, S> {
        ProjectLocationIssueModelIssueGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists issues.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource of the issue.
    pub fn locations_issue_models_issues_list(&self, parent: &str) -> ProjectLocationIssueModelIssueListCall<'a, S> {
        ProjectLocationIssueModelIssueListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an issue.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name of the issue. Format: projects/{project}/locations/{location}/issueModels/{issue_model}/issues/{issue}
    pub fn locations_issue_models_issues_patch(&self, request: GoogleCloudContactcenterinsightsV1Issue, name: &str) -> ProjectLocationIssueModelIssuePatchCall<'a, S> {
        ProjectLocationIssueModelIssuePatchCall {
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
    /// Gets an issue model's statistics.
    /// 
    /// # Arguments
    ///
    /// * `issueModel` - Required. The resource name of the issue model to query against.
    pub fn locations_issue_models_calculate_issue_model_stats(&self, issue_model: &str) -> ProjectLocationIssueModelCalculateIssueModelStatCall<'a, S> {
        ProjectLocationIssueModelCalculateIssueModelStatCall {
            hub: self.hub,
            _issue_model: issue_model.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an issue model.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the issue model.
    pub fn locations_issue_models_create(&self, request: GoogleCloudContactcenterinsightsV1IssueModel, parent: &str) -> ProjectLocationIssueModelCreateCall<'a, S> {
        ProjectLocationIssueModelCreateCall {
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
    /// Deletes an issue model.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the issue model to delete.
    pub fn locations_issue_models_delete(&self, name: &str) -> ProjectLocationIssueModelDeleteCall<'a, S> {
        ProjectLocationIssueModelDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deploys an issue model. Returns an error if a model is already deployed. An issue model can only be used in analysis after it has been deployed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The issue model to deploy.
    pub fn locations_issue_models_deploy(&self, request: GoogleCloudContactcenterinsightsV1DeployIssueModelRequest, name: &str) -> ProjectLocationIssueModelDeployCall<'a, S> {
        ProjectLocationIssueModelDeployCall {
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
    /// Gets an issue model.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the issue model to get.
    pub fn locations_issue_models_get(&self, name: &str) -> ProjectLocationIssueModelGetCall<'a, S> {
        ProjectLocationIssueModelGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists issue models.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource of the issue model.
    pub fn locations_issue_models_list(&self, parent: &str) -> ProjectLocationIssueModelListCall<'a, S> {
        ProjectLocationIssueModelListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an issue model.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name of the issue model. Format: projects/{project}/locations/{location}/issueModels/{issue_model}
    pub fn locations_issue_models_patch(&self, request: GoogleCloudContactcenterinsightsV1IssueModel, name: &str) -> ProjectLocationIssueModelPatchCall<'a, S> {
        ProjectLocationIssueModelPatchCall {
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
    /// Undeploys an issue model. An issue model can not be used in analysis after it has been undeployed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The issue model to undeploy.
    pub fn locations_issue_models_undeploy(&self, request: GoogleCloudContactcenterinsightsV1UndeployIssueModelRequest, name: &str) -> ProjectLocationIssueModelUndeployCall<'a, S> {
        ProjectLocationIssueModelUndeployCall {
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
    /// Creates a phrase matcher.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the phrase matcher. Required. The location to create a phrase matcher for. Format: `projects//locations/` or `projects//locations/`
    pub fn locations_phrase_matchers_create(&self, request: GoogleCloudContactcenterinsightsV1PhraseMatcher, parent: &str) -> ProjectLocationPhraseMatcherCreateCall<'a, S> {
        ProjectLocationPhraseMatcherCreateCall {
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
    /// Deletes a phrase matcher.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the phrase matcher to delete.
    pub fn locations_phrase_matchers_delete(&self, name: &str) -> ProjectLocationPhraseMatcherDeleteCall<'a, S> {
        ProjectLocationPhraseMatcherDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a phrase matcher.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the phrase matcher to get.
    pub fn locations_phrase_matchers_get(&self, name: &str) -> ProjectLocationPhraseMatcherGetCall<'a, S> {
        ProjectLocationPhraseMatcherGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists phrase matchers.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource of the phrase matcher.
    pub fn locations_phrase_matchers_list(&self, parent: &str) -> ProjectLocationPhraseMatcherListCall<'a, S> {
        ProjectLocationPhraseMatcherListCall {
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
    /// Updates a phrase matcher.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the phrase matcher. Format: projects/{project}/locations/{location}/phraseMatchers/{phrase_matcher}
    pub fn locations_phrase_matchers_patch(&self, request: GoogleCloudContactcenterinsightsV1PhraseMatcher, name: &str) -> ProjectLocationPhraseMatcherPatchCall<'a, S> {
        ProjectLocationPhraseMatcherPatchCall {
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
    /// Creates a view.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the view. Required. The location to create a view for. Format: `projects//locations/` or `projects//locations/`
    pub fn locations_views_create(&self, request: GoogleCloudContactcenterinsightsV1View, parent: &str) -> ProjectLocationViewCreateCall<'a, S> {
        ProjectLocationViewCreateCall {
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
    /// Deletes a view.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the view to delete.
    pub fn locations_views_delete(&self, name: &str) -> ProjectLocationViewDeleteCall<'a, S> {
        ProjectLocationViewDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a view.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the view to get.
    pub fn locations_views_get(&self, name: &str) -> ProjectLocationViewGetCall<'a, S> {
        ProjectLocationViewGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists views.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource of the views.
    pub fn locations_views_list(&self, parent: &str) -> ProjectLocationViewListCall<'a, S> {
        ProjectLocationViewListCall {
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
    /// Updates a view.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name of the view. Format: projects/{project}/locations/{location}/views/{view}
    pub fn locations_views_patch(&self, request: GoogleCloudContactcenterinsightsV1View, name: &str) -> ProjectLocationViewPatchCall<'a, S> {
        ProjectLocationViewPatchCall {
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
    /// Gets project-level settings.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the settings resource to get.
    pub fn locations_get_settings(&self, name: &str) -> ProjectLocationGetSettingCall<'a, S> {
        ProjectLocationGetSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates project-level settings.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name of the settings resource. Format: projects/{project}/locations/{location}/settings
    pub fn locations_update_settings(&self, request: GoogleCloudContactcenterinsightsV1Settings, name: &str) -> ProjectLocationUpdateSettingCall<'a, S> {
        ProjectLocationUpdateSettingCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



