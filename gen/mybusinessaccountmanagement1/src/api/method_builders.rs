use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`MyBusinessAccountManagement`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessaccountmanagement1 as mybusinessaccountmanagement1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusinessaccountmanagement1::{MyBusinessAccountManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusinessAccountManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `admins_create(...)`, `admins_delete(...)`, `admins_list(...)`, `admins_patch(...)`, `create(...)`, `get(...)`, `invitations_accept(...)`, `invitations_decline(...)`, `invitations_list(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusinessAccountManagement<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Invites the specified user to become an administrator for the specified account. The invitee must accept the invitation in order to be granted access to the account. See AcceptInvitation to programmatically accept an invitation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the account this admin is created for. `accounts/{account_id}`.
    pub fn admins_create(&self, request: Admin, parent: &str) -> AccountAdminCreateCall<'a, S> {
        AccountAdminCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the specified admin from the specified account.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the admin to remove from the account. `accounts/{account_id}/admins/{admin_id}`.
    pub fn admins_delete(&self, name: &str) -> AccountAdminDeleteCall<'a, S> {
        AccountAdminDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the admins for the specified account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the account from which to retrieve a list of admins. `accounts/{account_id}/admins`.
    pub fn admins_list(&self, parent: &str) -> AccountAdminListCall<'a, S> {
        AccountAdminListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Admin for the specified Account Admin.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name. For account admins, this is in the form: `accounts/{account_id}/admins/{admin_id}` For location admins, this is in the form: `locations/{location_id}/admins/{admin_id}` This field will be ignored if set during admin creation.
    pub fn admins_patch(&self, request: Admin, name: &str) -> AccountAdminPatchCall<'a, S> {
        AccountAdminPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Accepts the specified invitation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the invitation that is being accepted. `accounts/{account_id}/invitations/{invitation_id}`
    pub fn invitations_accept(&self, request: AcceptInvitationRequest, name: &str) -> AccountInvitationAcceptCall<'a, S> {
        AccountInvitationAcceptCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Declines the specified invitation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the account invitation that is being declined. `accounts/{account_id}/invitations/{invitation_id}`
    pub fn invitations_decline(&self, request: DeclineInvitationRequest, name: &str) -> AccountInvitationDeclineCall<'a, S> {
        AccountInvitationDeclineCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists pending invitations for the specified account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the account from which the list of invitations is being retrieved. `accounts/{account_id}/invitations`
    pub fn invitations_list(&self, parent: &str) -> AccountInvitationListCall<'a, S> {
        AccountInvitationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an account with the specified name and type under the given parent. - Personal accounts and Organizations cannot be created. - User Groups cannot be created with a Personal account as primary owner. - Location Groups cannot be created with a primary owner of a Personal account if the Personal account is in an Organization. - Location Groups cannot own Location Groups.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Account) -> AccountCreateCall<'a, S> {
        AccountCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified account. Returns `NOT_FOUND` if the account does not exist or if the caller does not have access rights to it.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the account to fetch.
    pub fn get(&self, name: &str) -> AccountGetCall<'a, S> {
        AccountGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all of the accounts for the authenticated user. This includes all accounts that the user owns, as well as any accounts for which the user has management rights.
    pub fn list(&self) -> AccountListCall<'a, S> {
        AccountListCall {
            hub: self.hub,
            _parent_account: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified business account. Personal accounts cannot be updated using this method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name, in the format `accounts/{account_id}`.
    pub fn patch(&self, request: Account, name: &str) -> AccountPatchCall<'a, S> {
        AccountPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *location* resources.
/// It is not used directly, but through the [`MyBusinessAccountManagement`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessaccountmanagement1 as mybusinessaccountmanagement1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusinessaccountmanagement1::{MyBusinessAccountManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusinessAccountManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `admins_create(...)`, `admins_delete(...)`, `admins_list(...)`, `admins_patch(...)` and `transfer(...)`
/// // to build up your call.
/// let rb = hub.locations();
/// # }
/// ```
pub struct LocationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusinessAccountManagement<S>,
}

impl<'a, S> client::MethodsBuilder for LocationMethods<'a, S> {}

impl<'a, S> LocationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Invites the specified user to become an administrator for the specified location. The invitee must accept the invitation in order to be granted access to the location. See AcceptInvitation to programmatically accept an invitation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the location this admin is created for. `locations/{location_id}/admins`.
    pub fn admins_create(&self, request: Admin, parent: &str) -> LocationAdminCreateCall<'a, S> {
        LocationAdminCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the specified admin as a manager of the specified location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the admin to remove from the location.
    pub fn admins_delete(&self, name: &str) -> LocationAdminDeleteCall<'a, S> {
        LocationAdminDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all of the admins for the specified location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the location to list admins of. `locations/{location_id}/admins`.
    pub fn admins_list(&self, parent: &str) -> LocationAdminListCall<'a, S> {
        LocationAdminListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Admin for the specified location. Only the AdminRole of the Admin can be updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name. For account admins, this is in the form: `accounts/{account_id}/admins/{admin_id}` For location admins, this is in the form: `locations/{location_id}/admins/{admin_id}` This field will be ignored if set during admin creation.
    pub fn admins_patch(&self, request: Admin, name: &str) -> LocationAdminPatchCall<'a, S> {
        LocationAdminPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves a location from an account that the user owns to another account that the same user administers. The user must be an owner of the account the location is currently associated with and must also be at least a manager of the destination account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the location to transfer. `locations/{location_id}`.
    pub fn transfer(&self, request: TransferLocationRequest, name: &str) -> LocationTransferCall<'a, S> {
        LocationTransferCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



