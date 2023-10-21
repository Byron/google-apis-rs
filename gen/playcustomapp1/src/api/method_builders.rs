use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`Playcustomapp`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_playcustomapp1 as playcustomapp1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use playcustomapp1::{Playcustomapp, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Playcustomapp::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `custom_apps_create(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Playcustomapp<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new custom app.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `account` - Developer account ID.
    pub fn custom_apps_create(&self, request: CustomApp, account: i64) -> AccountCustomAppCreateCall<'a, S> {
        AccountCustomAppCreateCall {
            hub: self.hub,
            _request: request,
            _account: account,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



