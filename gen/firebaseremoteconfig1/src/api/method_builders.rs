use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`FirebaseRemoteConfig`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_firebaseremoteconfig1 as firebaseremoteconfig1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use firebaseremoteconfig1::{FirebaseRemoteConfig, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = FirebaseRemoteConfig::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_remote_config(...)` and `update_remote_config(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a FirebaseRemoteConfig<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the latest version Remote Configuration for a project.
    /// Returns the RemoteConfig as the payload, and also the eTag as a
    /// response header.
    /// 
    /// # Arguments
    ///
    /// * `project` - The GMP project identifier. Required.
    ///               See note at the beginning of this file regarding project ids.
    pub fn get_remote_config(&self, project: &str) -> ProjectGetRemoteConfigCall<'a, S> {
        ProjectGetRemoteConfigCall {
            hub: self.hub,
            _project: project.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update a RemoteConfig. We treat this as an always-existing
    /// resource (when it is not found in our data store, we treat it as version
    /// 0, a template with zero conditions and zero parameters). Hence there are
    /// no Create or Delete operations. Returns the updated template when
    /// successful (and the updated eTag as a response header), or an error if
    /// things go wrong.
    /// Possible error messages:
    /// * VALIDATION_ERROR (HTTP status 400) with additional details if the
    /// template being passed in can not be validated.
    /// * AUTHENTICATION_ERROR (HTTP status 401) if the request can not be
    /// authenticate (e.g. no access token, or invalid access token).
    /// * AUTHORIZATION_ERROR (HTTP status 403) if the request can not be
    /// authorized (e.g. the user has no access to the specified project id).
    /// * VERSION_MISMATCH (HTTP status 412) when trying to update when the
    /// expected eTag (passed in via the "If-match" header) is not specified, or
    /// is specified but does does not match the current eTag.
    /// * Internal error (HTTP status 500) for Database problems or other internal
    /// errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The GMP project identifier. Required.
    ///               See note at the beginning of this file regarding project ids.
    pub fn update_remote_config(&self, request: RemoteConfig, project: &str) -> ProjectUpdateRemoteConfigCall<'a, S> {
        ProjectUpdateRemoteConfigCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _validate_only: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



