use super::*;
/// A builder providing access to all methods supported on *location* resources.
/// It is not used directly, but through the [`MyBusinessBusinessCalls`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessbusinesscalls1 as mybusinessbusinesscalls1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusinessbusinesscalls1::{MyBusinessBusinessCalls, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusinessBusinessCalls::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `businesscallsinsights_list(...)`, `get_businesscallssettings(...)` and `update_businesscallssettings(...)`
/// // to build up your call.
/// let rb = hub.locations();
/// # }
/// ```
pub struct LocationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusinessBusinessCalls<S>,
}

impl<'a, S> client::MethodsBuilder for LocationMethods<'a, S> {}

impl<'a, S> LocationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns insights for Business calls for a location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent location to fetch calls insights for. Format: locations/{location_id}
    pub fn businesscallsinsights_list(&self, parent: &str) -> LocationBusinesscallsinsightListCall<'a, S> {
        LocationBusinesscallsinsightListCall {
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
    /// Returns the Business calls settings resource for the given location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The BusinessCallsSettings to get. The `name` field is used to identify the business call settings to get. Format: locations/{location_id}/businesscallssettings.
    pub fn get_businesscallssettings(&self, name: &str) -> LocationGetBusinesscallssettingCall<'a, S> {
        LocationGetBusinesscallssettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Business call settings for the specified location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the calls settings. Format: locations/{location}/businesscallssettings
    pub fn update_businesscallssettings(&self, request: BusinessCallsSettings, name: &str) -> LocationUpdateBusinesscallssettingCall<'a, S> {
        LocationUpdateBusinesscallssettingCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



