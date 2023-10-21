use super::*;
/// A builder providing access to all methods supported on *activity* resources.
/// It is not used directly, but through the [`Appsactivity`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_appsactivity1 as appsactivity1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use appsactivity1::{Appsactivity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Appsactivity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.activities();
/// # }
/// ```
pub struct ActivityMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Appsactivity<S>,
}

impl<'a, S> client::MethodsBuilder for ActivityMethods<'a, S> {}

impl<'a, S> ActivityMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of activities visible to the current logged in user. Visible activities are determined by the visibility settings of the object that was acted on, e.g. Drive files a user can see. An activity is a record of past events. Multiple events may be merged if they are similar. A request is scoped to activities from a given Google service using the source parameter.
    pub fn list(&self) -> ActivityListCall<'a, S> {
        ActivityListCall {
            hub: self.hub,
            _user_id: Default::default(),
            _source: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _grouping_strategy: Default::default(),
            _drive_file_id: Default::default(),
            _drive_ancestor_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



