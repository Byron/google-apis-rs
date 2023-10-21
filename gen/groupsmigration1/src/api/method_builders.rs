use super::*;
/// A builder providing access to all methods supported on *archive* resources.
/// It is not used directly, but through the [`GroupsMigration`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_groupsmigration1 as groupsmigration1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use groupsmigration1::{GroupsMigration, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = GroupsMigration::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)`
/// // to build up your call.
/// let rb = hub.archive();
/// # }
/// ```
pub struct ArchiveMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a GroupsMigration<S>,
}

impl<'a, S> client::MethodsBuilder for ArchiveMethods<'a, S> {}

impl<'a, S> ArchiveMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new mail into the archive of the Google group.
    /// 
    /// # Arguments
    ///
    /// * `groupId` - The group ID
    pub fn insert(&self, group_id: &str) -> ArchiveInsertCall<'a, S> {
        ArchiveInsertCall {
            hub: self.hub,
            _group_id: group_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



