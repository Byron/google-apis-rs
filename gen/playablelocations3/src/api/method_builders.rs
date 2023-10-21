use super::*;
/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the [`PlayableLocations`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_playablelocations3 as playablelocations3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use playablelocations3::{PlayableLocations, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PlayableLocations::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `log_impressions(...)`, `log_player_reports(...)` and `sample_playable_locations(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PlayableLocations<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Logs new events when playable locations are displayed, and when they are
    /// interacted with.
    /// 
    /// Impressions are not partially saved; either all impressions are saved and
    /// this request succeeds, or no impressions are saved, and this request fails.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn log_impressions(&self, request: GoogleMapsPlayablelocationsV3LogImpressionsRequest) -> MethodLogImpressionCall<'a, S> {
        MethodLogImpressionCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Logs bad playable location reports submitted by players.
    /// 
    /// Reports are not partially saved; either all reports are saved and this
    /// request succeeds, or no reports are saved, and this request fails.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn log_player_reports(&self, request: GoogleMapsPlayablelocationsV3LogPlayerReportsRequest) -> MethodLogPlayerReportCall<'a, S> {
        MethodLogPlayerReportCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a set of playable locations that lie within a specified area,
    /// that satisfy optional filter criteria.
    /// 
    /// Note: Identical `SamplePlayableLocations` requests can return different
    /// results as the state of the world changes over time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn sample_playable_locations(&self, request: GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest) -> MethodSamplePlayableLocationCall<'a, S> {
        MethodSamplePlayableLocationCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



