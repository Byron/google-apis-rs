use super::*;
/// A builder providing access to all methods supported on *globalAccountsOperation* resources.
/// It is not used directly, but through the [`CloudUserAccounts`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_clouduseraccountsvm_beta as clouduseraccountsvm_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use clouduseraccountsvm_beta::{CloudUserAccounts, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudUserAccounts::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.global_accounts_operations();
/// # }
/// ```
pub struct GlobalAccountsOperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudUserAccounts<S>,
}

impl<'a, S> client::MethodsBuilder for GlobalAccountsOperationMethods<'a, S> {}

impl<'a, S> GlobalAccountsOperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified operation resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `operation` - Name of the Operations resource to delete.
    pub fn delete(&self, project: &str, operation: &str) -> GlobalAccountsOperationDeleteCall<'a, S> {
        GlobalAccountsOperationDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified operation resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `operation` - Name of the Operations resource to return.
    pub fn get(&self, project: &str, operation: &str) -> GlobalAccountsOperationGetCall<'a, S> {
        GlobalAccountsOperationGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of operation resources contained within the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> GlobalAccountsOperationListCall<'a, S> {
        GlobalAccountsOperationListCall {
            hub: self.hub,
            _project: project.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *group* resources.
/// It is not used directly, but through the [`CloudUserAccounts`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_clouduseraccountsvm_beta as clouduseraccountsvm_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use clouduseraccountsvm_beta::{CloudUserAccounts, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudUserAccounts::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_member(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `remove_member(...)`
/// // to build up your call.
/// let rb = hub.groups();
/// # }
/// ```
pub struct GroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudUserAccounts<S>,
}

impl<'a, S> client::MethodsBuilder for GroupMethods<'a, S> {}

impl<'a, S> GroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds users to the specified group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `groupName` - Name of the group for this request.
    pub fn add_member(&self, request: GroupsAddMemberRequest, project: &str, group_name: &str) -> GroupAddMemberCall<'a, S> {
        GroupAddMemberCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _group_name: group_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified Group resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `groupName` - Name of the Group resource to delete.
    pub fn delete(&self, project: &str, group_name: &str) -> GroupDeleteCall<'a, S> {
        GroupDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _group_name: group_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified Group resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `groupName` - Name of the Group resource to return.
    pub fn get(&self, project: &str, group_name: &str) -> GroupGetCall<'a, S> {
        GroupGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _group_name: group_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Group resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: Group, project: &str) -> GroupInsertCall<'a, S> {
        GroupInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of groups contained within the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> GroupListCall<'a, S> {
        GroupListCall {
            hub: self.hub,
            _project: project.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes users from the specified group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `groupName` - Name of the group for this request.
    pub fn remove_member(&self, request: GroupsRemoveMemberRequest, project: &str, group_name: &str) -> GroupRemoveMemberCall<'a, S> {
        GroupRemoveMemberCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _group_name: group_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *linux* resources.
/// It is not used directly, but through the [`CloudUserAccounts`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_clouduseraccountsvm_beta as clouduseraccountsvm_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use clouduseraccountsvm_beta::{CloudUserAccounts, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudUserAccounts::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_authorized_keys_view(...)` and `get_linux_account_views(...)`
/// // to build up your call.
/// let rb = hub.linux();
/// # }
/// ```
pub struct LinuxMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudUserAccounts<S>,
}

impl<'a, S> client::MethodsBuilder for LinuxMethods<'a, S> {}

impl<'a, S> LinuxMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of authorized public keys for a specific user account.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    /// * `user` - The user account for which you want to get a list of authorized public keys.
    /// * `instance` - The fully-qualified URL of the virtual machine requesting the view.
    pub fn get_authorized_keys_view(&self, project: &str, zone: &str, user: &str, instance: &str) -> LinuxGetAuthorizedKeysViewCall<'a, S> {
        LinuxGetAuthorizedKeysViewCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _user: user.to_string(),
            _instance: instance.to_string(),
            _login: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of user accounts for an instance within a specific project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    /// * `instance` - The fully-qualified URL of the virtual machine requesting the views.
    pub fn get_linux_account_views(&self, project: &str, zone: &str, instance: &str) -> LinuxGetLinuxAccountViewCall<'a, S> {
        LinuxGetLinuxAccountViewCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *user* resources.
/// It is not used directly, but through the [`CloudUserAccounts`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_clouduseraccountsvm_beta as clouduseraccountsvm_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use clouduseraccountsvm_beta::{CloudUserAccounts, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudUserAccounts::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_public_key(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `remove_public_key(...)`
/// // to build up your call.
/// let rb = hub.users();
/// # }
/// ```
pub struct UserMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudUserAccounts<S>,
}

impl<'a, S> client::MethodsBuilder for UserMethods<'a, S> {}

impl<'a, S> UserMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a public key to the specified User resource with the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `user` - Name of the user for this request.
    pub fn add_public_key(&self, request: PublicKey, project: &str, user: &str) -> UserAddPublicKeyCall<'a, S> {
        UserAddPublicKeyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _user: user.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified User resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `user` - Name of the user resource to delete.
    pub fn delete(&self, project: &str, user: &str) -> UserDeleteCall<'a, S> {
        UserDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _user: user.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified User resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `user` - Name of the user resource to return.
    pub fn get(&self, project: &str, user: &str) -> UserGetCall<'a, S> {
        UserGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _user: user.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a User resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: User, project: &str) -> UserInsertCall<'a, S> {
        UserInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of users contained within the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> UserListCall<'a, S> {
        UserListCall {
            hub: self.hub,
            _project: project.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the specified public key from the user.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `user` - Name of the user for this request.
    /// * `fingerprint` - The fingerprint of the public key to delete. Public keys are identified by their fingerprint, which is defined by RFC4716 to be the MD5 digest of the public key.
    pub fn remove_public_key(&self, project: &str, user: &str, fingerprint: &str) -> UserRemovePublicKeyCall<'a, S> {
        UserRemovePublicKeyCall {
            hub: self.hub,
            _project: project.to_string(),
            _user: user.to_string(),
            _fingerprint: fingerprint.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



