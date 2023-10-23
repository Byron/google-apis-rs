use super::*;
/// Representation of a Realtime Database instance. Details on interacting with contents of a DatabaseInstance can be found at: https://firebase.google.com/docs/database/rest/start.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances create projects](ProjectLocationInstanceCreateCall) (request|response)
/// * [locations instances delete projects](ProjectLocationInstanceDeleteCall) (response)
/// * [locations instances disable projects](ProjectLocationInstanceDisableCall) (response)
/// * [locations instances get projects](ProjectLocationInstanceGetCall) (response)
/// * [locations instances reenable projects](ProjectLocationInstanceReenableCall) (response)
/// * [locations instances undelete projects](ProjectLocationInstanceUndeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseInstance {
    /// Output only. Output Only. The globally unique hostname of the database.
    #[serde(rename="databaseUrl")]
    
    pub database_url: Option<String>,
    /// The fully qualified resource name of the database instance, in the form: `projects/{project-number}/locations/{location-id}/instances/{database-id}`.
    
    pub name: Option<String>,
    /// Output only. The resource name of the project this instance belongs to. For example: `projects/{project-number}`.
    
    pub project: Option<String>,
    /// Output only. The database's lifecycle state. Read-only.
    
    pub state: Option<DatabaseInstanceStateEnum>,
    /// Immutable. The database instance type. On creation only USER_DATABASE is allowed, which is also the default when omitted.
    #[serde(rename="type")]
    
    pub type_: Option<DatabaseInstanceTypeEnum>,
}

impl client::RequestValue for DatabaseInstance {}
impl client::ResponseResult for DatabaseInstance {}


/// The request sent to the DisableDatabaseInstance method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances disable projects](ProjectLocationInstanceDisableCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DisableDatabaseInstanceRequest { _never_set: Option<bool> }

impl client::RequestValue for DisableDatabaseInstanceRequest {}


/// The response from the ListDatabaseInstances method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances list projects](ProjectLocationInstanceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDatabaseInstancesResponse {
    /// List of each DatabaseInstance that is in the parent Firebase project.
    
    pub instances: Option<Vec<DatabaseInstance>>,
    /// If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results. This token can be used in a subsequent call to `ListDatabaseInstances` to find the next group of database instances. Page tokens are short-lived and should not be persisted.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDatabaseInstancesResponse {}


/// The request sent to the ReenableDatabaseInstance method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances reenable projects](ProjectLocationInstanceReenableCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReenableDatabaseInstanceRequest { _never_set: Option<bool> }

impl client::RequestValue for ReenableDatabaseInstanceRequest {}


/// The request sent to UndeleteDatabaseInstance method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances undelete projects](ProjectLocationInstanceUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteDatabaseInstanceRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteDatabaseInstanceRequest {}


