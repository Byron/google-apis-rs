use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`BinaryAuthorization`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_binaryauthorization1 as binaryauthorization1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use binaryauthorization1::{BinaryAuthorization, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = BinaryAuthorization::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `attestors_create(...)`, `attestors_delete(...)`, `attestors_get(...)`, `attestors_get_iam_policy(...)`, `attestors_list(...)`, `attestors_set_iam_policy(...)`, `attestors_test_iam_permissions(...)`, `attestors_update(...)`, `attestors_validate_attestation_occurrence(...)`, `get_policy(...)`, `policy_get_iam_policy(...)`, `policy_set_iam_policy(...)`, `policy_test_iam_permissions(...)` and `update_policy(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a BinaryAuthorization<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an attestor, and returns a copy of the new attestor. Returns NOT_FOUND if the project does not exist, INVALID_ARGUMENT if the request is malformed, ALREADY_EXISTS if the attestor already exists.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent of this attestor.
    pub fn attestors_create(&self, request: Attestor, parent: &str) -> ProjectAttestorCreateCall<'a, S> {
        ProjectAttestorCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _attestor_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an attestor. Returns NOT_FOUND if the attestor does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the attestors to delete, in the format `projects/*/attestors/*`.
    pub fn attestors_delete(&self, name: &str) -> ProjectAttestorDeleteCall<'a, S> {
        ProjectAttestorDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an attestor. Returns NOT_FOUND if the attestor does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the attestor to retrieve, in the format `projects/*/attestors/*`.
    pub fn attestors_get(&self, name: &str) -> ProjectAttestorGetCall<'a, S> {
        ProjectAttestorGetCall {
            hub: self.hub,
            _name: name.to_string(),
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn attestors_get_iam_policy(&self, resource: &str) -> ProjectAttestorGetIamPolicyCall<'a, S> {
        ProjectAttestorGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists attestors. Returns INVALID_ARGUMENT if the project does not exist.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the project associated with the attestors, in the format `projects/*`.
    pub fn attestors_list(&self, parent: &str) -> ProjectAttestorListCall<'a, S> {
        ProjectAttestorListCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn attestors_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectAttestorSetIamPolicyCall<'a, S> {
        ProjectAttestorSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn attestors_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectAttestorTestIamPermissionCall<'a, S> {
        ProjectAttestorTestIamPermissionCall {
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
    /// Updates an attestor. Returns NOT_FOUND if the attestor does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name, in the format: `projects/*/attestors/*`. This field may not be updated.
    pub fn attestors_update(&self, request: Attestor, name: &str) -> ProjectAttestorUpdateCall<'a, S> {
        ProjectAttestorUpdateCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns whether the given Attestation for the given image URI was signed by the given Attestor
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `attestor` - Required. The resource name of the Attestor of the occurrence, in the format `projects/*/attestors/*`.
    pub fn attestors_validate_attestation_occurrence(&self, request: ValidateAttestationOccurrenceRequest, attestor: &str) -> ProjectAttestorValidateAttestationOccurrenceCall<'a, S> {
        ProjectAttestorValidateAttestationOccurrenceCall {
            hub: self.hub,
            _request: request,
            _attestor: attestor.to_string(),
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn policy_get_iam_policy(&self, resource: &str) -> ProjectPolicyGetIamPolicyCall<'a, S> {
        ProjectPolicyGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
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
    pub fn policy_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectPolicySetIamPolicyCall<'a, S> {
        ProjectPolicySetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn policy_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectPolicyTestIamPermissionCall<'a, S> {
        ProjectPolicyTestIamPermissionCall {
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
    /// A policy specifies the attestors that must attest to a container image, before the project is allowed to deploy that image. There is at most one policy per project. All image admission requests are permitted if a project has no policy. Gets the policy for this project. Returns a default policy if the project does not have one.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the policy to retrieve, in the format `projects/*/policy`.
    pub fn get_policy(&self, name: &str) -> ProjectGetPolicyCall<'a, S> {
        ProjectGetPolicyCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates or updates a project's policy, and returns a copy of the new policy. A policy is always updated as a whole, to avoid race conditions with concurrent policy enforcement (or management!) requests. Returns NOT_FOUND if the project does not exist, INVALID_ARGUMENT if the request is malformed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name, in the format `projects/*/policy`. There is at most one policy per project.
    pub fn update_policy(&self, request: Policy, name: &str) -> ProjectUpdatePolicyCall<'a, S> {
        ProjectUpdatePolicyCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *systempolicy* resources.
/// It is not used directly, but through the [`BinaryAuthorization`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_binaryauthorization1 as binaryauthorization1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use binaryauthorization1::{BinaryAuthorization, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = BinaryAuthorization::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_policy(...)`
/// // to build up your call.
/// let rb = hub.systempolicy();
/// # }
/// ```
pub struct SystempolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a BinaryAuthorization<S>,
}

impl<'a, S> client::MethodsBuilder for SystempolicyMethods<'a, S> {}

impl<'a, S> SystempolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the current system policy in the specified location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name, in the format `locations/*/policy`. Note that the system policy is not associated with a project.
    pub fn get_policy(&self, name: &str) -> SystempolicyGetPolicyCall<'a, S> {
        SystempolicyGetPolicyCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



