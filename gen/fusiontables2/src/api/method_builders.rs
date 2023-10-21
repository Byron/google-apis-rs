use super::*;
/// A builder providing access to all methods supported on *column* resources.
/// It is not used directly, but through the [`Fusiontables`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_fusiontables2 as fusiontables2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use fusiontables2::{Fusiontables, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Fusiontables::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.column();
/// # }
/// ```
pub struct ColumnMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Fusiontables<S>,
}

impl<'a, S> client::MethodsBuilder for ColumnMethods<'a, S> {}

impl<'a, S> ColumnMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified column.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - Table from which the column is being deleted.
    /// * `columnId` - Name or identifier for the column being deleted.
    pub fn delete(&self, table_id: &str, column_id: &str) -> ColumnDeleteCall<'a, S> {
        ColumnDeleteCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _column_id: column_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a specific column by its ID.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - Table to which the column belongs.
    /// * `columnId` - Name or identifier for the column that is being requested.
    pub fn get(&self, table_id: &str, column_id: &str) -> ColumnGetCall<'a, S> {
        ColumnGetCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _column_id: column_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a new column to the table.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tableId` - Table for which a new column is being added.
    pub fn insert(&self, request: Column, table_id: &str) -> ColumnInsertCall<'a, S> {
        ColumnInsertCall {
            hub: self.hub,
            _request: request,
            _table_id: table_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of columns.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - Table whose columns are being listed.
    pub fn list(&self, table_id: &str) -> ColumnListCall<'a, S> {
        ColumnListCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the name or type of an existing column. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tableId` - Table for which the column is being updated.
    /// * `columnId` - Name or identifier for the column that is being updated.
    pub fn patch(&self, request: Column, table_id: &str, column_id: &str) -> ColumnPatchCall<'a, S> {
        ColumnPatchCall {
            hub: self.hub,
            _request: request,
            _table_id: table_id.to_string(),
            _column_id: column_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the name or type of an existing column.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tableId` - Table for which the column is being updated.
    /// * `columnId` - Name or identifier for the column that is being updated.
    pub fn update(&self, request: Column, table_id: &str, column_id: &str) -> ColumnUpdateCall<'a, S> {
        ColumnUpdateCall {
            hub: self.hub,
            _request: request,
            _table_id: table_id.to_string(),
            _column_id: column_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *query* resources.
/// It is not used directly, but through the [`Fusiontables`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_fusiontables2 as fusiontables2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use fusiontables2::{Fusiontables, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Fusiontables::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `sql(...)` and `sql_get(...)`
/// // to build up your call.
/// let rb = hub.query();
/// # }
/// ```
pub struct QueryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Fusiontables<S>,
}

impl<'a, S> client::MethodsBuilder for QueryMethods<'a, S> {}

impl<'a, S> QueryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Executes a Fusion Tables SQL statement, which can be any of 
    /// - SELECT
    /// - INSERT
    /// - UPDATE
    /// - DELETE
    /// - SHOW
    /// - DESCRIBE
    /// - CREATE statement.
    /// 
    /// # Arguments
    ///
    /// * `sql` - A Fusion Tables SQL statement, which can be any of 
    ///           - SELECT
    ///           - INSERT
    ///           - UPDATE
    ///           - DELETE
    ///           - SHOW
    ///           - DESCRIBE
    ///           - CREATE
    pub fn sql(&self, sql: &str) -> QuerySqlCall<'a, S> {
        QuerySqlCall {
            hub: self.hub,
            _sql: sql.to_string(),
            _typed: Default::default(),
            _hdrs: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Executes a SQL statement which can be any of 
    /// - SELECT
    /// - SHOW
    /// - DESCRIBE
    /// 
    /// # Arguments
    ///
    /// * `sql` - A SQL statement which can be any of 
    ///           - SELECT
    ///           - SHOW
    ///           - DESCRIBE
    pub fn sql_get(&self, sql: &str) -> QuerySqlGetCall<'a, S> {
        QuerySqlGetCall {
            hub: self.hub,
            _sql: sql.to_string(),
            _typed: Default::default(),
            _hdrs: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *style* resources.
/// It is not used directly, but through the [`Fusiontables`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_fusiontables2 as fusiontables2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use fusiontables2::{Fusiontables, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Fusiontables::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.style();
/// # }
/// ```
pub struct StyleMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Fusiontables<S>,
}

impl<'a, S> client::MethodsBuilder for StyleMethods<'a, S> {}

impl<'a, S> StyleMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a style.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - Table from which the style is being deleted
    /// * `styleId` - Identifier (within a table) for the style being deleted
    pub fn delete(&self, table_id: &str, style_id: i32) -> StyleDeleteCall<'a, S> {
        StyleDeleteCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _style_id: style_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a specific style.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - Table to which the requested style belongs
    /// * `styleId` - Identifier (integer) for a specific style in a table
    pub fn get(&self, table_id: &str, style_id: i32) -> StyleGetCall<'a, S> {
        StyleGetCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _style_id: style_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a new style for the table.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tableId` - Table for which a new style is being added
    pub fn insert(&self, request: StyleSetting, table_id: &str) -> StyleInsertCall<'a, S> {
        StyleInsertCall {
            hub: self.hub,
            _request: request,
            _table_id: table_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of styles.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - Table whose styles are being listed
    pub fn list(&self, table_id: &str) -> StyleListCall<'a, S> {
        StyleListCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing style. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tableId` - Table whose style is being updated.
    /// * `styleId` - Identifier (within a table) for the style being updated.
    pub fn patch(&self, request: StyleSetting, table_id: &str, style_id: i32) -> StylePatchCall<'a, S> {
        StylePatchCall {
            hub: self.hub,
            _request: request,
            _table_id: table_id.to_string(),
            _style_id: style_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing style.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tableId` - Table whose style is being updated.
    /// * `styleId` - Identifier (within a table) for the style being updated.
    pub fn update(&self, request: StyleSetting, table_id: &str, style_id: i32) -> StyleUpdateCall<'a, S> {
        StyleUpdateCall {
            hub: self.hub,
            _request: request,
            _table_id: table_id.to_string(),
            _style_id: style_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *table* resources.
/// It is not used directly, but through the [`Fusiontables`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_fusiontables2 as fusiontables2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use fusiontables2::{Fusiontables, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Fusiontables::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `copy(...)`, `delete(...)`, `get(...)`, `import_rows(...)`, `import_table(...)`, `insert(...)`, `list(...)`, `patch(...)`, `refetch_sheet(...)`, `replace_rows(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.table();
/// # }
/// ```
pub struct TableMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Fusiontables<S>,
}

impl<'a, S> client::MethodsBuilder for TableMethods<'a, S> {}

impl<'a, S> TableMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Copies a table.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - ID of the table that is being copied.
    pub fn copy(&self, table_id: &str) -> TableCopyCall<'a, S> {
        TableCopyCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _copy_presentation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a table.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - ID of the table to be deleted.
    pub fn delete(&self, table_id: &str) -> TableDeleteCall<'a, S> {
        TableDeleteCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a specific table by its ID.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - Identifier for the table being requested.
    pub fn get(&self, table_id: &str) -> TableGetCall<'a, S> {
        TableGetCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Imports more rows into a table.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - The table into which new rows are being imported.
    pub fn import_rows(&self, table_id: &str) -> TableImportRowCall<'a, S> {
        TableImportRowCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _start_line: Default::default(),
            _is_strict: Default::default(),
            _end_line: Default::default(),
            _encoding: Default::default(),
            _delimiter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Imports a new table.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name to be assigned to the new table.
    pub fn import_table(&self, name: &str) -> TableImportTableCall<'a, S> {
        TableImportTableCall {
            hub: self.hub,
            _name: name.to_string(),
            _encoding: Default::default(),
            _delimiter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new table.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: Table) -> TableInsertCall<'a, S> {
        TableInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of tables a user owns.
    pub fn list(&self) -> TableListCall<'a, S> {
        TableListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing table. Unless explicitly requested, only the name, description, and attribution will be updated. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tableId` - ID of the table that is being updated.
    pub fn patch(&self, request: Table, table_id: &str) -> TablePatchCall<'a, S> {
        TablePatchCall {
            hub: self.hub,
            _request: request,
            _table_id: table_id.to_string(),
            _replace_view_definition: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replaces rows of the table with the rows of the spreadsheet that is first imported from. Current rows remain visible until all replacement rows are ready.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - Table whose rows will be replaced from the spreadsheet.
    pub fn refetch_sheet(&self, table_id: &str) -> TableRefetchSheetCall<'a, S> {
        TableRefetchSheetCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replaces rows of an existing table. Current rows remain visible until all replacement rows are ready.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - Table whose rows will be replaced.
    pub fn replace_rows(&self, table_id: &str) -> TableReplaceRowCall<'a, S> {
        TableReplaceRowCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _start_line: Default::default(),
            _is_strict: Default::default(),
            _end_line: Default::default(),
            _encoding: Default::default(),
            _delimiter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing table. Unless explicitly requested, only the name, description, and attribution will be updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tableId` - ID of the table that is being updated.
    pub fn update(&self, request: Table, table_id: &str) -> TableUpdateCall<'a, S> {
        TableUpdateCall {
            hub: self.hub,
            _request: request,
            _table_id: table_id.to_string(),
            _replace_view_definition: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *task* resources.
/// It is not used directly, but through the [`Fusiontables`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_fusiontables2 as fusiontables2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use fusiontables2::{Fusiontables, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Fusiontables::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.task();
/// # }
/// ```
pub struct TaskMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Fusiontables<S>,
}

impl<'a, S> client::MethodsBuilder for TaskMethods<'a, S> {}

impl<'a, S> TaskMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a specific task by its ID, unless that task has already started running.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - Table from which the task is being deleted.
    /// * `taskId` - The identifier of the task to delete.
    pub fn delete(&self, table_id: &str, task_id: &str) -> TaskDeleteCall<'a, S> {
        TaskDeleteCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _task_id: task_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a specific task by its ID.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - Table to which the task belongs.
    /// * `taskId` - The identifier of the task to get.
    pub fn get(&self, table_id: &str, task_id: &str) -> TaskGetCall<'a, S> {
        TaskGetCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _task_id: task_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of tasks.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - Table whose tasks are being listed.
    pub fn list(&self, table_id: &str) -> TaskListCall<'a, S> {
        TaskListCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _start_index: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *template* resources.
/// It is not used directly, but through the [`Fusiontables`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_fusiontables2 as fusiontables2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use fusiontables2::{Fusiontables, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Fusiontables::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.template();
/// # }
/// ```
pub struct TemplateMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Fusiontables<S>,
}

impl<'a, S> client::MethodsBuilder for TemplateMethods<'a, S> {}

impl<'a, S> TemplateMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a template
    /// 
    /// # Arguments
    ///
    /// * `tableId` - Table from which the template is being deleted
    /// * `templateId` - Identifier for the template which is being deleted
    pub fn delete(&self, table_id: &str, template_id: i32) -> TemplateDeleteCall<'a, S> {
        TemplateDeleteCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _template_id: template_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a specific template by its id
    /// 
    /// # Arguments
    ///
    /// * `tableId` - Table to which the template belongs
    /// * `templateId` - Identifier for the template that is being requested
    pub fn get(&self, table_id: &str, template_id: i32) -> TemplateGetCall<'a, S> {
        TemplateGetCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _template_id: template_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new template for the table.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tableId` - Table for which a new template is being created
    pub fn insert(&self, request: Template, table_id: &str) -> TemplateInsertCall<'a, S> {
        TemplateInsertCall {
            hub: self.hub,
            _request: request,
            _table_id: table_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of templates.
    /// 
    /// # Arguments
    ///
    /// * `tableId` - Identifier for the table whose templates are being requested
    pub fn list(&self, table_id: &str) -> TemplateListCall<'a, S> {
        TemplateListCall {
            hub: self.hub,
            _table_id: table_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing template. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tableId` - Table to which the updated template belongs
    /// * `templateId` - Identifier for the template that is being updated
    pub fn patch(&self, request: Template, table_id: &str, template_id: i32) -> TemplatePatchCall<'a, S> {
        TemplatePatchCall {
            hub: self.hub,
            _request: request,
            _table_id: table_id.to_string(),
            _template_id: template_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing template
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tableId` - Table to which the updated template belongs
    /// * `templateId` - Identifier for the template that is being updated
    pub fn update(&self, request: Template, table_id: &str, template_id: i32) -> TemplateUpdateCall<'a, S> {
        TemplateUpdateCall {
            hub: self.hub,
            _request: request,
            _table_id: table_id.to_string(),
            _template_id: template_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



