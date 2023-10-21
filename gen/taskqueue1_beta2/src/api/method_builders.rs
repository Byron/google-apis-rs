use super::*;
/// A builder providing access to all methods supported on *taskqueue* resources.
/// It is not used directly, but through the [`Taskqueue`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_taskqueue1_beta2 as taskqueue1_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use taskqueue1_beta2::{Taskqueue, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Taskqueue::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.taskqueues();
/// # }
/// ```
pub struct TaskqueueMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Taskqueue<S>,
}

impl<'a, S> client::MethodsBuilder for TaskqueueMethods<'a, S> {}

impl<'a, S> TaskqueueMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get detailed information about a TaskQueue.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project under which the queue lies.
    /// * `taskqueue` - The id of the taskqueue to get the properties of.
    pub fn get(&self, project: &str, taskqueue: &str) -> TaskqueueGetCall<'a, S> {
        TaskqueueGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _taskqueue: taskqueue.to_string(),
            _get_stats: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *task* resources.
/// It is not used directly, but through the [`Taskqueue`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_taskqueue1_beta2 as taskqueue1_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use taskqueue1_beta2::{Taskqueue, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Taskqueue::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `lease(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.tasks();
/// # }
/// ```
pub struct TaskMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Taskqueue<S>,
}

impl<'a, S> client::MethodsBuilder for TaskMethods<'a, S> {}

impl<'a, S> TaskMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a task from a TaskQueue.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project under which the queue lies.
    /// * `taskqueue` - The taskqueue to delete a task from.
    /// * `task` - The id of the task to delete.
    pub fn delete(&self, project: &str, taskqueue: &str, task: &str) -> TaskDeleteCall<'a, S> {
        TaskDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _taskqueue: taskqueue.to_string(),
            _task: task.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a particular task from a TaskQueue.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project under which the queue lies.
    /// * `taskqueue` - The taskqueue in which the task belongs.
    /// * `task` - The task to get properties of.
    pub fn get(&self, project: &str, taskqueue: &str, task: &str) -> TaskGetCall<'a, S> {
        TaskGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _taskqueue: taskqueue.to_string(),
            _task: task.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Insert a new task in a TaskQueue
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project under which the queue lies
    /// * `taskqueue` - The taskqueue to insert the task into
    pub fn insert(&self, request: Task, project: &str, taskqueue: &str) -> TaskInsertCall<'a, S> {
        TaskInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _taskqueue: taskqueue.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lease 1 or more tasks from a TaskQueue.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project under which the queue lies.
    /// * `taskqueue` - The taskqueue to lease a task from.
    /// * `numTasks` - The number of tasks to lease.
    /// * `leaseSecs` - The lease in seconds.
    pub fn lease(&self, project: &str, taskqueue: &str, num_tasks: i32, lease_secs: i32) -> TaskLeaseCall<'a, S> {
        TaskLeaseCall {
            hub: self.hub,
            _project: project.to_string(),
            _taskqueue: taskqueue.to_string(),
            _num_tasks: num_tasks,
            _lease_secs: lease_secs,
            _tag: Default::default(),
            _group_by_tag: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List Tasks in a TaskQueue
    /// 
    /// # Arguments
    ///
    /// * `project` - The project under which the queue lies.
    /// * `taskqueue` - The id of the taskqueue to list tasks from.
    pub fn list(&self, project: &str, taskqueue: &str) -> TaskListCall<'a, S> {
        TaskListCall {
            hub: self.hub,
            _project: project.to_string(),
            _taskqueue: taskqueue.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update tasks that are leased out of a TaskQueue. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project under which the queue lies.
    /// * `taskqueue` - No description provided.
    /// * `task` - No description provided.
    /// * `newLeaseSeconds` - The new lease in seconds.
    pub fn patch(&self, request: Task, project: &str, taskqueue: &str, task: &str, new_lease_seconds: i32) -> TaskPatchCall<'a, S> {
        TaskPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _taskqueue: taskqueue.to_string(),
            _task: task.to_string(),
            _new_lease_seconds: new_lease_seconds,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update tasks that are leased out of a TaskQueue.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project under which the queue lies.
    /// * `taskqueue` - No description provided.
    /// * `task` - No description provided.
    /// * `newLeaseSeconds` - The new lease in seconds.
    pub fn update(&self, request: Task, project: &str, taskqueue: &str, task: &str, new_lease_seconds: i32) -> TaskUpdateCall<'a, S> {
        TaskUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _taskqueue: taskqueue.to_string(),
            _task: task.to_string(),
            _new_lease_seconds: new_lease_seconds,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



