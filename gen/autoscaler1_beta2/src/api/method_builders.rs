use super::*;
/// A builder providing access to all methods supported on *autoscaler* resources.
/// It is not used directly, but through the [`AutoscalerHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_autoscaler1_beta2 as autoscaler1_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use autoscaler1_beta2::{AutoscalerHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AutoscalerHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.autoscalers();
/// # }
/// ```
pub struct AutoscalerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AutoscalerHub<S>,
}

impl<'a, S> client::MethodsBuilder for AutoscalerMethods<'a, S> {}

impl<'a, S> AutoscalerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified Autoscaler resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID of Autoscaler resource.
    /// * `zone` - Zone name of Autoscaler resource.
    /// * `autoscaler` - Name of the Autoscaler resource.
    pub fn delete(&self, project: &str, zone: &str, autoscaler: &str) -> AutoscalerDeleteCall<'a, S> {
        AutoscalerDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _autoscaler: autoscaler.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified Autoscaler resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID of Autoscaler resource.
    /// * `zone` - Zone name of Autoscaler resource.
    /// * `autoscaler` - Name of the Autoscaler resource.
    pub fn get(&self, project: &str, zone: &str, autoscaler: &str) -> AutoscalerGetCall<'a, S> {
        AutoscalerGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _autoscaler: autoscaler.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds new Autoscaler resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID of Autoscaler resource.
    /// * `zone` - Zone name of Autoscaler resource.
    pub fn insert(&self, request: Autoscaler, project: &str, zone: &str) -> AutoscalerInsertCall<'a, S> {
        AutoscalerInsertCall {
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
    /// Lists all Autoscaler resources in this zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID of Autoscaler resource.
    /// * `zone` - Zone name of Autoscaler resource.
    pub fn list(&self, project: &str, zone: &str) -> AutoscalerListCall<'a, S> {
        AutoscalerListCall {
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
    /// Update the entire content of the Autoscaler resource. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID of Autoscaler resource.
    /// * `zone` - Zone name of Autoscaler resource.
    /// * `autoscaler` - Name of the Autoscaler resource.
    pub fn patch(&self, request: Autoscaler, project: &str, zone: &str, autoscaler: &str) -> AutoscalerPatchCall<'a, S> {
        AutoscalerPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _autoscaler: autoscaler.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update the entire content of the Autoscaler resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID of Autoscaler resource.
    /// * `zone` - Zone name of Autoscaler resource.
    /// * `autoscaler` - Name of the Autoscaler resource.
    pub fn update(&self, request: Autoscaler, project: &str, zone: &str, autoscaler: &str) -> AutoscalerUpdateCall<'a, S> {
        AutoscalerUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _autoscaler: autoscaler.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *zoneOperation* resources.
/// It is not used directly, but through the [`AutoscalerHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_autoscaler1_beta2 as autoscaler1_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use autoscaler1_beta2::{AutoscalerHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AutoscalerHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.zone_operations();
/// # }
/// ```
pub struct ZoneOperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AutoscalerHub<S>,
}

impl<'a, S> client::MethodsBuilder for ZoneOperationMethods<'a, S> {}

impl<'a, S> ZoneOperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified zone-specific operation resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - No description provided.
    /// * `zone` - No description provided.
    /// * `operation` - No description provided.
    pub fn delete(&self, project: &str, zone: &str, operation: &str) -> ZoneOperationDeleteCall<'a, S> {
        ZoneOperationDeleteCall {
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
    /// Retrieves the specified zone-specific operation resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - No description provided.
    /// * `zone` - No description provided.
    /// * `operation` - No description provided.
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
    /// Retrieves the list of operation resources contained within the specified zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - No description provided.
    /// * `zone` - No description provided.
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



/// A builder providing access to all methods supported on *zone* resources.
/// It is not used directly, but through the [`AutoscalerHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_autoscaler1_beta2 as autoscaler1_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use autoscaler1_beta2::{AutoscalerHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AutoscalerHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.zones();
/// # }
/// ```
pub struct ZoneMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AutoscalerHub<S>,
}

impl<'a, S> client::MethodsBuilder for ZoneMethods<'a, S> {}

impl<'a, S> ZoneMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `project` - No description provided.
    pub fn list(&self, project: &str) -> ZoneListCall<'a, S> {
        ZoneListCall {
            hub: self.hub,
            _project: project.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



