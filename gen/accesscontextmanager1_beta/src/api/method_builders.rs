use super::*;
/// A builder providing access to all methods supported on *accessPolicy* resources.
/// It is not used directly, but through the [`AccessContextManager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_accesscontextmanager1_beta as accesscontextmanager1_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use accesscontextmanager1_beta::{AccessContextManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AccessContextManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `access_levels_create(...)`, `access_levels_delete(...)`, `access_levels_get(...)`, `access_levels_list(...)`, `access_levels_patch(...)`, `create(...)`, `delete(...)`, `get(...)`, `list(...)`, `patch(...)`, `service_perimeters_create(...)`, `service_perimeters_delete(...)`, `service_perimeters_get(...)`, `service_perimeters_list(...)` and `service_perimeters_patch(...)`
/// // to build up your call.
/// let rb = hub.access_policies();
/// # }
/// ```
pub struct AccessPolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AccessContextManager<S>,
}

impl<'a, S> client::MethodsBuilder for AccessPolicyMethods<'a, S> {}

impl<'a, S> AccessPolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create an Access Level. The longrunning operation from this RPC will have a successful status once the Access Level has propagated to long-lasting storage. Access Levels containing errors will result in an error response for the first error encountered.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name for the access policy which owns this Access Level. Format: `accessPolicies/{policy_id}`
    pub fn access_levels_create(&self, request: AccessLevel, parent: &str) -> AccessPolicyAccessLevelCreateCall<'a, S> {
        AccessPolicyAccessLevelCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete an Access Level by resource name. The longrunning operation from this RPC will have a successful status once the Access Level has been removed from long-lasting storage.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for the Access Level. Format: `accessPolicies/{policy_id}/accessLevels/{access_level_id}`
    pub fn access_levels_delete(&self, name: &str) -> AccessPolicyAccessLevelDeleteCall<'a, S> {
        AccessPolicyAccessLevelDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get an Access Level by resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for the Access Level. Format: `accessPolicies/{policy_id}/accessLevels/{access_level_id}`
    pub fn access_levels_get(&self, name: &str) -> AccessPolicyAccessLevelGetCall<'a, S> {
        AccessPolicyAccessLevelGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _access_level_format: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all Access Levels for an access policy.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name for the access policy to list Access Levels from. Format: `accessPolicies/{policy_id}`
    pub fn access_levels_list(&self, parent: &str) -> AccessPolicyAccessLevelListCall<'a, S> {
        AccessPolicyAccessLevelListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _access_level_format: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update an Access Level. The longrunning operation from this RPC will have a successful status once the changes to the Access Level have propagated to long-lasting storage. Access Levels containing errors will result in an error response for the first error encountered.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name for the `AccessLevel`. Format: `accessPolicies/{access_policy}/accessLevels/{access_level}`. The `access_level` component must begin with a letter, followed by alphanumeric characters or `_`. Its maximum length is 50 characters. After you create an `AccessLevel`, you cannot change its `name`.
    pub fn access_levels_patch(&self, request: AccessLevel, name: &str) -> AccessPolicyAccessLevelPatchCall<'a, S> {
        AccessPolicyAccessLevelPatchCall {
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
    /// Create a Service Perimeter. The longrunning operation from this RPC will have a successful status once the Service Perimeter has propagated to long-lasting storage. Service Perimeters containing errors will result in an error response for the first error encountered.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name for the access policy which owns this Service Perimeter. Format: `accessPolicies/{policy_id}`
    pub fn service_perimeters_create(&self, request: ServicePerimeter, parent: &str) -> AccessPolicyServicePerimeterCreateCall<'a, S> {
        AccessPolicyServicePerimeterCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a Service Perimeter by resource name. The longrunning operation from this RPC will have a successful status once the Service Perimeter has been removed from long-lasting storage.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for the Service Perimeter. Format: `accessPolicies/{policy_id}/servicePerimeters/{service_perimeter_id}`
    pub fn service_perimeters_delete(&self, name: &str) -> AccessPolicyServicePerimeterDeleteCall<'a, S> {
        AccessPolicyServicePerimeterDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a Service Perimeter by resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for the Service Perimeter. Format: `accessPolicies/{policy_id}/servicePerimeters/{service_perimeters_id}`
    pub fn service_perimeters_get(&self, name: &str) -> AccessPolicyServicePerimeterGetCall<'a, S> {
        AccessPolicyServicePerimeterGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all Service Perimeters for an access policy.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name for the access policy to list Service Perimeters from. Format: `accessPolicies/{policy_id}`
    pub fn service_perimeters_list(&self, parent: &str) -> AccessPolicyServicePerimeterListCall<'a, S> {
        AccessPolicyServicePerimeterListCall {
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
    /// Update a Service Perimeter. The longrunning operation from this RPC will have a successful status once the changes to the Service Perimeter have propagated to long-lasting storage. Service Perimeter containing errors will result in an error response for the first error encountered.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name for the `ServicePerimeter`. Format: `accessPolicies/{access_policy}/servicePerimeters/{service_perimeter}`. The `service_perimeter` component must begin with a letter, followed by alphanumeric characters or `_`. After you create a `ServicePerimeter`, you cannot change its `name`.
    pub fn service_perimeters_patch(&self, request: ServicePerimeter, name: &str) -> AccessPolicyServicePerimeterPatchCall<'a, S> {
        AccessPolicyServicePerimeterPatchCall {
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
    /// Create an `AccessPolicy`. Fails if this organization already has a `AccessPolicy`. The longrunning Operation will have a successful status once the `AccessPolicy` has propagated to long-lasting storage. Syntactic and basic semantic errors will be returned in `metadata` as a BadRequest proto.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: AccessPolicy) -> AccessPolicyCreateCall<'a, S> {
        AccessPolicyCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete an AccessPolicy by resource name. The longrunning Operation will have a successful status once the AccessPolicy has been removed from long-lasting storage.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for the access policy to delete. Format `accessPolicies/{policy_id}`
    pub fn delete(&self, name: &str) -> AccessPolicyDeleteCall<'a, S> {
        AccessPolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get an AccessPolicy by name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for the access policy to get. Format `accessPolicies/{policy_id}`
    pub fn get(&self, name: &str) -> AccessPolicyGetCall<'a, S> {
        AccessPolicyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all AccessPolicies under a container.
    pub fn list(&self) -> AccessPolicyListCall<'a, S> {
        AccessPolicyListCall {
            hub: self.hub,
            _parent: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update an AccessPolicy. The longrunning Operation from this RPC will have a successful status once the changes to the AccessPolicy have propagated to long-lasting storage. Syntactic and basic semantic errors will be returned in `metadata` as a BadRequest proto.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the `AccessPolicy`. Format: `accessPolicies/{policy_id}`
    pub fn patch(&self, request: AccessPolicy, name: &str) -> AccessPolicyPatchCall<'a, S> {
        AccessPolicyPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`AccessContextManager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_accesscontextmanager1_beta as accesscontextmanager1_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use accesscontextmanager1_beta::{AccessContextManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AccessContextManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AccessContextManager<S>,
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



