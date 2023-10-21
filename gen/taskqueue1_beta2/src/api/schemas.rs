use super::*;
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete tasks](TaskDeleteCall) (none)
/// * [get tasks](TaskGetCall) (response)
/// * [insert tasks](TaskInsertCall) (request|response)
/// * [lease tasks](TaskLeaseCall) (none)
/// * [list tasks](TaskListCall) (none)
/// * [patch tasks](TaskPatchCall) (request|response)
/// * [update tasks](TaskUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    /// Time (in seconds since the epoch) at which the task was enqueued.
    #[serde(rename="enqueueTimestamp")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub enqueue_timestamp: Option<i64>,
    /// Name of the task.
    
    pub id: Option<String>,
    /// The kind of object returned, in this case set to task.
    
    pub kind: Option<String>,
    /// Time (in seconds since the epoch) at which the task lease will expire. This value is 0 if the task isnt currently leased out to a worker.
    #[serde(rename="leaseTimestamp")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub lease_timestamp: Option<i64>,
    /// A bag of bytes which is the task payload. The payload on the JSON side is always Base64 encoded.
    #[serde(rename="payloadBase64")]
    
    pub payload_base64: Option<String>,
    /// Name of the queue that the task is in.
    #[serde(rename="queueName")]
    
    pub queue_name: Option<String>,
    /// The number of leases applied to this task.
    
    pub retry_count: Option<i32>,
    /// Tag for the task, could be used later to lease tasks grouped by a specific tag.
    
    pub tag: Option<String>,
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
/// * [get taskqueues](TaskqueueGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaskQueue {
    /// ACLs that are applicable to this TaskQueue object.
    
    pub acl: Option<TaskQueueAcl>,
    /// Name of the taskqueue.
    
    pub id: Option<String>,
    /// The kind of REST object returned, in this case taskqueue.
    
    pub kind: Option<String>,
    /// The number of times we should lease out tasks before giving up on them. If unset we lease them out forever until a worker deletes the task.
    #[serde(rename="maxLeases")]
    
    pub max_leases: Option<i32>,
    /// Statistics for the TaskQueue object in question.
    
    pub stats: Option<TaskQueueStats>,
}

impl client::Resource for TaskQueue {}
impl client::ResponseResult for TaskQueue {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [lease tasks](TaskLeaseCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Tasks {
    /// The actual list of tasks returned as a result of the lease operation.
    
    pub items: Option<Vec<Task>>,
    /// The kind of object returned, a list of tasks.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for Tasks {}


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
pub struct Tasks2 {
    /// The actual list of tasks currently active in the TaskQueue.
    
    pub items: Option<Vec<Task>>,
    /// The kind of object returned, a list of tasks.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for Tasks2 {}


/// ACLs that are applicable to this TaskQueue object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaskQueueAcl {
    /// Email addresses of users who are "admins" of the TaskQueue. This means they can control the queue, eg set ACLs for the queue.
    #[serde(rename="adminEmails")]
    
    pub admin_emails: Option<Vec<String>>,
    /// Email addresses of users who can "consume" tasks from the TaskQueue. This means they can Dequeue and Delete tasks from the queue.
    #[serde(rename="consumerEmails")]
    
    pub consumer_emails: Option<Vec<String>>,
    /// Email addresses of users who can "produce" tasks into the TaskQueue. This means they can Insert tasks into the queue.
    #[serde(rename="producerEmails")]
    
    pub producer_emails: Option<Vec<String>>,
}

impl client::NestedType for TaskQueueAcl {}
impl client::Part for TaskQueueAcl {}


/// Statistics for the TaskQueue object in question.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaskQueueStats {
    /// Number of tasks leased in the last hour.
    #[serde(rename="leasedLastHour")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub leased_last_hour: Option<i64>,
    /// Number of tasks leased in the last minute.
    #[serde(rename="leasedLastMinute")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub leased_last_minute: Option<i64>,
    /// The timestamp (in seconds since the epoch) of the oldest unfinished task.
    #[serde(rename="oldestTask")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub oldest_task: Option<i64>,
    /// Number of tasks in the queue.
    #[serde(rename="totalTasks")]
    
    pub total_tasks: Option<i32>,
}

impl client::NestedType for TaskQueueStats {}
impl client::Part for TaskQueueStats {}


