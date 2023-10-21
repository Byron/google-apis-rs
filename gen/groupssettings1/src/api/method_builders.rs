use super::*;
/// A builder providing access to all methods supported on *group* resources.
/// It is not used directly, but through the [`Groupssettings`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_groupssettings1 as groupssettings1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use groupssettings1::{Groupssettings, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Groupssettings::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.groups();
/// # }
/// ```
pub struct GroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Groupssettings<S>,
}

impl<'a, S> client::MethodsBuilder for GroupMethods<'a, S> {}

impl<'a, S> GroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one resource by id.
    /// 
    /// # Arguments
    ///
    /// * `groupUniqueId` - The group's email address.
    pub fn get(&self, group_unique_id: &str) -> GroupGetCall<'a, S> {
        GroupGetCall {
            hub: self.hub,
            _group_unique_id: group_unique_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing resource. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `groupUniqueId` - The group's email address.
    pub fn patch(&self, request: Groups, group_unique_id: &str) -> GroupPatchCall<'a, S> {
        GroupPatchCall {
            hub: self.hub,
            _request: request,
            _group_unique_id: group_unique_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `groupUniqueId` - The group's email address.
    pub fn update(&self, request: Groups, group_unique_id: &str) -> GroupUpdateCall<'a, S> {
        GroupUpdateCall {
            hub: self.hub,
            _request: request,
            _group_unique_id: group_unique_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



