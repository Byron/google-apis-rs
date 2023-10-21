use super::*;
/// A builder providing access to all methods supported on *location* resources.
/// It is not used directly, but through the [`MyBusinessPlaceActions`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessplaceactions1 as mybusinessplaceactions1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusinessplaceactions1::{MyBusinessPlaceActions, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusinessPlaceActions::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `place_action_links_create(...)`, `place_action_links_delete(...)`, `place_action_links_get(...)`, `place_action_links_list(...)` and `place_action_links_patch(...)`
/// // to build up your call.
/// let rb = hub.locations();
/// # }
/// ```
pub struct LocationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusinessPlaceActions<S>,
}

impl<'a, S> client::MethodsBuilder for LocationMethods<'a, S> {}

impl<'a, S> LocationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a place action link associated with the specified location, and returns it. The request is considered duplicate if the `parent`, `place_action_link.uri` and `place_action_link.place_action_type` are the same as a previous request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the location where to create this place action link. `locations/{location_id}`.
    pub fn place_action_links_create(&self, request: PlaceActionLink, parent: &str) -> LocationPlaceActionLinkCreateCall<'a, S> {
        LocationPlaceActionLinkCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a place action link from the specified location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the place action link to remove from the location.
    pub fn place_action_links_delete(&self, name: &str) -> LocationPlaceActionLinkDeleteCall<'a, S> {
        LocationPlaceActionLinkDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified place action link.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the place action link to fetch.
    pub fn place_action_links_get(&self, name: &str) -> LocationPlaceActionLinkGetCall<'a, S> {
        LocationPlaceActionLinkGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the place action links for the specified location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the location whose place action links will be listed. `locations/{location_id}`.
    pub fn place_action_links_list(&self, parent: &str) -> LocationPlaceActionLinkListCall<'a, S> {
        LocationPlaceActionLinkListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified place action link and returns it.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Optional. The resource name, in the format `locations/{location_id}/placeActionLinks/{place_action_link_id}`. The name field will only be considered in UpdatePlaceActionLink and DeletePlaceActionLink requests for updating and deleting links respectively. However, it will be ignored in CreatePlaceActionLink request, where `place_action_link_id` will be assigned by the server on successful creation of a new link and returned as part of the response.
    pub fn place_action_links_patch(&self, request: PlaceActionLink, name: &str) -> LocationPlaceActionLinkPatchCall<'a, S> {
        LocationPlaceActionLinkPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *placeActionTypeMetadata* resources.
/// It is not used directly, but through the [`MyBusinessPlaceActions`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessplaceactions1 as mybusinessplaceactions1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusinessplaceactions1::{MyBusinessPlaceActions, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusinessPlaceActions::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.place_action_type_metadata();
/// # }
/// ```
pub struct PlaceActionTypeMetadataMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusinessPlaceActions<S>,
}

impl<'a, S> client::MethodsBuilder for PlaceActionTypeMetadataMethods<'a, S> {}

impl<'a, S> PlaceActionTypeMetadataMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of available place action types for a location or country.
    pub fn list(&self) -> PlaceActionTypeMetadataListCall<'a, S> {
        PlaceActionTypeMetadataListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



