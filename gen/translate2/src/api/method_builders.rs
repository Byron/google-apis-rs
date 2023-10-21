use super::*;
/// A builder providing access to all methods supported on *detection* resources.
/// It is not used directly, but through the [`Translate`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_translate2 as translate2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use translate2::{Translate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Translate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `detect(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.detections();
/// # }
/// ```
pub struct DetectionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Translate<S>,
}

impl<'a, S> client::MethodsBuilder for DetectionMethods<'a, S> {}

impl<'a, S> DetectionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Detects the language of text within a request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn detect(&self, request: DetectLanguageRequest) -> DetectionDetectCall<'a, S> {
        DetectionDetectCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Detects the language of text within a request.
    /// 
    /// # Arguments
    ///
    /// * `q` - The input text upon which to perform language detection. Repeat this
    ///         parameter to perform language detection on multiple text inputs.
    pub fn list(&self, q: &Vec<String>) -> DetectionListCall<'a, S> {
        DetectionListCall {
            hub: self.hub,
            _q: q.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *language* resources.
/// It is not used directly, but through the [`Translate`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_translate2 as translate2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use translate2::{Translate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Translate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.languages();
/// # }
/// ```
pub struct LanguageMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Translate<S>,
}

impl<'a, S> client::MethodsBuilder for LanguageMethods<'a, S> {}

impl<'a, S> LanguageMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of supported languages for translation.
    pub fn list(&self) -> LanguageListCall<'a, S> {
        LanguageListCall {
            hub: self.hub,
            _target: Default::default(),
            _model: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *translation* resources.
/// It is not used directly, but through the [`Translate`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_translate2 as translate2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use translate2::{Translate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Translate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `translate(...)`
/// // to build up your call.
/// let rb = hub.translations();
/// # }
/// ```
pub struct TranslationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Translate<S>,
}

impl<'a, S> client::MethodsBuilder for TranslationMethods<'a, S> {}

impl<'a, S> TranslationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Translates input text, returning translated text.
    /// 
    /// # Arguments
    ///
    /// * `q` - The input text to translate. Repeat this parameter to perform translation
    ///         operations on multiple text inputs.
    /// * `target` - The language to use for translation of the input text, set to one of the
    ///              language codes listed in Language Support.
    pub fn list(&self, q: &Vec<String>, target: &str) -> TranslationListCall<'a, S> {
        TranslationListCall {
            hub: self.hub,
            _q: q.clone(),
            _target: target.to_string(),
            _source: Default::default(),
            _model: Default::default(),
            _format: Default::default(),
            _cid: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Translates input text, returning translated text.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn translate(&self, request: TranslateTextRequest) -> TranslationTranslateCall<'a, S> {
        TranslationTranslateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



