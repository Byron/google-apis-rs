use super::*;
/// A builder providing access to all methods supported on *user* resources.
/// It is not used directly, but through the [`CloudOSLogin`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_oslogin1_beta as oslogin1_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oslogin1_beta::{CloudOSLogin, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudOSLogin::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_login_profile(...)`, `import_ssh_public_key(...)`, `projects_delete(...)`, `ssh_public_keys_create(...)`, `ssh_public_keys_delete(...)`, `ssh_public_keys_get(...)` and `ssh_public_keys_patch(...)`
/// // to build up your call.
/// let rb = hub.users();
/// # }
/// ```
pub struct UserMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudOSLogin<S>,
}

impl<'a, S> client::MethodsBuilder for UserMethods<'a, S> {}

impl<'a, S> UserMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a POSIX account.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A reference to the POSIX account to update. POSIX accounts are identified by the project ID they are associated with. A reference to the POSIX account is in format `users/{user}/projects/{project}`.
    pub fn projects_delete(&self, name: &str) -> UserProjectDeleteCall<'a, S> {
        UserProjectDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create an SSH public key
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The unique ID for the user in format `users/{user}`.
    pub fn ssh_public_keys_create(&self, request: SshPublicKey, parent: &str) -> UserSshPublicKeyCreateCall<'a, S> {
        UserSshPublicKeyCreateCall {
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
    /// Deletes an SSH public key.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fingerprint of the public key to update. Public keys are identified by their SHA-256 fingerprint. The fingerprint of the public key is in format `users/{user}/sshPublicKeys/{fingerprint}`.
    pub fn ssh_public_keys_delete(&self, name: &str) -> UserSshPublicKeyDeleteCall<'a, S> {
        UserSshPublicKeyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an SSH public key.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fingerprint of the public key to retrieve. Public keys are identified by their SHA-256 fingerprint. The fingerprint of the public key is in format `users/{user}/sshPublicKeys/{fingerprint}`.
    pub fn ssh_public_keys_get(&self, name: &str) -> UserSshPublicKeyGetCall<'a, S> {
        UserSshPublicKeyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an SSH public key and returns the profile information. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The fingerprint of the public key to update. Public keys are identified by their SHA-256 fingerprint. The fingerprint of the public key is in format `users/{user}/sshPublicKeys/{fingerprint}`.
    pub fn ssh_public_keys_patch(&self, request: SshPublicKey, name: &str) -> UserSshPublicKeyPatchCall<'a, S> {
        UserSshPublicKeyPatchCall {
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
    /// Retrieves the profile information used for logging in to a virtual machine on Google Compute Engine.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The unique ID for the user in format `users/{user}`.
    pub fn get_login_profile(&self, name: &str) -> UserGetLoginProfileCall<'a, S> {
        UserGetLoginProfileCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _system_id: Default::default(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds an SSH public key and returns the profile information. Default POSIX account information is set when no username and UID exist as part of the login profile.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The unique ID for the user in format `users/{user}`.
    pub fn import_ssh_public_key(&self, request: SshPublicKey, parent: &str) -> UserImportSshPublicKeyCall<'a, S> {
        UserImportSshPublicKeyCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _view: Default::default(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



