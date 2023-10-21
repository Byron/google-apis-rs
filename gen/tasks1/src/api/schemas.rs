use super::*;
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clear tasks](TaskClearCall) (none)
/// * [delete tasks](TaskDeleteCall) (none)
/// * [get tasks](TaskGetCall) (response)
/// * [insert tasks](TaskInsertCall) (request|response)
/// * [list tasks](TaskListCall) (none)
/// * [move tasks](TaskMoveCall) (response)
/// * [patch tasks](TaskPatchCall) (request|response)
/// * [update tasks](TaskUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    /// Completion date of the task (as a RFC 3339 timestamp). This field is omitted if the task has not been completed.
    
    pub completed: Option<String>,
    /// Flag indicating whether the task has been deleted. The default is False.
    
    pub deleted: Option<bool>,
    /// Due date of the task (as a RFC 3339 timestamp). Optional. The due date only records date information; the time portion of the timestamp is discarded when setting the due date. It isn't possible to read or write the time that a task is due via the API.
    
    pub due: Option<String>,
    /// ETag of the resource.
    
    pub etag: Option<String>,
    /// Flag indicating whether the task is hidden. This is the case if the task had been marked completed when the task list was last cleared. The default is False. This field is read-only.
    
    pub hidden: Option<bool>,
    /// Task identifier.
    
    pub id: Option<String>,
    /// Type of the resource. This is always "tasks#task".
    
    pub kind: Option<String>,
    /// Collection of links. This collection is read-only.
    
    pub links: Option<Vec<TaskLinks>>,
    /// Notes describing the task. Optional.
    
    pub notes: Option<String>,
    /// Parent task identifier. This field is omitted if it is a top-level task. This field is read-only. Use the "move" method to move the task under a different parent or to the top level.
    
    pub parent: Option<String>,
    /// String indicating the position of the task among its sibling tasks under the same parent task or at the top level. If this string is greater than another task's corresponding position string according to lexicographical ordering, the task is positioned after the other task under the same parent task (or at the top level). This field is read-only. Use the "move" method to move the task to another position.
    
    pub position: Option<String>,
    /// URL pointing to this task. Used to retrieve, update, or delete this task.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Status of the task. This is either "needsAction" or "completed".
    
    pub status: Option<String>,
    /// Title of the task.
    
    pub title: Option<String>,
    /// Last modification time of the task (as a RFC 3339 timestamp).
    
    pub updated: Option<String>,
}

impl client::RequestValue for Task {}
impl client::Resource for Task {}
impl client::ResponseResult for Task {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get tasklists](TasklistGetCall) (response)
/// * [insert tasklists](TasklistInsertCall) (request|response)
/// * [patch tasklists](TasklistPatchCall) (request|response)
/// * [update tasklists](TasklistUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaskList {
    /// ETag of the resource.
    
    pub etag: Option<String>,
    /// Task list identifier.
    
    pub id: Option<String>,
    /// Type of the resource. This is always "tasks#taskList".
    
    pub kind: Option<String>,
    /// URL pointing to this task list. Used to retrieve, update, or delete this task list.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Title of the task list.
    
    pub title: Option<String>,
    /// Last modification time of the task list (as a RFC 3339 timestamp).
    
    pub updated: Option<String>,
}

impl client::RequestValue for TaskList {}
impl client::Resource for TaskList {}
impl client::ResponseResult for TaskList {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list tasklists](TasklistListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaskLists {
    /// ETag of the resource.
    
    pub etag: Option<String>,
    /// Collection of task lists.
    
    pub items: Option<Vec<TaskList>>,
    /// Type of the resource. This is always "tasks#taskLists".
    
    pub kind: Option<String>,
    /// Token that can be used to request the next page of this result.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for TaskLists {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list tasks](TaskListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Tasks {
    /// ETag of the resource.
    
    pub etag: Option<String>,
    /// Collection of tasks.
    
    pub items: Option<Vec<Task>>,
    /// Type of the resource. This is always "tasks#tasks".
    
    pub kind: Option<String>,
    /// Token used to access the next page of this result.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for Tasks {}


/// Collection of links. This collection is read-only.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaskLinks {
    /// The description. In HTML speak: Everything between <a> and </a>.
    
    pub description: Option<String>,
    /// The URL.
    
    pub link: Option<String>,
    /// Type of the link, e.g. "email".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for TaskLinks {}
impl client::Part for TaskLinks {}


