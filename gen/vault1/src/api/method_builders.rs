use super::*;
/// A builder providing access to all methods supported on *matter* resources.
/// It is not used directly, but through the [`Vault`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_vault1 as vault1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use vault1::{Vault, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Vault::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_permissions(...)`, `close(...)`, `count(...)`, `create(...)`, `delete(...)`, `exports_create(...)`, `exports_delete(...)`, `exports_get(...)`, `exports_list(...)`, `get(...)`, `holds_accounts_create(...)`, `holds_accounts_delete(...)`, `holds_accounts_list(...)`, `holds_add_held_accounts(...)`, `holds_create(...)`, `holds_delete(...)`, `holds_get(...)`, `holds_list(...)`, `holds_remove_held_accounts(...)`, `holds_update(...)`, `list(...)`, `remove_permissions(...)`, `reopen(...)`, `saved_queries_create(...)`, `saved_queries_delete(...)`, `saved_queries_get(...)`, `saved_queries_list(...)`, `undelete(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.matters();
/// # }
/// ```
pub struct MatterMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Vault<S>,
}

impl<'a, S> client::MethodsBuilder for MatterMethods<'a, S> {}

impl<'a, S> MatterMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an export.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `matterId` - The matter ID.
    pub fn exports_create(&self, request: Export, matter_id: &str) -> MatterExportCreateCall<'a, S> {
        MatterExportCreateCall {
            hub: self.hub,
            _request: request,
            _matter_id: matter_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an export.
    /// 
    /// # Arguments
    ///
    /// * `matterId` - The matter ID.
    /// * `exportId` - The export ID.
    pub fn exports_delete(&self, matter_id: &str, export_id: &str) -> MatterExportDeleteCall<'a, S> {
        MatterExportDeleteCall {
            hub: self.hub,
            _matter_id: matter_id.to_string(),
            _export_id: export_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an export.
    /// 
    /// # Arguments
    ///
    /// * `matterId` - The matter ID.
    /// * `exportId` - The export ID.
    pub fn exports_get(&self, matter_id: &str, export_id: &str) -> MatterExportGetCall<'a, S> {
        MatterExportGetCall {
            hub: self.hub,
            _matter_id: matter_id.to_string(),
            _export_id: export_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists details about the exports in the specified matter.
    /// 
    /// # Arguments
    ///
    /// * `matterId` - The matter ID.
    pub fn exports_list(&self, matter_id: &str) -> MatterExportListCall<'a, S> {
        MatterExportListCall {
            hub: self.hub,
            _matter_id: matter_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds an account to a hold. Accounts can be added only to a hold that does not have an organizational unit set. If you try to add an account to an organizational unit-based hold, an error is returned.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `matterId` - The matter ID.
    /// * `holdId` - The hold ID.
    pub fn holds_accounts_create(&self, request: HeldAccount, matter_id: &str, hold_id: &str) -> MatterHoldAccountCreateCall<'a, S> {
        MatterHoldAccountCreateCall {
            hub: self.hub,
            _request: request,
            _matter_id: matter_id.to_string(),
            _hold_id: hold_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes an account from a hold.
    /// 
    /// # Arguments
    ///
    /// * `matterId` - The matter ID.
    /// * `holdId` - The hold ID.
    /// * `accountId` - The ID of the account to remove from the hold.
    pub fn holds_accounts_delete(&self, matter_id: &str, hold_id: &str, account_id: &str) -> MatterHoldAccountDeleteCall<'a, S> {
        MatterHoldAccountDeleteCall {
            hub: self.hub,
            _matter_id: matter_id.to_string(),
            _hold_id: hold_id.to_string(),
            _account_id: account_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the accounts covered by a hold. This can list only individually-specified accounts covered by the hold. If the hold covers an organizational unit, use the [Admin SDK](https://developers.google.com/admin-sdk/). to list the members of the organizational unit on hold.
    /// 
    /// # Arguments
    ///
    /// * `matterId` - The matter ID.
    /// * `holdId` - The hold ID.
    pub fn holds_accounts_list(&self, matter_id: &str, hold_id: &str) -> MatterHoldAccountListCall<'a, S> {
        MatterHoldAccountListCall {
            hub: self.hub,
            _matter_id: matter_id.to_string(),
            _hold_id: hold_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds accounts to a hold. Returns a list of accounts that have been successfully added. Accounts can be added only to an existing account-based hold.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `matterId` - The matter ID.
    /// * `holdId` - The hold ID.
    pub fn holds_add_held_accounts(&self, request: AddHeldAccountsRequest, matter_id: &str, hold_id: &str) -> MatterHoldAddHeldAccountCall<'a, S> {
        MatterHoldAddHeldAccountCall {
            hub: self.hub,
            _request: request,
            _matter_id: matter_id.to_string(),
            _hold_id: hold_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a hold in the specified matter.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `matterId` - The matter ID.
    pub fn holds_create(&self, request: Hold, matter_id: &str) -> MatterHoldCreateCall<'a, S> {
        MatterHoldCreateCall {
            hub: self.hub,
            _request: request,
            _matter_id: matter_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the specified hold and releases the accounts or organizational unit covered by the hold. If the data is not preserved by another hold or retention rule, it might be purged.
    /// 
    /// # Arguments
    ///
    /// * `matterId` - The matter ID.
    /// * `holdId` - The hold ID.
    pub fn holds_delete(&self, matter_id: &str, hold_id: &str) -> MatterHoldDeleteCall<'a, S> {
        MatterHoldDeleteCall {
            hub: self.hub,
            _matter_id: matter_id.to_string(),
            _hold_id: hold_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified hold.
    /// 
    /// # Arguments
    ///
    /// * `matterId` - The matter ID.
    /// * `holdId` - The hold ID.
    pub fn holds_get(&self, matter_id: &str, hold_id: &str) -> MatterHoldGetCall<'a, S> {
        MatterHoldGetCall {
            hub: self.hub,
            _matter_id: matter_id.to_string(),
            _hold_id: hold_id.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the holds in a matter.
    /// 
    /// # Arguments
    ///
    /// * `matterId` - The matter ID.
    pub fn holds_list(&self, matter_id: &str) -> MatterHoldListCall<'a, S> {
        MatterHoldListCall {
            hub: self.hub,
            _matter_id: matter_id.to_string(),
            _view: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the specified accounts from a hold. Returns a list of statuses in the same order as the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `matterId` - The matter ID.
    /// * `holdId` - The hold ID.
    pub fn holds_remove_held_accounts(&self, request: RemoveHeldAccountsRequest, matter_id: &str, hold_id: &str) -> MatterHoldRemoveHeldAccountCall<'a, S> {
        MatterHoldRemoveHeldAccountCall {
            hub: self.hub,
            _request: request,
            _matter_id: matter_id.to_string(),
            _hold_id: hold_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the scope (organizational unit or accounts) and query parameters of a hold. You cannot add accounts to a hold that covers an organizational unit, nor can you add organizational units to a hold that covers individual accounts. If you try, the unsupported values are ignored.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `matterId` - The matter ID.
    /// * `holdId` - The ID of the hold.
    pub fn holds_update(&self, request: Hold, matter_id: &str, hold_id: &str) -> MatterHoldUpdateCall<'a, S> {
        MatterHoldUpdateCall {
            hub: self.hub,
            _request: request,
            _matter_id: matter_id.to_string(),
            _hold_id: hold_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a saved query.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `matterId` - The ID of the matter to create the saved query in.
    pub fn saved_queries_create(&self, request: SavedQuery, matter_id: &str) -> MatterSavedQueryCreateCall<'a, S> {
        MatterSavedQueryCreateCall {
            hub: self.hub,
            _request: request,
            _matter_id: matter_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified saved query.
    /// 
    /// # Arguments
    ///
    /// * `matterId` - The ID of the matter to delete the saved query from.
    /// * `savedQueryId` - ID of the saved query to delete.
    pub fn saved_queries_delete(&self, matter_id: &str, saved_query_id: &str) -> MatterSavedQueryDeleteCall<'a, S> {
        MatterSavedQueryDeleteCall {
            hub: self.hub,
            _matter_id: matter_id.to_string(),
            _saved_query_id: saved_query_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified saved query.
    /// 
    /// # Arguments
    ///
    /// * `matterId` - The ID of the matter to get the saved query from.
    /// * `savedQueryId` - ID of the saved query to retrieve.
    pub fn saved_queries_get(&self, matter_id: &str, saved_query_id: &str) -> MatterSavedQueryGetCall<'a, S> {
        MatterSavedQueryGetCall {
            hub: self.hub,
            _matter_id: matter_id.to_string(),
            _saved_query_id: saved_query_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the saved queries in a matter.
    /// 
    /// # Arguments
    ///
    /// * `matterId` - The ID of the matter to get the saved queries for.
    pub fn saved_queries_list(&self, matter_id: &str) -> MatterSavedQueryListCall<'a, S> {
        MatterSavedQueryListCall {
            hub: self.hub,
            _matter_id: matter_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds an account as a matter collaborator.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `matterId` - The matter ID.
    pub fn add_permissions(&self, request: AddMatterPermissionsRequest, matter_id: &str) -> MatterAddPermissionCall<'a, S> {
        MatterAddPermissionCall {
            hub: self.hub,
            _request: request,
            _matter_id: matter_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Closes the specified matter. Returns the matter with updated state.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `matterId` - The matter ID.
    pub fn close(&self, request: CloseMatterRequest, matter_id: &str) -> MatterCloseCall<'a, S> {
        MatterCloseCall {
            hub: self.hub,
            _request: request,
            _matter_id: matter_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Counts the accounts processed by the specified query.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `matterId` - The matter ID.
    pub fn count(&self, request: CountArtifactsRequest, matter_id: &str) -> MatterCountCall<'a, S> {
        MatterCountCall {
            hub: self.hub,
            _request: request,
            _matter_id: matter_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a matter with the given name and description. The initial state is open, and the owner is the method caller. Returns the created matter with default view.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Matter) -> MatterCreateCall<'a, S> {
        MatterCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified matter. Returns the matter with updated state.
    /// 
    /// # Arguments
    ///
    /// * `matterId` - The matter ID
    pub fn delete(&self, matter_id: &str) -> MatterDeleteCall<'a, S> {
        MatterDeleteCall {
            hub: self.hub,
            _matter_id: matter_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified matter.
    /// 
    /// # Arguments
    ///
    /// * `matterId` - The matter ID.
    pub fn get(&self, matter_id: &str) -> MatterGetCall<'a, S> {
        MatterGetCall {
            hub: self.hub,
            _matter_id: matter_id.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists matters the requestor has access to.
    pub fn list(&self) -> MatterListCall<'a, S> {
        MatterListCall {
            hub: self.hub,
            _view: Default::default(),
            _state: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes an account as a matter collaborator.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `matterId` - The matter ID.
    pub fn remove_permissions(&self, request: RemoveMatterPermissionsRequest, matter_id: &str) -> MatterRemovePermissionCall<'a, S> {
        MatterRemovePermissionCall {
            hub: self.hub,
            _request: request,
            _matter_id: matter_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reopens the specified matter. Returns the matter with updated state.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `matterId` - The matter ID.
    pub fn reopen(&self, request: ReopenMatterRequest, matter_id: &str) -> MatterReopenCall<'a, S> {
        MatterReopenCall {
            hub: self.hub,
            _request: request,
            _matter_id: matter_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Undeletes the specified matter. Returns the matter with updated state.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `matterId` - The matter ID.
    pub fn undelete(&self, request: UndeleteMatterRequest, matter_id: &str) -> MatterUndeleteCall<'a, S> {
        MatterUndeleteCall {
            hub: self.hub,
            _request: request,
            _matter_id: matter_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified matter. This updates only the name and description of the matter, identified by matter ID. Changes to any other fields are ignored. Returns the default view of the matter.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `matterId` - The matter ID.
    pub fn update(&self, request: Matter, matter_id: &str) -> MatterUpdateCall<'a, S> {
        MatterUpdateCall {
            hub: self.hub,
            _request: request,
            _matter_id: matter_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`Vault`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_vault1 as vault1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use vault1::{Vault, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Vault::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel(...)`, `delete(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Vault<S>,
}

impl<'a, S> client::MethodsBuilder for OperationMethods<'a, S> {}

impl<'a, S> OperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn cancel(&self, request: CancelOperationRequest, name: &str) -> OperationCancelCall<'a, S> {
        OperationCancelCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn delete(&self, name: &str) -> OperationDeleteCall<'a, S> {
        OperationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn list(&self, name: &str) -> OperationListCall<'a, S> {
        OperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



