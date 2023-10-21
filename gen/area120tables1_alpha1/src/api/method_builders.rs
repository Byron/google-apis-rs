use super::*;
/// A builder providing access to all methods supported on *table* resources.
/// It is not used directly, but through the [`Area120Tables`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_area120tables1_alpha1 as area120tables1_alpha1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use area120tables1_alpha1::{Area120Tables, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Area120Tables::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)`, `rows_batch_create(...)`, `rows_batch_delete(...)`, `rows_batch_update(...)`, `rows_create(...)`, `rows_delete(...)`, `rows_get(...)`, `rows_list(...)` and `rows_patch(...)`
/// // to build up your call.
/// let rb = hub.tables();
/// # }
/// ```
pub struct TableMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Area120Tables<S>,
}

impl<'a, S> client::MethodsBuilder for TableMethods<'a, S> {}

impl<'a, S> TableMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates multiple rows.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent table where the rows will be created. Format: tables/{table}
    pub fn rows_batch_create(&self, request: BatchCreateRowsRequest, parent: &str) -> TableRowBatchCreateCall<'a, S> {
        TableRowBatchCreateCall {
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
    /// Deletes multiple rows.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent table shared by all rows being deleted. Format: tables/{table}
    pub fn rows_batch_delete(&self, request: BatchDeleteRowsRequest, parent: &str) -> TableRowBatchDeleteCall<'a, S> {
        TableRowBatchDeleteCall {
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
    /// Updates multiple rows.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent table shared by all rows being updated. Format: tables/{table}
    pub fn rows_batch_update(&self, request: BatchUpdateRowsRequest, parent: &str) -> TableRowBatchUpdateCall<'a, S> {
        TableRowBatchUpdateCall {
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
    /// Creates a row.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent table where this row will be created. Format: tables/{table}
    pub fn rows_create(&self, request: Row, parent: &str) -> TableRowCreateCall<'a, S> {
        TableRowCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a row.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the row to delete. Format: tables/{table}/rows/{row}
    pub fn rows_delete(&self, name: &str) -> TableRowDeleteCall<'a, S> {
        TableRowDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a row. Returns NOT_FOUND if the row does not exist in the table.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the row to retrieve. Format: tables/{table}/rows/{row}
    pub fn rows_get(&self, name: &str) -> TableRowGetCall<'a, S> {
        TableRowGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists rows in a table. Returns NOT_FOUND if the table does not exist.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent table. Format: tables/{table}
    pub fn rows_list(&self, parent: &str) -> TableRowListCall<'a, S> {
        TableRowListCall {
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
    /// Updates a row.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the row. Row names have the form `tables/{table}/rows/{row}`. The name is ignored when creating a row.
    pub fn rows_patch(&self, request: Row, name: &str) -> TableRowPatchCall<'a, S> {
        TableRowPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _view: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a table. Returns NOT_FOUND if the table does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the table to retrieve. Format: tables/{table}
    pub fn get(&self, name: &str) -> TableGetCall<'a, S> {
        TableGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists tables for the user.
    pub fn list(&self) -> TableListCall<'a, S> {
        TableListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *workspace* resources.
/// It is not used directly, but through the [`Area120Tables`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_area120tables1_alpha1 as area120tables1_alpha1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use area120tables1_alpha1::{Area120Tables, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Area120Tables::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.workspaces();
/// # }
/// ```
pub struct WorkspaceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Area120Tables<S>,
}

impl<'a, S> client::MethodsBuilder for WorkspaceMethods<'a, S> {}

impl<'a, S> WorkspaceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a workspace. Returns NOT_FOUND if the workspace does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the workspace to retrieve. Format: workspaces/{workspace}
    pub fn get(&self, name: &str) -> WorkspaceGetCall<'a, S> {
        WorkspaceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists workspaces for the user.
    pub fn list(&self) -> WorkspaceListCall<'a, S> {
        WorkspaceListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



