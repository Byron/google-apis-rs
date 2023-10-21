use super::*;
/// A builder providing access to all methods supported on *site* resources.
/// It is not used directly, but through the [`AdExperienceReport`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexperiencereport1 as adexperiencereport1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexperiencereport1::{AdExperienceReport, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExperienceReport::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.sites();
/// # }
/// ```
pub struct SiteMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExperienceReport<S>,
}

impl<'a, S> client::MethodsBuilder for SiteMethods<'a, S> {}

impl<'a, S> SiteMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a site's Ad Experience Report summary.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the site whose summary to get, e.g. `sites/http%3A%2F%2Fwww.google.com%2F`. Format: `sites/{site}`
    pub fn get(&self, name: &str) -> SiteGetCall<'a, S> {
        SiteGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *violatingSite* resources.
/// It is not used directly, but through the [`AdExperienceReport`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexperiencereport1 as adexperiencereport1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexperiencereport1::{AdExperienceReport, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExperienceReport::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.violating_sites();
/// # }
/// ```
pub struct ViolatingSiteMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExperienceReport<S>,
}

impl<'a, S> client::MethodsBuilder for ViolatingSiteMethods<'a, S> {}

impl<'a, S> ViolatingSiteMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists sites that are failing in the Ad Experience Report on at least one platform.
    pub fn list(&self) -> ViolatingSiteListCall<'a, S> {
        ViolatingSiteListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



