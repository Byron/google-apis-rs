use super::*;
/// This is a JSON template for an app state resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get states](StateGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetResponse {
    /// The current app state version.
    #[serde(rename="currentStateVersion")]
    
    pub current_state_version: Option<String>,
    /// The requested data.
    
    pub data: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string appstate#getResponse.
    
    pub kind: Option<String>,
    /// The key for the data.
    #[serde(rename="stateKey")]
    
    pub state_key: Option<i32>,
}

impl client::ResponseResult for GetResponse {}


/// This is a JSON template to convert a list-response for app state.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list states](StateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListResponse {
    /// The app state data.
    
    pub items: Option<Vec<GetResponse>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string appstate#listResponse.
    
    pub kind: Option<String>,
    /// The maximum number of keys allowed for this user.
    #[serde(rename="maximumKeyCount")]
    
    pub maximum_key_count: Option<i32>,
}

impl client::ResponseResult for ListResponse {}


/// This is a JSON template for a requests which update app state
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update states](StateUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    /// The new app state data that your application is trying to update with.
    
    pub data: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string appstate#updateRequest.
    
    pub kind: Option<String>,
}

impl client::RequestValue for UpdateRequest {}


/// This is a JSON template for an app state write result.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [clear states](StateClearCall) (response)
/// * [update states](StateUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WriteResult {
    /// The version of the data for this key on the server.
    #[serde(rename="currentStateVersion")]
    
    pub current_state_version: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string appstate#writeResult.
    
    pub kind: Option<String>,
    /// The written key.
    #[serde(rename="stateKey")]
    
    pub state_key: Option<i32>,
}

impl client::ResponseResult for WriteResult {}


