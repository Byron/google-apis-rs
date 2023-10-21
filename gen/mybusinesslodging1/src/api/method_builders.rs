use super::*;
/// A builder providing access to all methods supported on *location* resources.
/// It is not used directly, but through the [`MyBusinessLodging`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinesslodging1 as mybusinesslodging1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusinesslodging1::{MyBusinessLodging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusinessLodging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_lodging(...)`, `lodging_get_google_updated(...)` and `update_lodging(...)`
/// // to build up your call.
/// let rb = hub.locations();
/// # }
/// ```
pub struct LocationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusinessLodging<S>,
}

impl<'a, S> client::MethodsBuilder for LocationMethods<'a, S> {}

impl<'a, S> LocationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the Google updated Lodging of a specific location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Google identifier for this location in the form: `accounts/{account_id}/locations/{location_id}/lodging`
    pub fn lodging_get_google_updated(&self, name: &str) -> LocationLodgingGetGoogleUpdatedCall<'a, S> {
        LocationLodgingGetGoogleUpdatedCall {
            hub: self.hub,
            _name: name.to_string(),
            _read_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the Lodging of a specific location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Google identifier for this location in the form: `locations/{location_id}/lodging`
    pub fn get_lodging(&self, name: &str) -> LocationGetLodgingCall<'a, S> {
        LocationGetLodgingCall {
            hub: self.hub,
            _name: name.to_string(),
            _read_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Lodging of a specific location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Google identifier for this location in the form: `locations/{location_id}/lodging`
    pub fn update_lodging(&self, request: Lodging, name: &str) -> LocationUpdateLodgingCall<'a, S> {
        LocationUpdateLodgingCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



