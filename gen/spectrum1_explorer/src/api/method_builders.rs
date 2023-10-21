use super::*;
/// A builder providing access to all methods supported on *paw* resources.
/// It is not used directly, but through the [`Spectrum`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_spectrum1_explorer as spectrum1_explorer;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use spectrum1_explorer::{Spectrum, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Spectrum::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_spectrum(...)`, `get_spectrum_batch(...)`, `init(...)`, `notify_spectrum_use(...)`, `register(...)` and `verify_device(...)`
/// // to build up your call.
/// let rb = hub.paws();
/// # }
/// ```
pub struct PawMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Spectrum<S>,
}

impl<'a, S> client::MethodsBuilder for PawMethods<'a, S> {}

impl<'a, S> PawMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Requests information about the available spectrum for a device at a location. Requests from a fixed-mode device must include owner information so the device can be registered with the database.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn get_spectrum(&self, request: PawsGetSpectrumRequest) -> PawGetSpectrumCall<'a, S> {
        PawGetSpectrumCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// The Google Spectrum Database does not support batch requests, so this method always yields an UNIMPLEMENTED error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn get_spectrum_batch(&self, request: PawsGetSpectrumBatchRequest) -> PawGetSpectrumBatchCall<'a, S> {
        PawGetSpectrumBatchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Initializes the connection between a white space device and the database.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn init(&self, request: PawsInitRequest) -> PawInitCall<'a, S> {
        PawInitCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Notifies the database that the device has selected certain frequency ranges for transmission. Only to be invoked when required by the regulator. The Google Spectrum Database does not operate in domains that require notification, so this always yields an UNIMPLEMENTED error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn notify_spectrum_use(&self, request: PawsNotifySpectrumUseRequest) -> PawNotifySpectrumUseCall<'a, S> {
        PawNotifySpectrumUseCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// The Google Spectrum Database implements registration in the getSpectrum method. As such this always returns an UNIMPLEMENTED error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn register(&self, request: PawsRegisterRequest) -> PawRegisterCall<'a, S> {
        PawRegisterCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Validates a device for white space use in accordance with regulatory rules. The Google Spectrum Database does not support master/slave configurations, so this always yields an UNIMPLEMENTED error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn verify_device(&self, request: PawsVerifyDeviceRequest) -> PawVerifyDeviceCall<'a, S> {
        PawVerifyDeviceCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



