use super::*;
/// A builder providing access to all methods supported on *challenge* resources.
/// It is not used directly, but through the [`Verifiedaccess`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_verifiedaccess1 as verifiedaccess1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use verifiedaccess1::{Verifiedaccess, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Verifiedaccess::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)` and `verify(...)`
/// // to build up your call.
/// let rb = hub.challenge();
/// # }
/// ```
pub struct ChallengeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Verifiedaccess<S>,
}

impl<'a, S> client::MethodsBuilder for ChallengeMethods<'a, S> {}

impl<'a, S> ChallengeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// CreateChallenge API
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Empty) -> ChallengeCreateCall<'a, S> {
        ChallengeCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// VerifyChallengeResponse API
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn verify(&self, request: VerifyChallengeResponseRequest) -> ChallengeVerifyCall<'a, S> {
        ChallengeVerifyCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



