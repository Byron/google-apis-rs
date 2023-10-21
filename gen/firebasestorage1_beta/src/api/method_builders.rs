use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Firebasestorage`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_firebasestorage1_beta as firebasestorage1_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use firebasestorage1_beta::{Firebasestorage, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Firebasestorage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `buckets_add_firebase(...)`, `buckets_get(...)`, `buckets_list(...)` and `buckets_remove_firebase(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Firebasestorage<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Links a Google Cloud Storage bucket to a Firebase project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Required. Resource name of the bucket, mirrors the ID of the underlying Google Cloud Storage bucket, `projects/{project_number}/buckets/{bucket_id}`.
    pub fn buckets_add_firebase(&self, request: AddFirebaseRequest, bucket: &str) -> ProjectBucketAddFirebaseCall<'a, S> {
        ProjectBucketAddFirebaseCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single linked storage bucket.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the bucket, mirrors the ID of the underlying Google Cloud Storage bucket, `projects/{project_number}/buckets/{bucket_id}`.
    pub fn buckets_get(&self, name: &str) -> ProjectBucketGetCall<'a, S> {
        ProjectBucketGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the linked storage buckets for a project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the parent Firebase project, `projects/{project_number}`.
    pub fn buckets_list(&self, parent: &str) -> ProjectBucketListCall<'a, S> {
        ProjectBucketListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Unlinks a linked Google Cloud Storage bucket from a Firebase project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Required. Resource name of the bucket, mirrors the ID of the underlying Google Cloud Storage bucket, `projects/{project_number}/buckets/{bucket_id}`.
    pub fn buckets_remove_firebase(&self, request: RemoveFirebaseRequest, bucket: &str) -> ProjectBucketRemoveFirebaseCall<'a, S> {
        ProjectBucketRemoveFirebaseCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



