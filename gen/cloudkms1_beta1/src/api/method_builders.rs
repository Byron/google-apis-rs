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
/// extern crate google_cloudkms1_beta1 as cloudkms1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudkms1_beta1::{CloudKMS, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudKMS::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_get(...)`, `locations_key_rings_create(...)`, `locations_key_rings_crypto_keys_create(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_create(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_destroy(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_get(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_list(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_patch(...)`, `locations_key_rings_crypto_keys_crypto_key_versions_restore(...)`, `locations_key_rings_crypto_keys_decrypt(...)`, `locations_key_rings_crypto_keys_encrypt(...)`, `locations_key_rings_crypto_keys_get(...)`, `locations_key_rings_crypto_keys_get_iam_policy(...)`, `locations_key_rings_crypto_keys_list(...)`, `locations_key_rings_crypto_keys_patch(...)`, `locations_key_rings_crypto_keys_set_iam_policy(...)`, `locations_key_rings_crypto_keys_test_iam_permissions(...)`, `locations_key_rings_crypto_keys_update_primary_version(...)`, `locations_key_rings_get(...)`, `locations_key_rings_get_iam_policy(...)`, `locations_key_rings_list(...)`, `locations_key_rings_set_iam_policy(...)`, `locations_key_rings_test_iam_permissions(...)` and `locations_list(...)`
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
    /// Lists CryptoKeyVersions.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the CryptoKey to list, in the format
    ///              `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    pub fn locations_key_rings_crypto_keys_crypto_key_versions_list(&self, parent: &str) -> ProjectLocationKeyRingCryptoKeyCryptoKeyVersionListCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyCryptoKeyVersionListCall {
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
    /// Create a new CryptoKeyVersion in a CryptoKey.
    /// 
    /// The server will assign the next sequential id. If unset,
    /// state will be set to
    /// ENABLED.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the CryptoKey associated with
    ///              the CryptoKeyVersions.
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
    /// Schedule a CryptoKeyVersion for destruction.
    /// 
    /// Upon calling this method, CryptoKeyVersion.state will be set to
    /// DESTROY_SCHEDULED
    /// and destroy_time will be set to a time 24
    /// hours in the future, at which point the state
    /// will be changed to
    /// DESTROYED, and the key
    /// material will be irrevocably destroyed.
    /// 
    /// Before the destroy_time is reached,
    /// RestoreCryptoKeyVersion may be called to reverse the process.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the CryptoKeyVersion to destroy.
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
    /// Restore a CryptoKeyVersion in the
    /// DESTROY_SCHEDULED,
    /// state.
    /// 
    /// Upon restoration of the CryptoKeyVersion, state
    /// will be set to DISABLED,
    /// and destroy_time will be cleared.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the CryptoKeyVersion to restore.
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
    /// Update a CryptoKeyVersion's metadata.
    /// 
    /// state may be changed between
    /// ENABLED and
    /// DISABLED using this
    /// method. See DestroyCryptoKeyVersion and RestoreCryptoKeyVersion to
    /// move between other states.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name for this CryptoKeyVersion in the format
    ///            `projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`.
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
    /// Returns metadata for a given CryptoKeyVersion.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the CryptoKeyVersion to get.
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
    /// Update a CryptoKey.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name for this CryptoKey in the format
    ///            `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
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
    /// Returns metadata for a given CryptoKey, as well as its
    /// primary CryptoKeyVersion.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the CryptoKey to get.
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
    /// Returns permissions that a caller has on the specified resource.
    /// If the resource does not exist, this will return an empty set of
    /// permissions, not a NOT_FOUND error.
    /// 
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested.
    ///                See the operation documentation for the appropriate value for this field.
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
    /// Decrypt data that was protected by Encrypt.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the CryptoKey to use for decryption.
    ///            The server will choose the appropriate version.
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
    /// Lists CryptoKeys.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the KeyRing to list, in the format
    ///              `projects/*/locations/*/keyRings/*`.
    pub fn locations_key_rings_crypto_keys_list(&self, parent: &str) -> ProjectLocationKeyRingCryptoKeyListCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyListCall {
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
    /// Encrypt data, so that it can only be recovered by a call to Decrypt.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the CryptoKey or CryptoKeyVersion
    ///            to use for encryption.
    ///            If a CryptoKey is specified, the server will use its
    ///            primary version.
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
    /// Create a new CryptoKey within a KeyRing.
    /// 
    /// CryptoKey.purpose is required.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the KeyRing associated with the
    ///              CryptoKeys.
    pub fn locations_key_rings_crypto_keys_create(&self, request: CryptoKey, parent: &str) -> ProjectLocationKeyRingCryptoKeyCreateCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _crypto_key_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any
    /// existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified.
    ///                See the operation documentation for the appropriate value for this field.
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
    /// Update the version of a CryptoKey that will be used in Encrypt
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the CryptoKey to update.
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
    /// Gets the access control policy for a resource.
    /// Returns an empty policy if the resource exists and does not have a policy
    /// set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested.
    ///                See the operation documentation for the appropriate value for this field.
    pub fn locations_key_rings_crypto_keys_get_iam_policy(&self, resource: &str) -> ProjectLocationKeyRingCryptoKeyGetIamPolicyCall<'a, S> {
        ProjectLocationKeyRingCryptoKeyGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource.
    /// Returns an empty policy if the resource exists and does not have a policy
    /// set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested.
    ///                See the operation documentation for the appropriate value for this field.
    pub fn locations_key_rings_get_iam_policy(&self, resource: &str) -> ProjectLocationKeyRingGetIamPolicyCall<'a, S> {
        ProjectLocationKeyRingGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
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
    /// * `name` - The name of the KeyRing to get.
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
    /// Returns permissions that a caller has on the specified resource.
    /// If the resource does not exist, this will return an empty set of
    /// permissions, not a NOT_FOUND error.
    /// 
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested.
    ///                See the operation documentation for the appropriate value for this field.
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
    /// Lists KeyRings.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the location associated with the
    ///              KeyRings, in the format `projects/*/locations/*`.
    pub fn locations_key_rings_list(&self, parent: &str) -> ProjectLocationKeyRingListCall<'a, S> {
        ProjectLocationKeyRingListCall {
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
    /// Sets the access control policy on the specified resource. Replaces any
    /// existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified.
    ///                See the operation documentation for the appropriate value for this field.
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
    /// Create a new KeyRing in a given Project and Location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the location associated with the
    ///              KeyRings, in the format `projects/*/locations/*`.
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get information about a location.
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
}



