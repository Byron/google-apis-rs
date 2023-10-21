use super::*;
/// A builder providing access to all methods supported on *encodedFullHash* resources.
/// It is not used directly, but through the [`Safebrowsing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_safebrowsing4 as safebrowsing4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use safebrowsing4::{Safebrowsing, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Safebrowsing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.encoded_full_hashes();
/// # }
/// ```
pub struct EncodedFullHashMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Safebrowsing<S>,
}

impl<'a, S> client::MethodsBuilder for EncodedFullHashMethods<'a, S> {}

impl<'a, S> EncodedFullHashMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `encodedRequest` - A serialized FindFullHashesRequest proto.
    pub fn get(&self, encoded_request: Vec<u8>) -> EncodedFullHashGetCall<'a, S> {
        EncodedFullHashGetCall {
            hub: self.hub,
            _encoded_request: encoded_request,
            _client_version: Default::default(),
            _client_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *encodedUpdate* resources.
/// It is not used directly, but through the [`Safebrowsing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_safebrowsing4 as safebrowsing4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use safebrowsing4::{Safebrowsing, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Safebrowsing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.encoded_updates();
/// # }
/// ```
pub struct EncodedUpdateMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Safebrowsing<S>,
}

impl<'a, S> client::MethodsBuilder for EncodedUpdateMethods<'a, S> {}

impl<'a, S> EncodedUpdateMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `encodedRequest` - A serialized FetchThreatListUpdatesRequest proto.
    pub fn get(&self, encoded_request: Vec<u8>) -> EncodedUpdateGetCall<'a, S> {
        EncodedUpdateGetCall {
            hub: self.hub,
            _encoded_request: encoded_request,
            _client_version: Default::default(),
            _client_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *fullHash* resources.
/// It is not used directly, but through the [`Safebrowsing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_safebrowsing4 as safebrowsing4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use safebrowsing4::{Safebrowsing, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Safebrowsing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `find(...)`
/// // to build up your call.
/// let rb = hub.full_hashes();
/// # }
/// ```
pub struct FullHashMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Safebrowsing<S>,
}

impl<'a, S> client::MethodsBuilder for FullHashMethods<'a, S> {}

impl<'a, S> FullHashMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Finds the full hashes that match the requested hash prefixes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn find(&self, request: GoogleSecuritySafebrowsingV4FindFullHashesRequest) -> FullHashFindCall<'a, S> {
        FullHashFindCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *threatHit* resources.
/// It is not used directly, but through the [`Safebrowsing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_safebrowsing4 as safebrowsing4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use safebrowsing4::{Safebrowsing, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Safebrowsing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`
/// // to build up your call.
/// let rb = hub.threat_hits();
/// # }
/// ```
pub struct ThreatHitMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Safebrowsing<S>,
}

impl<'a, S> client::MethodsBuilder for ThreatHitMethods<'a, S> {}

impl<'a, S> ThreatHitMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reports a Safe Browsing threat list hit to Google. Only projects with TRUSTED_REPORTER visibility can use this method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: GoogleSecuritySafebrowsingV4ThreatHit) -> ThreatHitCreateCall<'a, S> {
        ThreatHitCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *threatListUpdate* resources.
/// It is not used directly, but through the [`Safebrowsing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_safebrowsing4 as safebrowsing4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use safebrowsing4::{Safebrowsing, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Safebrowsing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `fetch(...)`
/// // to build up your call.
/// let rb = hub.threat_list_updates();
/// # }
/// ```
pub struct ThreatListUpdateMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Safebrowsing<S>,
}

impl<'a, S> client::MethodsBuilder for ThreatListUpdateMethods<'a, S> {}

impl<'a, S> ThreatListUpdateMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the most recent threat list updates. A client can request updates for multiple lists at once.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn fetch(&self, request: GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest) -> ThreatListUpdateFetchCall<'a, S> {
        ThreatListUpdateFetchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *threatList* resources.
/// It is not used directly, but through the [`Safebrowsing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_safebrowsing4 as safebrowsing4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use safebrowsing4::{Safebrowsing, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Safebrowsing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.threat_lists();
/// # }
/// ```
pub struct ThreatListMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Safebrowsing<S>,
}

impl<'a, S> client::MethodsBuilder for ThreatListMethods<'a, S> {}

impl<'a, S> ThreatListMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Safe Browsing threat lists available for download.
    pub fn list(&self) -> ThreatListListCall<'a, S> {
        ThreatListListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *threatMatch* resources.
/// It is not used directly, but through the [`Safebrowsing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_safebrowsing4 as safebrowsing4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use safebrowsing4::{Safebrowsing, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Safebrowsing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `find(...)`
/// // to build up your call.
/// let rb = hub.threat_matches();
/// # }
/// ```
pub struct ThreatMatchMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Safebrowsing<S>,
}

impl<'a, S> client::MethodsBuilder for ThreatMatchMethods<'a, S> {}

impl<'a, S> ThreatMatchMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Finds the threat entries that match the Safe Browsing lists.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn find(&self, request: GoogleSecuritySafebrowsingV4FindThreatMatchesRequest) -> ThreatMatchFindCall<'a, S> {
        ThreatMatchFindCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



