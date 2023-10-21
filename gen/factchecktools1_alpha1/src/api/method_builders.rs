use super::*;
/// A builder providing access to all methods supported on *claim* resources.
/// It is not used directly, but through the [`FactCheckTools`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_factchecktools1_alpha1 as factchecktools1_alpha1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use factchecktools1_alpha1::{FactCheckTools, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = FactCheckTools::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `search(...)`
/// // to build up your call.
/// let rb = hub.claims();
/// # }
/// ```
pub struct ClaimMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a FactCheckTools<S>,
}

impl<'a, S> client::MethodsBuilder for ClaimMethods<'a, S> {}

impl<'a, S> ClaimMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Search through fact-checked claims.
    pub fn search(&self) -> ClaimSearchCall<'a, S> {
        ClaimSearchCall {
            hub: self.hub,
            _review_publisher_site_filter: Default::default(),
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _offset: Default::default(),
            _max_age_days: Default::default(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *page* resources.
/// It is not used directly, but through the [`FactCheckTools`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_factchecktools1_alpha1 as factchecktools1_alpha1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use factchecktools1_alpha1::{FactCheckTools, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = FactCheckTools::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.pages();
/// # }
/// ```
pub struct PageMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a FactCheckTools<S>,
}

impl<'a, S> client::MethodsBuilder for PageMethods<'a, S> {}

impl<'a, S> PageMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create `ClaimReview` markup on a page.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkupPage) -> PageCreateCall<'a, S> {
        PageCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete all `ClaimReview` markup on a page.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the resource to delete, in the form of `pages/{page_id}`.
    pub fn delete(&self, name: &str) -> PageDeleteCall<'a, S> {
        PageDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get all `ClaimReview` markup on a page.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the resource to get, in the form of `pages/{page_id}`.
    pub fn get(&self, name: &str) -> PageGetCall<'a, S> {
        PageGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the `ClaimReview` markup pages for a specific URL or for an organization.
    pub fn list(&self) -> PageListCall<'a, S> {
        PageListCall {
            hub: self.hub,
            _url: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _organization: Default::default(),
            _offset: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update for all `ClaimReview` markup on a page Note that this is a full update. To retain the existing `ClaimReview` markup on a page, first perform a Get operation, then modify the returned markup, and finally call Update with the entire `ClaimReview` markup as the body.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of this `ClaimReview` markup page resource, in the form of `pages/{page_id}`. Except for update requests, this field is output-only and should not be set by the user.
    pub fn update(&self, request: GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkupPage, name: &str) -> PageUpdateCall<'a, S> {
        PageUpdateCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



