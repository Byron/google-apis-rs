use super::*;
/// A builder providing access to all methods supported on *controller* resources.
/// It is not used directly, but through the [`CloudDebugger`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_clouddebugger2 as clouddebugger2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use clouddebugger2::{CloudDebugger, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudDebugger::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `debuggees_breakpoints_list(...)`, `debuggees_breakpoints_update(...)` and `debuggees_register(...)`
/// // to build up your call.
/// let rb = hub.controller();
/// # }
/// ```
pub struct ControllerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudDebugger<S>,
}

impl<'a, S> client::MethodsBuilder for ControllerMethods<'a, S> {}

impl<'a, S> ControllerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all active breakpoints for the debuggee. The breakpoint specification (`location`, `condition`, and `expressions` fields) is semantically immutable, although the field values may change. For example, an agent may update the location line number to reflect the actual line where the breakpoint was set, but this doesn't change the breakpoint semantics. This means that an agent does not need to check if a breakpoint has changed when it encounters the same breakpoint on a successive call. Moreover, an agent should remember the breakpoints that are completed until the controller removes them from the active list to avoid setting those breakpoints again.
    /// 
    /// # Arguments
    ///
    /// * `debuggeeId` - Required. Identifies the debuggee.
    pub fn debuggees_breakpoints_list(&self, debuggee_id: &str) -> ControllerDebuggeeBreakpointListCall<'a, S> {
        ControllerDebuggeeBreakpointListCall {
            hub: self.hub,
            _debuggee_id: debuggee_id.to_string(),
            _wait_token: Default::default(),
            _success_on_timeout: Default::default(),
            _agent_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the breakpoint state or mutable fields. The entire Breakpoint message must be sent back to the controller service. Updates to active breakpoint fields are only allowed if the new value does not change the breakpoint specification. Updates to the `location`, `condition` and `expressions` fields should not alter the breakpoint semantics. These may only make changes such as canonicalizing a value or snapping the location to the correct line of code.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `debuggeeId` - Required. Identifies the debuggee being debugged.
    /// * `id` - Breakpoint identifier, unique in the scope of the debuggee.
    pub fn debuggees_breakpoints_update(&self, request: UpdateActiveBreakpointRequest, debuggee_id: &str, id: &str) -> ControllerDebuggeeBreakpointUpdateCall<'a, S> {
        ControllerDebuggeeBreakpointUpdateCall {
            hub: self.hub,
            _request: request,
            _debuggee_id: debuggee_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Registers the debuggee with the controller service. All agents attached to the same application must call this method with exactly the same request content to get back the same stable `debuggee_id`. Agents should call this method again whenever `google.rpc.Code.NOT_FOUND` is returned from any controller method. This protocol allows the controller service to disable debuggees, recover from data loss, or change the `debuggee_id` format. Agents must handle `debuggee_id` value changing upon re-registration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn debuggees_register(&self, request: RegisterDebuggeeRequest) -> ControllerDebuggeeRegisterCall<'a, S> {
        ControllerDebuggeeRegisterCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *debugger* resources.
/// It is not used directly, but through the [`CloudDebugger`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_clouddebugger2 as clouddebugger2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use clouddebugger2::{CloudDebugger, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudDebugger::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `debuggees_breakpoints_delete(...)`, `debuggees_breakpoints_get(...)`, `debuggees_breakpoints_list(...)`, `debuggees_breakpoints_set(...)` and `debuggees_list(...)`
/// // to build up your call.
/// let rb = hub.debugger();
/// # }
/// ```
pub struct DebuggerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudDebugger<S>,
}

impl<'a, S> client::MethodsBuilder for DebuggerMethods<'a, S> {}

impl<'a, S> DebuggerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the breakpoint from the debuggee.
    /// 
    /// # Arguments
    ///
    /// * `debuggeeId` - Required. ID of the debuggee whose breakpoint to delete.
    /// * `breakpointId` - Required. ID of the breakpoint to delete.
    pub fn debuggees_breakpoints_delete(&self, debuggee_id: &str, breakpoint_id: &str) -> DebuggerDebuggeeBreakpointDeleteCall<'a, S> {
        DebuggerDebuggeeBreakpointDeleteCall {
            hub: self.hub,
            _debuggee_id: debuggee_id.to_string(),
            _breakpoint_id: breakpoint_id.to_string(),
            _client_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets breakpoint information.
    /// 
    /// # Arguments
    ///
    /// * `debuggeeId` - Required. ID of the debuggee whose breakpoint to get.
    /// * `breakpointId` - Required. ID of the breakpoint to get.
    pub fn debuggees_breakpoints_get(&self, debuggee_id: &str, breakpoint_id: &str) -> DebuggerDebuggeeBreakpointGetCall<'a, S> {
        DebuggerDebuggeeBreakpointGetCall {
            hub: self.hub,
            _debuggee_id: debuggee_id.to_string(),
            _breakpoint_id: breakpoint_id.to_string(),
            _client_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all breakpoints for the debuggee.
    /// 
    /// # Arguments
    ///
    /// * `debuggeeId` - Required. ID of the debuggee whose breakpoints to list.
    pub fn debuggees_breakpoints_list(&self, debuggee_id: &str) -> DebuggerDebuggeeBreakpointListCall<'a, S> {
        DebuggerDebuggeeBreakpointListCall {
            hub: self.hub,
            _debuggee_id: debuggee_id.to_string(),
            _wait_token: Default::default(),
            _strip_results: Default::default(),
            _include_inactive: Default::default(),
            _include_all_users: Default::default(),
            _client_version: Default::default(),
            _action_value: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the breakpoint to the debuggee.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `debuggeeId` - Required. ID of the debuggee where the breakpoint is to be set.
    pub fn debuggees_breakpoints_set(&self, request: Breakpoint, debuggee_id: &str) -> DebuggerDebuggeeBreakpointSetCall<'a, S> {
        DebuggerDebuggeeBreakpointSetCall {
            hub: self.hub,
            _request: request,
            _debuggee_id: debuggee_id.to_string(),
            _client_version: Default::default(),
            _canary_option: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the debuggees that the user has access to.
    pub fn debuggees_list(&self) -> DebuggerDebuggeeListCall<'a, S> {
        DebuggerDebuggeeListCall {
            hub: self.hub,
            _project: Default::default(),
            _include_inactive: Default::default(),
            _client_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



