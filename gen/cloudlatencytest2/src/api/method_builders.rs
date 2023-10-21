use super::*;
/// A builder providing access to all methods supported on *statscollection* resources.
/// It is not used directly, but through the [`Cloudlatencytest`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudlatencytest2 as cloudlatencytest2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudlatencytest2::{Cloudlatencytest, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Cloudlatencytest::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `updateaggregatedstats(...)` and `updatestats(...)`
/// // to build up your call.
/// let rb = hub.statscollection();
/// # }
/// ```
pub struct StatscollectionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Cloudlatencytest<S>,
}

impl<'a, S> client::MethodsBuilder for StatscollectionMethods<'a, S> {}

impl<'a, S> StatscollectionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// RPC to update the new TCP stats.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn updateaggregatedstats(&self, request: AggregatedStats) -> StatscollectionUpdateaggregatedstatCall<'a, S> {
        StatscollectionUpdateaggregatedstatCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// RPC to update the new TCP stats.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn updatestats(&self, request: Stats) -> StatscollectionUpdatestatCall<'a, S> {
        StatscollectionUpdatestatCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



