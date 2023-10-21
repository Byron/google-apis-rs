use super::*;
/// A builder providing access to all methods supported on *trip* resources.
/// It is not used directly, but through the [`QPXExpress`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_qpxexpress1 as qpxexpress1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use qpxexpress1::{QPXExpress, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = QPXExpress::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `search(...)`
/// // to build up your call.
/// let rb = hub.trips();
/// # }
/// ```
pub struct TripMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a QPXExpress<S>,
}

impl<'a, S> client::MethodsBuilder for TripMethods<'a, S> {}

impl<'a, S> TripMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of flights.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn search(&self, request: TripsSearchRequest) -> TripSearchCall<'a, S> {
        TripSearchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



