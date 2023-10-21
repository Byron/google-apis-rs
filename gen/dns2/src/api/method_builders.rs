use super::*;
/// A builder providing access to all methods supported on *change* resources.
/// It is not used directly, but through the [`Dns`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dns2 as dns2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dns2::{Dns, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dns::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.changes();
/// # }
/// ```
pub struct ChangeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dns<S>,
}

impl<'a, S> client::MethodsBuilder for ChangeMethods<'a, S> {}

impl<'a, S> ChangeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Atomically updates the ResourceRecordSet collection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - No description provided.
    /// * `managedZone` - Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    pub fn create(&self, request: Change, project: &str, location: &str, managed_zone: &str) -> ChangeCreateCall<'a, S> {
        ChangeCreateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the representation of an existing Change.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - No description provided.
    /// * `managedZone` - Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    /// * `changeId` - The identifier of the requested change, from a previous ResourceRecordSetsChangeResponse.
    pub fn get(&self, project: &str, location: &str, managed_zone: &str, change_id: &str) -> ChangeGetCall<'a, S> {
        ChangeGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _change_id: change_id.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Enumerates Changes to a ResourceRecordSet collection.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - No description provided.
    /// * `managedZone` - Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    pub fn list(&self, project: &str, location: &str, managed_zone: &str) -> ChangeListCall<'a, S> {
        ChangeListCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _sort_order: Default::default(),
            _sort_by: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *dnsKey* resources.
/// It is not used directly, but through the [`Dns`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dns2 as dns2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dns2::{Dns, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dns::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.dns_keys();
/// # }
/// ```
pub struct DnsKeyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dns<S>,
}

impl<'a, S> client::MethodsBuilder for DnsKeyMethods<'a, S> {}

impl<'a, S> DnsKeyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the representation of an existing DnsKey.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `managedZone` - Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    /// * `dnsKeyId` - The identifier of the requested DnsKey.
    pub fn get(&self, project: &str, location: &str, managed_zone: &str, dns_key_id: &str) -> DnsKeyGetCall<'a, S> {
        DnsKeyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _dns_key_id: dns_key_id.to_string(),
            _digest_type: Default::default(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Enumerates DnsKeys to a ResourceRecordSet collection.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `managedZone` - Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    pub fn list(&self, project: &str, location: &str, managed_zone: &str) -> DnsKeyListCall<'a, S> {
        DnsKeyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _digest_type: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *managedZoneOperation* resources.
/// It is not used directly, but through the [`Dns`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dns2 as dns2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dns2::{Dns, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dns::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.managed_zone_operations();
/// # }
/// ```
pub struct ManagedZoneOperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dns<S>,
}

impl<'a, S> client::MethodsBuilder for ManagedZoneOperationMethods<'a, S> {}

impl<'a, S> ManagedZoneOperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the representation of an existing Operation.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `managedZone` - Identifies the managed zone addressed by this request.
    /// * `operation` - Identifies the operation addressed by this request (ID of the operation).
    pub fn get(&self, project: &str, location: &str, managed_zone: &str, operation: &str) -> ManagedZoneOperationGetCall<'a, S> {
        ManagedZoneOperationGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _operation: operation.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Enumerates Operations for the given ManagedZone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `managedZone` - Identifies the managed zone addressed by this request.
    pub fn list(&self, project: &str, location: &str, managed_zone: &str) -> ManagedZoneOperationListCall<'a, S> {
        ManagedZoneOperationListCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _sort_by: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *managedZone* resources.
/// It is not used directly, but through the [`Dns`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dns2 as dns2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dns2::{Dns, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dns::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `get_iam_policy(...)`, `list(...)`, `patch(...)`, `set_iam_policy(...)`, `test_iam_permissions(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.managed_zones();
/// # }
/// ```
pub struct ManagedZoneMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dns<S>,
}

impl<'a, S> client::MethodsBuilder for ManagedZoneMethods<'a, S> {}

impl<'a, S> ManagedZoneMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new ManagedZone.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    pub fn create(&self, request: ManagedZone, project: &str, location: &str) -> ManagedZoneCreateCall<'a, S> {
        ManagedZoneCreateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _location: location.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a previously created ManagedZone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `managedZone` - Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    pub fn delete(&self, project: &str, location: &str, managed_zone: &str) -> ManagedZoneDeleteCall<'a, S> {
        ManagedZoneDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the representation of an existing ManagedZone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `managedZone` - Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    pub fn get(&self, project: &str, location: &str, managed_zone: &str) -> ManagedZoneGetCall<'a, S> {
        ManagedZoneGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn get_iam_policy(&self, request: GoogleIamV1GetIamPolicyRequest, resource: &str) -> ManagedZoneGetIamPolicyCall<'a, S> {
        ManagedZoneGetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Enumerates ManagedZones that have been created but not yet deleted.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    pub fn list(&self, project: &str, location: &str) -> ManagedZoneListCall<'a, S> {
        ManagedZoneListCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _dns_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Applies a partial update to an existing ManagedZone.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `managedZone` - Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    pub fn patch(&self, request: ManagedZone, project: &str, location: &str, managed_zone: &str) -> ManagedZonePatchCall<'a, S> {
        ManagedZonePatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ManagedZoneSetIamPolicyCall<'a, S> {
        ManagedZoneSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this returns an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ManagedZoneTestIamPermissionCall<'a, S> {
        ManagedZoneTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing ManagedZone.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `managedZone` - Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    pub fn update(&self, request: ManagedZone, project: &str, location: &str, managed_zone: &str) -> ManagedZoneUpdateCall<'a, S> {
        ManagedZoneUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *policy* resources.
/// It is not used directly, but through the [`Dns`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dns2 as dns2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dns2::{Dns, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dns::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.policies();
/// # }
/// ```
pub struct PolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dns<S>,
}

impl<'a, S> client::MethodsBuilder for PolicyMethods<'a, S> {}

impl<'a, S> PolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new Policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    pub fn create(&self, request: Policy, project: &str, location: &str) -> PolicyCreateCall<'a, S> {
        PolicyCreateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _location: location.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a previously created Policy. Fails if the policy is still being referenced by a network.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `policy` - User given friendly name of the policy addressed by this request.
    pub fn delete(&self, project: &str, location: &str, policy: &str) -> PolicyDeleteCall<'a, S> {
        PolicyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _policy: policy.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the representation of an existing Policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `policy` - User given friendly name of the policy addressed by this request.
    pub fn get(&self, project: &str, location: &str, policy: &str) -> PolicyGetCall<'a, S> {
        PolicyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _policy: policy.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Enumerates all Policies associated with a project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    pub fn list(&self, project: &str, location: &str) -> PolicyListCall<'a, S> {
        PolicyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Applies a partial update to an existing Policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `policy` - User given friendly name of the policy addressed by this request.
    pub fn patch(&self, request: Policy, project: &str, location: &str, policy: &str) -> PolicyPatchCall<'a, S> {
        PolicyPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _location: location.to_string(),
            _policy: policy.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing Policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `policy` - User given friendly name of the policy addressed by this request.
    pub fn update(&self, request: Policy, project: &str, location: &str, policy: &str) -> PolicyUpdateCall<'a, S> {
        PolicyUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _location: location.to_string(),
            _policy: policy.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Dns`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dns2 as dns2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dns2::{Dns, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dns::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dns<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the representation of an existing Project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - No description provided.
    pub fn get(&self, project: &str, location: &str) -> ProjectGetCall<'a, S> {
        ProjectGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *resourceRecordSet* resources.
/// It is not used directly, but through the [`Dns`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dns2 as dns2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dns2::{Dns, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dns::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.resource_record_sets();
/// # }
/// ```
pub struct ResourceRecordSetMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dns<S>,
}

impl<'a, S> client::MethodsBuilder for ResourceRecordSetMethods<'a, S> {}

impl<'a, S> ResourceRecordSetMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new ResourceRecordSet.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `managedZone` - Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    pub fn create(&self, request: ResourceRecordSet, project: &str, location: &str, managed_zone: &str) -> ResourceRecordSetCreateCall<'a, S> {
        ResourceRecordSetCreateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a previously created ResourceRecordSet.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `managedZone` - Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    /// * `name` - Fully qualified domain name.
    /// * `type` - RRSet type.
    pub fn delete(&self, project: &str, location: &str, managed_zone: &str, name: &str, type_: &str) -> ResourceRecordSetDeleteCall<'a, S> {
        ResourceRecordSetDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _name: name.to_string(),
            _type_: type_.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the representation of an existing ResourceRecordSet.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `managedZone` - Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    /// * `name` - Fully qualified domain name.
    /// * `type` - RRSet type.
    pub fn get(&self, project: &str, location: &str, managed_zone: &str, name: &str, type_: &str) -> ResourceRecordSetGetCall<'a, S> {
        ResourceRecordSetGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _name: name.to_string(),
            _type_: type_.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Enumerates ResourceRecordSets that you have created but not yet deleted.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `managedZone` - Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    pub fn list(&self, project: &str, location: &str, managed_zone: &str) -> ResourceRecordSetListCall<'a, S> {
        ResourceRecordSetListCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _type_: Default::default(),
            _page_token: Default::default(),
            _name: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Applies a partial update to an existing ResourceRecordSet.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `managedZone` - Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    /// * `name` - Fully qualified domain name.
    /// * `type` - RRSet type.
    pub fn patch(&self, request: ResourceRecordSet, project: &str, location: &str, managed_zone: &str, name: &str, type_: &str) -> ResourceRecordSetPatchCall<'a, S> {
        ResourceRecordSetPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _location: location.to_string(),
            _managed_zone: managed_zone.to_string(),
            _name: name.to_string(),
            _type_: type_.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *responsePolicy* resources.
/// It is not used directly, but through the [`Dns`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dns2 as dns2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dns2::{Dns, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dns::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.response_policies();
/// # }
/// ```
pub struct ResponsePolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dns<S>,
}

impl<'a, S> client::MethodsBuilder for ResponsePolicyMethods<'a, S> {}

impl<'a, S> ResponsePolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new Response Policy
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource, only applicable in the v APIs. This information will be used for routing and will be part of the resource name.
    pub fn create(&self, request: ResponsePolicy, project: &str, location: &str) -> ResponsePolicyCreateCall<'a, S> {
        ResponsePolicyCreateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _location: location.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a previously created Response Policy. Fails if the response policy is non-empty or still being referenced by a network.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `responsePolicy` - User assigned name of the Response Policy addressed by this request.
    pub fn delete(&self, project: &str, location: &str, response_policy: &str) -> ResponsePolicyDeleteCall<'a, S> {
        ResponsePolicyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _response_policy: response_policy.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the representation of an existing Response Policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `responsePolicy` - User assigned name of the Response Policy addressed by this request.
    pub fn get(&self, project: &str, location: &str, response_policy: &str) -> ResponsePolicyGetCall<'a, S> {
        ResponsePolicyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _response_policy: response_policy.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Enumerates all Response Policies associated with a project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    pub fn list(&self, project: &str, location: &str) -> ResponsePolicyListCall<'a, S> {
        ResponsePolicyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Applies a partial update to an existing Response Policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `responsePolicy` - User assigned name of the response policy addressed by this request.
    pub fn patch(&self, request: ResponsePolicy, project: &str, location: &str, response_policy: &str) -> ResponsePolicyPatchCall<'a, S> {
        ResponsePolicyPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _location: location.to_string(),
            _response_policy: response_policy.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing Response Policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `responsePolicy` - User assigned name of the Response Policy addressed by this request.
    pub fn update(&self, request: ResponsePolicy, project: &str, location: &str, response_policy: &str) -> ResponsePolicyUpdateCall<'a, S> {
        ResponsePolicyUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _location: location.to_string(),
            _response_policy: response_policy.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *responsePolicyRule* resources.
/// It is not used directly, but through the [`Dns`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dns2 as dns2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dns2::{Dns, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dns::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.response_policy_rules();
/// # }
/// ```
pub struct ResponsePolicyRuleMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dns<S>,
}

impl<'a, S> client::MethodsBuilder for ResponsePolicyRuleMethods<'a, S> {}

impl<'a, S> ResponsePolicyRuleMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new Response Policy Rule.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `responsePolicy` - User assigned name of the Response Policy containing the Response Policy Rule.
    pub fn create(&self, request: ResponsePolicyRule, project: &str, location: &str, response_policy: &str) -> ResponsePolicyRuleCreateCall<'a, S> {
        ResponsePolicyRuleCreateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _location: location.to_string(),
            _response_policy: response_policy.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a previously created Response Policy Rule.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `responsePolicy` - User assigned name of the Response Policy containing the Response Policy Rule.
    /// * `responsePolicyRule` - User assigned name of the Response Policy Rule addressed by this request.
    pub fn delete(&self, project: &str, location: &str, response_policy: &str, response_policy_rule: &str) -> ResponsePolicyRuleDeleteCall<'a, S> {
        ResponsePolicyRuleDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _response_policy: response_policy.to_string(),
            _response_policy_rule: response_policy_rule.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the representation of an existing Response Policy Rule.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `responsePolicy` - User assigned name of the Response Policy containing the Response Policy Rule.
    /// * `responsePolicyRule` - User assigned name of the Response Policy Rule addressed by this request.
    pub fn get(&self, project: &str, location: &str, response_policy: &str, response_policy_rule: &str) -> ResponsePolicyRuleGetCall<'a, S> {
        ResponsePolicyRuleGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _response_policy: response_policy.to_string(),
            _response_policy_rule: response_policy_rule.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Enumerates all Response Policy Rules associated with a project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `responsePolicy` - User assigned name of the Response Policy to list.
    pub fn list(&self, project: &str, location: &str, response_policy: &str) -> ResponsePolicyRuleListCall<'a, S> {
        ResponsePolicyRuleListCall {
            hub: self.hub,
            _project: project.to_string(),
            _location: location.to_string(),
            _response_policy: response_policy.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Applies a partial update to an existing Response Policy Rule.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `responsePolicy` - User assigned name of the Response Policy containing the Response Policy Rule.
    /// * `responsePolicyRule` - User assigned name of the Response Policy Rule addressed by this request.
    pub fn patch(&self, request: ResponsePolicyRule, project: &str, location: &str, response_policy: &str, response_policy_rule: &str) -> ResponsePolicyRulePatchCall<'a, S> {
        ResponsePolicyRulePatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _location: location.to_string(),
            _response_policy: response_policy.to_string(),
            _response_policy_rule: response_policy_rule.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing Response Policy Rule.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Identifies the project addressed by this request.
    /// * `location` - Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    /// * `responsePolicy` - User assigned name of the Response Policy containing the Response Policy Rule.
    /// * `responsePolicyRule` - User assigned name of the Response Policy Rule addressed by this request.
    pub fn update(&self, request: ResponsePolicyRule, project: &str, location: &str, response_policy: &str, response_policy_rule: &str) -> ResponsePolicyRuleUpdateCall<'a, S> {
        ResponsePolicyRuleUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _location: location.to_string(),
            _response_policy: response_policy.to_string(),
            _response_policy_rule: response_policy_rule.to_string(),
            _client_operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



