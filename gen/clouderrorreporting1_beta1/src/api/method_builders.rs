use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Clouderrorreporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_clouderrorreporting1_beta1 as clouderrorreporting1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use clouderrorreporting1_beta1::{Clouderrorreporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Clouderrorreporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete_events(...)`, `events_list(...)`, `events_report(...)`, `group_stats_list(...)`, `groups_get(...)` and `groups_update(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Clouderrorreporting<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the specified events.
    /// 
    /// # Arguments
    ///
    /// * `projectName` - Required. The resource name of the Google Cloud Platform project. Written as `projects/{projectID}`, where `{projectID}` is the [Google Cloud Platform project ID](https://support.google.com/cloud/answer/6158840). Example: `projects/my-project-123`.
    pub fn events_list(&self, project_name: &str) -> ProjectEventListCall<'a, S> {
        ProjectEventListCall {
            hub: self.hub,
            _project_name: project_name.to_string(),
            _time_range_period: Default::default(),
            _service_filter_version: Default::default(),
            _service_filter_service: Default::default(),
            _service_filter_resource_type: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _group_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Report an individual error event and record the event to a log. This endpoint accepts **either** an OAuth token, **or** an [API key](https://support.google.com/cloud/answer/6158862) for authentication. To use an API key, append it to the URL as the value of a `key` parameter. For example: `POST https://clouderrorreporting.googleapis.com/v1beta1/{projectName}/events:report?key=123ABC456` **Note:** [Error Reporting] (https://cloud.google.com/error-reporting) is a global service built on Cloud Logging and doesn't analyze logs stored in regional log buckets or logs routed to other Google Cloud projects. For more information, see [Using Error Reporting with regionalized logs] (https://cloud.google.com/error-reporting/docs/regionalization).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectName` - Required. The resource name of the Google Cloud Platform project. Written as `projects/{projectId}`, where `{projectId}` is the [Google Cloud Platform project ID](https://support.google.com/cloud/answer/6158840). Example: // `projects/my-project-123`.
    pub fn events_report(&self, request: ReportedErrorEvent, project_name: &str) -> ProjectEventReportCall<'a, S> {
        ProjectEventReportCall {
            hub: self.hub,
            _request: request,
            _project_name: project_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the specified groups.
    /// 
    /// # Arguments
    ///
    /// * `projectName` - Required. The resource name of the Google Cloud Platform project. Written as `projects/{projectID}` or `projects/{projectNumber}`, where `{projectID}` and `{projectNumber}` can be found in the [Google Cloud console](https://support.google.com/cloud/answer/6158840). Examples: `projects/my-project-123`, `projects/5551234`.
    pub fn group_stats_list(&self, project_name: &str) -> ProjectGroupStatListCall<'a, S> {
        ProjectGroupStatListCall {
            hub: self.hub,
            _project_name: project_name.to_string(),
            _timed_count_duration: Default::default(),
            _time_range_period: Default::default(),
            _service_filter_version: Default::default(),
            _service_filter_service: Default::default(),
            _service_filter_resource_type: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order: Default::default(),
            _group_id: Default::default(),
            _alignment_time: Default::default(),
            _alignment: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the specified group.
    /// 
    /// # Arguments
    ///
    /// * `groupName` - Required. The group resource name. Written as `projects/{projectID}/groups/{group_name}`. Call groupStats.list to return a list of groups belonging to this project. Example: `projects/my-project-123/groups/my-group`
    pub fn groups_get(&self, group_name: &str) -> ProjectGroupGetCall<'a, S> {
        ProjectGroupGetCall {
            hub: self.hub,
            _group_name: group_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replace the data for the specified group. Fails if the group does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The group resource name. Example: projects/my-project-123/groups/CNSgkpnppqKCUw
    pub fn groups_update(&self, request: ErrorGroup, name: &str) -> ProjectGroupUpdateCall<'a, S> {
        ProjectGroupUpdateCall {
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
    /// Deletes all error events of a given project.
    /// 
    /// # Arguments
    ///
    /// * `projectName` - Required. The resource name of the Google Cloud Platform project. Written as `projects/{projectID}`, where `{projectID}` is the [Google Cloud Platform project ID](https://support.google.com/cloud/answer/6158840). Example: `projects/my-project-123`.
    pub fn delete_events(&self, project_name: &str) -> ProjectDeleteEventCall<'a, S> {
        ProjectDeleteEventCall {
            hub: self.hub,
            _project_name: project_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



