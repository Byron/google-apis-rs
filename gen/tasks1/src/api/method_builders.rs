use super::*;
/// A builder providing access to all methods supported on *tasklist* resources.
/// It is not used directly, but through the [`TasksHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_tasks1 as tasks1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use tasks1::{TasksHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = TasksHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.tasklists();
/// # }
/// ```
pub struct TasklistMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a TasksHub<S>,
}

impl<'a, S> client::MethodsBuilder for TasklistMethods<'a, S> {}

impl<'a, S> TasklistMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the authenticated user's specified task list.
    /// 
    /// # Arguments
    ///
    /// * `tasklist` - Task list identifier.
    pub fn delete(&self, tasklist: &str) -> TasklistDeleteCall<'a, S> {
        TasklistDeleteCall {
            hub: self.hub,
            _tasklist: tasklist.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the authenticated user's specified task list.
    /// 
    /// # Arguments
    ///
    /// * `tasklist` - Task list identifier.
    pub fn get(&self, tasklist: &str) -> TasklistGetCall<'a, S> {
        TasklistGetCall {
            hub: self.hub,
            _tasklist: tasklist.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new task list and adds it to the authenticated user's task lists.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: TaskList) -> TasklistInsertCall<'a, S> {
        TasklistInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns all the authenticated user's task lists.
    pub fn list(&self) -> TasklistListCall<'a, S> {
        TasklistListCall {
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
    /// Updates the authenticated user's specified task list. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tasklist` - Task list identifier.
    pub fn patch(&self, request: TaskList, tasklist: &str) -> TasklistPatchCall<'a, S> {
        TasklistPatchCall {
            hub: self.hub,
            _request: request,
            _tasklist: tasklist.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the authenticated user's specified task list.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tasklist` - Task list identifier.
    pub fn update(&self, request: TaskList, tasklist: &str) -> TasklistUpdateCall<'a, S> {
        TasklistUpdateCall {
            hub: self.hub,
            _request: request,
            _tasklist: tasklist.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *task* resources.
/// It is not used directly, but through the [`TasksHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_tasks1 as tasks1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use tasks1::{TasksHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = TasksHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `clear(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `move_(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.tasks();
/// # }
/// ```
pub struct TaskMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a TasksHub<S>,
}

impl<'a, S> client::MethodsBuilder for TaskMethods<'a, S> {}

impl<'a, S> TaskMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears all completed tasks from the specified task list. The affected tasks will be marked as 'hidden' and no longer be returned by default when retrieving all tasks for a task list.
    /// 
    /// # Arguments
    ///
    /// * `tasklist` - Task list identifier.
    pub fn clear(&self, tasklist: &str) -> TaskClearCall<'a, S> {
        TaskClearCall {
            hub: self.hub,
            _tasklist: tasklist.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified task from the task list.
    /// 
    /// # Arguments
    ///
    /// * `tasklist` - Task list identifier.
    /// * `task` - Task identifier.
    pub fn delete(&self, tasklist: &str, task: &str) -> TaskDeleteCall<'a, S> {
        TaskDeleteCall {
            hub: self.hub,
            _tasklist: tasklist.to_string(),
            _task: task.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified task.
    /// 
    /// # Arguments
    ///
    /// * `tasklist` - Task list identifier.
    /// * `task` - Task identifier.
    pub fn get(&self, tasklist: &str, task: &str) -> TaskGetCall<'a, S> {
        TaskGetCall {
            hub: self.hub,
            _tasklist: tasklist.to_string(),
            _task: task.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new task on the specified task list.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tasklist` - Task list identifier.
    pub fn insert(&self, request: Task, tasklist: &str) -> TaskInsertCall<'a, S> {
        TaskInsertCall {
            hub: self.hub,
            _request: request,
            _tasklist: tasklist.to_string(),
            _previous: Default::default(),
            _parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns all tasks in the specified task list.
    /// 
    /// # Arguments
    ///
    /// * `tasklist` - Task list identifier.
    pub fn list(&self, tasklist: &str) -> TaskListCall<'a, S> {
        TaskListCall {
            hub: self.hub,
            _tasklist: tasklist.to_string(),
            _updated_min: Default::default(),
            _show_hidden: Default::default(),
            _show_deleted: Default::default(),
            _show_completed: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _due_min: Default::default(),
            _due_max: Default::default(),
            _completed_min: Default::default(),
            _completed_max: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves the specified task to another position in the task list. This can include putting it as a child task under a new parent and/or move it to a different position among its sibling tasks.
    /// 
    /// # Arguments
    ///
    /// * `tasklist` - Task list identifier.
    /// * `task` - Task identifier.
    pub fn move_(&self, tasklist: &str, task: &str) -> TaskMoveCall<'a, S> {
        TaskMoveCall {
            hub: self.hub,
            _tasklist: tasklist.to_string(),
            _task: task.to_string(),
            _previous: Default::default(),
            _parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified task. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tasklist` - Task list identifier.
    /// * `task` - Task identifier.
    pub fn patch(&self, request: Task, tasklist: &str, task: &str) -> TaskPatchCall<'a, S> {
        TaskPatchCall {
            hub: self.hub,
            _request: request,
            _tasklist: tasklist.to_string(),
            _task: task.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified task.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `tasklist` - Task list identifier.
    /// * `task` - Task identifier.
    pub fn update(&self, request: Task, tasklist: &str, task: &str) -> TaskUpdateCall<'a, S> {
        TaskUpdateCall {
            hub: self.hub,
            _request: request,
            _tasklist: tasklist.to_string(),
            _task: task.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



