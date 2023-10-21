use super::*;
/// A builder providing access to all methods supported on *zoneOperation* resources.
/// It is not used directly, but through the [`Resourceviews`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_resourceviews1_beta2 as resourceviews1_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use resourceviews1_beta2::{Resourceviews, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Resourceviews::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.zone_operations();
/// # }
/// ```
pub struct ZoneOperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Resourceviews<S>,
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
    /// Retrieves the list of operation resources contained within the specified zone.
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



/// A builder providing access to all methods supported on *zoneView* resources.
/// It is not used directly, but through the [`Resourceviews`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_resourceviews1_beta2 as resourceviews1_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use resourceviews1_beta2::{Resourceviews, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Resourceviews::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_resources(...)`, `delete(...)`, `get(...)`, `get_service(...)`, `insert(...)`, `list(...)`, `list_resources(...)`, `remove_resources(...)` and `set_service(...)`
/// // to build up your call.
/// let rb = hub.zone_views();
/// # }
/// ```
pub struct ZoneViewMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Resourceviews<S>,
}

impl<'a, S> client::MethodsBuilder for ZoneViewMethods<'a, S> {}

impl<'a, S> ZoneViewMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Add resources to the view.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project name of the resource view.
    /// * `zone` - The zone name of the resource view.
    /// * `resourceView` - The name of the resource view.
    pub fn add_resources(&self, request: ZoneViewsAddResourcesRequest, project: &str, zone: &str, resource_view: &str) -> ZoneViewAddResourceCall<'a, S> {
        ZoneViewAddResourceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource_view: resource_view.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a resource view.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project name of the resource view.
    /// * `zone` - The zone name of the resource view.
    /// * `resourceView` - The name of the resource view.
    pub fn delete(&self, project: &str, zone: &str, resource_view: &str) -> ZoneViewDeleteCall<'a, S> {
        ZoneViewDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource_view: resource_view.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the information of a zonal resource view.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project name of the resource view.
    /// * `zone` - The zone name of the resource view.
    /// * `resourceView` - The name of the resource view.
    pub fn get(&self, project: &str, zone: &str, resource_view: &str) -> ZoneViewGetCall<'a, S> {
        ZoneViewGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource_view: resource_view.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the service information of a resource view or a resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project name of the resource view.
    /// * `zone` - The zone name of the resource view.
    /// * `resourceView` - The name of the resource view.
    pub fn get_service(&self, project: &str, zone: &str, resource_view: &str) -> ZoneViewGetServiceCall<'a, S> {
        ZoneViewGetServiceCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource_view: resource_view.to_string(),
            _resource_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a resource view.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project name of the resource view.
    /// * `zone` - The zone name of the resource view.
    pub fn insert(&self, request: ResourceView, project: &str, zone: &str) -> ZoneViewInsertCall<'a, S> {
        ZoneViewInsertCall {
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
    /// List resource views.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project name of the resource view.
    /// * `zone` - The zone name of the resource view.
    pub fn list(&self, project: &str, zone: &str) -> ZoneViewListCall<'a, S> {
        ZoneViewListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the resources of the resource view.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project name of the resource view.
    /// * `zone` - The zone name of the resource view.
    /// * `resourceView` - The name of the resource view.
    pub fn list_resources(&self, project: &str, zone: &str, resource_view: &str) -> ZoneViewListResourceCall<'a, S> {
        ZoneViewListResourceCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource_view: resource_view.to_string(),
            _service_name: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _list_state: Default::default(),
            _format: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Remove resources from the view.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project name of the resource view.
    /// * `zone` - The zone name of the resource view.
    /// * `resourceView` - The name of the resource view.
    pub fn remove_resources(&self, request: ZoneViewsRemoveResourcesRequest, project: &str, zone: &str, resource_view: &str) -> ZoneViewRemoveResourceCall<'a, S> {
        ZoneViewRemoveResourceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource_view: resource_view.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update the service information of a resource view or a resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project name of the resource view.
    /// * `zone` - The zone name of the resource view.
    /// * `resourceView` - The name of the resource view.
    pub fn set_service(&self, request: ZoneViewsSetServiceRequest, project: &str, zone: &str, resource_view: &str) -> ZoneViewSetServiceCall<'a, S> {
        ZoneViewSetServiceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource_view: resource_view.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



