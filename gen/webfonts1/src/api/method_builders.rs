use super::*;
/// A builder providing access to all methods supported on *webfont* resources.
/// It is not used directly, but through the [`Webfonts`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_webfonts1 as webfonts1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use webfonts1::{Webfonts, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Webfonts::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.webfonts();
/// # }
/// ```
pub struct WebfontMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Webfonts<S>,
}

impl<'a, S> client::MethodsBuilder for WebfontMethods<'a, S> {}

impl<'a, S> WebfontMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of fonts currently served by the Google Fonts Developer API.
    pub fn list(&self) -> WebfontListCall<'a, S> {
        WebfontListCall {
            hub: self.hub,
            _sort: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



