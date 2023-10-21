use super::*;
/// A builder providing access to all methods supported on *pagespeedapi* resources.
/// It is not used directly, but through the [`Pagespeedonline`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_pagespeedonline2 as pagespeedonline2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use pagespeedonline2::{Pagespeedonline, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Pagespeedonline::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `runpagespeed(...)`
/// // to build up your call.
/// let rb = hub.pagespeedapi();
/// # }
/// ```
pub struct PagespeedapiMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Pagespeedonline<S>,
}

impl<'a, S> client::MethodsBuilder for PagespeedapiMethods<'a, S> {}

impl<'a, S> PagespeedapiMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.
    /// 
    /// # Arguments
    ///
    /// * `url` - The URL to fetch and analyze
    pub fn runpagespeed(&self, url: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        PagespeedapiRunpagespeedCall {
            hub: self.hub,
            _url: url.to_string(),
            _strategy: Default::default(),
            _screenshot: Default::default(),
            _rule: Default::default(),
            _locale: Default::default(),
            _filter_third_party_resources: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



