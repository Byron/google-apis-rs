use super::*;
/// A builder providing access to all methods supported on *managedShortLink* resources.
/// It is not used directly, but through the [`FirebaseDynamicLinks`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use firebasedynamiclinks1::{FirebaseDynamicLinks, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = FirebaseDynamicLinks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`
/// // to build up your call.
/// let rb = hub.managed_short_links();
/// # }
/// ```
pub struct ManagedShortLinkMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a FirebaseDynamicLinks<S>,
}

impl<'a, S> client::MethodsBuilder for ManagedShortLinkMethods<'a, S> {}

impl<'a, S> ManagedShortLinkMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a managed short Dynamic Link given either a valid long Dynamic Link or details such as Dynamic Link domain, Android and iOS app information. The created short Dynamic Link will not expire. This differs from CreateShortDynamicLink in the following ways: - The request will also contain a name for the link (non unique name for the front end). - The response must be authenticated with an auth token (generated with the admin service account). - The link will appear in the FDL list of links in the console front end. The Dynamic Link domain in the request must be owned by requester's Firebase project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: CreateManagedShortLinkRequest) -> ManagedShortLinkCreateCall<'a, S> {
        ManagedShortLinkCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *shortLink* resources.
/// It is not used directly, but through the [`FirebaseDynamicLinks`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use firebasedynamiclinks1::{FirebaseDynamicLinks, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = FirebaseDynamicLinks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`
/// // to build up your call.
/// let rb = hub.short_links();
/// # }
/// ```
pub struct ShortLinkMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a FirebaseDynamicLinks<S>,
}

impl<'a, S> client::MethodsBuilder for ShortLinkMethods<'a, S> {}

impl<'a, S> ShortLinkMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a short Dynamic Link given either a valid long Dynamic Link or details such as Dynamic Link domain, Android and iOS app information. The created short Dynamic Link will not expire. Repeated calls with the same long Dynamic Link or Dynamic Link information will produce the same short Dynamic Link. The Dynamic Link domain in the request must be owned by requester's Firebase project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: CreateShortDynamicLinkRequest) -> ShortLinkCreateCall<'a, S> {
        ShortLinkCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the [`FirebaseDynamicLinks`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use firebasedynamiclinks1::{FirebaseDynamicLinks, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = FirebaseDynamicLinks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_link_stats(...)`, `install_attribution(...)` and `reopen_attribution(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a FirebaseDynamicLinks<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches analytics stats of a short Dynamic Link for a given duration. Metrics include number of clicks, redirects, installs, app first opens, and app reopens.
    /// 
    /// # Arguments
    ///
    /// * `dynamicLink` - Dynamic Link URL. e.g. https://abcd.app.goo.gl/wxyz
    pub fn get_link_stats(&self, dynamic_link: &str) -> MethodGetLinkStatCall<'a, S> {
        MethodGetLinkStatCall {
            hub: self.hub,
            _dynamic_link: dynamic_link.to_string(),
            _sdk_version: Default::default(),
            _duration_days: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get iOS strong/weak-match info for post-install attribution.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn install_attribution(&self, request: GetIosPostInstallAttributionRequest) -> MethodInstallAttributionCall<'a, S> {
        MethodInstallAttributionCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get iOS reopen attribution for app universal link open deeplinking.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn reopen_attribution(&self, request: GetIosReopenAttributionRequest) -> MethodReopenAttributionCall<'a, S> {
        MethodReopenAttributionCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



