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
/// extern crate google_privateca1 as privateca1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use privateca1::{CertificateAuthorityService, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CertificateAuthorityService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_ca_pools_certificate_authorities_activate(...)`, `locations_ca_pools_certificate_authorities_certificate_revocation_lists_get(...)`, `locations_ca_pools_certificate_authorities_certificate_revocation_lists_get_iam_policy(...)`, `locations_ca_pools_certificate_authorities_certificate_revocation_lists_list(...)`, `locations_ca_pools_certificate_authorities_certificate_revocation_lists_patch(...)`, `locations_ca_pools_certificate_authorities_certificate_revocation_lists_set_iam_policy(...)`, `locations_ca_pools_certificate_authorities_certificate_revocation_lists_test_iam_permissions(...)`, `locations_ca_pools_certificate_authorities_create(...)`, `locations_ca_pools_certificate_authorities_delete(...)`, `locations_ca_pools_certificate_authorities_disable(...)`, `locations_ca_pools_certificate_authorities_enable(...)`, `locations_ca_pools_certificate_authorities_fetch(...)`, `locations_ca_pools_certificate_authorities_get(...)`, `locations_ca_pools_certificate_authorities_list(...)`, `locations_ca_pools_certificate_authorities_patch(...)`, `locations_ca_pools_certificate_authorities_undelete(...)`, `locations_ca_pools_certificates_create(...)`, `locations_ca_pools_certificates_get(...)`, `locations_ca_pools_certificates_list(...)`, `locations_ca_pools_certificates_patch(...)`, `locations_ca_pools_certificates_revoke(...)`, `locations_ca_pools_create(...)`, `locations_ca_pools_delete(...)`, `locations_ca_pools_fetch_ca_certs(...)`, `locations_ca_pools_get(...)`, `locations_ca_pools_get_iam_policy(...)`, `locations_ca_pools_list(...)`, `locations_ca_pools_patch(...)`, `locations_ca_pools_set_iam_policy(...)`, `locations_ca_pools_test_iam_permissions(...)`, `locations_certificate_templates_create(...)`, `locations_certificate_templates_delete(...)`, `locations_certificate_templates_get(...)`, `locations_certificate_templates_get_iam_policy(...)`, `locations_certificate_templates_list(...)`, `locations_certificate_templates_patch(...)`, `locations_certificate_templates_set_iam_policy(...)`, `locations_certificate_templates_test_iam_permissions(...)`, `locations_get(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)` and `locations_operations_list(...)`
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
    pub fn locations_ca_pools_certificate_authorities_certificate_revocation_lists_get(&self, name: &str) -> ProjectLocationCaPoolCertificateAuthorityCertificateRevocationListGetCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityCertificateRevocationListGetCall {
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
    pub fn locations_ca_pools_certificate_authorities_certificate_revocation_lists_get_iam_policy(&self, resource: &str) -> ProjectLocationCaPoolCertificateAuthorityCertificateRevocationListGetIamPolicyCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityCertificateRevocationListGetIamPolicyCall {
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
    /// * `parent` - Required. The resource name of the location associated with the CertificateRevocationLists, in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    pub fn locations_ca_pools_certificate_authorities_certificate_revocation_lists_list(&self, parent: &str) -> ProjectLocationCaPoolCertificateAuthorityCertificateRevocationListListCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityCertificateRevocationListListCall {
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
    /// * `name` - Output only. The resource name for this CertificateRevocationList in the format `projects/*/locations/*/caPools/*certificateAuthorities/*/ certificateRevocationLists/*`.
    pub fn locations_ca_pools_certificate_authorities_certificate_revocation_lists_patch(&self, request: CertificateRevocationList, name: &str) -> ProjectLocationCaPoolCertificateAuthorityCertificateRevocationListPatchCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityCertificateRevocationListPatchCall {
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
    pub fn locations_ca_pools_certificate_authorities_certificate_revocation_lists_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationCaPoolCertificateAuthorityCertificateRevocationListSetIamPolicyCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityCertificateRevocationListSetIamPolicyCall {
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
    pub fn locations_ca_pools_certificate_authorities_certificate_revocation_lists_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationCaPoolCertificateAuthorityCertificateRevocationListTestIamPermissionCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityCertificateRevocationListTestIamPermissionCall {
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
    /// Activate a CertificateAuthority that is in state AWAITING_USER_ACTIVATION and is of type SUBORDINATE. After the parent Certificate Authority signs a certificate signing request from FetchCertificateAuthorityCsr, this method can complete the activation process.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    pub fn locations_ca_pools_certificate_authorities_activate(&self, request: ActivateCertificateAuthorityRequest, name: &str) -> ProjectLocationCaPoolCertificateAuthorityActivateCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityActivateCall {
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
    /// * `parent` - Required. The resource name of the CaPool associated with the CertificateAuthorities, in the format `projects/*/locations/*/caPools/*`.
    pub fn locations_ca_pools_certificate_authorities_create(&self, request: CertificateAuthority, parent: &str) -> ProjectLocationCaPoolCertificateAuthorityCreateCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityCreateCall {
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
    /// Delete a CertificateAuthority.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    pub fn locations_ca_pools_certificate_authorities_delete(&self, name: &str) -> ProjectLocationCaPoolCertificateAuthorityDeleteCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _skip_grace_period: Default::default(),
            _request_id: Default::default(),
            _ignore_active_certificates: Default::default(),
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
    /// * `name` - Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    pub fn locations_ca_pools_certificate_authorities_disable(&self, request: DisableCertificateAuthorityRequest, name: &str) -> ProjectLocationCaPoolCertificateAuthorityDisableCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityDisableCall {
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
    /// * `name` - Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    pub fn locations_ca_pools_certificate_authorities_enable(&self, request: EnableCertificateAuthorityRequest, name: &str) -> ProjectLocationCaPoolCertificateAuthorityEnableCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityEnableCall {
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
    /// Fetch a certificate signing request (CSR) from a CertificateAuthority that is in state AWAITING_USER_ACTIVATION and is of type SUBORDINATE. The CSR must then be signed by the desired parent Certificate Authority, which could be another CertificateAuthority resource, or could be an on-prem certificate authority. See also ActivateCertificateAuthority.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    pub fn locations_ca_pools_certificate_authorities_fetch(&self, name: &str) -> ProjectLocationCaPoolCertificateAuthorityFetchCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityFetchCall {
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
    pub fn locations_ca_pools_certificate_authorities_get(&self, name: &str) -> ProjectLocationCaPoolCertificateAuthorityGetCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityGetCall {
            hub: self.hub,
            _name: name.to_string(),
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
    /// * `parent` - Required. The resource name of the CaPool associated with the CertificateAuthorities, in the format `projects/*/locations/*/caPools/*`.
    pub fn locations_ca_pools_certificate_authorities_list(&self, parent: &str) -> ProjectLocationCaPoolCertificateAuthorityListCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityListCall {
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
    /// * `name` - Output only. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    pub fn locations_ca_pools_certificate_authorities_patch(&self, request: CertificateAuthority, name: &str) -> ProjectLocationCaPoolCertificateAuthorityPatchCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityPatchCall {
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
    /// Undelete a CertificateAuthority that has been deleted.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    pub fn locations_ca_pools_certificate_authorities_undelete(&self, request: UndeleteCertificateAuthorityRequest, name: &str) -> ProjectLocationCaPoolCertificateAuthorityUndeleteCall<'a, S> {
        ProjectLocationCaPoolCertificateAuthorityUndeleteCall {
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
    /// Create a new Certificate in a given Project, Location from a particular CaPool.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the CaPool associated with the Certificate, in the format `projects/*/locations/*/caPools/*`.
    pub fn locations_ca_pools_certificates_create(&self, request: Certificate, parent: &str) -> ProjectLocationCaPoolCertificateCreateCall<'a, S> {
        ProjectLocationCaPoolCertificateCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _request_id: Default::default(),
            _issuing_certificate_authority_id: Default::default(),
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
    pub fn locations_ca_pools_certificates_get(&self, name: &str) -> ProjectLocationCaPoolCertificateGetCall<'a, S> {
        ProjectLocationCaPoolCertificateGetCall {
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
    /// * `parent` - Required. The resource name of the location associated with the Certificates, in the format `projects/*/locations/*/caPools/*`.
    pub fn locations_ca_pools_certificates_list(&self, parent: &str) -> ProjectLocationCaPoolCertificateListCall<'a, S> {
        ProjectLocationCaPoolCertificateListCall {
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
    /// * `name` - Output only. The resource name for this Certificate in the format `projects/*/locations/*/caPools/*/certificates/*`.
    pub fn locations_ca_pools_certificates_patch(&self, request: Certificate, name: &str) -> ProjectLocationCaPoolCertificatePatchCall<'a, S> {
        ProjectLocationCaPoolCertificatePatchCall {
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
    /// * `name` - Required. The resource name for this Certificate in the format `projects/*/locations/*/caPools/*/certificates/*`.
    pub fn locations_ca_pools_certificates_revoke(&self, request: RevokeCertificateRequest, name: &str) -> ProjectLocationCaPoolCertificateRevokeCall<'a, S> {
        ProjectLocationCaPoolCertificateRevokeCall {
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
    /// Create a CaPool.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the location associated with the CaPool, in the format `projects/*/locations/*`.
    pub fn locations_ca_pools_create(&self, request: CaPool, parent: &str) -> ProjectLocationCaPoolCreateCall<'a, S> {
        ProjectLocationCaPoolCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _ca_pool_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a CaPool.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name for this CaPool in the format `projects/*/locations/*/caPools/*`.
    pub fn locations_ca_pools_delete(&self, name: &str) -> ProjectLocationCaPoolDeleteCall<'a, S> {
        ProjectLocationCaPoolDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// FetchCaCerts returns the current trust anchor for the CaPool. This will include CA certificate chains for all ACTIVE CertificateAuthority resources in the CaPool.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `caPool` - Required. The resource name for the CaPool in the format `projects/*/locations/*/caPools/*`.
    pub fn locations_ca_pools_fetch_ca_certs(&self, request: FetchCaCertsRequest, ca_pool: &str) -> ProjectLocationCaPoolFetchCaCertCall<'a, S> {
        ProjectLocationCaPoolFetchCaCertCall {
            hub: self.hub,
            _request: request,
            _ca_pool: ca_pool.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a CaPool.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the CaPool to get.
    pub fn locations_ca_pools_get(&self, name: &str) -> ProjectLocationCaPoolGetCall<'a, S> {
        ProjectLocationCaPoolGetCall {
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
    pub fn locations_ca_pools_get_iam_policy(&self, resource: &str) -> ProjectLocationCaPoolGetIamPolicyCall<'a, S> {
        ProjectLocationCaPoolGetIamPolicyCall {
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
    /// Lists CaPools.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the location associated with the CaPools, in the format `projects/*/locations/*`.
    pub fn locations_ca_pools_list(&self, parent: &str) -> ProjectLocationCaPoolListCall<'a, S> {
        ProjectLocationCaPoolListCall {
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
    /// Update a CaPool.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name for this CaPool in the format `projects/*/locations/*/caPools/*`.
    pub fn locations_ca_pools_patch(&self, request: CaPool, name: &str) -> ProjectLocationCaPoolPatchCall<'a, S> {
        ProjectLocationCaPoolPatchCall {
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
    pub fn locations_ca_pools_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationCaPoolSetIamPolicyCall<'a, S> {
        ProjectLocationCaPoolSetIamPolicyCall {
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
    pub fn locations_ca_pools_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationCaPoolTestIamPermissionCall<'a, S> {
        ProjectLocationCaPoolTestIamPermissionCall {
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
    /// Create a new CertificateTemplate in a given Project and Location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the location associated with the CertificateTemplate, in the format `projects/*/locations/*`.
    pub fn locations_certificate_templates_create(&self, request: CertificateTemplate, parent: &str) -> ProjectLocationCertificateTemplateCreateCall<'a, S> {
        ProjectLocationCertificateTemplateCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _certificate_template_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// DeleteCertificateTemplate deletes a CertificateTemplate.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name for this CertificateTemplate in the format `projects/*/locations/*/certificateTemplates/*`.
    pub fn locations_certificate_templates_delete(&self, name: &str) -> ProjectLocationCertificateTemplateDeleteCall<'a, S> {
        ProjectLocationCertificateTemplateDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a CertificateTemplate.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the CertificateTemplate to get.
    pub fn locations_certificate_templates_get(&self, name: &str) -> ProjectLocationCertificateTemplateGetCall<'a, S> {
        ProjectLocationCertificateTemplateGetCall {
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
    pub fn locations_certificate_templates_get_iam_policy(&self, resource: &str) -> ProjectLocationCertificateTemplateGetIamPolicyCall<'a, S> {
        ProjectLocationCertificateTemplateGetIamPolicyCall {
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
    /// Lists CertificateTemplates.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the location associated with the CertificateTemplates, in the format `projects/*/locations/*`.
    pub fn locations_certificate_templates_list(&self, parent: &str) -> ProjectLocationCertificateTemplateListCall<'a, S> {
        ProjectLocationCertificateTemplateListCall {
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
    /// Update a CertificateTemplate.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name for this CertificateTemplate in the format `projects/*/locations/*/certificateTemplates/*`.
    pub fn locations_certificate_templates_patch(&self, request: CertificateTemplate, name: &str) -> ProjectLocationCertificateTemplatePatchCall<'a, S> {
        ProjectLocationCertificateTemplatePatchCall {
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
    pub fn locations_certificate_templates_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationCertificateTemplateSetIamPolicyCall<'a, S> {
        ProjectLocationCertificateTemplateSetIamPolicyCall {
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
    pub fn locations_certificate_templates_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationCertificateTemplateTestIamPermissionCall<'a, S> {
        ProjectLocationCertificateTemplateTestIamPermissionCall {
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



