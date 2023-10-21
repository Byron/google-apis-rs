use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudKMS`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudkms1 as cloudkms1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudkms1::{CloudKMS, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudKMS::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_ekm_config_get_iam_policy(...)`, `locations_ekm_config_set_iam_policy(...)`, `locations_ekm_config_test_iam_permissions(...)`, `locations_ekm_connections_create(...)`, `locations_ekm_connections_get(...)`, `locations_ekm_connections_get_iam_policy(...)`, `locations_ekm_connections_list(...)`, `locations_ekm_connections_patch(...)`, `locations_ekm_connections_set_iam_policy(...)`, `locations_ekm_connections_test_iam_permissions(...)`, `locations_generate_random_bytes(...)`, `locations_get(...)`, `locations_key_rings_create(...)`, `locations_key_rings_crypto_keys_create(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_asymmetric_decrypt(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_asymmetric_sign(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_create(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_destroy(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_get(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_get_public_key(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_import(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_list(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_mac_sign(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_mac_verify(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_patch(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_restore(...)`, `locations_key_rings_crypto_keys_decrypt(...)`, `locations_key_rings_crypto_keys_encrypt(...)`, `locations_key_rings_crypto_keys_get(...)`, `locations_key_rings_crypto_keys_get_iam_policy(...)`, `locations_key_rings_crypto_keys_list(...)`, `locations_key_rings_crypto_keys_patch(...)`, `locations_key_rings_crypto_keys_set_iam_policy(...)`, `locations_key_rings_crypto_keys_test_iam_permissions(...)`, `locations_key_rings_crypto_keys_update_primary_version(...)`, `locations_key_rings_get(...)`, `locations_key_rings_get_iam_policy(...)`, `locations_key_rings_import_jobs_create(...)`, `locations_key_rings_import_jobs_get(...)`, `locations_key_rings_import_jobs_get_iam_policy(...)`, `locations_key_rings_import_jobs_list(...)`, `locations_key_rings_import_jobs_set_iam_policy(...)`, `locations_key_rings_import_jobs_test_iam_permissions(...)`, `locations_key_rings_list(...)`, `locations_key_rings_set_iam_policy(...)`, `locations_key_rings_test_iam_permissions(...)` and `locations_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudKMS<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_ekm_config_get_iam_policy(&self, resource: &str) -> ProjectLocationEkmConfigGetIamPolicyCall<'a, S> {
        ProjectLocationEkmConfigGetIamPolicyCall {
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
    pub fn locations_ekm_config_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationEkmConfigSetIamPolicyCall<'a, S> {
        ProjectLocationEkmConfigSetIamPolicyCall {
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
    pub fn locations_ekm_config_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationEkmConfigTestIamPermissionCall<'a, S> {
        ProjectLocationEkmConfigTestIamPermissionCall {
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
    /// Creates a new EkmConnection in a given Project and Location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the location associated with the EkmConnection, in the format `projects/*/locations/*`.
    pub fn locations_ekm_connections_create(&self, request: EkmConnection, parent: &str) -> ProjectLocationEkmConnectionCreateCall<'a, S> {
        ProjectLocationEkmConnectionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _ekm_connection_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata for a given EkmConnection.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the EkmConnection to get.
    pub fn locations_ekm_connections_get(&self, name: &str) -> ProjectLocationEkmConnectionGetCall<'a, S> {
        ProjectLocationEkmConnectionGetCall {
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
    pub fn locations_ekm_connections_get_iam_policy(&self, resource: &str) -> ProjectLocationEkmConnectionGetIamPolicyCall<'a, S> {
        ProjectLocationEkmConnectionGetIamPolicyCall {
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
    /// Lists EkmConnections.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the location associated with the EkmConnections to list, in the format `projects/*/locations/*`.
    pub fn locations_ekm_connections_list(&self, parent: &str) -> ProjectLocationEkmConnectionListCall<'a, S> {
        ProjectLocationEkmConnectionListCall {
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
    /// Updates an EkmConnection's metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name for the EkmConnection in the format `projects/*/locations/*/ekmConnections/*`.
    pub fn locations_ekm_connections_patch(&self, request: EkmConnection, name: &str) -> ProjectLocationEkmConnectionPatchCall<'a, S> {
        ProjectLocationEkmConnectionPatchCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_ekm_connections_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationEkmConnectionSetIamPolicyCall<'a, S> {
        ProjectLocationEkmConnectionSetIamPolicyCall {
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
    pub fn locations_ekm_connections_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationEkmConnectionTestIamPermissionCall<'a, S> {
        ProjectLocationEkmConnectionTestIamPermissionCall {
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
    /// Decrypts data that was encrypted with a public key retrieved from GetPublicKey corresponding to a CryptoKeyVersion with CryptoKey.purpose ASYMMETRIC_DECRYPT.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the CryptoKeyVersion to use for decryption.
    pub fn locations_key_rings_crypto_keys_crypto_key_versions_asymmetric_decrypt(&self, request: AsymmetricDecryptRequest, name: &str) -> ProjectLocationKeyRingCryptoKeyCryptoKeyVersionAsymmetricDecryptCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyCryptoKeyVersionAsymmetricDecryptCall {
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
    /// Signs data using a CryptoKeyVersion with CryptoKey.purpose ASYMMETRIC_SIGN, producing a signature that can be verified with the public key retrieved from GetPublicKey.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the CryptoKeyVersion to use for signing.
    pub fn locations_key_rings_crypto_keys_crypto_key_versions_asymmetric_sign(&self, request: AsymmetricSignRequest, name: &str) -> ProjectLocationKeyRingCryptoKeyCryptoKeyVersionAsymmetricSignCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyCryptoKeyVersionAsymmetricSignCall {
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
    /// Create a new CryptoKeyVersion in a CryptoKey. The server will assign the next sequential id. If unset, state will be set to ENABLED.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the CryptoKey associated with the CryptoKeyVersions.
    pub fn locations_key_rings_crypto_keys_crypto_key_versions_create(&self, request: CryptoKeyVersion, parent: &str) -> ProjectLocationKeyRingCryptoKeyCryptoKeyVersionCreateCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyCryptoKeyVersionCreateCall {
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
    /// Schedule a CryptoKeyVersion for destruction. Upon calling this method, CryptoKeyVersion.state will be set to DESTROY_SCHEDULED, and destroy_time will be set to the time destroy_scheduled_duration in the future. At that time, the state will automatically change to DESTROYED, and the key material will be irrevocably destroyed. Before the destroy_time is reached, RestoreCryptoKeyVersion may be called to reverse the process.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the CryptoKeyVersion to destroy.
    pub fn locations_key_rings_crypto_keys_crypto_key_versions_destroy(&self, request: DestroyCryptoKeyVersionRequest, name: &str) -> ProjectLocationKeyRingCryptoKeyCryptoKeyVersionDestroyCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyCryptoKeyVersionDestroyCall {
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
    /// Returns metadata for a given CryptoKeyVersion.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the CryptoKeyVersion to get.
    pub fn locations_key_rings_crypto_keys_crypto_key_versions_get(&self, name: &str) -> ProjectLocationKeyRingCryptoKeyCryptoKeyVersionGetCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyCryptoKeyVersionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the public key for the given CryptoKeyVersion. The CryptoKey.purpose must be ASYMMETRIC_SIGN or ASYMMETRIC_DECRYPT.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the CryptoKeyVersion public key to get.
    pub fn locations_key_rings_crypto_keys_crypto_key_versions_get_public_key(&self, name: &str) -> ProjectLocationKeyRingCryptoKeyCryptoKeyVersionGetPublicKeyCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyCryptoKeyVersionGetPublicKeyCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Import wrapped key material into a CryptoKeyVersion. All requests must specify a CryptoKey. If a CryptoKeyVersion is additionally specified in the request, key material will be reimported into that version. Otherwise, a new version will be created, and will be assigned the next sequential id within the CryptoKey.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the CryptoKey to be imported into. The create permission is only required on this key when creating a new CryptoKeyVersion.
    pub fn locations_key_rings_crypto_keys_crypto_key_versions_import(&self, request: ImportCryptoKeyVersionRequest, parent: &str) -> ProjectLocationKeyRingCryptoKeyCryptoKeyVersionImportCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyCryptoKeyVersionImportCall {
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
    /// Lists CryptoKeyVersions.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the CryptoKey to list, in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    pub fn locations_key_rings_crypto_keys_crypto_key_versions_list(&self, parent: &str) -> ProjectLocationKeyRingCryptoKeyCryptoKeyVersionListCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyCryptoKeyVersionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _view: Default::default(),
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
    /// Signs data using a CryptoKeyVersion with CryptoKey.purpose MAC, producing a tag that can be verified by another source with the same key.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the CryptoKeyVersion to use for signing.
    pub fn locations_key_rings_crypto_keys_crypto_key_versions_mac_sign(&self, request: MacSignRequest, name: &str) -> ProjectLocationKeyRingCryptoKeyCryptoKeyVersionMacSignCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyCryptoKeyVersionMacSignCall {
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
    /// Verifies MAC tag using a CryptoKeyVersion with CryptoKey.purpose MAC, and returns a response that indicates whether or not the verification was successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the CryptoKeyVersion to use for verification.
    pub fn locations_key_rings_crypto_keys_crypto_key_versions_mac_verify(&self, request: MacVerifyRequest, name: &str) -> ProjectLocationKeyRingCryptoKeyCryptoKeyVersionMacVerifyCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyCryptoKeyVersionMacVerifyCall {
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
    /// Update a CryptoKeyVersion's metadata. state may be changed between ENABLED and DISABLED using this method. See DestroyCryptoKeyVersion and RestoreCryptoKeyVersion to move between other states.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name for this CryptoKeyVersion in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`.
    pub fn locations_key_rings_crypto_keys_crypto_key_versions_patch(&self, request: CryptoKeyVersion, name: &str) -> ProjectLocationKeyRingCryptoKeyCryptoKeyVersionPatchCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyCryptoKeyVersionPatchCall {
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
    /// Restore a CryptoKeyVersion in the DESTROY_SCHEDULED state. Upon restoration of the CryptoKeyVersion, state will be set to DISABLED, and destroy_time will be cleared.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the CryptoKeyVersion to restore.
    pub fn locations_key_rings_crypto_keys_crypto_key_versions_restore(&self, request: RestoreCryptoKeyVersionRequest, name: &str) -> ProjectLocationKeyRingCryptoKeyCryptoKeyVersionRestoreCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyCryptoKeyVersionRestoreCall {
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
    /// Create a new CryptoKey within a KeyRing. CryptoKey.purpose and CryptoKey.version_template.algorithm are required.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the KeyRing associated with the CryptoKeys.
    pub fn locations_key_rings_crypto_keys_create(&self, request: CryptoKey, parent: &str) -> ProjectLocationKeyRingCryptoKeyCreateCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _skip_initial_version_creation: Default::default(),
            _crypto_key_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Decrypts data that was protected by Encrypt. The CryptoKey.purpose must be ENCRYPT_DECRYPT.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the CryptoKey to use for decryption. The server will choose the appropriate version.
    pub fn locations_key_rings_crypto_keys_decrypt(&self, request: DecryptRequest, name: &str) -> ProjectLocationKeyRingCryptoKeyDecryptCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyDecryptCall {
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
    /// Encrypts data, so that it can only be recovered by a call to Decrypt. The CryptoKey.purpose must be ENCRYPT_DECRYPT.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the CryptoKey or CryptoKeyVersion to use for encryption. If a CryptoKey is specified, the server will use its primary version.
    pub fn locations_key_rings_crypto_keys_encrypt(&self, request: EncryptRequest, name: &str) -> ProjectLocationKeyRingCryptoKeyEncryptCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyEncryptCall {
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
    /// Returns metadata for a given CryptoKey, as well as its primary CryptoKeyVersion.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the CryptoKey to get.
    pub fn locations_key_rings_crypto_keys_get(&self, name: &str) -> ProjectLocationKeyRingCryptoKeyGetCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyGetCall {
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
    pub fn locations_key_rings_crypto_keys_get_iam_policy(&self, resource: &str) -> ProjectLocationKeyRingCryptoKeyGetIamPolicyCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyGetIamPolicyCall {
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
    /// Lists CryptoKeys.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the KeyRing to list, in the format `projects/*/locations/*/keyRings/*`.
    pub fn locations_key_rings_crypto_keys_list(&self, parent: &str) -> ProjectLocationKeyRingCryptoKeyListCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _version_view: Default::default(),
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
    /// Update a CryptoKey.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name for this CryptoKey in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    pub fn locations_key_rings_crypto_keys_patch(&self, request: CryptoKey, name: &str) -> ProjectLocationKeyRingCryptoKeyPatchCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyPatchCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_key_rings_crypto_keys_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationKeyRingCryptoKeySetIamPolicyCall<'a, S> {
        ProjectLocationKeyRingCryptoKeySetIamPolicyCall {
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
    pub fn locations_key_rings_crypto_keys_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationKeyRingCryptoKeyTestIamPermissionCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyTestIamPermissionCall {
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
    /// Update the version of a CryptoKey that will be used in Encrypt. Returns an error if called on a key whose purpose is not ENCRYPT_DECRYPT.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the CryptoKey to update.
    pub fn locations_key_rings_crypto_keys_update_primary_version(&self, request: UpdateCryptoKeyPrimaryVersionRequest, name: &str) -> ProjectLocationKeyRingCryptoKeyUpdatePrimaryVersionCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyUpdatePrimaryVersionCall {
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
    /// Create a new ImportJob within a KeyRing. ImportJob.import_method is required.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the KeyRing associated with the ImportJobs.
    pub fn locations_key_rings_import_jobs_create(&self, request: ImportJob, parent: &str) -> ProjectLocationKeyRingImportJobCreateCall<'a, S> {
        ProjectLocationKeyRingImportJobCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _import_job_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata for a given ImportJob.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the ImportJob to get.
    pub fn locations_key_rings_import_jobs_get(&self, name: &str) -> ProjectLocationKeyRingImportJobGetCall<'a, S> {
        ProjectLocationKeyRingImportJobGetCall {
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
    pub fn locations_key_rings_import_jobs_get_iam_policy(&self, resource: &str) -> ProjectLocationKeyRingImportJobGetIamPolicyCall<'a, S> {
        ProjectLocationKeyRingImportJobGetIamPolicyCall {
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
    /// Lists ImportJobs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the KeyRing to list, in the format `projects/*/locations/*/keyRings/*`.
    pub fn locations_key_rings_import_jobs_list(&self, parent: &str) -> ProjectLocationKeyRingImportJobListCall<'a, S> {
        ProjectLocationKeyRingImportJobListCall {
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
    pub fn locations_key_rings_import_jobs_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationKeyRingImportJobSetIamPolicyCall<'a, S> {
        ProjectLocationKeyRingImportJobSetIamPolicyCall {
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
    pub fn locations_key_rings_import_jobs_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationKeyRingImportJobTestIamPermissionCall<'a, S> {
        ProjectLocationKeyRingImportJobTestIamPermissionCall {
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
    /// Create a new KeyRing in a given Project and Location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the location associated with the KeyRings, in the format `projects/*/locations/*`.
    pub fn locations_key_rings_create(&self, request: KeyRing, parent: &str) -> ProjectLocationKeyRingCreateCall<'a, S> {
        ProjectLocationKeyRingCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _key_ring_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata for a given KeyRing.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the KeyRing to get.
    pub fn locations_key_rings_get(&self, name: &str) -> ProjectLocationKeyRingGetCall<'a, S> {
        ProjectLocationKeyRingGetCall {
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
    pub fn locations_key_rings_get_iam_policy(&self, resource: &str) -> ProjectLocationKeyRingGetIamPolicyCall<'a, S> {
        ProjectLocationKeyRingGetIamPolicyCall {
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
    /// Lists KeyRings.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the location associated with the KeyRings, in the format `projects/*/locations/*`.
    pub fn locations_key_rings_list(&self, parent: &str) -> ProjectLocationKeyRingListCall<'a, S> {
        ProjectLocationKeyRingListCall {
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
    pub fn locations_key_rings_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationKeyRingSetIamPolicyCall<'a, S> {
        ProjectLocationKeyRingSetIamPolicyCall {
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
    pub fn locations_key_rings_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationKeyRingTestIamPermissionCall<'a, S> {
        ProjectLocationKeyRingTestIamPermissionCall {
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
    /// Generate random bytes using the Cloud KMS randomness source in the provided location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `location` - The project-specific location in which to generate random bytes. For example, "projects/my-project/locations/us-central1".
    pub fn locations_generate_random_bytes(&self, request: GenerateRandomBytesRequest, location: &str) -> ProjectLocationGenerateRandomByteCall<'a, S> {
        ProjectLocationGenerateRandomByteCall {
            hub: self.hub,
            _request: request,
            _location: location.to_string(),
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



