use super::*;
/// A builder providing access to all methods supported on *urlNotification* resources.
/// It is not used directly, but through the [`Indexing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_indexing3 as indexing3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use indexing3::{Indexing, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Indexing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_metadata(...)` and `publish(...)`
/// // to build up your call.
/// let rb = hub.url_notifications();
/// # }
/// ```
pub struct UrlNotificationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Indexing<S>,
}

impl<'a, S> client::MethodsBuilder for UrlNotificationMethods<'a, S> {}

impl<'a, S> UrlNotificationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets metadata about a Web Document. This method can _only_ be used to query URLs that were previously seen in successful Indexing API notifications. Includes the latest `UrlNotification` received via this API.
    pub fn get_metadata(&self) -> UrlNotificationGetMetadataCall<'a, S> {
        UrlNotificationGetMetadataCall {
            hub: self.hub,
            _url: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Notifies that a URL has been updated or deleted.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn publish(&self, request: UrlNotification) -> UrlNotificationPublishCall<'a, S> {
        UrlNotificationPublishCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



