use super::*;
/// A builder providing access to all methods supported on *rollingUpdate* resources.
/// It is not used directly, but through the [`Replicapoolupdater`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_replicapoolupdater1_beta1 as replicapoolupdater1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use replicapoolupdater1_beta1::{Replicapoolupdater, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Replicapoolupdater::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel(...)`, `get(...)`, `insert(...)`, `list(...)`, `list_instance_updates(...)`, `pause(...)`, `resume(...)` and `rollback(...)`
/// // to build up your call.
/// let rb = hub.rolling_updates();
/// # }
/// ```
pub struct RollingUpdateMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Replicapoolupdater<S>,
}

impl<'a, S> client::MethodsBuilder for RollingUpdateMethods<'a, S> {}

impl<'a, S> RollingUpdateMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels an update. The update must be PAUSED before it can be cancelled. This has no effect if the update is already CANCELLED.
    /// 
    /// # Arguments
    ///
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the update's target resides.
    /// * `rollingUpdate` - The name of the update.
    pub fn cancel(&self, project: &str, zone: &str, rolling_update: &str) -> RollingUpdateCancelCall<'a, S> {
        RollingUpdateCancelCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _rolling_update: rolling_update.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about an update.
    /// 
    /// # Arguments
    ///
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the update's target resides.
    /// * `rollingUpdate` - The name of the update.
    pub fn get(&self, project: &str, zone: &str, rolling_update: &str) -> RollingUpdateGetCall<'a, S> {
        RollingUpdateGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _rolling_update: rolling_update.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts and starts a new update.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the update's target resides.
    pub fn insert(&self, request: RollingUpdate, project: &str, zone: &str) -> RollingUpdateInsertCall<'a, S> {
        RollingUpdateInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists recent updates for a given managed instance group, in reverse chronological order and paginated format.
    /// 
    /// # Arguments
    ///
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the update's target resides.
    pub fn list(&self, project: &str, zone: &str) -> RollingUpdateListCall<'a, S> {
        RollingUpdateListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the current status for each instance within a given update.
    /// 
    /// # Arguments
    ///
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the update's target resides.
    /// * `rollingUpdate` - The name of the update.
    pub fn list_instance_updates(&self, project: &str, zone: &str, rolling_update: &str) -> RollingUpdateListInstanceUpdateCall<'a, S> {
        RollingUpdateListInstanceUpdateCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _rolling_update: rolling_update.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Pauses the update in state from ROLLING_FORWARD or ROLLING_BACK. Has no effect if invoked when the state of the update is PAUSED.
    /// 
    /// # Arguments
    ///
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the update's target resides.
    /// * `rollingUpdate` - The name of the update.
    pub fn pause(&self, project: &str, zone: &str, rolling_update: &str) -> RollingUpdatePauseCall<'a, S> {
        RollingUpdatePauseCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _rolling_update: rolling_update.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Continues an update in PAUSED state. Has no effect if invoked when the state of the update is ROLLED_OUT.
    /// 
    /// # Arguments
    ///
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the update's target resides.
    /// * `rollingUpdate` - The name of the update.
    pub fn resume(&self, project: &str, zone: &str, rolling_update: &str) -> RollingUpdateResumeCall<'a, S> {
        RollingUpdateResumeCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _rolling_update: rolling_update.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rolls back the update in state from ROLLING_FORWARD or PAUSED. Has no effect if invoked when the state of the update is ROLLED_BACK.
    /// 
    /// # Arguments
    ///
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the update's target resides.
    /// * `rollingUpdate` - The name of the update.
    pub fn rollback(&self, project: &str, zone: &str, rolling_update: &str) -> RollingUpdateRollbackCall<'a, S> {
        RollingUpdateRollbackCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _rolling_update: rolling_update.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *zoneOperation* resources.
/// It is not used directly, but through the [`Replicapoolupdater`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_replicapoolupdater1_beta1 as replicapoolupdater1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use replicapoolupdater1_beta1::{Replicapoolupdater, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Replicapoolupdater::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.zone_operations();
/// # }
/// ```
pub struct ZoneOperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Replicapoolupdater<S>,
}

impl<'a, S> client::MethodsBuilder for ZoneOperationMethods<'a, S> {}

impl<'a, S> ZoneOperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified zone-specific operation resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    /// * `zone` - Name of the zone scoping this request.
    /// * `operation` - Name of the operation resource to return.
    pub fn get(&self, project: &str, zone: &str, operation: &str) -> ZoneOperationGetCall<'a, S> {
        ZoneOperationGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of Operation resources contained within the specified zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    /// * `zone` - Name of the zone scoping this request.
    pub fn list(&self, project: &str, zone: &str) -> ZoneOperationListCall<'a, S> {
        ZoneOperationListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



