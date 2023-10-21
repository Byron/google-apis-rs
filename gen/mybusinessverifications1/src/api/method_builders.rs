use super::*;
/// A builder providing access to all methods supported on *location* resources.
/// It is not used directly, but through the [`MyBusinessVerifications`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessverifications1 as mybusinessverifications1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusinessverifications1::{MyBusinessVerifications, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusinessVerifications::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `fetch_verification_options(...)`, `get_voice_of_merchant_state(...)`, `verifications_complete(...)`, `verifications_list(...)` and `verify(...)`
/// // to build up your call.
/// let rb = hub.locations();
/// # }
/// ```
pub struct LocationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusinessVerifications<S>,
}

impl<'a, S> client::MethodsBuilder for LocationMethods<'a, S> {}

impl<'a, S> LocationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Completes a `PENDING` verification. It is only necessary for non `AUTO` verification methods. `AUTO` verification request is instantly `VERIFIED` upon creation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of the verification to complete.
    pub fn verifications_complete(&self, request: CompleteVerificationRequest, name: &str) -> LocationVerificationCompleteCall<'a, S> {
        LocationVerificationCompleteCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List verifications of a location, ordered by create time.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the location that verification requests belong to.
    pub fn verifications_list(&self, parent: &str) -> LocationVerificationListCall<'a, S> {
        LocationVerificationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reports all eligible verification options for a location in a specific language.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `location` - Required. The location to verify.
    pub fn fetch_verification_options(&self, request: FetchVerificationOptionsRequest, location: &str) -> LocationFetchVerificationOptionCall<'a, S> {
        LocationFetchVerificationOptionCall {
            hub: self.hub,
            _request: request,
            _location: location.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the VoiceOfMerchant state.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the location.
    pub fn get_voice_of_merchant_state(&self, name: &str) -> LocationGetVoiceOfMerchantStateCall<'a, S> {
        LocationGetVoiceOfMerchantStateCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts the verification process for a location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of the location to verify.
    pub fn verify(&self, request: VerifyLocationRequest, name: &str) -> LocationVerifyCall<'a, S> {
        LocationVerifyCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *verificationToken* resources.
/// It is not used directly, but through the [`MyBusinessVerifications`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessverifications1 as mybusinessverifications1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusinessverifications1::{MyBusinessVerifications, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusinessVerifications::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `generate(...)`
/// // to build up your call.
/// let rb = hub.verification_tokens();
/// # }
/// ```
pub struct VerificationTokenMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusinessVerifications<S>,
}

impl<'a, S> client::MethodsBuilder for VerificationTokenMethods<'a, S> {}

impl<'a, S> VerificationTokenMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a token for the provided location data as a vetted [partner](https://support.google.com/business/answer/7674102). Throws PERMISSION_DENIED if the caller is not a vetted partner account. Throws FAILED_PRECONDITION if the caller's VettedStatus is INVALID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn generate(&self, request: GenerateVerificationTokenRequest) -> VerificationTokenGenerateCall<'a, S> {
        VerificationTokenGenerateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



