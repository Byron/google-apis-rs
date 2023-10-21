use super::*;
/// A builder providing access to all methods supported on *assetlink* resources.
/// It is not used directly, but through the [`Digitalassetlinks`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_digitalassetlinks1 as digitalassetlinks1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use digitalassetlinks1::{Digitalassetlinks, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Digitalassetlinks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `bulk_check(...)` and `check(...)`
/// // to build up your call.
/// let rb = hub.assetlinks();
/// # }
/// ```
pub struct AssetlinkMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Digitalassetlinks<S>,
}

impl<'a, S> client::MethodsBuilder for AssetlinkMethods<'a, S> {}

impl<'a, S> AssetlinkMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Send a bundle of statement checks in a single RPC to minimize latency and service load. Statements need not be all for the same source and/or target. We recommend using this method when you need to check more than one statement in a short period of time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn bulk_check(&self, request: BulkCheckRequest) -> AssetlinkBulkCheckCall<'a, S> {
        AssetlinkBulkCheckCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Determines whether the specified (directional) relationship exists between the specified source and target assets. The relation describes the intent of the link between the two assets as claimed by the source asset. An example for such relationships is the delegation of privileges or permissions. This command is most often used by infrastructure systems to check preconditions for an action. For example, a client may want to know if it is OK to send a web URL to a particular mobile app instead. The client can check for the relevant asset link from the website to the mobile app to decide if the operation should be allowed. A note about security: if you specify a secure asset as the source, such as an HTTPS website or an Android app, the API will ensure that any statements used to generate the response have been made in a secure way by the owner of that asset. Conversely, if the source asset is an insecure HTTP website (that is, the URL starts with `http://` instead of `https://`), the API cannot verify its statements securely, and it is not possible to ensure that the website's statements have not been altered by a third party. For more information, see the [Digital Asset Links technical design specification](https://github.com/google/digitalassetlinks/blob/master/well-known/details.md).
    pub fn check(&self) -> AssetlinkCheckCall<'a, S> {
        AssetlinkCheckCall {
            hub: self.hub,
            _target_web_site: Default::default(),
            _target_android_app_package_name: Default::default(),
            _target_android_app_certificate_sha256_fingerprint: Default::default(),
            _source_web_site: Default::default(),
            _source_android_app_package_name: Default::default(),
            _source_android_app_certificate_sha256_fingerprint: Default::default(),
            _relation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *statement* resources.
/// It is not used directly, but through the [`Digitalassetlinks`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_digitalassetlinks1 as digitalassetlinks1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use digitalassetlinks1::{Digitalassetlinks, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Digitalassetlinks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.statements();
/// # }
/// ```
pub struct StatementMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Digitalassetlinks<S>,
}

impl<'a, S> client::MethodsBuilder for StatementMethods<'a, S> {}

impl<'a, S> StatementMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of all statements from a given source that match the specified target and statement string. The API guarantees that all statements with secure source assets, such as HTTPS websites or Android apps, have been made in a secure way by the owner of those assets, as described in the [Digital Asset Links technical design specification](https://github.com/google/digitalassetlinks/blob/master/well-known/details.md). Specifically, you should consider that for insecure websites (that is, where the URL starts with `http://` instead of `https://`), this guarantee cannot be made. The `List` command is most useful in cases where the API client wants to know all the ways in which two assets are related, or enumerate all the relationships from a particular source asset. Example: a feature that helps users navigate to related items. When a mobile app is running on a device, the feature would make it easy to navigate to the corresponding web site or Google+ profile.
    pub fn list(&self) -> StatementListCall<'a, S> {
        StatementListCall {
            hub: self.hub,
            _source_web_site: Default::default(),
            _source_android_app_package_name: Default::default(),
            _source_android_app_certificate_sha256_fingerprint: Default::default(),
            _relation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



