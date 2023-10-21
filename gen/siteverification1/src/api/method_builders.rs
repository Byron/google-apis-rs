use super::*;
/// A builder providing access to all methods supported on *webResource* resources.
/// It is not used directly, but through the [`SiteVerification`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_siteverification1 as siteverification1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use siteverification1::{SiteVerification, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SiteVerification::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `get_token(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.web_resource();
/// # }
/// ```
pub struct WebResourceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SiteVerification<S>,
}

impl<'a, S> client::MethodsBuilder for WebResourceMethods<'a, S> {}

impl<'a, S> WebResourceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Relinquish ownership of a website or domain.
    /// 
    /// # Arguments
    ///
    /// * `id` - The id of a verified site or domain.
    pub fn delete(&self, id: &str) -> WebResourceDeleteCall<'a, S> {
        WebResourceDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the most current data for a website or domain.
    /// 
    /// # Arguments
    ///
    /// * `id` - The id of a verified site or domain.
    pub fn get(&self, id: &str) -> WebResourceGetCall<'a, S> {
        WebResourceGetCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a verification token for placing on a website or domain.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn get_token(&self, request: SiteVerificationWebResourceGettokenRequest) -> WebResourceGetTokenCall<'a, S> {
        WebResourceGetTokenCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Attempt verification of a website or domain.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `verificationMethod` - The method to use for verifying a site or domain.
    pub fn insert(&self, request: SiteVerificationWebResourceResource, verification_method: &str) -> WebResourceInsertCall<'a, S> {
        WebResourceInsertCall {
            hub: self.hub,
            _request: request,
            _verification_method: verification_method.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the list of your verified websites and domains.
    pub fn list(&self) -> WebResourceListCall<'a, S> {
        WebResourceListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modify the list of owners for your website or domain. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - The id of a verified site or domain.
    pub fn patch(&self, request: SiteVerificationWebResourceResource, id: &str) -> WebResourcePatchCall<'a, S> {
        WebResourcePatchCall {
            hub: self.hub,
            _request: request,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modify the list of owners for your website or domain.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - The id of a verified site or domain.
    pub fn update(&self, request: SiteVerificationWebResourceResource, id: &str) -> WebResourceUpdateCall<'a, S> {
        WebResourceUpdateCall {
            hub: self.hub,
            _request: request,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



