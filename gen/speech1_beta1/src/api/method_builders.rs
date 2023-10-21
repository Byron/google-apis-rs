use super::*;
/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`Speech`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_speech1_beta1 as speech1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use speech1_beta1::{Speech, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Speech::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Speech<S>,
}

impl<'a, S> client::MethodsBuilder for OperationMethods<'a, S> {}

impl<'a, S> OperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the
    /// server doesn't support this method, it returns `UNIMPLEMENTED`.
    /// 
    /// NOTE: the `name` binding allows API services to override the binding
    /// to use different resource name schemes, such as `users/*/operations`. To
    /// override the binding, API services can add a binding such as
    /// `"/v1/{name=users/*}/operations"` to their service configuration.
    /// For backwards compatibility, the default name includes the operations
    /// collection id, however overriding users must ensure the name binding
    /// is the parent resource, without the operations collection id.
    pub fn list(&self) -> OperationListCall<'a, S> {
        OperationListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _name: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation.  Clients can use this
    /// method to poll the operation result at intervals as recommended by the API
    /// service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn get(&self, name: &str) -> OperationGetCall<'a, S> {
        OperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *speech* resources.
/// It is not used directly, but through the [`Speech`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_speech1_beta1 as speech1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use speech1_beta1::{Speech, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Speech::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `asyncrecognize(...)` and `syncrecognize(...)`
/// // to build up your call.
/// let rb = hub.speech();
/// # }
/// ```
pub struct SpeechMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Speech<S>,
}

impl<'a, S> client::MethodsBuilder for SpeechMethods<'a, S> {}

impl<'a, S> SpeechMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs asynchronous speech recognition: receive results via the
    /// \[google.longrunning.Operations\]
    /// (/speech/reference/rest/v1beta1/operations#Operation)
    /// interface. Returns either an
    /// `Operation.error` or an `Operation.response` which contains
    /// an `AsyncRecognizeResponse` message.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn asyncrecognize(&self, request: AsyncRecognizeRequest) -> SpeechAsyncrecognizeCall<'a, S> {
        SpeechAsyncrecognizeCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs synchronous speech recognition: receive results after all audio
    /// has been sent and processed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn syncrecognize(&self, request: SyncRecognizeRequest) -> SpeechSyncrecognizeCall<'a, S> {
        SpeechSyncrecognizeCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



