use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`WorkflowExecutions`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_workflowexecutions1 as workflowexecutions1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use workflowexecutions1::{WorkflowExecutions, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = WorkflowExecutions::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_workflows_executions_cancel(...)`, `locations_workflows_executions_create(...)`, `locations_workflows_executions_get(...)`, `locations_workflows_executions_list(...)` and `locations_workflows_trigger_pubsub_execution(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a WorkflowExecutions<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels an execution of the given name.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the execution to be cancelled. Format: projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution}
    pub fn locations_workflows_executions_cancel(&self, request: CancelExecutionRequest, name: &str) -> ProjectLocationWorkflowExecutionCancelCall<'a, S> {
        ProjectLocationWorkflowExecutionCancelCall {
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
    /// Creates a new execution using the latest revision of the given workflow.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the workflow for which an execution should be created. Format: projects/{project}/locations/{location}/workflows/{workflow} The latest revision of the workflow will be used.
    pub fn locations_workflows_executions_create(&self, request: Execution, parent: &str) -> ProjectLocationWorkflowExecutionCreateCall<'a, S> {
        ProjectLocationWorkflowExecutionCreateCall {
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
    /// Returns an execution of the given name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the execution to be retrieved. Format: projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution}
    pub fn locations_workflows_executions_get(&self, name: &str) -> ProjectLocationWorkflowExecutionGetCall<'a, S> {
        ProjectLocationWorkflowExecutionGetCall {
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
    /// Returns a list of executions which belong to the workflow with the given name. The method returns executions of all workflow revisions. Returned executions are ordered by their start time (newest first).
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the workflow for which the executions should be listed. Format: projects/{project}/locations/{location}/workflows/{workflow}
    pub fn locations_workflows_executions_list(&self, parent: &str) -> ProjectLocationWorkflowExecutionListCall<'a, S> {
        ProjectLocationWorkflowExecutionListCall {
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
    /// Triggers a new execution using the latest revision of the given workflow by a Pub/Sub push notification.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `workflow` - Required. Name of the workflow for which an execution should be created. Format: projects/{project}/locations/{location}/workflows/{workflow}
    pub fn locations_workflows_trigger_pubsub_execution(&self, request: TriggerPubsubExecutionRequest, workflow: &str) -> ProjectLocationWorkflowTriggerPubsubExecutionCall<'a, S> {
        ProjectLocationWorkflowTriggerPubsubExecutionCall {
            hub: self.hub,
            _request: request,
            _workflow: workflow.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



