use super::*;
/// A builder providing access to all methods supported on *platform* resources.
/// It is not used directly, but through the [`VersionHistory`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_versionhistory1 as versionhistory1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use versionhistory1::{VersionHistory, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = VersionHistory::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `channels_list(...)`, `channels_versions_list(...)`, `channels_versions_releases_list(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.platforms();
/// # }
/// ```
pub struct PlatformMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a VersionHistory<S>,
}

impl<'a, S> client::MethodsBuilder for PlatformMethods<'a, S> {}

impl<'a, S> PlatformMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns list of releases of the given version.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The version, which owns this collection of releases. Format: {product}/platforms/{platform}/channels/{channel}/versions/{version}
    pub fn channels_versions_releases_list(&self, parent: &str) -> PlatformChannelVersionReleaseListCall<'a, S> {
        PlatformChannelVersionReleaseListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns list of version for the given platform/channel.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The channel, which owns this collection of versions. Format: {product}/platforms/{platform}/channels/{channel}
    pub fn channels_versions_list(&self, parent: &str) -> PlatformChannelVersionListCall<'a, S> {
        PlatformChannelVersionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns list of channels that are available for a given platform.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The platform, which owns this collection of channels. Format: {product}/platforms/{platform}
    pub fn channels_list(&self, parent: &str) -> PlatformChannelListCall<'a, S> {
        PlatformChannelListCall {
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
    /// Returns list of platforms that are available for a given product. The resource "product" has no resource name in its name.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The product, which owns this collection of platforms. Format: {product}
    pub fn list(&self, parent: &str) -> PlatformListCall<'a, S> {
        PlatformListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



