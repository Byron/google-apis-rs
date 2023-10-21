use super::*;
/// A builder providing access to all methods supported on *iam* resources.
/// It is not used directly, but through the [`PolicyTroubleshooter`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_policytroubleshooter1 as policytroubleshooter1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use policytroubleshooter1::{PolicyTroubleshooter, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PolicyTroubleshooter::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `troubleshoot(...)`
/// // to build up your call.
/// let rb = hub.iam();
/// # }
/// ```
pub struct IamMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PolicyTroubleshooter<S>,
}

impl<'a, S> client::MethodsBuilder for IamMethods<'a, S> {}

impl<'a, S> IamMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Checks whether a principal has a specific permission for a specific resource, and explains why the principal does or does not have that permission.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn troubleshoot(&self, request: GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyRequest) -> IamTroubleshootCall<'a, S> {
        IamTroubleshootCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



