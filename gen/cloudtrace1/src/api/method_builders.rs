use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudTrace`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudtrace1 as cloudtrace1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudtrace1::{CloudTrace, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudTrace::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `patch_traces(...)`, `traces_get(...)` and `traces_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudTrace<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single trace by its ID.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. ID of the Cloud project where the trace data is stored.
    /// * `traceId` - Required. ID of the trace to return.
    pub fn traces_get(&self, project_id: &str, trace_id: &str) -> ProjectTraceGetCall<'a, S> {
        ProjectTraceGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _trace_id: trace_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of traces that match the specified filter conditions.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. ID of the Cloud project where the trace data is stored.
    pub fn traces_list(&self, project_id: &str) -> ProjectTraceListCall<'a, S> {
        ProjectTraceListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _view: Default::default(),
            _start_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _end_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sends new traces to Cloud Trace or updates existing traces. If the ID of a trace that you send matches that of an existing trace, any fields in the existing trace and its spans are overwritten by the provided values, and any new fields provided are merged with the existing trace data. If the ID does not match, a new trace is created.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. ID of the Cloud project where the trace data is stored.
    pub fn patch_traces(&self, request: Traces, project_id: &str) -> ProjectPatchTraceCall<'a, S> {
        ProjectPatchTraceCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



