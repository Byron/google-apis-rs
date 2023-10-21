use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CertificateAuthorityService`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_privateca1_beta1 as privateca1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use privateca1_beta1::{CertificateAuthorityService, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CertificateAuthorityService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_certificate_authorities_activate(...)`, `locations_certificate_authorities_certificate_revocation_lists_get(...)`, `locations_certificate_authorities_certificate_revocation_lists_get_iam_policy(...)`, `locations_certificate_authorities_certificate_revocation_lists_list(...)`, `locations_certificate_authorities_certificate_revocation_lists_patch(...)`, `locations_certificate_authorities_certificate_revocation_lists_set_iam_policy(...)`, `locations_certificate_authorities_certificate_revocation_lists_test_iam_permissions(...)`, `locations_certificate_authorities_certificates_create(...)`, `locations_certificate_authorities_certificates_get(...)`, `locations_certificate_authorities_certificates_list(...)`, `locations_certificate_authorities_certificates_patch(...)`, `locations_certificate_authorities_certificates_revoke(...)`, `locations_certificate_authorities_create(...)`, `locations_certificate_authorities_disable(...)`, `locations_certificate_authorities_enable(...)`, `locations_certificate_authorities_fetch(...)`, `locations_certificate_authorities_get(...)`, `locations_certificate_authorities_get_iam_policy(...)`, `locations_certificate_authorities_list(...)`, `locations_certificate_authorities_patch(...)`, `locations_certificate_authorities_restore(...)`, `locations_certificate_authorities_schedule_delete(...)`, `locations_certificate_authorities_set_iam_policy(...)`, `locations_certificate_authorities_test_iam_permissions(...)`, `locations_get(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_reusable_configs_get(...)`, `locations_reusable_configs_get_iam_policy(...)`, `locations_reusable_configs_list(...)`, `locations_reusable_configs_set_iam_policy(...)` and `locations_reusable_configs_test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CertificateAuthorityService<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a CertificateRevocationList.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the CertificateRevocationList to get.
    pub fn locations_certificate_authorities_certificate_revocation_lists_get(&self, name: &str) -> ProjectLocationCertificateAuthorityCertificateRevocationListGetCall<'a, S> {
        ProjectLocationCertificateAuthorityCertificateRevocationListGetCall {
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
    pub fn locations_certificate_authorities_certificate_revocation_lists_get_iam_policy(&self, resource: &str) -> ProjectLocationCertificateAuthorityCertificateRevocationListGetIamPolicyCall<'a, S> {
        ProjectLocationCertificateAuthorityCertificateRevocationListGetIamPolicyCall {
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
    /// Lists CertificateRevocationLists.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the location associated with the CertificateRevocationLists, in the format `projects/*/locations/*/certificateauthorities/*`.
    pub fn locations_certificate_authorities_certificate_revocation_lists_list(&self, parent: &str) -> ProjectLocationCertificateAuthorityCertificateRevocationListListCall<'a, S> {
        ProjectLocationCertificateAuthorityCertificateRevocationListListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update a CertificateRevocationList.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource path for this CertificateRevocationList in the format `projects/*/locations/*/certificateAuthorities/*/ certificateRevocationLists/*`.
    pub fn locations_certificate_authorities_certificate_revocation_lists_patch(&self, request: CertificateRevocationList, name: &str) -> ProjectLocationCertificateAuthorityCertificateRevocationListPatchCall<'a, S> {
        ProjectLocationCertificateAuthorityCertificateRevocationListPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
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
    pub fn locations_certificate_authorities_certificate_revocation_lists_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationCertificateAuthorityCertificateRevocationListSetIamPolicyCall<'a, S> {
        ProjectLocationCertificateAuthorityCertificateRevocationListSetIamPolicyCall {
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
    pub fn locations_certificate_authorities_certificate_revocation_lists_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationCertificateAuthorityCertificateRevocationListTestIamPermissionCall<'a, S> {
        ProjectLocationCertificateAuthorityCertificateRevocationListTestIamPermissionCall {
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
    /// Create a new Certificate in a given Project, Location from a particular CertificateAuthority.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the location and CertificateAuthority associated with the Certificate, in the format `projects/*/locations/*/certificateAuthorities/*`.
    pub fn locations_certificate_authorities_certificates_create(&self, request: Certificate, parent: &str) -> ProjectLocationCertificateAuthorityCertificateCreateCall<'a, S> {
        ProjectLocationCertificateAuthorityCertificateCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _certificate_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a Certificate.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Certificate to get.
    pub fn locations_certificate_authorities_certificates_get(&self, name: &str) -> ProjectLocationCertificateAuthorityCertificateGetCall<'a, S> {
        ProjectLocationCertificateAuthorityCertificateGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Certificates.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the location associated with the Certificates, in the format `projects/*/locations/*/certificateauthorities/*`.
    pub fn locations_certificate_authorities_certificates_list(&self, parent: &str) -> ProjectLocationCertificateAuthorityCertificateListCall<'a, S> {
        ProjectLocationCertificateAuthorityCertificateListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update a Certificate. Currently, the only field you can update is the labels field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource path for this Certificate in the format `projects/*/locations/*/certificateAuthorities/*/certificates/*`.
    pub fn locations_certificate_authorities_certificates_patch(&self, request: Certificate, name: &str) -> ProjectLocationCertificateAuthorityCertificatePatchCall<'a, S> {
        ProjectLocationCertificateAuthorityCertificatePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Revoke a Certificate.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name for this Certificate in the format `projects/*/locations/*/certificateAuthorities/*/certificates/*`.
    pub fn locations_certificate_authorities_certificates_revoke(&self, request: RevokeCertificateRequest, name: &str) -> ProjectLocationCertificateAuthorityCertificateRevokeCall<'a, S> {
        ProjectLocationCertificateAuthorityCertificateRevokeCall {
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
    /// Activate a CertificateAuthority that is in state PENDING_ACTIVATION and is of type SUBORDINATE. After the parent Certificate Authority signs a certificate signing request from FetchCertificateAuthorityCsr, this method can complete the activation process.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/certificateAuthorities/*`.
    pub fn locations_certificate_authorities_activate(&self, request: ActivateCertificateAuthorityRequest, name: &str) -> ProjectLocationCertificateAuthorityActivateCall<'a, S> {
        ProjectLocationCertificateAuthorityActivateCall {
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
    /// Create a new CertificateAuthority in a given Project and Location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the location associated with the CertificateAuthorities, in the format `projects/*/locations/*`.
    pub fn locations_certificate_authorities_create(&self, request: CertificateAuthority, parent: &str) -> ProjectLocationCertificateAuthorityCreateCall<'a, S> {
        ProjectLocationCertificateAuthorityCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _certificate_authority_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Disable a CertificateAuthority.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/certificateAuthorities/*`.
    pub fn locations_certificate_authorities_disable(&self, request: DisableCertificateAuthorityRequest, name: &str) -> ProjectLocationCertificateAuthorityDisableCall<'a, S> {
        ProjectLocationCertificateAuthorityDisableCall {
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
    /// Enable a CertificateAuthority.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/certificateAuthorities/*`.
    pub fn locations_certificate_authorities_enable(&self, request: EnableCertificateAuthorityRequest, name: &str) -> ProjectLocationCertificateAuthorityEnableCall<'a, S> {
        ProjectLocationCertificateAuthorityEnableCall {
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
    /// Fetch a certificate signing request (CSR) from a CertificateAuthority that is in state PENDING_ACTIVATION and is of type SUBORDINATE. The CSR must then be signed by the desired parent Certificate Authority, which could be another CertificateAuthority resource, or could be an on-prem certificate authority. See also ActivateCertificateAuthority.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/certificateAuthorities/*`.
    pub fn locations_certificate_authorities_fetch(&self, name: &str) -> ProjectLocationCertificateAuthorityFetchCall<'a, S> {
        ProjectLocationCertificateAuthorityFetchCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a CertificateAuthority.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the CertificateAuthority to get.
    pub fn locations_certificate_authorities_get(&self, name: &str) -> ProjectLocationCertificateAuthorityGetCall<'a, S> {
        ProjectLocationCertificateAuthorityGetCall {
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
    pub fn locations_certificate_authorities_get_iam_policy(&self, resource: &str) -> ProjectLocationCertificateAuthorityGetIamPolicyCall<'a, S> {
        ProjectLocationCertificateAuthorityGetIamPolicyCall {
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
    /// Lists CertificateAuthorities.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the location associated with the CertificateAuthorities, in the format `projects/*/locations/*`.
    pub fn locations_certificate_authorities_list(&self, parent: &str) -> ProjectLocationCertificateAuthorityListCall<'a, S> {
        ProjectLocationCertificateAuthorityListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update a CertificateAuthority.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name for this CertificateAuthority in the format `projects/*/locations/*/certificateAuthorities/*`.
    pub fn locations_certificate_authorities_patch(&self, request: CertificateAuthority, name: &str) -> ProjectLocationCertificateAuthorityPatchCall<'a, S> {
        ProjectLocationCertificateAuthorityPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Restore a CertificateAuthority that is scheduled for deletion.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/certificateAuthorities/*`.
    pub fn locations_certificate_authorities_restore(&self, request: RestoreCertificateAuthorityRequest, name: &str) -> ProjectLocationCertificateAuthorityRestoreCall<'a, S> {
        ProjectLocationCertificateAuthorityRestoreCall {
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
    /// Schedule a CertificateAuthority for deletion.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/certificateAuthorities/*`.
    pub fn locations_certificate_authorities_schedule_delete(&self, request: ScheduleDeleteCertificateAuthorityRequest, name: &str) -> ProjectLocationCertificateAuthorityScheduleDeleteCall<'a, S> {
        ProjectLocationCertificateAuthorityScheduleDeleteCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_certificate_authorities_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationCertificateAuthoritySetIamPolicyCall<'a, S> {
        ProjectLocationCertificateAuthoritySetIamPolicyCall {
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
    pub fn locations_certificate_authorities_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationCertificateAuthorityTestIamPermissionCall<'a, S> {
        ProjectLocationCertificateAuthorityTestIamPermissionCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn locations_operations_cancel(&self, request: CancelOperationRequest, name: &str) -> ProjectLocationOperationCancelCall<'a, S> {
        ProjectLocationOperationCancelCall {
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
    /// Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn locations_operations_delete(&self, name: &str) -> ProjectLocationOperationDeleteCall<'a, S> {
        ProjectLocationOperationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_operations_get(&self, name: &str) -> ProjectLocationOperationGetCall<'a, S> {
        ProjectLocationOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn locations_operations_list(&self, name: &str) -> ProjectLocationOperationListCall<'a, S> {
        ProjectLocationOperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a ReusableConfig.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the ReusableConfigs to get.
    pub fn locations_reusable_configs_get(&self, name: &str) -> ProjectLocationReusableConfigGetCall<'a, S> {
        ProjectLocationReusableConfigGetCall {
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
    pub fn locations_reusable_configs_get_iam_policy(&self, resource: &str) -> ProjectLocationReusableConfigGetIamPolicyCall<'a, S> {
        ProjectLocationReusableConfigGetIamPolicyCall {
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
    /// Lists ReusableConfigs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the location associated with the ReusableConfigs, in the format `projects/*/locations/*`.
    pub fn locations_reusable_configs_list(&self, parent: &str) -> ProjectLocationReusableConfigListCall<'a, S> {
        ProjectLocationReusableConfigListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
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
    pub fn locations_reusable_configs_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationReusableConfigSetIamPolicyCall<'a, S> {
        ProjectLocationReusableConfigSetIamPolicyCall {
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
    pub fn locations_reusable_configs_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationReusableConfigTestIamPermissionCall<'a, S> {
        ProjectLocationReusableConfigTestIamPermissionCall {
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
    /// Gets information about a location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Resource name for the location.
    pub fn locations_get(&self, name: &str) -> ProjectLocationGetCall<'a, S> {
        ProjectLocationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists information about the supported locations for this service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource that owns the locations collection, if applicable.
    pub fn locations_list(&self, name: &str) -> ProjectLocationListCall<'a, S> {
        ProjectLocationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



