use super::*;
/// A builder providing access to all methods supported on *video* resources.
/// It is not used directly, but through the [`CloudVideoIntelligence`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_videointelligence1_beta1 as videointelligence1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use videointelligence1_beta1::{CloudVideoIntelligence, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudVideoIntelligence::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `annotate(...)`
/// // to build up your call.
/// let rb = hub.videos();
/// # }
/// ```
pub struct VideoMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudVideoIntelligence<S>,
}

impl<'a, S> client::MethodsBuilder for VideoMethods<'a, S> {}

impl<'a, S> VideoMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs asynchronous video annotation. Progress and results can be
    /// retrieved through the `google.longrunning.Operations` interface.
    /// `Operation.metadata` contains `AnnotateVideoProgress` (progress).
    /// `Operation.response` contains `AnnotateVideoResponse` (results).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn annotate(&self, request: GoogleCloudVideointelligenceV1beta1_AnnotateVideoRequest) -> VideoAnnotateCall<'a, S> {
        VideoAnnotateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



