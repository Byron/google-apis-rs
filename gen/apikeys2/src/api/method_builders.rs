use super::*;
/// A builder providing access to all methods supported on *key* resources.
/// It is not used directly, but through the [`ApiKeysService`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_apikeys2 as apikeys2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use apikeys2::{ApiKeysService, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ApiKeysService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `lookup_key(...)`
/// // to build up your call.
/// let rb = hub.keys();
/// # }
/// ```
pub struct KeyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ApiKeysService<S>,
}

impl<'a, S> client::MethodsBuilder for KeyMethods<'a, S> {}

impl<'a, S> KeyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Find the parent project and resource name of the API key that matches the key string in the request. If the API key has been purged, resource name will not be set. The service account must have the `apikeys.keys.lookup` permission on the parent project.
    pub fn lookup_key(&self) -> KeyLookupKeyCall<'a, S> {
        KeyLookupKeyCall {
            hub: self.hub,
            _key_string: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`ApiKeysService`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_apikeys2 as apikeys2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use apikeys2::{ApiKeysService, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ApiKeysService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ApiKeysService<S>,
}

impl<'a, S> client::MethodsBuilder for OperationMethods<'a, S> {}

impl<'a, S> OperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn get(&self, name: &str) -> OperationGetCall<'a, S> {
        OperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`ApiKeysService`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_apikeys2 as apikeys2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use apikeys2::{ApiKeysService, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ApiKeysService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_keys_create(...)`, `locations_keys_delete(...)`, `locations_keys_get(...)`, `locations_keys_get_key_string(...)`, `locations_keys_list(...)`, `locations_keys_patch(...)` and `locations_keys_undelete(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ApiKeysService<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new API key. NOTE: Key is a global resource; hence the only supported value for location is `global`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project in which the API key is created.
    pub fn locations_keys_create(&self, request: V2Key, parent: &str) -> ProjectLocationKeyCreateCall<'a, S> {
        ProjectLocationKeyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _key_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an API key. Deleted key can be retrieved within 30 days of deletion. Afterward, key will be purged from the project. NOTE: Key is a global resource; hence the only supported value for location is `global`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the API key to be deleted.
    pub fn locations_keys_delete(&self, name: &str) -> ProjectLocationKeyDeleteCall<'a, S> {
        ProjectLocationKeyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _etag: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the metadata for an API key. The key string of the API key isn't included in the response. NOTE: Key is a global resource; hence the only supported value for location is `global`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the API key to get.
    pub fn locations_keys_get(&self, name: &str) -> ProjectLocationKeyGetCall<'a, S> {
        ProjectLocationKeyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the key string for an API key. NOTE: Key is a global resource; hence the only supported value for location is `global`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the API key to be retrieved.
    pub fn locations_keys_get_key_string(&self, name: &str) -> ProjectLocationKeyGetKeyStringCall<'a, S> {
        ProjectLocationKeyGetKeyStringCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the API keys owned by a project. The key string of the API key isn't included in the response. NOTE: Key is a global resource; hence the only supported value for location is `global`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Lists all API keys associated with this project.
    pub fn locations_keys_list(&self, parent: &str) -> ProjectLocationKeyListCall<'a, S> {
        ProjectLocationKeyListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the modifiable fields of an API key. The key string of the API key isn't included in the response. NOTE: Key is a global resource; hence the only supported value for location is `global`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of the key. The `name` has the form: `projects//locations/global/keys/`. For example: `projects/123456867718/locations/global/keys/b7ff1f9f-8275-410a-94dd-3855ee9b5dd2` NOTE: Key is a global resource; hence the only supported value for location is `global`.
    pub fn locations_keys_patch(&self, request: V2Key, name: &str) -> ProjectLocationKeyPatchCall<'a, S> {
        ProjectLocationKeyPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Undeletes an API key which was deleted within 30 days. NOTE: Key is a global resource; hence the only supported value for location is `global`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the API key to be undeleted.
    pub fn locations_keys_undelete(&self, request: V2UndeleteKeyRequest, name: &str) -> ProjectLocationKeyUndeleteCall<'a, S> {
        ProjectLocationKeyUndeleteCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



