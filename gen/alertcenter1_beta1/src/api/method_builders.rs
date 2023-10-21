use super::*;
/// A builder providing access to all methods supported on *alert* resources.
/// It is not used directly, but through the [`AlertCenter`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_alertcenter1_beta1 as alertcenter1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use alertcenter1_beta1::{AlertCenter, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AlertCenter::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_delete(...)`, `batch_undelete(...)`, `delete(...)`, `feedback_create(...)`, `feedback_list(...)`, `get(...)`, `get_metadata(...)`, `list(...)` and `undelete(...)`
/// // to build up your call.
/// let rb = hub.alerts();
/// # }
/// ```
pub struct AlertMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AlertCenter<S>,
}

impl<'a, S> client::MethodsBuilder for AlertMethods<'a, S> {}

impl<'a, S> AlertMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates new feedback for an alert. Attempting to create a feedback for a non-existent alert returns `NOT_FOUND` error. Attempting to create a feedback for an alert that is marked for deletion returns `FAILED_PRECONDITION' error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `alertId` - Required. The identifier of the alert this feedback belongs to.
    pub fn feedback_create(&self, request: AlertFeedback, alert_id: &str) -> AlertFeedbackCreateCall<'a, S> {
        AlertFeedbackCreateCall {
            hub: self.hub,
            _request: request,
            _alert_id: alert_id.to_string(),
            _customer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the feedback for an alert. Attempting to list feedbacks for a non-existent alert returns `NOT_FOUND` error.
    /// 
    /// # Arguments
    ///
    /// * `alertId` - Required. The alert identifier. The "-" wildcard could be used to represent all alerts.
    pub fn feedback_list(&self, alert_id: &str) -> AlertFeedbackListCall<'a, S> {
        AlertFeedbackListCall {
            hub: self.hub,
            _alert_id: alert_id.to_string(),
            _filter: Default::default(),
            _customer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs batch delete operation on alerts.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn batch_delete(&self, request: BatchDeleteAlertsRequest) -> AlertBatchDeleteCall<'a, S> {
        AlertBatchDeleteCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs batch undelete operation on alerts.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn batch_undelete(&self, request: BatchUndeleteAlertsRequest) -> AlertBatchUndeleteCall<'a, S> {
        AlertBatchUndeleteCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks the specified alert for deletion. An alert that has been marked for deletion is removed from Alert Center after 30 days. Marking an alert for deletion has no effect on an alert which has already been marked for deletion. Attempting to mark a nonexistent alert for deletion results in a `NOT_FOUND` error.
    /// 
    /// # Arguments
    ///
    /// * `alertId` - Required. The identifier of the alert to delete.
    pub fn delete(&self, alert_id: &str) -> AlertDeleteCall<'a, S> {
        AlertDeleteCall {
            hub: self.hub,
            _alert_id: alert_id.to_string(),
            _customer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified alert. Attempting to get a nonexistent alert returns `NOT_FOUND` error.
    /// 
    /// # Arguments
    ///
    /// * `alertId` - Required. The identifier of the alert to retrieve.
    pub fn get(&self, alert_id: &str) -> AlertGetCall<'a, S> {
        AlertGetCall {
            hub: self.hub,
            _alert_id: alert_id.to_string(),
            _customer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the metadata of an alert. Attempting to get metadata for a non-existent alert returns `NOT_FOUND` error.
    /// 
    /// # Arguments
    ///
    /// * `alertId` - Required. The identifier of the alert this metadata belongs to.
    pub fn get_metadata(&self, alert_id: &str) -> AlertGetMetadataCall<'a, S> {
        AlertGetMetadataCall {
            hub: self.hub,
            _alert_id: alert_id.to_string(),
            _customer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the alerts.
    pub fn list(&self) -> AlertListCall<'a, S> {
        AlertListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _customer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Restores, or "undeletes", an alert that was marked for deletion within the past 30 days. Attempting to undelete an alert which was marked for deletion over 30 days ago (which has been removed from the Alert Center database) or a nonexistent alert returns a `NOT_FOUND` error. Attempting to undelete an alert which has not been marked for deletion has no effect.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `alertId` - Required. The identifier of the alert to undelete.
    pub fn undelete(&self, request: UndeleteAlertRequest, alert_id: &str) -> AlertUndeleteCall<'a, S> {
        AlertUndeleteCall {
            hub: self.hub,
            _request: request,
            _alert_id: alert_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the [`AlertCenter`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_alertcenter1_beta1 as alertcenter1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use alertcenter1_beta1::{AlertCenter, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AlertCenter::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_settings(...)` and `update_settings(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AlertCenter<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns customer-level settings.
    pub fn get_settings(&self) -> MethodGetSettingCall<'a, S> {
        MethodGetSettingCall {
            hub: self.hub,
            _customer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the customer-level settings.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_settings(&self, request: Settings) -> MethodUpdateSettingCall<'a, S> {
        MethodUpdateSettingCall {
            hub: self.hub,
            _request: request,
            _customer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



